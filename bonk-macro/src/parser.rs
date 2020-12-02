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
    pub value: u8,
    pub idx: usize,
}

impl Init {
    fn new(value: u8, idx: usize) -> Self {
        Self { value, idx }
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
    pub statics: Dict,
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
    pub idx: usize,
    pub class_id: usize,
    pub span: Range<usize>,
}

impl Change {
    fn new(idx: usize, class_id: usize, span: Range<usize>) -> Self {
        Self {
            idx,
            class_id,
            span,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Dict {
    pub map: HashMap<Cow<'static, str>, usize>,
    class_id: usize,
}

impl Dict {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            class_id: 0,
        }
    }
    /// Returns a usize that identifies each unique string
    fn generate_id(&mut self, class: Cow<'static, str>) -> usize {
        // can't use entry() because or_insert_with() will capture all of 
        // self to update class_id in the closure, which results in 2 mutable 
        // borrows to self
        if let Some(&class_id) = self.map.get(&class) {
            class_id
        } else {
            self.map.insert(class, self.class_id);
            // keep old id so we don't hand out the incremented version
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
    dict: Dict,
}

impl Parser {
    fn new(src: &str) -> Result<Self, ParseErr> {
        Ok(Self {
            tokens: Lexer::new(src).tokens()?.into_iter().peekable(),
            buffer_idx: 0,
            changes: vec![],
            inits: vec![],
            dict: Dict::new(),
        })
    }
    fn consume_repeat(&mut self) -> Option<usize> {
        if let Some(&Token::Repeat(n)) = self.tokens.peek() {
            self.tokens.next();
            Some(n - 1)
        } else {
            None
        }
    }
    fn char(&mut self, c: u8) {
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
                Token::Char(c) => self.char(c as u8),
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
}
