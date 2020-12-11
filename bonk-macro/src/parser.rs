use super::{Lexer, ParseErr, Token};
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use std::iter::Peekable;

pub enum Partition {
    Weak,
    Aggressive,
}


#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub val: u8,
    pub buf_idx: usize,
}

impl Init {
    fn new(buf_idx: usize, val: u8) -> Self {
        Self { val, buf_idx }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Change {
    pub buf_idx: usize,
    pub class_id: usize,
    pub lower: usize,
    pub upper: usize,
}

impl Change {
    fn new(idx: usize, class_id: usize, upper: usize) -> Self {
        Self {
            buf_idx: idx,
            class_id,
            lower: 0,
            upper,
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
    fn visit_bounds(&mut self) -> impl Iterator<Item = (&mut usize, &mut usize)> {
        self.changes
            .iter_mut()
            .map(|c| (&mut c.lower, &mut c.upper))
    }
    fn append<F, I, T>(cons: F, tokens: &mut Peekable<I>, target: &mut Vec<T>, other_len: usize)
    where
        I: Iterator<Item = Token>,
        F: Fn(usize) -> T,
    {
        let buf_idx = target.len() + other_len;
        target.push(cons(buf_idx));
        if let Some(&Token::Repeat(n)) = tokens.peek() {
            tokens.next();
            if n > 1 {
                let buf_idx = target.len() + other_len;
                // n - 1 because we always push once before checking for repeats
                target.extend((buf_idx..buf_idx + n - 1).map(cons));
            } else if n == 0 {
                target.pop();
            }
        }
    }
    fn new(tokens: &[Token]) -> Self {
        let mut tokens = tokens.iter().cloned().peekable();
        let mut inits = vec![];
        let mut changes = vec![];
        while let Some(t) = tokens.next() {
            match t {
                Token::Char(c) => Self::append(
                    |buf_idx| Init::new(buf_idx, c),
                    &mut tokens,
                    &mut inits,
                    changes.len(),
                ),
                Token::Class { id, len } => Self::append(
                    |buf_idx| Change::new(buf_idx, id, len),
                    &mut tokens,
                    &mut changes,
                    inits.len(),
                ),
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
        for (slot, max) in self.buf.iter().zip(self.maxs.iter()).rev() {
            let modulo = rem % max;
            slot.set(modulo);
            rem = (rem - modulo) / max;
        }

        self.buf.iter().map(|c| c.get())
    }

    fn total(&self) -> usize {
        // FIXME: this can easily overflow for long queries, maybe enforce u64?
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
    pub fn new(src: &str, num_threads: usize, part: Partition) -> Result<Self, ParseErr> {
        let (statics, mut tokens) = Lexer::tokenize(src)?;
        let (bounds, range_idxs): (Vec<_>, Vec<_>) = tokens
            .iter()
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

        let max_size = runs
            .iter()
            .map(|run| run.len)
            .max()
            .expect("runs is nonempty");

        let tasks = match part {
            Partition::Weak => Self::weak_partition(runs, num_threads),
            Partition::Aggressive => Self::aggressive_partition(runs, num_threads),
        };

        Ok(Self {
            max_size,
            tasks,
            statics,
        })
    }
    fn aggressive_partition(runs: Vec<Run>, n: usize) -> Vec<Vec<Run>>{
        // let changes = self.tasks
        //     .first()
        //     .expect()
        //     .iter()
        //     .map(|run| run.changes.iter().map(|c| c.upper).product())
        //     .collect::<Vec<usize>>();
        // vec![runs; num_tasks]
        vec![runs; n]
    }
    fn weak_partition(runs: Vec<Run>, n: usize) -> Vec<Vec<Run>> {
        vec![runs; n]
    }
}
