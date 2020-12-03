use super::{Lexer, ParseErr, Token};
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub value: char,
    pub idx: usize,
}

impl Init {
    fn new(value: char, idx: usize) -> Self {
        Self { value, idx }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Run {
    pub inits: Vec<Init>,
    pub changes: Vec<Change>,
    pub len: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Final {
    pub tasks: Vec<Vec<Run>>,
    pub max_buffer_size: usize,
    pub statics: HashMap<Cow<'static, str>, usize>,
}

impl Run {
    fn new(len: usize, inits: Vec<Init>, changes: Vec<Change>) -> Self {
        Self {
            inits,
            changes,
            len,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Change {
    pub idx: usize,
    pub class_id: usize,
    pub start: usize,
    pub end: usize,
}

impl Change {
    fn new(idx: usize, class_id: usize, start: usize, end: usize) -> Self {
        Self {
            idx,
            class_id,
            start,
            end,
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
    fn generate_id(&mut self, class: &Cow<'static, str>) -> usize {
        // FIXME: can't use entry() because it always takes the key argument by value (we only want to
        // clone if the key isn't inserted yet); see https://github.com/rust-lang/rfcs/issues/1203
        // for using raw_entry() on nightly (hopefully, this gets stabilized sooner or later)
        if let Some(&class_id) = self.map.get(class) {
            class_id
        } else {
            self.class_id += 1;
            // cloning here is much better than cloning all the tokens + possibly owned 
            // strings in the tokens to handle ranges; it'll just be a Cow::Borrowed most
            // of the time anyway
            self.map.insert(class.clone(), self.class_id);
            self.class_id
        }
    }
}

struct Parser<'src> {
    dict: Dict,
    runs: Vec<Run>,
    lexer: Lexer<'src>,
}

struct Indexer {
    maxs: Vec<usize>,
    buf: Vec<usize>,
}

impl Indexer {
    fn new(maxs: Vec<usize>) -> Self {
        Self {
            buf: vec![0; maxs.len()],
            maxs,
        }
    }
    fn get_iter(&mut self, i: u64) -> impl Iterator<Item = usize> + '_ {
        let mut rem = i;
        for (slot, max) in self.buf.iter_mut().zip(self.maxs.iter().copied()).rev() {
            let modulo = rem % (max as u64);
            *slot = modulo as usize;
            rem = (rem - modulo) / (max as u64);
        }

        self.buf.iter().copied()
    }
    // fn get_all(&mut self) -> impl Iterator<Item = impl Iterator<Item = usize>> {
    //     (0..self.total()).map(|i| self.get_iter(i))
    // }
    fn total(&self) -> u64 {
        self.maxs.iter().map(|&i| i as u64).product::<u64>()
    }
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            dict: Dict::new(),
            runs: vec![],
            lexer: Lexer::new(src),
        }
    }
    fn consume_repeat<'a, I>(tokens: &mut Peekable<I>) -> Option<usize>
    where
        I: Iterator<Item = &'a Token>,
    {
        if let Some(Token::Repeat(n)) = tokens.peek() {
            tokens.next();
            Some(*n)
        } else {
            None
        }
    }
    fn make_run<'a, I>(mut tokens: Peekable<I>, dict: &mut Dict) -> Run
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut buffer_idx = 0;
        let mut inits = vec![];
        let mut changes = vec![];
        while let Some(token) = tokens.next() {
            match token {
                &Token::Char(c) => {
                    inits.push(Init::new(c, buffer_idx));
                    buffer_idx += 1;
                    if let Some(n) = Self::consume_repeat(&mut tokens) {
                        // do nothing if n == 1
                        if n > 1 {
                            inits.extend((buffer_idx..buffer_idx + n - 1).map(|i| Init::new(c, i)));
                            buffer_idx += n - 1;
                        } else if n == 0 {
                            inits.pop();
                            buffer_idx -= 1;
                        }
                    }
                }
                Token::Class(class) => {
                    let class_id = dict.generate_id(class);
                    changes.push(Change::new(buffer_idx, class_id, 0, class.len()));
                    buffer_idx += 1;
                    if let Some(n) = Self::consume_repeat(&mut tokens) {
                        // do nothing if n == 1
                        if n > 1 {
                            changes.extend(
                                (buffer_idx..buffer_idx + n - 1)
                                    .map(|i| Change::new(i, class_id, 0, class.len())),
                            );
                            buffer_idx += n - 1;
                        } else if n == 0 {
                            changes.pop();
                            buffer_idx -= 1;
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        Run::new(inits.len() + changes.len(), inits, changes)
    }
    fn parse(mut self) -> Result<Final, ParseErr> {
        let mut tokens = self.lexer.tokens()?;
        let range_idxs = tokens
            .iter()
            .enumerate()
            .filter_map(|(i, t)| match t {
                Token::Range(range) => Some((i, range.start, range.end - range.start)),
                _ => None,
            })
            .collect::<Vec<(usize, usize, usize)>>(); // (idx, start, len)

        if range_idxs.is_empty() {
            self.runs
                .push(Self::make_run(tokens.iter().peekable(), &mut self.dict));
        } else {
            let mut indexer = Indexer::new(range_idxs.iter().map(|t| t.2).collect());
            // can't use map/extend because FnMut closures cannot let their captures outlive the
            // closure itself
            self.runs.reserve(indexer.total() as usize);
            for i in 0..indexer.total() {
                for (val, idx) in indexer
                    .get_iter(i)
                    .zip(range_idxs.iter().map(|start| start.1))
                    .map(|(base, start)| base + start)
                    .zip(range_idxs.iter().map(|idx| idx.0))
                {
                    tokens[idx] = Token::Repeat(val);
                }
                println!("{:?}", tokens);
                self.runs
                    .push(Self::make_run(tokens.iter().peekable(), &mut self.dict));
            }
        }

        let max_buffer_size = self.runs.iter().map(|run| run.len).max().unwrap();

        Ok(Final {
            max_buffer_size,
            tasks: vec![self.runs],
            statics: self.dict.map,
        })
    }
}

impl FromStr for Final {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse()
    }
}
