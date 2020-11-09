use std::iter::Peekable;
use std::str::CharIndices;

const DIGITS: &'static str = "0123456789";
const UPPERCASE_ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LOWERCASE_HEX: &'static str = "0123456789abcdef";
const UPPERCASE_HEX: &'static str = "0123456789ABCDEF";

/// Transforms a &str into a sequence of Tokens
pub struct Lexer<'src> {
    chars: Peekable<CharIndices<'src>>,
    src: &'src str,
}

impl<'src> Lexer<'src> {
    /// Create a new Lexer from a &str
    pub fn new(src: &'src str) -> Self {
        Self {
            chars: src.char_indices().peekable(),
            src,
        }
    }
    /// Convenience method for directly allocating into a Vec
    pub fn parse(self) -> Result<Vec<Tok<'src>>, LexErr> {
        self.into_iter().collect()
    }
    /// Transforms escaped characters into their specific meanings:
    /// - "\\" is a literal backslash
    /// - "\(" is a literal opening parenthesis
    /// - "\)" is a literal closing parenthesis
    /// - "\[" is a literal opening bracket
    /// - "\]" is a literal closing bracket
    /// - "\{" is a literal opening curly brace
    /// - "\}" is a literal closing curly brace
    /// - "\A" is a character class of the uppercase letters
    /// - "\a" is a character class of the lowercase letters
    /// - "\d" is a character class of the digits in base 10
    /// - "\h" is a character class of the digits in base 16 (lowercase)
    /// - "\H" is a character class of the digits in base 16 (uppercase)
    fn backslash(&mut self, i: usize) -> Result<Tok<'src>, LexErr> {
        match self.chars.next() {
            Some((_, 'd')) => Ok(Tok::Class(DIGITS)),
            Some((_, 'A')) => Ok(Tok::Class(UPPERCASE_ALPHABET)),
            Some((_, 'a')) => Ok(Tok::Class(LOWERCASE_ALPHABET)),
            Some((_, 'H')) => Ok(Tok::Class(UPPERCASE_HEX)),
            Some((_, 'h')) => Ok(Tok::Class(LOWERCASE_HEX)),
            Some((_, '\\')) => Ok(Tok::Char('\\')),
            Some((_, '(')) => Ok(Tok::Char('(')),
            Some((_, ')')) => Ok(Tok::Char(')')),
            Some((_, '[')) => Ok(Tok::Char('[')),
            Some((_, ']')) => Ok(Tok::Char(']')),
            Some((_, '{')) => Ok(Tok::Char('{')),
            Some((_, '}')) => Ok(Tok::Char('}')),
            Some((i, c)) if c.is_digit(10) => Ok(Tok::BackRef(self.number(i))),
            Some(_) => Err(LexErr {
                msg: "backslashes must be followed by a '\\', 'd', 'A', 'a', 'H', or 'h'",
                offset: i,
            }),
            None => Err(LexErr {
                msg: "unexpected end of input",
                offset: i,
            }),
        }
    }
    /// Cautiously consumes digits from self.chars (e.g. given "123A", the 'A' remains)
    /// and parses as a usize
    fn number(&mut self, i: usize) -> usize {
        let mut end = i;
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_digit(10) {
                self.chars.next();
                end += 1;
            } else {
                break;
            }
        }
        self.src[i..end + 1]
            .parse::<usize>()
            .expect("src[i..end + 1] should only contain ASCII digits")
    }
    /// Returns a range or repeat token
    fn left_curly(&mut self, i: usize) -> Result<Tok<'src>, LexErr> {
        if let Some((j, c)) = self.chars.next() {
            if c.is_digit(10) {
                // j is starting index of the lower bound
                let lower = self.number(j);
                // check for comma after first number
                if let Some(&(_, ',')) = self.chars.peek() {
                    self.chars.next();
                    if let Some((k, c)) = self.chars.next() {
                        if c.is_digit(10) {
                            // k is starting index of the upper bound
                            let upper = self.number(k);
                            if let Some(&(_, '}')) = self.chars.peek() {
                                self.chars.next();
                                // lower must be less than upper to be a valid range
                                if lower < upper {
                                    Ok(Tok::Range(lower, upper))
                                } else if lower == upper {
                                    Err(LexErr { msg: "bounds cannot be equal in a range; consider using the repeat syntax", offset: i})
                                } else {
                                    Err(LexErr {
                                        msg: "lower bound must be less than upper bound",
                                        offset: i,
                                    })
                                }
                            } else {
                                Err(LexErr {
                                    msg: "expected closing '}' for range",
                                    offset: i,
                                })
                            }
                        } else {
                            Err(LexErr {
                                msg: "ranges must be provided 2 numbers",
                                offset: i,
                            })
                        }
                    } else {
                        Err(LexErr {
                            msg: "ranges must be provided 2 numbers",
                            offset: i,
                        })
                    }
                } else if let Some(&(_, '}')) = self.chars.peek() {
                    self.chars.next();
                    Ok(Tok::Repeat(lower))
                } else {
                    Err(LexErr {
                        msg: "expected closing '}' for range",
                        offset: i,
                    })
                }
            } else {
                Err(LexErr {
                    msg: "ranges must be provided a number",
                    offset: i,
                })
            }
        } else {
            Err(LexErr {
                msg: "unexpected end of input",
                offset: i,
            })
        }
    }
    /// Returns a character class; as of this writing, brackets within the class
    /// must be balanced for lexing to succeed (they cannot exist independently)
    // FIXME: figure out how to grab the longest possible slice between [] while
    // still respecting valid pairs outside the [] and allowing broken [] inside
    fn left_bracket(&mut self, i: usize) -> Result<Tok<'src>, LexErr> {
        let mut depth = 1;
        while let Some(&(j, c)) = self.chars.peek() {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        self.chars.next();
                        let rv = &self.src[i + 1..j];
                        return if rv.len() != 0 {
                            Ok(Tok::Class(rv))
                        } else {
                            Err(LexErr {
                                msg: "character class must contain a nonzero amount of characters",
                                offset: i,
                            })
                        };
                    }
                }
                _ => (),
            }
            self.chars.next();
        }
        Err(LexErr {
            msg: "opening '[' has no corresponding closing ']'",
            offset: i,
        })
    }
    /// Returns a group by recursively creating a new Parser
    fn left_paren(&mut self, i: usize) -> Result<Tok<'src>, LexErr> {
        let mut depth = 1;
        while let Some(&(j, c)) = self.chars.peek() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        self.chars.next();
                        let rv = &self.src[i + 1..j];
                        return if rv.len() != 0 {
                            let inner = Lexer::new(rv).parse();
                            // we need to add to the offset because the error message is relative
                            // to the &str passed in
                            Ok(Tok::Group(inner.map_err(|e| LexErr {
                                msg: e.msg,
                                offset: e.offset + i + 1,
                            })?))
                        } else {
                            Err(LexErr {
                                msg: "group must contain a nonzero amount of characters",
                                offset: i,
                            })
                        };
                    }
                }
                _ => (),
            }
            self.chars.next();
        }
        Err(LexErr {
            msg: "opening '(' has no corresponding closing ')'",
            offset: i,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexErr {
    pub msg: &'static str,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tok<'src> {
    Class(&'src str),
    Char(char),
    BackRef(usize),
    Repeat(usize),
    Range(usize, usize),
    Group(Vec<Tok<'src>>),
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Tok<'src>, LexErr>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some((i, '\\')) => Some(self.backslash(i)),
            Some((i, '{')) => Some(self.left_curly(i)),
            Some((i, '[')) => Some(self.left_bracket(i)),
            Some((i, '(')) => Some(self.left_paren(i)),
            Some((i, ')')) => Some(Err(LexErr {
                msg: "unexpected )",
                offset: i,
            })),
            Some((i, ']')) => Some(Err(LexErr {
                msg: "unexpected ]",
                offset: i,
            })),
            Some((i, '}')) => Some(Err(LexErr {
                msg: "unexpected }",
                offset: i,
            })),
            Some((_, c @ _)) => Some(Ok(Tok::Char(c))),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(Lexer::new("").parse(), Ok(vec![]))
    }
    #[test]
    fn literal() {
        assert_eq!(
            Lexer::new("foo").parse(),
            Ok(vec![Tok::Char('f'), Tok::Char('o'), Tok::Char('o')])
        );
        assert_eq!(
            Lexer::new(r"\H\h\A\a\d").parse(),
            Ok(vec![
                Tok::Class(UPPERCASE_HEX),
                Tok::Class(LOWERCASE_HEX),
                Tok::Class(UPPERCASE_ALPHABET),
                Tok::Class(LOWERCASE_ALPHABET),
                Tok::Class(DIGITS)
            ])
        );
        assert_eq!(
            Lexer::new(r"\[\]\(\)\{\}").parse(),
            Ok(vec![
                Tok::Char('['),
                Tok::Char(']'),
                Tok::Char('('),
                Tok::Char(')'),
                Tok::Char('{'),
                Tok::Char('}'),
            ])
        );
    }
    #[test]
    fn character_class() {
        assert_eq!(Lexer::new("[foo]").parse(), Ok(vec![Tok::Class("foo")]));
        assert_eq!(
            Lexer::new("[foo][bar]").parse(),
            Ok(vec![Tok::Class("foo"), Tok::Class("bar")])
        );
    }
    #[test]
    fn range() {
        assert_eq!(Lexer::new("{3}").parse(), Ok(vec![Tok::Repeat(3)]));
        assert_eq!(Lexer::new("{3,5}").parse(), Ok(vec![(Tok::Range(3, 5))]));
        assert_eq!(
            Lexer::new("{}").parse(),
            Err(LexErr {
                msg: "ranges must be provided a number",
                offset: 0
            })
        );
    }
    #[test]
    fn group() {
        assert_eq!(
            Lexer::new("(a)").parse(),
            Ok(vec![Tok::Group(vec![Tok::Char('a')])])
        );
        assert_eq!(
            Lexer::new("([ab]c)").parse(),
            Ok(vec![Tok::Group(vec![Tok::Class("ab"), Tok::Char('c')])])
        );
    }
    #[test]
    fn back_ref() {
        assert_eq!(
            Lexer::new(r"(a)\1").parse(),
            Ok(vec![Tok::Group(vec![Tok::Char('a')]), Tok::BackRef(1)])
        );
    }
    #[test]
    fn user() {
        assert_eq!(
            Lexer::new(r"SKY-(\A)\A{2}\1-\d{4}").parse(),
            Ok(vec![
                Tok::Char('S'),
                Tok::Char('K'),
                Tok::Char('Y'),
                Tok::Char('-'),
                Tok::Group(vec![Tok::Class(UPPERCASE_ALPHABET)]),
                Tok::Class(UPPERCASE_ALPHABET),
                Tok::Repeat(2),
                Tok::BackRef(1),
                Tok::Char('-'),
                Tok::Class(DIGITS),
                Tok::Repeat(4)
            ])
        );
    }
}
