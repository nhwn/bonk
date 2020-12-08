use super::{Lexer, ParseErr, Token};
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub val: char,
    pub idx: usize,
}

impl Init {
    fn new(idx: usize, val: char) -> Self {
        Self { val, idx }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Change {
    pub idx: usize,
    pub class_id: usize,
    pub lower: usize,
    pub upper: usize,
}

impl Change {
    fn new(idx: usize, class_id: usize, end: usize) -> Self {
        Self {
            idx,
            class_id,
            lower: 0,
            upper: end,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Run {
    pub inits: Vec<Init>,
    pub changes: Vec<Change>,
    pub len: usize,
}

impl Run {
    fn visit_uppers(&mut self) -> impl Iterator<Item = &mut usize> {
        self.changes.iter_mut().map(|c| &mut c.upper)
    }
    fn visit_lowers(&mut self) -> impl Iterator<Item = &mut usize> {
        self.changes.iter_mut().map(|c| &mut c.lower)
    }
    fn new(ts: &[Token]) -> Self {
        let mut ts = ts.iter().copied().peekable();
        let mut inits = vec![];
        let mut changes = vec![];
        while let Some(t) = ts.next() {
            match t {
                Token::Char(c) => {
                    let idx = inits.len() + changes.len();
                    inits.push(Init::new(idx, c));
                    if let Some(&Token::Repeat(n)) = ts.peek() {
                        ts.next();
                        if n > 1 {
                            let idx = inits.len() + changes.len();
                            inits.extend((idx..idx + n - 1).map(|i| Init::new(i, c)));
                        } else if n == 0 {
                            inits.pop();
                        }
                    }
                }
                Token::Class { id, len } => {
                    let idx = inits.len() + changes.len();
                    changes.push(Change::new(idx, id, len));
                    if let Some(&Token::Repeat(n)) = ts.peek() {
                        ts.next();
                        if n > 1 {
                            let idx = inits.len() + changes.len();
                            changes.extend((idx..idx + n - 1).map(|i| Change::new(i, id, len)));
                        } else if n == 0 {
                            changes.pop();
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Run {
            len: inits.len() + changes.len(),
            inits,
            changes,
        }
    }
}

struct Permutor {
    maxs: Vec<usize>,
    buf: Vec<Cell<usize>>, // use interior mutability to avoid handing out mutable references
}

impl Permutor {
    fn new(maxs: Vec<usize>) -> Self {
        Self {
            buf: vec![Cell::new(0); maxs.len()],
            maxs,
        }
    }
    fn get_iter(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        let mut rem = i;
        // we reverse to start incrementing from the back
        for (slot, max) in self.buf.iter().zip(self.maxs.iter().copied()).rev() {
            let modulo = rem % max;
            slot.set(modulo);
            rem = (rem - modulo) / max;
        }

        self.buf.iter().map(|c| c.get())
    }

    fn total(&self) -> usize {
        self.maxs.iter().product()
    }

    fn permutations(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        (0..self.total()).map(move |i| self.get_iter(i))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Final {
    pub tasks: Vec<Vec<Run>>,
    pub max_size: usize,
    pub statics: HashMap<Cow<'static, str>, usize>,
}

impl Final {
    pub fn new(src: &str, num_tasks: usize) -> Result<Final, ParseErr> {
        let (statics, mut tokens) = Lexer::tokenize(src)?;
        let (bounds, range_idxs): (Vec<_>, Vec<_>) = tokens
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, t)| {
                if let Token::Range { lower, upper } = t {
                    Some(((lower, upper - lower + 1), i))
                } else {
                    None
                }
            })
            .unzip();
        let runs = if range_idxs.is_empty() {
            vec![Run::new(&tokens)]
        } else {
            let (lows, lens): (Vec<_>, Vec<_>) = bounds.into_iter().unzip();
            Permutor::new(lens)
                .permutations()
                .map(|p| {
                    p.zip(lows.iter())
                        .map(|(n, lower)| n + lower)
                        .zip(range_idxs.iter().copied())
                        .for_each(|(val, idx)| tokens[idx] = Token::Repeat(val));
                    Run::new(&tokens)
                })
                .collect()
        };

        let max_size = runs.iter().map(|run| run.len).max().unwrap();
        let tasks = Self::partition(runs, num_tasks);

        Ok(Final {
            max_size,
            tasks,
            statics,
        })
    }

    fn partition(mut runs: Vec<Run>, num_tasks: usize) -> Vec<Vec<Run>> {
        // let changes = runs
        //     .iter()
        //     .map(|run| run.changes.iter().map(|c| c.upper).product())
        //     .collect::<Vec<usize>>();
        // vec![runs; num_tasks]
        vec![runs]
    }
}
