use bonk::bonk;

fn main() {
    let time = std::time::Instant::now();
    bonk!(r"SKY-\A{4}-\d{4}");
    println!("{:?}", time.elapsed());
}
