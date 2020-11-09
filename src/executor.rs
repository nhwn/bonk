use num_cpus;

pub struct Executor<T: Brute> {
    brutes: Vec<T>
}

impl<T: Brute> Executor<T> {
    pub fn run<F>(pattern: &'static str, initializer: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        Self {
            brutes: (0..num_cpus::get()).map(initializer).collect()
        }
    }
}

pub trait Brute {
    /// return true when a match has been found to signal termination
    fn brute(&mut self, current_guess: &[u8]) -> bool;
}
