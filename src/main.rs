mod code_gen;
mod parser;
mod tokenizer;

use std::process::exit;

use code_gen::code_gen;
use parser::from_tokens;
use tokenizer::filter_all;

use crate::parser::parse_;
use crate::tokenizer::{filter50s, tokenize};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let code = std::fs::read_to_string(path).unwrap();
    let mut tokens = tokenize(&code);
    tokens = filter50s(tokens.clone());
    tokens = filter_all(tokens.clone());

    let ast = parse_(from_tokens(tokens.clone()), tokens.clone());

    let ast_ = match ast {
        Ok(ast) => ast,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    println!("{:#?}", ast_);

    let out = code_gen(ast_);
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
    println!("{o:#?}\n\n{}", o.program);
}
