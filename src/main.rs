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
    println!("{code}\n\n");
    let mut tokens = tokenize(&code);
    println!("{:#?}\n\n", tokens);
    tokens = filter50s(tokens.clone());
    println!("{:#?}\n\n", tokens);
    tokens = filter_all(tokens.clone());
    println!("{:#?}\n\n", tokens);

    let tokens1 = from_tokens(tokens.clone());
    let tokens2 = tokens.clone();
    let ast = parse_(tokens1, tokens2);

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

    std::fs::write("out.txt", o.program).unwrap();

    let executer = std::process::Command::new("qbackend")
        .arg("out.txt")
        .arg("| cat args.txt")
        .output()
        .expect("Failed to execute qbackend");

    println!("{}", String::from_utf8_lossy(&executer.stdout));
}
