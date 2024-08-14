include!("code_gen.rs");

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let args: Vec<std::string::String> = from_stdin(input);
    let toks = parse(args);
    let root = from_tokens(toks);
    println!("{:?}", root);
}
