use bonk::Lexer;

fn main() {
    let toks = Lexer::new("{ 1 }").parse();
    println!("{:?}", toks);
}
