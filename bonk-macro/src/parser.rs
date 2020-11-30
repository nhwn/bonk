use super::Lexer;
use super::ParseErr;
use super::Token;
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;
use std::vec::IntoIter;

#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub value: char,
    pub idx: usize,
}

impl Init {
    fn new(value: char, idx: usize) -> Self {
        Self {
            value,
            idx,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Run {
    pub inits: Vec<Init>,
    pub changes: Vec<Change>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Final {
    pub tasks: Vec<Vec<Run>>,
    pub max_buffer_size: usize,
    pub statics: Dictionary,
}

impl Run {
    fn new(inits: Vec<Init>, changes: Vec<Change>) -> Self {
        Self { inits, changes }
    }
    pub fn len(&self) -> usize {
        self.inits.len() + self.changes.len()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Change {
    pub buffer_idx: usize,
    pub class_id: usize,
    pub span: Range<usize>,
}

impl Change {
    fn new(buffer_idx: usize, class_id: usize, span: Range<usize>) -> Self {
        Self {
            buffer_idx,
            class_id,
            span,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Dictionary {
    pub map: HashMap<Cow<'static, str>, usize>,
    class_id: usize,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            class_id: 0,
        }
    }
    fn generate_id(&mut self, class: Cow<'static, str>) -> usize {
        if let Some(&idx) = self.map.get(&class) {
            idx
        } else {
            self.map.insert(class, self.class_id);
            // keep old one so we don't hand out the incremented version
            let old_id = self.class_id;
            self.class_id += 1;
            old_id
        }
    }
}

#[derive(Debug)]
struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    buffer_idx: usize,
    changes: Vec<Change>,
    inits: Vec<Init>,
    dict: Dictionary,
}

impl Parser {
    fn new(src: &str) -> Result<Self, ParseErr> {
        Ok(Self {
            tokens: Lexer::new(src).tokens()?.into_iter().peekable(),
            buffer_idx: 0,
            changes: vec![],
            inits: vec![],
            dict: Dictionary::new(),
        })
    }
    fn consume_repeat(&mut self) -> Option<usize> {
        // HACK: move peek() outside the condition to avoid moot borrow twice
        if let Some(&Token::Repeat(n)) = self.tokens.peek() {
            self.tokens.next();
            Some(n - 1)
        } else {
            None
        }
    }
    fn char(&mut self, c: char) {
        self.inits.push(Init::new(c, self.buffer_idx));
        self.buffer_idx += 1;
        if let Some(n) = self.consume_repeat() {
            self.inits
                .extend((self.buffer_idx..self.buffer_idx + n).map(|i| Init::new(c, i)));
            self.buffer_idx += n;
        }
    }
    fn class(&mut self, class: Cow<'static, str>) {
        let len = class.len();
        let class_id = self.dict.generate_id(class);
        self.changes
            .push(Change::new(self.buffer_idx, class_id, 0..len));
        self.buffer_idx += 1;
        if let Some(n) = self.consume_repeat() {
            self.changes.extend(
                (self.buffer_idx..self.buffer_idx + n).map(|i| Change::new(i, class_id, 0..len)),
            );
            self.buffer_idx += n;
        }
    }
    fn parse(mut self) -> Final {
        // TODO: generalize this to support range tokens and partitioning
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Char(c) => self.char(c),
                Token::Class(c) => self.class(c),
                _ => unimplemented!(),
            }
        }
        let run = Run::new(self.inits, self.changes);
        let max_buffer_size = run.len();
        Final {
            max_buffer_size,
            tasks: vec![vec![run]],
            statics: self.dict,
        }
    }
}

impl FromStr for Final {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser::new(s)?.parse())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bad() {
        let parser = Parser::new("{3}").unwrap_err();
        println!("{:?}", parser);
    }
}