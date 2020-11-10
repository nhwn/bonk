use std::borrow::Cow;
use std::cmp::Ordering;
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

impl LexErr {
    fn new(msg: &'static str, offset: usize) -> Self {
        Self { msg, offset }
    }
}

impl<'src> Lexer<'src> {
    /// Create a new Lexer from a &str
    pub fn new(src: &'src str) -> Self {
        Self {
            chars: src.char_indices().peekable(),
            src,
        }
    }
    /// Convenience method for directly allocating into a Veg
    pub fn parse(self) -> Result<Vec<Tok<'src>>, LexErr> {
        self.into_iter().collect()
    }
    /// Peeks character, then consumes it if it matches, returning
    /// true on success and false on failure
    fn eat_if(&mut self, c: char) -> bool {
        if let Some(&(_, c_inner)) = self.chars.peek() {
            if c_inner == c {
                self.chars.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Blindly takes the next char. If it's a digit, we proceed to parse
    /// a number and return it in a Some; otherwise, we return None.
    /// Note that the initial character will be consumed.
    fn consuming_eat_if_digit(&mut self) -> Option<usize> {
        if let Some((i, c)) = self.chars.next() {
            if c.is_digit(10) {
                Some(self.number(i))
            } else {
                None
            }
        } else {
            None
        }
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
            Some((_, 'd')) => Ok(Tok::Class(Cow::Borrowed(DIGITS))),
            Some((_, 'A')) => Ok(Tok::Class(Cow::Borrowed(UPPERCASE_ALPHABET))),
            Some((_, 'a')) => Ok(Tok::Class(Cow::Borrowed(LOWERCASE_ALPHABET))),
            Some((_, 'H')) => Ok(Tok::Class(Cow::Borrowed(UPPERCASE_HEX))),
            Some((_, 'h')) => Ok(Tok::Class(Cow::Borrowed(LOWERCASE_HEX))),
            Some((_, '\\')) => Ok(Tok::Char('\\')),
            Some((_, '(')) => Ok(Tok::Char('(')),
            Some((_, ')')) => Ok(Tok::Char(')')),
            Some((_, '[')) => Ok(Tok::Char('[')),
            Some((_, ']')) => Ok(Tok::Char(']')),
            Some((_, '{')) => Ok(Tok::Char('{')),
            Some((_, '}')) => Ok(Tok::Char('}')),
            Some((i, c)) if c.is_digit(10) => Ok(Tok::BackRef(self.number(i))),
            Some(_) => Err(LexErr::new(
                "backslashes must be followed by a '\\', 'd', 'A', 'a', 'H', or 'h'",
                i,
            )),
            None => Err(LexErr::new("unexpected end of input", i)),
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
        let lower = self
            .consuming_eat_if_digit()
            .ok_or(LexErr::new("expected number after {", i))?;
        if self.eat_if(',') {
            let upper = self
                .consuming_eat_if_digit()
                .ok_or(LexErr::new("range must be provided 2 numbers", i))?;
            if self.eat_if('}') {
                match lower.cmp(&upper) {
                    Ordering::Less => Ok(Tok::Range(lower, upper)),
                    Ordering::Equal => Err(LexErr::new(
                        "bounds cannot be equal in a range; consider using the repetition syntax",
                        i,
                    )),
                    Ordering::Greater => {
                        Err(LexErr::new("lower bound must be less than upper bound", i))
                    }
                }
            } else {
                Err(LexErr::new("expected closing '}' for range", i))
            }
        } else if self.eat_if('}') {
            Ok(Tok::Repeat(lower))
        } else {
            Err(LexErr::new("expected closing '}' for repetition", i))
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
                            Ok(Tok::Class(Cow::Borrowed(rv)))
                        } else {
                            Err(LexErr::new(
                                "character class must contain a nonzero amount of characters",
                                i,
                            ))
                        };
                    }
                }
                _ => (),
            }
            self.chars.next();
        }
        Err(LexErr::new(
            "opening '[' has no corresponding closing ']'",
            i,
        ))
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
                        let substring = &self.src[i + 1..j];
                        return if substring.len() != 0 {
                            let inner = Lexer::new(substring).parse();
                            // we need to add to the offset because the error message is relative
                            // to the &str passed in
                            Ok(Tok::Group(
                                inner.map_err(|e| LexErr::new(e.msg, e.offset + i + 1))?,
                            ))
                        } else {
                            Err(LexErr::new(
                                "group must contain a nonzero amount of characters",
                                i,
                            ))
                        };
                    }
                }
                _ => (),
            }
            self.chars.next();
        }
        Err(LexErr::new(
            "opening '(' has no corresponding closing ')'",
            i,
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexErr {
    pub msg: &'static str,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tok<'src> {
    Class(Cow<'src, str>),
    Char(char),
    BackRef(usize),
    Repeat(usize),
    Range(usize, usize),
    Group(Vec<Tok<'src>>),
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Tok<'src>, LexErr>;
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(i, c)| match c {
            '\\' => self.backslash(i),
            '{' => self.left_curly(i),
            '[' => self.left_bracket(i),
            '(' => self.left_paren(i),
            ')' => Err(LexErr::new("unexpected )", i)),
            ']' => Err(LexErr::new("unexpected ]", i)),
            '}' => Err(LexErr::new("unexpected }", i)),
            _ => Ok(Tok::Char(c)),
        })
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
                Tok::Class(Cow::Borrowed(UPPERCASE_HEX)),
                Tok::Class(Cow::Borrowed(LOWERCASE_HEX)),
                Tok::Class(Cow::Borrowed(UPPERCASE_ALPHABET)),
                Tok::Class(Cow::Borrowed(LOWERCASE_ALPHABET)),
                Tok::Class(Cow::Borrowed(DIGITS))
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
        assert_eq!(
            Lexer::new("[foo]").parse(),
            Ok(vec![Tok::Class(Cow::Owned("foo".to_owned()))])
        );
        assert_eq!(
            Lexer::new("[foo][bar]").parse(),
            Ok(vec![
                Tok::Class(Cow::Owned("foo".to_owned())),
                Tok::Class(Cow::Owned("bar".to_owned()))
            ])
        );
    }
    #[test]
    fn range() {
        assert_eq!(Lexer::new("{3}").parse(), Ok(vec![Tok::Repeat(3)]));
        assert_eq!(Lexer::new("{3,5}").parse(), Ok(vec![(Tok::Range(3, 5))]));
        assert_eq!(
            Lexer::new("{30,50}").parse(),
            Ok(vec![(Tok::Range(30, 50))])
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
            Ok(vec![Tok::Group(vec![
                Tok::Class(Cow::Owned("ab".to_owned())),
                Tok::Char('c')
            ])])
        );
        assert_eq!(
            Lexer::new("(a)(b)(c)").parse(),
            Ok(vec![
                Tok::Group(vec![Tok::Char('a')]),
                Tok::Group(vec![Tok::Char('b')]),
                Tok::Group(vec![Tok::Char('c')])
            ])
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
                Tok::Group(vec![Tok::Class(Cow::Borrowed(UPPERCASE_ALPHABET))]),
                Tok::Class(Cow::Borrowed(UPPERCASE_ALPHABET)),
                Tok::Repeat(2),
                Tok::BackRef(1),
                Tok::Char('-'),
                Tok::Class(Cow::Borrowed(DIGITS)),
                Tok::Repeat(4)
            ])
        );
    }
}
