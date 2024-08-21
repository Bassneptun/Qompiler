mod parser;
mod tokenizer;

use parser::{from_tokens, ASTNode};
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

    let ast = parse_(from_tokens(tokens.clone()), tokens.clone());
    println!("{:#?}", ast);
    /*
    if let Ok(ASTNode::Program(ref children)) = ast {
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
    */
}
