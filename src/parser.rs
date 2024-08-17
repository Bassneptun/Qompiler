use std::iter::Peekable;

use crate::tokenizer::is_num;
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>), // Root node containing the whole program
    FunctionDef {
        name: String,
        ret_type: Box<ASTNode>,
        in_type: Vec<ASTNode>,
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
    IntCall {
        name: String,
    },
    PointerType {
        type_: Box<ASTNode>,
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

#[derive(Debug)]
enum Tok {
    OBracket,
    CBracket,
    OCBracket,
    CCBracket,
    OSBracket,
    CSBracket,
    DoublePoint,
    Semicolon,
    Comma,
    Equal,
    Reference,
    Star,
    DotDot,
    VarDecl,
    ConstDecl,
    Struct,
    Qbit,
    Void,
    Hash,
    Macro,
    GateDecl,
    GateCall,
    Dot,
    If,
    For,
    In,
    Return,
    Break,
    PHPRef,
    New,
    Old,
    Num,
}

fn from_tokens(tokens: Vec<Token>) -> Vec<Tok> {
    let mut toks = Vec::new();
    for t in tokens {
        match t.token {
            1 => toks.push(Tok::OBracket),
            2 => toks.push(Tok::CBracket),
            3 => toks.push(Tok::OCBracket),
            4 => toks.push(Tok::CCBracket),
            5 => toks.push(Tok::OSBracket),
            6 => toks.push(Tok::CSBracket),
            7 => toks.push(Tok::DoublePoint),
            8 => toks.push(Tok::Semicolon),
            9 => toks.push(Tok::Comma),
            10 => toks.push(Tok::Equal),
            11 => toks.push(Tok::Reference),
            12 => toks.push(Tok::Star),
            13 => toks.push(Tok::DotDot),
            14 => toks.push(Tok::VarDecl),
            15 => toks.push(Tok::ConstDecl),
            16 => toks.push(Tok::Struct),
            17 => toks.push(Tok::Qbit),
            18 => toks.push(Tok::Void),
            19 => toks.push(Tok::Hash),
            20 => toks.push(Tok::Macro),
            21 => toks.push(Tok::GateDecl),
            22..=37 => toks.push(Tok::GateCall),
            38 => toks.push(Tok::Dot),
            39 => toks.push(Tok::If),
            40 => toks.push(Tok::For),
            41 => toks.push(Tok::In),
            42 => toks.push(Tok::Return),
            43 => toks.push(Tok::Break),
            44 => toks.push(Tok::PHPRef),
            50 => toks.push(Tok::New),
            51 => toks.push(Tok::Old),
            52 => toks.push(Tok::Num),
            _ => panic!("I don't know how this would even ever happen"),
        }
    }
    toks
}

fn parse_(tokens: Vec<Tok>, tokens2: Vec<Token>) -> Result<ASTNode, String> {
    let mut token_iter = tokens.into_iter().peekable(); // This returns Peekable<IntoIter<Tok>>
    let mut token_iter2 = tokens2.into_iter().peekable(); // This returns Peekable<IntoIter<Tok>>
    parse_program(&mut token_iter, &mut token_iter2)
}

fn parse_program<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<ASTNode, String>
where
    I: Iterator<Item = Tok>, // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>,
{
    let mut nodes = Vec::new();
    while let Some(node) = parse_statement(tokens, tokens2)? {
        nodes.push(node);
    }
    Ok(ASTNode::Program(nodes))
}

fn parse_statement<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    match tokens.peek() {
        Some(Tok::Qbit) => parse_function_def_(tokens, tokens2),
        Some(Tok::Void) => parse_function_def_(tokens, tokens2),
        Some(Tok::If) => parse_if(tokens, tokens2),
        Some(Tok::For) => parse_for_(tokens, tokens2),
        Some(Tok::VarDecl) => parse_var_decl(tokens, tokens2),
        Some(Tok::Struct) => parse_struct_def_(tokens, tokens2),
        Some(Tok::GateCall) => parse_gate_call_(tokens, tokens2),
        Some(Tok::ConstDecl) => parse_var_decl(tokens, tokens2), // disambiguity ends here, now it
        // gets really fucked.
        Some(Tok::Old) => parse_any_(tokens, tokens2),
        Some(Tok::Star) => parse_any_2(tokens, tokens2),
        None => Ok(None),
        _ => Ok(Some(ASTNode::Void)),
    }
}

fn advance<I, I2>(tokens: &mut Peekable<I>, tokens2: &mut Peekable<I2>) -> ()
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    tokens.next();
    tokens2.next();
}

fn parse_template<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}

fn parse_function_def_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let type_ = parse_type_(tokens, tokens2);
    let mut type__ = ASTNode::Void;
    match type_ {
        Ok(v) => type__ = v.expect("Error: didn't find valid type in function declaration"),
        Err(e) => println!("Error: {e:?}"),
    }
    let name_ = parse_name(tokens, tokens2);
    let mut name = String::new();
    match name_ {
        Ok(v) => name = v,
        Err(e) => println!("Error: {e:?}"),
    }
    let arguments = parse_arguments_(tokens, tokens2);
    let mut arguments_ = vec![];
    match arguments {
        Ok(o) => arguments_ = o,
        Err(e) => println!("Error: {e:?}"),
    }
    let mut body = None;
    let body_ = parse_body_(tokens, tokens2);
    match body_ {
        Ok(Some(v)) => body = Some(Box::new(v)),
        Ok(None) => body = None,
        Err(e) => println!("Error: {e:?}"),
    }

    Ok(Some(ASTNode::FunctionDef {
        name,
        ret_type: Box::new(type__),
        in_type: arguments_,
        body,
    }))
}
fn parse_type_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    match tokens.peek() {
        None => Err("expected type, got None".to_string()),
        Some(Tok::Void) => Ok(Some(ASTNode::Void)),
        Some(Tok::Qbit) => {
            advance(tokens, tokens2);
            match tokens.peek() {
                None => Ok(Some(ASTNode::Qbit)),
                Some(Tok::OSBracket) => {
                    advance(tokens, tokens2);
                    match tokens.peek() {
                        Some(Tok::Num) => {
                            let n: i32 = tokens2.peek().unwrap().clone().value.parse().unwrap();
                            advance(tokens, tokens2);
                            match tokens.peek() {
                                None => Err("Expected ], got None".to_string()),
                                Some(Tok::CSBracket) => {
                                    advance(tokens, tokens2);
                                    Ok(Some(ASTNode::ArrayType {
                                        type_: Box::new(ASTNode::Qbit),
                                        size: Box::new(ASTNode::Num(n)),
                                    }))
                                }
                                Some(_) => Err(format!(
                                    "expected ], got {}",
                                    tokens2.peek().unwrap().value.clone()
                                )),
                            }
                        }
                        Some(Tok::PHPRef) => {
                            advance(tokens, tokens2);
                            match tokens.peek() {
                                None => Err("Expected index to external variable array, got None"
                                    .to_string()),
                                Some(Tok::Num) => {
                                    let n: u32 =
                                        tokens2.peek().unwrap().clone().value.parse().unwrap();
                                    advance(tokens, tokens2);
                                    match tokens.peek() {
                                        None => Err("Expected ], got None".to_string()),
                                        Some(Tok::CSBracket) => {
                                            advance(tokens, tokens2);
                                            Ok(Some(ASTNode::ArrayType {
                                                type_: Box::new(ASTNode::Qbit),
                                                size: Box::new(ASTNode::ExternArg {
                                                    idx: Box::new(ASTNode::ArrayIndex(n)),
                                                }),
                                            }))
                                        }
                                        Some(_) => Err(format!(
                                            "expected ], got {}",
                                            tokens2.peek().unwrap().value.clone()
                                        )),
                                    }
                                }
                                Some(Tok::Old) => {
                                    let n = tokens2.peek().unwrap().value.clone();
                                    advance(tokens, tokens2);
                                    match tokens.peek() {
                                        None => Err("Expected ], got None".to_string()),
                                        Some(Tok::CSBracket) => {
                                            advance(tokens, tokens2);
                                            Ok(Some(ASTNode::ArrayType {
                                                type_: Box::new(ASTNode::Qbit),
                                                size: Box::new(ASTNode::ExternArg {
                                                    idx: Box::new(ASTNode::IntCall { name: n }),
                                                }),
                                            }))
                                        }
                                        Some(_) => Err(format!(
                                            "expected ], got {}",
                                            tokens2.peek().unwrap().value.clone()
                                        )),
                                    }
                                }
                                Some(_) => Err(format!(
                                    "expected num or for variable, got {}",
                                    tokens2.peek().unwrap().value.clone()
                                )),
                            }
                        }
                        Some(Tok::Old) => {
                            let name = tokens2.peek().unwrap().value.clone();
                            advance(tokens, tokens2);
                            if let Some(Tok::CSBracket) = tokens.peek() {
                                advance(tokens, tokens2);
                                Ok(Some(ASTNode::ArrayType {
                                    type_: Box::new(ASTNode::Qbit),
                                    size: Box::new(ASTNode::IntCall { name }),
                                }))
                            } else {
                                Err(format!(
                                    "expected ], got {}",
                                    tokens2.peek().unwrap().value.clone()
                                ))
                            }
                        }
                        _ => Err(format!(
                            "Expected literal or iterator varible, got {}",
                            tokens2
                                .peek()
                                .expect("Expected literal or iterator variable, got None")
                                .value
                                .clone()
                        )),
                    }
                }
                Some(_) => Ok(Some(ASTNode::Qbit)),
            }
        }
        Some(Tok::Star) => {
            advance(tokens, tokens2);
            Ok(Some(ASTNode::PointerType {
                type_: Box::new(ASTNode::Type {
                    name: "tmp".to_string(),
                    specifier: Box::new(
                        parse_type_(tokens, tokens2)
                            .expect("Error: valid type not found")
                            .expect("Error: valid type not found"),
                    ),
                }),
            }))
        }
        Some(Tok::Old) => {
            let name = tokens2.peek().unwrap().value.clone();
            advance(tokens, tokens2);
            match tokens.peek() {
                None => Ok(Some(ASTNode::Type {
                    name,
                    specifier: Box::new(ASTNode::Custom),
                })),
                Some(Tok::OSBracket) => {
                    advance(tokens, tokens2);
                    match tokens.peek() {
                        Some(Tok::Num) => {
                            let n: i32 = tokens2.peek().unwrap().clone().value.parse().unwrap();
                            advance(tokens, tokens2);
                            match tokens.peek() {
                                None => Err("Expected ], got None".to_string()),
                                Some(Tok::CSBracket) => {
                                    advance(tokens, tokens2);
                                    Ok(Some(ASTNode::ArrayType {
                                        type_: Box::new(ASTNode::Qbit),
                                        size: Box::new(ASTNode::Num(n)),
                                    }))
                                }
                                Some(_) => Err(format!(
                                    "expected ], got {}",
                                    tokens2.peek().unwrap().value.clone()
                                )),
                            }
                        }
                        Some(Tok::PHPRef) => {
                            advance(tokens, tokens2);
                            match tokens.peek() {
                                None => Err("Expected index to external variable array, got None"
                                    .to_string()),
                                Some(Tok::Num) => {
                                    let n: u32 =
                                        tokens2.peek().unwrap().clone().value.parse().unwrap();
                                    advance(tokens, tokens2);
                                    match tokens.peek() {
                                        None => Err("Expected ], got None".to_string()),
                                        Some(Tok::CSBracket) => {
                                            advance(tokens, tokens2);
                                            Ok(Some(ASTNode::ArrayType {
                                                type_: Box::new(ASTNode::Qbit),
                                                size: Box::new(ASTNode::ExternArg {
                                                    idx: Box::new(ASTNode::ArrayIndex(n)),
                                                }),
                                            }))
                                        }
                                        Some(_) => Err(format!(
                                            "expected ], got {}",
                                            tokens2.peek().unwrap().value.clone()
                                        )),
                                    }
                                }
                                Some(Tok::Old) => {
                                    let n = tokens2.peek().unwrap().value.clone();
                                    advance(tokens, tokens2);
                                    match tokens.peek() {
                                        None => Err("Expected ], got None".to_string()),
                                        Some(Tok::CSBracket) => {
                                            advance(tokens, tokens2);
                                            Ok(Some(ASTNode::ArrayType {
                                                type_: Box::new(ASTNode::Qbit),
                                                size: Box::new(ASTNode::ExternArg {
                                                    idx: Box::new(ASTNode::IntCall { name: n }),
                                                }),
                                            }))
                                        }
                                        Some(_) => Err(format!(
                                            "expected ], got {}",
                                            tokens2.peek().unwrap().value.clone()
                                        )),
                                    }
                                }
                                Some(_) => Err(format!(
                                    "expected num or for variable, got {}",
                                    tokens2.peek().unwrap().value.clone()
                                )),
                            }
                        }
                        Some(Tok::Old) => {
                            let name = tokens2.peek().unwrap().value.clone();
                            advance(tokens, tokens2);
                            if let Some(Tok::CSBracket) = tokens.peek() {
                                advance(tokens, tokens2);
                                Ok(Some(ASTNode::ArrayType {
                                    type_: Box::new(ASTNode::Qbit),
                                    size: Box::new(ASTNode::IntCall { name }),
                                }))
                            } else {
                                Err(format!(
                                    "expected ], got {}",
                                    tokens2.peek().unwrap().value.clone()
                                ))
                            }
                        }
                        _ => Err(format!(
                            "Expected literal or iterator varible, got {}",
                            tokens2
                                .peek()
                                .expect("Expected literal or iterator variable, got None")
                                .value
                                .clone()
                        )),
                    }
                }
                Some(_) => Ok(Some(ASTNode::Type {
                    name,
                    specifier: Box::new(ASTNode::Custom),
                })),
            }
        }
        Some(_) => Err(format!(
            "expected custom type, void qbit or variantions with these types, got {}",
            tokens2.peek().unwrap().value.clone()
        )),
    }
}
fn parse_name<I, I2>(tokens: &mut Peekable<I>, tokens2: &mut Peekable<I2>) -> Result<String, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    if let Some(Tok::New) = tokens.peek() {
        let n = tokens2.peek().unwrap().value.clone();
        advance(tokens, tokens2);
        Ok(n)
    } else {
        Err(
            "didn't find name, bear in mind, function names must not be defined in current scope"
                .to_string(),
        )
    }
}
fn parse_body_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let mut nodes = Vec::new();
    while let Some(node) = parse_statement(tokens, tokens2)? {
        nodes.push(node);
    }
    if !nodes.is_empty() {
        Ok(Some(ASTNode::Block(nodes)))
    } else {
        Ok(None)
    }
}
fn parse_arguments_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Vec<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let mut nodes: Vec<ASTNode> = vec![];
    loop {
        let name;
        match parse_name(tokens, tokens2) {
            Ok(n) => name = n,
            Err(_) => break,
        }
        advance(tokens, tokens2); // just gonna assume that there's a : here, because when I tried
                                  // to match it, I ran into some issues I really don't want to deal with
        match parse_type_(tokens, tokens2) {
            Ok(Some(v)) => nodes.push(ASTNode::VariableDecl {
                name,
                value: None,
                type_: Some(Box::new(v)),
                token: 11,
            }),
            Ok(None) => {}
            Err(_) => break,
        }
    }
    Ok(nodes)
}

fn parse_if<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_for_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_var_decl<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_struct_def_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_any_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_any_2<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
fn parse_gate_call_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    Ok(Some(ASTNode::Void))
}
