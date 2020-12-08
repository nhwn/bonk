use bonk::{bonk, Bonk};

pub struct Attacker;

impl Bonk for Attacker {
    fn new(_id: usize) -> Self {
        Self {}
    }
    fn check(&mut self, buf: &[u8]) -> bool {
        println!("{:?}", std::str::from_utf8(&buf).unwrap());
        false
    }
}

fn main() {
    bonk! {
        r"a{3}\a{10}",
        Attacker
    }
}
