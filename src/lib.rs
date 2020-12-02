pub use bonk_macro::bonk;

pub trait Bonk {
    fn new(thread_id: usize) -> Self;
    fn check(&mut self, buf: &[u8]) -> bool;
}
