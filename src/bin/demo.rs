use bonk::{bonk, Bonk};

pub struct Attacker;

impl Bonk for Attacker {
    fn new(_id: usize) -> Self {
        Self {}
    }
    fn check(&mut self, buf: &[u8]) -> bool {
        if buf == b"69NICE69" {
            println!("haha got it");
            true
        } else {
            false
        }
    }
}

fn main() {
    bonk! {
        r"\d{2}\A{4}\d{2}",
        Attacker
    }
}
