use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

static DIGITS: &str = "0123456789";
static UPPERCASE_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LOWERCASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
static LOWERCASE_HEX: &str = "0123456789abcdef";
static UPPERCASE_HEX: &str = "0123456789ABCDEF";

#[derive(Debug, PartialEq)]
pub struct ParseErr {
    pub msg: &'static str,
    pub offset: usize,
}

impl ParseErr {
    fn new(msg: &'static str, offset: usize) -> Self {
        Self { msg, offset }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Class { id: usize, len: usize },
    Range { lower: usize, upper: usize },
    Char(char),
    Repeat(usize),
}

impl Token {
    fn is_varying(&self) -> bool {
        matches!(self, Self::Repeat(_) | Self::Range {..})
    }
}

type Dict = HashMap<Cow<'static, str>, usize>;

pub struct Lexer<'s, 'd> {
    chars: Peekable<CharIndices<'s>>,
    src: &'s str,
    idx: usize,         // the index of the char that was just consumed
    dict: &'d mut Dict, // this is a reference so we can use collect() and still move the HashMap out
    id: usize,
}

impl<'s, 'd> Lexer<'s, 'd> {
    pub fn tokenize(src: &str) -> Result<(Dict, Vec<Token>), ParseErr> {
        let mut map = HashMap::new();
        let tokens = Lexer::new(src, &mut map).collect::<Result<Vec<_>, _>>()?;

        if tokens.is_empty() {
            return Err(ParseErr::new("pattern must be nonempty", 0));
        }

        if tokens.first().unwrap().1.is_varying() {
            return Err(ParseErr::new(
                "pattern must begin with a character or character class",
                0,
            ));
        }

        if let Some(i) = tokens
            .windows(2)
            .find(|s| s.iter().all(|t| t.1.is_varying()))
            .map(|s| s[1].0)
        {
            return Err(ParseErr::new(
                "ranges and repetitions must be preceded by characters or character classes",
                i,
            ));
        }

        Ok((map, tokens.into_iter().map(|(_, t)| t).collect()))
    }
    /// Create a new Lexer from a &str
    fn new(src: &'s str, dict: &'d mut Dict) -> Self {
        Self {
            chars: src.char_indices().peekable(),
            src,
            idx: 0,
            id: 0,
            dict,
        }
    }

    fn class(&mut self, class: Cow<'static, str>) -> Token {
        let len = class.len();
        self.id = *self.dict.entry(class).or_insert(self.id + 1);
        Token::Class { len, id: self.id }
    }

    /// Consumes char, setting the current index to the byte offset of said
    /// char; callers of this function assume that there are more chars
    /// to lex, so this will return an error if there are no more chars
    fn consume_char(&mut self) -> Result<char, ParseErr> {
        let (i, c) = self.chars.next().ok_or_else(|| self.eoi())?;
        self.idx = i;
        Ok(c)
    }
    /// Peeks char, then returns it if the predicate matches
    fn consume_if<F>(&mut self, predicate: F) -> Result<Option<char>, ParseErr>
    where
        F: Fn(char) -> bool,
    {
        if let Some(&(_, c_inner)) = self.chars.peek() {
            if (predicate)(c_inner) {
                // we just successfully peeked
                self.consume_char().unwrap();
                Ok(Some(c_inner))
            } else {
                Ok(None)
            }
        } else {
            Err(self.eoi())
        }
    }
    /// Takes chars while they're ASCII digits, then parses to a usize; must
    /// be positive
    fn consume_number(&mut self) -> Result<usize, ParseErr> {
        if self.consume_char()?.is_ascii_digit() {
            let start = self.idx;
            // NOTE: can't use take_while here because we don't want to consume a non-digit
            while self.consume_if(|c| c.is_ascii_digit())?.is_some() {}
            // we just ensured valid digits
            Ok(self.src[start..=self.idx].parse().unwrap())
        } else {
            Err(self.err("expected positive integer"))
        }
    }
    /// helper method to create a ParseErr with the current index in the source
    fn err(&self, msg: &'static str) -> ParseErr {
        ParseErr::new(msg, self.idx)
    }
    /// helper method to create an "unexpected end of input" message
    fn eoi(&self) -> ParseErr {
        ParseErr::new("unexpected end of input", self.idx)
    }
    /// Transforms escaped characters into their specific meanings:
    /// - "\\" is a literal backslash
    /// - "\[" is a literal opening bracket
    /// - "\]" is a literal closing bracket
    /// - "\{" is a literal opening curly brace
    /// - "\}" is a literal closing curly brace
    /// - "\A" is a character class of the uppercase letters
    /// - "\a" is a character class of the lowercase letters
    /// - "\d" is a character class of the digits in base 10
    /// - "\h" is a character class of the digits in base 16 (lowercase)
    /// - "\H" is a character class of the digits in base 16 (uppercase)
    fn backslash(&mut self) -> Result<Token, ParseErr> {
        match self.consume_char()? {
            'd' => Ok(self.class(DIGITS.into())),
            'A' => Ok(self.class(UPPERCASE_ALPHABET.into())),
            'a' => Ok(self.class(LOWERCASE_ALPHABET.into())),
            'H' => Ok(self.class(UPPERCASE_HEX.into())),
            'h' => Ok(self.class(LOWERCASE_HEX.into())),
            '\\' => Ok(Token::Char('\\')),
            '[' => Ok(Token::Char('[')),
            ']' => Ok(Token::Char(']')),
            '{' => Ok(Token::Char('{')),
            '}' => Ok(Token::Char('}')),
            _ => {
                Err(self.err("backslashes must be followed by a '\\', 'd', 'A', 'a', 'H', or 'h'"))
            }
        }
    }
    /// Returns a range or repeat token
    fn left_curly(&mut self) -> Result<Token, ParseErr> {
        let opening_idx = self.idx;
        let lower = self.consume_number()?;
        match self.consume_char()? {
            ',' => {
                let upper = self.consume_number()?;
                if self.consume_char()? == '}' {
                    use Ordering::*;
                    match lower.cmp(&upper) {
                        Less => Ok(Token::Range {lower, upper}),
                        Equal => Err(self.err(
                            "bounds cannot be equal in a range; consider using the repetition syntax",
                        )),
                        Greater => Err(self.err("lower bound must be less than upper bound")),
                    }
                } else {
                    Err(self.err("expected closing '}' for range"))
                }
            }
            '}' => {
                if lower <= 1 {
                    Err(ParseErr::new(
                        "number of repetition must be greater than 1",
                        opening_idx + 1,
                    ))
                } else {
                    Ok(Token::Repeat(lower))
                }
            }
            _ => Err(self.err("expected closing '}' for repetition")),
        }
    }
    /// Returns a character class token
    fn left_bracket(&mut self) -> Result<Token, ParseErr> {
        let mut buf = String::new();
        loop {
            match self.consume_char()? {
                ']' => return Ok(self.class(buf.into())),
                '[' => return Err(self.err("escape [ in character classes")),
                '\\' => {
                    let c = self
                        .consume_if(|c| matches!(c, '\\' | '[' | ']'))?
                        .ok_or_else(|| {
                            self.err(
                            "backslashes in character classes must be followed by a ']', '[', or '\\'",
                        )
                        })?;
                    buf.push(c);
                }
                c => buf.push(c),
            }
        }
    }
}

impl<'s, 'd> Iterator for Lexer<'s, 'd> {
    type Item = Result<(usize, Token), ParseErr>;
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(i, c)| {
            self.idx = i;
            match c {
                '\\' => self.backslash(),
                '{' => self.left_curly(),
                '[' => self.left_bracket(),
                ']' => Err(self.err("] has no matching [")),
                '}' => Err(self.err("} has no matching {")),
                _ => Ok(Token::Char(c)),
            }
            .map(|t| (i, t))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn ok_parse(src: &str, v: Vec<Token>) {
        let toks: Vec<Token> = tokenize(src).unwrap().1;
        assert_eq!(v, toks)
    }
    fn err_parse(src: &str) {
        let err = tokenize(src).unwrap_err();
        println!("error: {}", err.msg);
        println!("  |");
        println!("  | {}", src);
        println!("  | {}^", " ".repeat(err.offset));
        println!("  |");
    }
    #[test]
    fn empty() {
        err_parse("");
    }
    #[test]
    fn literal() {
        ok_parse(
            "foo",
            vec![Token::Char('f'), Token::Char('o'), Token::Char('o')],
        );
        ok_parse(
            r"\H\h\A\a\d",
            vec![
                Token::Class { len: 16, id: 1 },
                Token::Class { len: 16, id: 2 },
                Token::Class { len: 26, id: 3 },
                Token::Class { len: 26, id: 4 },
                Token::Class { len: 10, id: 5 },
            ],
        );
        ok_parse(
            r"\[\]\{\}",
            vec![
                Token::Char('['),
                Token::Char(']'),
                Token::Char('{'),
                Token::Char('}'),
            ],
        );
    }
    #[test]
    fn character_class() {
        ok_parse(
            "[foo][bar]",
            vec![
                Token::Class { id: 1, len: 3 },
                Token::Class { id: 2, len: 3 },
            ],
        );
        ok_parse(r"[\\]", vec![Token::Class { id: 1, len: 1 }]);
        ok_parse(r"[\[\]]", vec![Token::Class { id: 1, len: 2 }]);
        err_parse("[");
        err_parse("[\\a]");
    }
    #[test]
    fn range() {
        ok_parse("a{3}", vec![Token::Char('a'), Token::Repeat(3)]);
        ok_parse(
            "a{3,5}",
            vec![Token::Char('a'), Token::Range { lower: 3, upper: 5 }],
        );
        ok_parse(
            "a{30,50}",
            vec![
                Token::Char('a'),
                Token::Range {
                    lower: 30,
                    upper: 50,
                },
            ],
        );
        err_parse("a{}");
        err_parse("a{,}");
        err_parse("a{12,}");
        err_parse("a{12,12}");
        err_parse("a{3,1}");
    }
}
