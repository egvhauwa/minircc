use std::env;

use minircc::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let file = &args[1];

    let lexer = Lexer::new(
        "{}() 456return !
    int pancake         
    ",
    );

    for token in lexer {
        println!("{:?}", token);
    }
}
