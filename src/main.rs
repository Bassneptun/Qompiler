mod code_gen;
mod parser;
mod tokenizer;

use std::collections::HashMap;

use code_gen::{convert_to_string, Comptime};
use parser::ASTNode;

use crate::parser::parse;
use crate::tokenizer::{filter50s, tokenize, __TOKENS};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let code = std::fs::read_to_string(path).unwrap();
    let mut tokens = tokenize(&code);
    tokens = filter50s(tokens.clone());

    for (i, tok) in __TOKENS.iter().enumerate() {
        println!("{}, {}", tok, i);
    }

    for (i, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }

    println!("{:?}", tokens.iter().map(|t| t.token).collect::<Vec<_>>());

    let ast = parse(tokens);
    println!("{:#?}", ast);
    for (i, token) in __TOKENS.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }
    if let ASTNode::Program(ref children) = ast {
        for child in children {
            println!("{}", convert_to_string(&child))
        }
    }
    let cmpt: Comptime = Comptime {
        program: String::new(),
        functions: HashMap::new(),
        function_info: HashMap::new(),
        vars: HashMap::new(),
        var_info: HashMap::new(),
        tmp_vars: HashMap::new(),
        tmp_var_info: HashMap::new(),
        types: HashMap::new(),
        structs: HashMap::new(),
        i: 0,
    };
    let out = code_gen::generate_code(&ast, cmpt).program;
    println!("{:?}", out)
}
