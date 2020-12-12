use bonk::{bonk, Bonk};

pub struct Attacker {}

impl Bonk for Attacker {
    fn new(_id: usize) -> Self {
        Self {}
    }
    fn check(&mut self, buf: &[u8]) -> bool {
        if buf == b"CTF{bru3e_f0rc3}" {
            println!("{:?}", &buf);
            true
        } else {
            false
        }
    }
}

fn main() {
    bonk! {
        r"LOL\a{0,3}",
        Attacker,
        true,
        true
    }
}
