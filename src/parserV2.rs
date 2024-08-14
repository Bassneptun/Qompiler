// this is version 2 of the parser, the old parser had a faulty design which made it unable to
// parse a lot of things

include!("tokenizer.rs");

enum ASTNode{
    Program(Vec<ASTNode>),
    Variable{
        is_const: bool,
        name: String,
        type_: Option<ASTNode>,
        value: Option<ASTNode>,
    }, // variable declarations with let or const bind to the next token always. whether or not it
    // will bind to something next is determined by the next token.
    Function{
        type_: Box<ASTNode>,
        name: String,
        args: Box<ASTNode>,
        body: Box<ASTNode>,
    }, // types always create a function definition if it is not a bind call. the args node is
    // supposed to be an "initializer" node, the body the "body" node
    Gate_{
        name: String,
        args: Vec<ASTNode>,
        body: Box<ASTNode>,
    }, // "gate" binds to name as usual, args should consist of types found in ASTNode, qbit and
    // qdit, body should be of type int, double, complex, cos-, sin- or exp- call. may contain
    // another of those types.
    For{
        alias: String,
        container: Box<ASTNode>,
        body: Box<ASTNode>,
    }, // "for" binds to name as usual, container should be of array type or range, body as usual
    Type{
        name: String,
        specifier: Box<ASTNode>,
        size: Box<ASTNode>,
    }, // a type must always have a name, the specifier is of initializer type so that I won't have
    // to create another node that does the same fucking thing. size is of type int and dictates
    // the size the type has in qbits.
    Range{
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        size: Box<ASTNode>,
    }, // all of the parameters are of type int, their function should be self explanatory
    Array{
        size: Box<ASTNode>,
        type_: Box<ASTNode>,
    }, // the array type is created by binding backwards from when the "[" token is found. then it
    // will take the last node as the type and the next token as the size
    Pointer{
        type_: Box<ASTNode>,
    }, // a pointer type is created by binding forwards from when the "*" token is found
    Initializer{
        types: Vec<ASTNode>,
        names: Vec<String>,
    },
    Name(String),
    Block(Vec<ASTNode>),
    Gate(Vec<ASTNode>),
    Int(i32),
    Double(f64),
    Complex(f64, f64),
    Cos(Box<ASTNode>),
    Sin(Box<ASTNode>),
    Exp(Box<ASTNode>),
}

fn parsev2(input: Token, tokens: Vec<Token>, position: usize, is_bind_call: bool, last: Option<ASTNode>) -> (ASTNode, usize) {
    match token.token {
        3 => (ASTNode::Initializer{
            types: tokens[position+1..].to_vec().iter().take_while(|t| t.token != 4).map(|t| parsev2(t.clone(), tokens.clone(), position, false, last).0).filter(|n| matches!(n, ASTNode::Type{..})).collect(), // logic faulty with last var
            types: tokens[position+1..].to_vec().iter().take_while(|t| t.token != 4).map(|t| parsev2(t.clone(), tokens.clone(), position, false, last).0).filter(|n| matches!(n, ASTNode::Name{..})).map(|m| m.0).collect(), // same here
        }, position + tokens[position+1..].to_vec().iter().take_while(|t| t.token != 4).count()),
        4 => continue,
        5 => {
            if is_bind_call{
                (ASTNode::Block(tokens[position+1..].to_vec().iter().map(|t| parsev2(t.clone(), tokens.clone(), position, false, last).0).collect()), position + tokens[position+1..].to_vec().iter().count()) // here as well
            } else {
                todo!()
            }
        },
        6 => continue,
        7 => {
            match last {
                Some(ASTNode::Type{..}) => {
                    last.unwrap().size = Box::new(ASTNode::Int(tokens[position+1].value.parse().unwrap()) * last.unwrap().size;
                    last.unwrap().type_ = Box::new(ASTNode::Array{
                        type_: Box::new(last.unwrap().type_),
                    });
                }
            }
        } 
    }
}
