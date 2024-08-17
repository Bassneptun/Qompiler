use crate::tokenizer::is_num;
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>), // Root node containing the whole program
    FunctionDef {
        name: String,
        params: Vec<String>,
        signature: (Box<ASTNode>, Vec<ASTNode>),
        body: Option<Box<ASTNode>>,
    },
    VariableDecl {
        name: String,
        value: Option<Box<ASTNode>>,
        type_: Option<Box<ASTNode>>, // not optional yet
        token: i32,
    },
    Block(Vec<ASTNode>), // A block of statements
    For {
        container: Box<ASTNode>,
        alias: String,
        body: Option<Box<ASTNode>>,
    },
    Return(Box<ASTNode>),
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    GateCall {
        name: String,
        args: Vec<ASTNode>,
    },
    Gate {
        name: String,
        gate: Vec<Vec<f32>>,
        args: Vec<ASTNode>,
        arg_names: Vec<String>,
    },
    Struct {
        name: String,
        types: Vec<ASTNode>,
    },
    Array {
        name: String,
        type_: Box<ASTNode>,
        elements: Vec<ASTNode>,
        size: Box<ASTNode>,
    },
    ArrayIndex(u32),
    ArrayAccess {
        name: String,
        index: Box<ASTNode>,
    },
    Reference {
        name: String,
    },
    Dereference {
        name: String,
    },
    Break,
    Void,
    Qbit,
    Qdit,
    Custom,
    Num(i32),
    Type {
        name: String,
        specifier: Box<ASTNode>,
    },
    Range {
        start: Box<ASTNode>,
        end: Box<ASTNode>,
    },
    VariableCall {
        name: String,
    },
    Assignment {
        name: String,
        arr_index: Option<Box<ASTNode>>,
        value: Box<ASTNode>,
    },
    Pointer {
        name: String,
        value: Option<Box<ASTNode>>,
        type_: Box<ASTNode>,
    },
    StructAccess {
        name: String,
        member: String,
    },
    ArrayType {
        type_: Box<ASTNode>,
        size: Box<ASTNode>,
    },
    ExternArg {
        idx: Box<ASTNode>,
    },
}

pub fn create_type(token: Vec<Token>) -> Option<Box<ASTNode>> {
    if token.len() == 1 {
        match token[0].token {
            20 => Some(Box::new(ASTNode::Type {
                name: token[0].value.clone(),
                specifier: Box::new(ASTNode::Qbit),
            })),
            21 => Some(Box::new(ASTNode::Type {
                name: token[0].value.clone(),
                specifier: Box::new(ASTNode::Void),
            })),
            _ => Some(Box::new(ASTNode::Type {
                name: token[0].value.clone(),
                specifier: Box::new(ASTNode::Custom),
            })),
        }
    } else {
        match token[1].token {
            7 => Some(Box::new(ASTNode::ArrayType {
                type_: create_type(vec![token[0].clone()])?,
                size: Box::new(ASTNode::Num(token[2].value.parse().unwrap())),
            })),
            20 => Some(Box::new(ASTNode::Pointer {
                name: token[1].value.clone(),
                value: None,
                type_: Box::new(ASTNode::Type {
                    name: token[1].value.clone(),
                    specifier: Box::new(ASTNode::Qbit),
                }),
            })),
            21 => Some(Box::new(ASTNode::Pointer {
                name: token[1].value.clone(),
                value: None,
                type_: Box::new(ASTNode::Type {
                    name: token[1].value.clone(),
                    specifier: Box::new(ASTNode::Void),
                }),
            })),
            _ => Some(Box::new(ASTNode::Pointer {
                name: token[1].value.clone(),
                value: None,
                type_: Box::new(ASTNode::Type {
                    name: token[1].value.clone(),
                    specifier: Box::new(ASTNode::Custom),
                }),
            })),
        }
    }
}

pub fn parse_arguments(tokens: Vec<Token>) -> (Vec<ASTNode>, usize) {
    let mut args: Vec<ASTNode> = vec![];
    let mut i = 0;
    let mut inside = 1;
    while i < tokens.len() && inside > 0 {
        match tokens[i].token {
            3 => inside += 1,
            4 => inside -= 1,
            11 => {
                i += 1;
            }
            13 => {
                args.push(ASTNode::Reference {
                    name: tokens[i + 1].value.clone(),
                });
                i += 2
            }
            14 => {
                args.push(ASTNode::Dereference {
                    name: tokens[i + 1].value.clone(),
                });
                i += 2
            }
            25..=39 => {
                args.push(ASTNode::GateCall {
                    name: tokens[i + 1].value.clone(),
                    args: parse_arguments(tokens[i + 2..].to_vec()).0,
                });
                i += parse_arguments(tokens[i + 2..].to_vec()).1
            }
            51 => {
                args.push(parse_51(tokens[i..].to_vec()).0);
                i += parse_51(tokens[i..].to_vec()).1
            }
            _ => panic!("Invalid token: {:?}", tokens[i].token),
        }
    }
    (args, i + 3)
}

pub fn parse_51(tokens: Vec<Token>) -> (ASTNode, usize) {
    match tokens[1].token {
        3 => (
            ASTNode::FunctionCall {
                name: tokens[0].value.clone(),
                args: parse_arguments(tokens[2..].to_vec()).0,
            },
            parse_arguments(tokens[2..].to_vec()).1 + 1,
        ),
        7 => {
            if tokens[2].token == 51 {
                (
                    ASTNode::ArrayAccess {
                        name: tokens[0].value.clone(),
                        index: Box::new(ASTNode::ArrayIndex(
                            tokens[2]
                                .value
                                .parse()
                                .expect("this should be a number at this point..."),
                        )),
                    },
                    3,
                )
            } else {
                (
                    ASTNode::ExternArg {
                        idx: Box::new(ASTNode::ArrayIndex(
                            tokens[3]
                                .value
                                .parse()
                                .expect("this should be a number as well"),
                        )),
                    },
                    4,
                )
            }
        }
        40 => (
            ASTNode::StructAccess {
                name: tokens[0].value.clone(),
                member: tokens[2].value.clone(),
            },
            3,
        ),
        _ => (
            ASTNode::VariableCall {
                name: tokens[0].value.clone(),
            },
            1,
        ),
    }
}

pub fn elems(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut elements: Vec<ASTNode> = vec![];
    let mut inside = 0 as usize;
    let mut i = 0;
    for tok in tokens.clone() {
        match tok.token {
            51 => elements.push(parse_51(tokens.clone()).0),
            52 => elements.push(ASTNode::Num(tok.value.parse().unwrap())),
            5 => inside += 1,
            6 => inside -= 1,
            _ => panic!("Invalid token: {:?}", tok.token),
        }
        if inside == 0 && i != 0 {
            break;
        }
        i += 1;
    }
    elements
}

pub fn parse_array_decl(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    let index = if tokens[3].token == 14 { 6 } else { 5 };
    if tokens.iter().position(|x| x.token == 5).is_some() {
        if tokens.iter().position(|x| x.token == 10).unwrap()
            < tokens.iter().position(|x| x.token == 5).unwrap()
        {
            (
                ASTNode::Array {
                    name: tokens[1].value.clone(),
                    size: Box::new(ASTNode::Num(tokens[index].value.parse().unwrap())),
                    type_: create_type(tokens[3..(index - 1)].to_vec().clone()).unwrap(),
                    elements: vec![],
                },
                tokens.iter().position(|x| x.token == 10).unwrap() + 1 + q,
            )
        } else {
            (
                ASTNode::Array {
                    name: tokens[1].value.clone(),
                    size: Box::new(ASTNode::Num(tokens[index].value.parse().unwrap())),
                    type_: create_type(tokens[3..(index - 1)].to_vec().clone()).unwrap(),
                    elements: elems(
                        tokens[tokens.iter().position(|x| x.token == 5).unwrap()..]
                            .to_vec()
                            .clone(),
                    ),
                },
                tokens.iter().position(|x| x.token == 10).unwrap() + 1 + q,
            )
        }
    } else {
        (
            ASTNode::Array {
                name: tokens[1].value.clone(),
                size: Box::new(ASTNode::Num(tokens[index].value.parse().unwrap())),
                type_: create_type(tokens[3..(index - 1)].to_vec().clone()).unwrap(),
                elements: vec![],
            },
            tokens.iter().position(|x| x.token == 10).unwrap() + 1 + q,
        )
    }
}

pub fn parse_variable_decl(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    if tokens[4].token == 7 {
        return parse_array_decl(tokens, q);
    }
    if tokens[2].token == 10 {
        // semicolon found right after variable declaration
        return (
            ASTNode::VariableDecl {
                name: tokens[1].value.clone(),
                value: None,
                type_: None,
                token: tokens[2].token as i32,
            },
            3 + q,
        );
    } else if tokens[2].token == 9 {
        // type specifier found, is expected yet
        if tokens[4].token == 10 || (tokens[3].token == 14 && tokens[5].token == 10) {
            // semicolon found right after variable declaration, or the same but with a pointer type
            return (
                ASTNode::VariableDecl {
                    name: tokens[1].value.clone(),
                    value: None,
                    type_: create_type(vec![tokens[3].clone()]),
                    token: tokens[0].token as i32,
                },
                5 + q + (if tokens[3].token == 14 { 1 } else { 0 }),
            );
        } else if tokens[2].token == 9 {
            // equal sign expected here, but not checked for. Will be checked later
            let index = if tokens[5].token == 12 { 6 } else { 5 };
            match tokens[index].token {
                13 => {
                    return (
                        ASTNode::VariableDecl {
                            name: tokens[1].value.clone(),
                            value: Some(Box::new(ASTNode::Reference {
                                name: tokens[index + 1].value.clone(),
                            })),
                            type_: create_type(tokens[3..index].to_vec().clone()),
                            token: tokens[0].token as i32,
                        },
                        9 + q,
                    )
                }
                14 => {
                    return (
                        ASTNode::VariableDecl {
                            name: tokens[1].value.clone(),
                            value: Some(Box::new(ASTNode::Dereference {
                                name: tokens[index + 1].value.clone(),
                            })),
                            type_: create_type(tokens[3..index].to_vec().clone()),
                            token: tokens[0].token as i32,
                        },
                        8 + q,
                    )
                }
                25..=39 => {
                    return (
                        ASTNode::VariableDecl {
                            name: tokens[1].value.clone(),
                            value: Some(Box::new(ASTNode::GateCall {
                                name: tokens[index].value.clone(),
                                args: parse_arguments(tokens[index + 2..].to_vec()).0,
                            })),
                            type_: create_type(tokens[3..index].to_vec().clone()),
                            token: tokens[0].token as i32,
                        },
                        parse_arguments(tokens[index + 2..].to_vec()).1
                            + q
                            + [3..index].to_vec().len()
                            - 1,
                    )
                }
                51 => {
                    return (
                        ASTNode::VariableDecl {
                            name: tokens[1].value.clone(),
                            value: Some(Box::new(parse_51(tokens[index..].to_vec()).0)),
                            type_: create_type(tokens[3..index].to_vec().clone()),
                            token: tokens[0].token as i32,
                        },
                        parse_51(tokens[index..].to_vec()).1 + q + [3..index].to_vec().len() - 1,
                    )
                }
                52 => {
                    return (
                        ASTNode::VariableDecl {
                            name: tokens[1].value.clone(),
                            value: Some(Box::new(ASTNode::Num(
                                tokens[index].value.parse().unwrap(),
                            ))),
                            type_: create_type(tokens[3..index].to_vec().clone()),
                            token: tokens[0].token as i32,
                        },
                        7 + q,
                    )
                }
                _ => panic!("Invalid token: {:?}", tokens[6].token),
            }
        } else {
            panic!("Invalid token: {:?}", tokens[2].token);
        }
    } else if tokens[2].token == 12 {
        // equal sign found, type deduction is expected
        match tokens[3].token {
            13 => {
                return (
                    ASTNode::VariableDecl {
                        name: tokens[1].value.clone(),
                        value: Some(Box::new(ASTNode::Reference {
                            name: tokens[4].value.clone(),
                        })),
                        type_: None,
                        token: tokens[0].token as i32,
                    },
                    8 + q,
                )
            }
            14 => {
                return (
                    ASTNode::VariableDecl {
                        name: tokens[1].value.clone(),
                        value: Some(Box::new(ASTNode::Dereference {
                            name: tokens[4].value.clone(),
                        })),
                        type_: None,
                        token: tokens[0].token as i32,
                    },
                    8 + q,
                )
            }
            25..=39 => {
                return (
                    ASTNode::VariableDecl {
                        name: tokens[1].value.clone(),
                        value: Some(Box::new(ASTNode::GateCall {
                            name: tokens[3].value.clone(),
                            args: parse_arguments(tokens[5..].to_vec()).0,
                        })),
                        type_: None,
                        token: tokens[0].token as i32,
                    },
                    parse_arguments(tokens[5..].to_vec()).1 + q + 3,
                )
            }
            51 => {
                return (
                    ASTNode::VariableDecl {
                        name: tokens[1].value.clone(),
                        value: Some(Box::new(parse_51(tokens[3..].to_vec()).0)),
                        type_: None,
                        token: tokens[0].token as i32,
                    },
                    parse_51(tokens[3..].to_vec()).1 + q,
                )
            }
            52 => {
                return (
                    ASTNode::VariableDecl {
                        name: tokens[1].value.clone(),
                        value: Some(Box::new(ASTNode::Num(tokens[3].value.parse().unwrap()))),
                        type_: None,
                        token: tokens[0].token as i32,
                    },
                    5 + q,
                )
            }
            _ => panic!("Invalid token: {:?}", tokens[6].token),
        }
    } else {
        panic!("this should really not happen");
    }
}

pub fn parse_rval(tokens: Vec<Token>) -> (ASTNode, usize) {
    match tokens[0].token {
        13 => (
            ASTNode::Reference {
                name: tokens[1].value.clone(),
            },
            2,
        ),
        14 => (
            ASTNode::Dereference {
                name: tokens[1].value.clone(),
            },
            2,
        ),
        25..=39 => (
            ASTNode::GateCall {
                name: tokens[0].value.clone(),
                args: parse_arguments(tokens[2..].to_vec()).0,
            },
            parse_arguments(tokens[2..].to_vec()).1,
        ),
        51 => {
            if tokens.len() > 2 {
                (
                    parse_51(tokens[2..].to_vec()).0,
                    parse_51(tokens[2..].to_vec()).1,
                )
            } else {
                (
                    ASTNode::VariableCall {
                        name: tokens[0].value.clone(),
                    },
                    1,
                )
            }
        }
        52 => (ASTNode::Num(tokens[0].value.parse().unwrap()), 1),
        _ => panic!(
            "Invalid token: {:?}, {:?}",
            tokens[0].token, tokens[0].value
        ),
    }
}

pub fn find_args_and_signature(tokens: Vec<Token>) -> (Vec<String>, Vec<ASTNode>) {
    let mut args: Vec<String> = vec![];
    let mut signature: Vec<ASTNode> = vec![];
    let mut is_type = false;
    for tok in tokens[tokens.iter().position(|s| s.token == 3).unwrap()..].iter() {
        match tok.token {
            9 => is_type = true,
            4 => break,
            11 => is_type = false,
            20 => {
                if is_type {
                    signature.push(ASTNode::Type {
                        name: tok.value.clone(),
                        specifier: Box::new(ASTNode::Qbit),
                    });
                    is_type = false;
                } else {
                    panic!(
                        "expected name but got compiler-reserved keyword: {:?}",
                        tok.value
                    );
                }
            }
            21 => panic!("void is neither a valid name nor type in function defenitions"),
            50 => {
                if !is_type {
                    args.push(tok.value.clone());
                    is_type = true;
                } else {
                    panic!("unknown type_: {:?}", tok.value);
                }
            }
            51 => {
                if !is_type {
                    args.push(tok.value.clone());
                    is_type = true;
                } else {
                    signature.push(*create_type(vec![tok.clone()]).unwrap());
                    is_type = false;
                }
            }
            _ => {}
        }
    }
    (args, signature)
}

pub fn find_bounds(tokens: Vec<Token>) -> (usize, usize) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    for tok in &tokens {
        if tok.token == 5 {
            j += 1;
            k = 1;
            break;
        } else {
            i += 1;
            j += 1;
        }
    }
    while k != 0 {
        if tokens[j].token == 5 {
            k += 1;
        } else if tokens[j].token == 6 {
            k -= 1;
        }
        j += 1;
    }

    (i + 1, j)
}

pub fn extract(node: ASTNode) -> Box<ASTNode> {
    if let ASTNode::Program(ref r) = node {
        return Box::new(ASTNode::Block((*r).clone()));
    } else {
        panic!("this should not happen");
    }
}

pub fn parse_function_def(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    let args: Vec<String> = find_args_and_signature(tokens.clone()).0;
    let signature = (
        create_type(tokens.clone()).unwrap(),
        find_args_and_signature(tokens.clone()).1,
    );
    let name = tokens[1].value.clone();

    let (b_1, b_2) = find_bounds(tokens.clone());
    let body: Option<ASTNode>;
    let a_body;

    if b_2 - b_1 > 3 {
        body = Some(parse(tokens[b_1..b_2].to_vec()));
    } else {
        body = None;
    }

    if !body.is_none() {
        a_body = Some(extract(parse(tokens[b_1..b_2].to_vec())));
    } else {
        a_body = None;
    }

    let node = ASTNode::FunctionDef {
        name,
        params: args,
        signature,
        body: a_body,
    };
    (node, find_bounds(tokens.clone()).1 + q)
}

pub fn fetch_args_and_names(tokens: Vec<Token>) -> (Vec<ASTNode>, Vec<String>) {
    let mut args: Vec<ASTNode> = vec![];
    let mut names: Vec<String> = vec![];
    let mut is_type = false;
    for tok in tokens[1..].iter() {
        match tok.token {
            9 => is_type = true,
            4 => break,
            15 => {
                if !is_type {
                    panic!("error");
                } else {
                    args.push(ASTNode::Type {
                        name: tok.value.clone(),
                        specifier: Box::new(ASTNode::Qdit),
                    });
                }
            }
            50 => {
                if !is_type {
                    names.push(tok.value.clone());
                    is_type = false;
                } else {
                    panic!("unknown type_: {:?}", tok.value);
                }
            }
            51 => {
                if !is_type {
                    names.push(tok.value.clone());
                    let _ = !is_type;
                } else {
                    args.push(*create_type(vec![tok.clone()]).unwrap());
                    let _ = !is_type;
                }
            }
            _ => {}
        }
    }
    (args, names)
}

pub fn parse_gate_body(tokens: Vec<Token>) -> (Vec<Vec<f32>>, usize) {
    let (start, end) = find_bounds(tokens.clone());

    let mut args: Vec<Vec<f32>> = vec![];
    let mut i = start;
    let mut current: Vec<f32> = vec![];
    while i < end - 1 {
        match tokens[i].token {
            6 => {
                args.push(current.clone());
                current = vec![];
            }
            50 => {
                current.push(tokens[i].value.parse::<f32>().unwrap());
            }
            _ => {}
        }
        i += 1;
    }
    (args, end)
}

pub fn parse_gate(tokens: Vec<Token>) -> (ASTNode, usize) {
    let name = tokens[1].value.clone();
    let (args, arg_names) = fetch_args_and_names(tokens[2..].to_vec());

    let body = parse_gate_body(tokens[2..].to_vec());

    let node = ASTNode::Gate {
        name,
        args,
        arg_names,
        gate: body.0,
    };
    (node, body.1)
}

pub fn parse_gate_call(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    (
        ASTNode::GateCall {
            name: tokens[0].value.clone(),
            args: parse_arguments(tokens[2..].to_vec()).0,
        },
        parse_arguments(tokens[2..].to_vec()).1 + q + 1,
    )
}

pub fn parse_range(tokens: Vec<Token>) -> (ASTNode, usize) {
    (
        ASTNode::Range {
            start: Box::new(ASTNode::Num(tokens[0].value.parse().unwrap())),
            end: Box::new(ASTNode::Num(tokens[2].value.parse().unwrap())),
        },
        3,
    )
}

pub fn parse_for(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    let is_range = tokens[..tokens.iter().position(|x| x.token == 5).unwrap()]
        .to_vec()
        .contains(&Token {
            token: 15,
            value: String::from(".."),
        });
    let i = tokens.iter().position(|x| x.token == 3).unwrap();
    let alias = tokens[i + 1].value.clone();
    let collection: ASTNode;

    if !is_range {
        collection =
            parse_rval(tokens[i + 3..tokens.iter().position(|x| x.token == 4).unwrap()].to_vec()).0;
    } else {
        collection = parse_range(
            tokens[tokens.iter().position(|x| is_num(x.value.clone())).unwrap()
                ..=tokens.iter().position(|x| is_num(x.value.clone())).unwrap() + 2]
                .to_vec()
                .clone(),
        )
        .0;
    }

    let (b_1, b_2) = find_bounds(tokens.clone());
    //print!("{}, {},\n {:?},\n {:?}", b_1, b_2, tokens[b_1], tokens[b_2]);
    let body: Option<ASTNode>;
    let a_body;

    if b_2 - b_1 > 3 {
        body = Some(parse(tokens[b_1..b_2].to_vec()));
    } else {
        body = None;
    }

    if !body.is_none() {
        a_body = Some(extract(parse(tokens[b_1..b_2].to_vec())));
    } else {
        a_body = None;
    }

    (
        ASTNode::For {
            alias,
            container: Box::new(collection),
            body: a_body,
        },
        b_2 + q,
    )
}

pub fn parse_assignment(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    if tokens[1].token == 12 {
        (
            ASTNode::Assignment {
                name: tokens[0].value.clone(),
                value: Box::new(
                    parse_rval(
                        tokens[2..tokens.iter().position(|x| x.token == 10).unwrap()].to_vec(),
                    )
                    .0,
                ),
                arr_index: None,
            },
            parse_rval(tokens[2..].to_vec()).1 + q + 3,
        )
    } else {
        (
            ASTNode::Assignment {
                name: tokens[0].value.clone(),
                arr_index: Some(Box::new(ASTNode::ArrayIndex(
                    tokens[2].value.parse().expect("this should be a number..."),
                ))),
                value: Box::new(
                    parse_rval(
                        tokens[5..tokens.iter().position(|x| x.token == 10).unwrap()].to_vec(),
                    )
                    .0,
                ),
            },
            parse_rval(tokens[5..].to_vec()).1 + q,
        )
    }
}

pub fn parse_51_2(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    if tokens[..tokens.iter().position(|x| x.token == 10).unwrap()]
        .to_vec()
        .contains(&Token {
            token: 12,
            value: String::from("="),
        })
    {
        return parse_assignment(tokens, q);
    } else {
        let (a, b) = parse_51(tokens);
        return (a, b + q);
    }
}

pub fn get_types(tokens: Vec<Token>) -> (Vec<ASTNode>, usize) {
    let bound = find_bounds(tokens.clone()).1 - 1;
    let tokens_copy = tokens[1..].to_vec().clone();
    let mut i = 0;
    let mut types: Vec<ASTNode> = vec![];
    while i < bound - 2 {
        types.push(parse_variable_decl(tokens_copy[i..].to_vec(), i).0);
        i = parse_variable_decl(tokens_copy[i..].to_vec(), i).1;
    }

    (types, i + 1)
}

pub fn parse_struct_def(tokens: Vec<Token>, q: usize) -> (ASTNode, usize) {
    (
        ASTNode::Struct {
            name: tokens[1].value.clone(),
            types: get_types(tokens[2..].to_vec()).0,
        },
        get_types(tokens[2..].to_vec()).1 + q + 3,
    )
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut root: ASTNode = ASTNode::Program(vec![]);
    let mut i = 0;

    while i < tokens.len() {
        println!("{:?}", tokens[i]);
        match tokens[i].token {
            16..=18 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_variable_decl(tokens[i..].to_vec(), i).0, );
                } else {
                    panic!("this serves no purpose, I am afraid of the Compiler");
                }
                i = parse_variable_decl(tokens[i..].to_vec(), i).1;
            },
            19 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_struct_def(tokens[i..].to_vec(), i).0);
                } else {
                    panic!("same as before");
                }
                i = parse_struct_def(tokens[i..].to_vec(), i).1;
            },
            20..=21 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_function_def(tokens[i..].to_vec(), i).0);
                } else {
                    panic!("same as before");
                }
                i = parse_function_def(tokens[i..].to_vec(), i).1;
            },
            24 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_gate(tokens[i..].to_vec()).0);
                } else {
                    panic!("same as before");
                }
                i = parse_gate(tokens[i..].to_vec()).1;
            },
            25..=39 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_gate_call(tokens[i..].to_vec(), i).0);
                } else {
                    panic!("same as before");
                }
                i = parse_gate_call(tokens[i..].to_vec(), i).1;
            },
            42 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_for(tokens[i..].to_vec(), i).0);
                } else {
                    panic!("same as before");
                }
                i = parse_for(tokens[i..].to_vec(), i).1;
            },
            44 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(ASTNode::Return(
                        Box::new(parse_rval(tokens[i..tokens.iter().position(|x| x.token == 10).unwrap()].to_vec()).0)
                    ));
                } else {
                    panic!("same as before");
                }
                i = parse_rval(tokens[i..tokens.iter().position(|x| x.token == 10).unwrap()].to_vec()).1;
            },
            45 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(ASTNode::Break);
                } else {
                    panic!("same as before");
                }
                i += 2;
            },
            51 => {
                if let ASTNode::Program(ref mut r) = root {
                    r.push(parse_51_2(tokens[i..].to_vec(), i).0);
                } else {
                    panic!("same as before");
                }
                i = parse_51_2(tokens[i..].to_vec(), i).1;
            }
            _ => panic!("if you see this, something went wrong, and I am afraid there's a chance it isn't even your fault. current Token: {}, {}, token nr. {}", tokens[i].token, tokens[i].value, i),
        }
    }
    root
}
