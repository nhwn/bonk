use bonk::{Brute, Executor};

struct MyBrute();

impl Brute for MyBrute {
    fn brute(&mut self, current_guess: &[u8]) -> bool {
        println!("{:?}", current_guess);
        false
    }
}

fn main() {
    Executor::run("ASDF", |_thread_id| MyBrute());
}
