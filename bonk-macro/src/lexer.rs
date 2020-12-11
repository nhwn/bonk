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
static ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";

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

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Class { id: usize, len: usize },
    Range { lower: usize, upper: usize },
    Char(u8),
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
    dict: &'d mut Dict, // this is a reference so we can use collect() and still use the HashMap
    id: usize,
}

impl<'s, 'd> Lexer<'s, 'd> {
    pub fn tokenize(src: &str) -> Result<(Dict, Vec<Token>), ParseErr> {
        if let Some(i) = src
            .char_indices()
            .find_map(|(i, c)| if !c.is_ascii() { Some(i) } else { None })
        {
            return Err(ParseErr::new(
                "pattern should only contain ASCII characters",
                i,
            ));
        }

        let mut map = HashMap::new();
        let tokens = Lexer::new(src, &mut map).collect::<Result<Vec<_>, _>>()?;

        if tokens
            .first()
            .ok_or_else(|| ParseErr::new("pattern should be nonempty", 0))?
            .1
            .is_varying()
        {
            return Err(ParseErr::new(
                "pattern should begin with a character or character class",
                0,
            ));
        };

        if let Some(i) = tokens.windows(2).find_map(|s| {
            if s.iter().all(|t| t.1.is_varying()) {
                Some(s[1].0)
            } else {
                None
            }
        }) {
            return Err(ParseErr::new(
                "ranges and repeats should be preceded by characters or character classes",
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
                self.consume_char().expect("we just peeked");
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
            // NOTE: take_while isn't an option because we don't want to consume a non-digit,
            // and we need to somehow advance the index to parse the substring
            while self.consume_if(|c| c.is_ascii_digit())?.is_some() {}
            self.src[start..=self.idx]
                .parse()
                .map_err(|_| ParseErr::new("range bounds should not exceed a usize", start))
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
    /// - "\A" is a character class of [A-Z]
    /// - "\a" is a character class of [a-z]
    /// - "\d" is a character class of [0-9]
    /// - "\h" is a character class of [0-9A-F]
    /// - "\H" is a character class of [0-9a-f]
    /// - "\w" is a character class of [A-Za-z0-9_]
    fn backslash(&mut self) -> Result<Token, ParseErr> {
        match self.consume_char()? {
            'd' => Ok(self.class(DIGITS.into())),
            'A' => Ok(self.class(UPPERCASE_ALPHABET.into())),
            'a' => Ok(self.class(LOWERCASE_ALPHABET.into())),
            'H' => Ok(self.class(UPPERCASE_HEX.into())),
            'h' => Ok(self.class(LOWERCASE_HEX.into())),
            'w' => Ok(self.class(ALPHANUMERIC.into())),
            '\\' => Ok(Token::Char(b'\\')),
            '[' => Ok(Token::Char(b'[')),
            ']' => Ok(Token::Char(b']')),
            '{' => Ok(Token::Char(b'{')),
            '}' => Ok(Token::Char(b'}')),
            _ => Err(self.err("'\\' should be followed by '\\', 'd', 'A', 'a', 'H', 'h', or 'w'")),
        }
    }
    /// Returns a range or repeat token
    fn left_curly(&mut self) -> Result<Token, ParseErr> {
        let start = self.idx;
        let lower = self.consume_number()?;
        match self.consume_char()? {
            ',' => {
                let upper = self.consume_number()?;
                if self.consume_char()? == '}' {
                    use Ordering::*;
                    match lower.cmp(&upper) {
                        Less => Ok(Token::Range {lower, upper}),
                        Equal => Err(ParseErr::new("a range's bounds should not be equal; consider using the repeat syntax", start)),
                        Greater => Err(ParseErr::new("a range's lower bound should be less the upper bound", start)),
                    }
                } else {
                    Err(self.err("expected closing '}' for range"))
                }
            }
            '}' => {
                if lower > 1 {
                    Ok(Token::Repeat(lower))
                } else {
                    Err(ParseErr::new(
                        "number of repeats in a repetition should be greater than 1",
                        start,
                    ))
                }
            }
            _ => Err(self.err("expected closing '}' for repeat")),
        }
    }
    /// Returns a character class token
    fn left_bracket(&mut self) -> Result<Token, ParseErr> {
        let mut buf = String::new();
        loop {
            match self.consume_char()? {
                ']' => return Ok(self.class(buf.into())),
                '[' => {
                    return Err(self.err("'[' should be preceded with a '\\' in character classes"))
                }
                '\\' => {
                    let c = self
                        .consume_if(|c| matches!(c, '\\' | '[' | ']'))?
                        .ok_or_else(|| {
                            self.err(
                                "'\\' should be followed by ']', '[', or '\\' in character classes",
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
                ']' => Err(self.err("unexpected ']'")),
                '}' => Err(self.err("unexpected '}'")),
                _ => Ok(Token::Char(c as u8)),
            }
            .map(|t| (i, t))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn ok_parse(src: &str, v: Vec<Token>) {
        assert_eq!(Lexer::tokenize(src).unwrap().1, v);
    }
    fn err_parse(src: &str) {
        let err = Lexer::tokenize(src).unwrap_err();
        eprintln!("error: {}", err.msg);
        eprintln!("  |");
        eprintln!("  | {}", src);
        eprintln!("  | {}^", " ".repeat(err.offset));
        eprintln!("  |");
    }
    #[test]
    fn misc_errors() {
        err_parse("");
        err_parse("]");
        err_parse("}");
        err_parse("\\s");
        err_parse("definitely ascii❤");
    }
    #[test]
    fn literal() {
        ok_parse("\x00", vec![Token::Char(0)]);
        ok_parse("\x7F", vec![Token::Char(127)]);
        ok_parse(
            "foo",
            vec![Token::Char(b'f'), Token::Char(b'o'), Token::Char(b'o')],
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
                Token::Char(b'['),
                Token::Char(b']'),
                Token::Char(b'{'),
                Token::Char(b'}'),
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
    fn repeat() {
        ok_parse("a{3}", vec![Token::Char(b'a'), Token::Repeat(3)]);
        err_parse("a{12,}");
        err_parse("{1}");
        err_parse("a{0}");
        err_parse("a{234092348903248032948392342349089}");
    }
    #[test]
    fn range() {
        ok_parse(
            "a{3,5}",
            vec![Token::Char(b'a'), Token::Range { lower: 3, upper: 5 }],
        );
        ok_parse(
            "a{30,50}",
            vec![
                Token::Char(b'a'),
                Token::Range {
                    lower: 30,
                    upper: 50,
                },
            ],
        );
        ok_parse(
            r"\a{1,10}",
            vec![
                Token::Class { id: 1, len: 26 },
                Token::Range {
                    lower: 1,
                    upper: 10,
                },
            ],
        );
        err_parse("a{}");
        err_parse("a{,}");
        err_parse("a{12,12}");
        err_parse("a{3,1}");
        err_parse("a{1,234092348903248032948392342349089}");
    }
}
