include!("parser.rs");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let code = std::fs::read_to_string(path).unwrap();
    let mut tokens = tokenize(&code);
    tokens = filter50s(tokens.clone());
    for (i, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }

    println!("{:?}", tokens.iter().map(|t| t.token).collect::<Vec<_>>());

    let ast = parse(tokens);
    println!("{:#?}", ast);
    for (i, token) in __TOKENS.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }
}
