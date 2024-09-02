mod code_gen;
mod parser;
mod tokenizer;

use std::process::exit;

use code_gen::code_gen;
use parser::from_tokens;
use tokenizer::filter_all;

use crate::parser::parse_;
use crate::tokenizer::{filter50s, tokenize, __TOKENS};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let code = std::fs::read_to_string(path).unwrap();
    let mut tokens = tokenize(&code);
    tokens = filter50s(tokens.clone());
    tokens = filter_all(tokens.clone());

    for (i, tok) in __TOKENS[3..].iter().enumerate() {
        println!("{}, {}", tok, i + 1);
    }

    for (i, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }

    println!("{:?}", tokens.iter().map(|t| t.token).collect::<Vec<_>>());

    let ast = parse_(from_tokens(tokens.clone()), tokens.clone()).expect("failed to parse");
    println!("{:#?}", ast);
    let out = code_gen(ast);
    println!("1");
    let o;
    match out {
        Ok(out) => {
            o = out;
        }
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
    println!("{:?}", o)
}
