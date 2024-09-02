use std::iter::Peekable;

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
    /*
    Gate {
        name: String,
        gate: Vec<Vec<f32>>,
        args: Vec<ASTNode>,
        arg_names: Vec<String>,
    },
    */ // not yet implemented, probably not gonna happen either, there's no transformation to not
    // be performed with existing gates. combination of those can easily be achieved with funtions.
    Struct {
        name: String,
        types: Vec<ASTNode>,
    },
    ArrayIndex(u32),
    ArrayAccess {
        name: Box<ASTNode>,
        index: Box<ASTNode>,
    },
    Reference {
        value: Box<ASTNode>,
    },
    Dereference {
        value: Box<ASTNode>,
    },
    // Break, that's not implemented yet either, I have not found the need for it yet
    Void,
    Qbit,
    // Qdit, this is extremely usefull, but not implemented either yet
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
        lval: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    StructAccess {
        structure: Box<ASTNode>,
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
    Qdit,
}

#[derive(Debug)]
pub enum Tok {
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
    Qudit,
}

pub fn from_tokens(tokens: Vec<Token>) -> Vec<Tok> {
    let mut toks = Vec::new();
    for t in tokens {
        match t.token {
            3 => toks.push(Tok::OBracket),
            4 => toks.push(Tok::CBracket),
            5 => toks.push(Tok::OCBracket),
            6 => toks.push(Tok::CCBracket),
            7 => toks.push(Tok::OSBracket),
            8 => toks.push(Tok::CSBracket),
            9 => toks.push(Tok::DoublePoint),
            10 => toks.push(Tok::Semicolon),
            11 => toks.push(Tok::Comma),
            12 => toks.push(Tok::Equal),
            13 => toks.push(Tok::Reference),
            14 => toks.push(Tok::Star),
            15 => toks.push(Tok::DotDot),
            16 => toks.push(Tok::VarDecl),
            17 => toks.push(Tok::ConstDecl),
            18 => toks.push(Tok::Struct),
            19 => toks.push(Tok::Qbit),
            20 => toks.push(Tok::Void),
            21 => toks.push(Tok::Hash),
            22 => toks.push(Tok::Macro),
            23 => toks.push(Tok::GateDecl),
            24..=38 => toks.push(Tok::GateCall),
            39 => toks.push(Tok::Dot),
            40 => toks.push(Tok::If),
            41 => toks.push(Tok::For),
            42 => toks.push(Tok::In),
            43 => toks.push(Tok::Return),
            44 => toks.push(Tok::Break),
            45 => toks.push(Tok::PHPRef),
            46 => toks.push(Tok::Qudit),
            50 => toks.push(Tok::New),
            51 => toks.push(Tok::Old),
            52 => toks.push(Tok::Num),
            _ => panic!("I don't know how this would even ever happen"),
        }
    }
    toks
}

pub fn parse_(tokens: Vec<Tok>, tokens2: Vec<Token>) -> Result<ASTNode, String> {
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
    println!("{:?}", tokens.peek().unwrap());
    match tokens.peek() {
        Some(Tok::Qbit) => parse_function_def_(tokens, tokens2),
        Some(Tok::Void) => parse_function_def_(tokens, tokens2),
        Some(Tok::Qudit) => parse_function_def_(tokens, tokens2),
        //Some(Tok::If) => parse_if(tokens, tokens2),
        Some(Tok::For) => parse_for_(tokens, tokens2),
        Some(Tok::VarDecl) => parse_var_decl(tokens, tokens2),
        Some(Tok::Struct) => parse_struct_def_(tokens, tokens2),
        Some(Tok::GateCall) => parse_gate_call_(tokens, tokens2),
        Some(Tok::ConstDecl) => parse_var_decl(tokens, tokens2), // disambiguity ends here, now it
        Some(Tok::Return) => {
            advance(tokens, tokens2);
            parse_return_(tokens, tokens2)
        }
        // gets really fucked.
        Some(Tok::Old) => parse_any_(tokens, tokens2, None),
        Some(Tok::Star) => parse_any_2(tokens, tokens2),
        Some(Tok::Reference) => parse_any_3(tokens, tokens2),
        Some(Tok::Num) => {
            let num = tokens2.peek().unwrap().value.parse::<i32>().unwrap();
            advance(tokens, tokens2);
            match tokens.peek() {
                Some(Tok::DotDot) => parse_range_(tokens, tokens2, num),
                _ => Ok(Some(ASTNode::Num(num))),
            }
        }
        Some(Tok::CBracket) => {
            advance(tokens, tokens2);
            parse_statement(tokens, tokens2)
        }
        Some(Tok::Semicolon) => {
            advance(tokens, tokens2);
            if tokens.peek().is_some() {
                parse_statement(tokens, tokens2)
            } else {
                return Ok(None);
            }
        }
        Some(Tok::PHPRef) => {
            advance(tokens, tokens2);
            if tokens2.peek().unwrap().value.parse::<u32>().is_ok() {
                let tmp = tokens2
                    .peek()
                    .unwrap()
                    .value
                    .clone()
                    .parse::<u32>()
                    .unwrap();
                advance(tokens, tokens2);
                return Ok(Some(ASTNode::ExternArg {
                    idx: Box::new(ASTNode::ArrayIndex(tmp)),
                }));
            } else {
                let nam_ = tokens2.peek().unwrap().value.clone();
                advance(tokens, tokens2);
                return Ok(Some(ASTNode::ExternArg {
                    idx: Box::new(ASTNode::IntCall { name: nam_ }),
                }));
            }
        }
        None => Ok(None),
        _ => Ok(None),
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

fn parse_return_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let ret = parse_statement(tokens, tokens2);
    let ret_;
    match ret {
        Ok(o) => ret_ = o.expect("expected statement, got None"),
        Err(e) => return Err(e),
    }
    Ok(Some(ASTNode::Return(Box::new(ret_))))
}

fn parse_any_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    current: Option<ASTNode>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let mut current_cpy = current.clone();
    match tokens.peek() {
        None => return Err("Expected expression, got None".to_string()),
        Some(Tok::CBracket) => return Ok(current_cpy),
        Some(Tok::Comma) => return Ok(current_cpy),
        Some(_) => {}
    }
    if current_cpy.is_none() {
        // function call, variable reference, assignment
        let first = tokens2.peek().unwrap().value.clone();
        advance(tokens, tokens2);
        current_cpy = Some(ASTNode::VariableCall {
            name: first.clone(),
        });
        match tokens.peek() {
            None => return Err("Expected Expression, got None".to_string()),
            Some(Tok::CBracket) => {
                return Ok(current_cpy);
            }
            Some(Tok::Comma) => return Ok(current_cpy),
            Some(Tok::OBracket) => current_cpy = parse_function_call_(tokens, tokens2, first)?,
            Some(Tok::Equal) => {
                return Ok(parse_assignment_(tokens, tokens2, current_cpy.unwrap())?)
            }
            Some(Tok::Dot) => {
                current_cpy = parse_struct_access(tokens, tokens2, current_cpy.unwrap())?
            }
            Some(Tok::OSBracket) => {
                current_cpy = parse_array_access(tokens, tokens2, current_cpy.unwrap())?
            }
            Some(Tok::Semicolon) => return Ok(current_cpy),
            Some(Tok::PHPRef) => {
                advance(tokens, tokens2);
                if tokens2.peek().unwrap().value.parse::<u32>().is_ok() {
                    let tmp = tokens2
                        .peek()
                        .unwrap()
                        .value
                        .clone()
                        .parse::<u32>()
                        .unwrap();
                    advance(tokens, tokens2);
                    return Ok(Some(ASTNode::ExternArg {
                        idx: Box::new(ASTNode::ArrayIndex(tmp)),
                    }));
                } else {
                    let nam_ = tokens2.peek().unwrap().value.clone();
                    advance(tokens, tokens2);
                    return Ok(Some(ASTNode::ExternArg {
                        idx: Box::new(ASTNode::IntCall { name: nam_ }),
                    }));
                }
            }
            Some(other) => {
                return Err(format!(
                    "Expected '(', ',', '[', '=', '.', ')', but got {other:?}1"
                ))
            }
        }
    } else {
        // function call, variable reference, assignment
        let first = tokens2.peek().unwrap().value.clone();
        match tokens.peek() {
            None => return Err("Expected Expression, got None".to_string()),
            Some(Tok::CBracket) => return Ok(current_cpy),
            Some(Tok::Comma) => return Ok(current_cpy),
            Some(Tok::OBracket) => current_cpy = parse_function_call_(tokens, tokens2, first)?,
            Some(Tok::Equal) => {
                return Ok(parse_assignment_(tokens, tokens2, current_cpy.unwrap())?)
            }
            Some(Tok::Dot) => {
                current_cpy = parse_struct_access(tokens, tokens2, current_cpy.unwrap())?
            }
            Some(Tok::OSBracket) => {
                current_cpy = parse_array_access(tokens, tokens2, current_cpy.unwrap())?
            }
            Some(Tok::Semicolon) => return Ok(current),
            Some(Tok::PHPRef) => {
                advance(tokens, tokens2);
                if tokens2.peek().unwrap().value.parse::<u32>().is_ok() {
                    let tmp = tokens2
                        .peek()
                        .unwrap()
                        .value
                        .clone()
                        .parse::<u32>()
                        .unwrap();
                    advance(tokens, tokens2);
                    return Ok(Some(ASTNode::ExternArg {
                        idx: Box::new(ASTNode::ArrayIndex(tmp)),
                    }));
                } else {
                    let nam_ = tokens2.peek().unwrap().value.clone();
                    advance(tokens, tokens2);
                    return Ok(Some(ASTNode::ExternArg {
                        idx: Box::new(ASTNode::IntCall { name: nam_ }),
                    }));
                }
            }
            Some(other) => {
                return Err(format!(
                    "Expected '(', ',', '[', '=', '.', ')', but got {other:?}2"
                ))
            }
        }
    }
    parse_any_(tokens, tokens2, current_cpy)
}
fn parse_range_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    num: i32,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    match tokens.peek() {
        None => return Err("Error: Expected '..', got None".to_string()),
        Some(Tok::DotDot) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected '..', got {other:?}")),
    }
    let second_num = tokens2
        .peek()
        .unwrap()
        .value
        .clone()
        .parse::<i32>()
        .unwrap();
    advance(tokens, tokens2);
    Ok(Some(ASTNode::Range {
        start: Box::new(ASTNode::Num(num)),
        end: Box::new(ASTNode::Num(second_num)),
    }))
}
fn parse_assignment_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    prev: ASTNode,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);

    let value_ = parse_statement(tokens, tokens2);
    let value;
    match value_ {
        Ok(o) => value = o.expect("Error: expected r-value, got None"),
        Err(e) => return Err(e),
    }

    Ok(Some(ASTNode::Assignment {
        lval: Box::new(prev),
        value: Box::new(value),
    }))
}

fn parse_struct_access<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    prev: ASTNode,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);

    let name_ = parse_name(tokens, tokens2);
    let name;
    match name_ {
        Ok(o) => name = o,
        Err(e) => return Err(e),
    }

    Ok(Some(ASTNode::StructAccess {
        structure: Box::new(prev),
        member: name,
    }))
}
fn parse_array_access<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    prev: ASTNode,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);

    match tokens.peek() {
        Some(Tok::Num) => {
            let n: u32 = tokens2.peek().unwrap().clone().value.parse().unwrap();
            advance(tokens, tokens2);
            match tokens.peek() {
                None => Err("Expected ], got None".to_string()),
                Some(Tok::CSBracket) => {
                    advance(tokens, tokens2);
                    Ok(Some(ASTNode::ArrayAccess {
                        name: Box::new(prev),
                        index: Box::new(ASTNode::ArrayIndex(n)),
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
                None => Err("Expected index to external variable array, got None".to_string()),
                Some(Tok::Num) => {
                    let n: u32 = tokens2.peek().unwrap().clone().value.parse().unwrap();
                    advance(tokens, tokens2);
                    match tokens.peek() {
                        None => Err("Expected ], got None".to_string()),
                        Some(Tok::CSBracket) => {
                            advance(tokens, tokens2);
                            Ok(Some(ASTNode::ArrayAccess {
                                name: Box::new(prev),
                                index: Box::new(ASTNode::ExternArg {
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
                            Ok(Some(ASTNode::ArrayAccess {
                                name: Box::new(prev),
                                index: Box::new(ASTNode::ExternArg {
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
                Ok(Some(ASTNode::ArrayAccess {
                    name: Box::new(prev),
                    index: Box::new(ASTNode::IntCall { name }),
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

fn parse_function_call_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
    prev: String,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);
    let tmp = parse_call_args(tokens, tokens2);
    match tmp {
        Err(e) => Err(e),
        Ok(o) => {
            advance(tokens, tokens2);
            Ok(Some(ASTNode::FunctionCall {
                name: prev,
                args: o,
            }))
        }
    }
}
fn parse_any_3<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);
    let scnd = parse_any_(tokens, tokens2, None);
    match scnd {
        Err(e) => Err(e),
        Ok(o) => Ok(Some(ASTNode::Reference {
            value: Box::new(o.expect("expected rval after '*', but got None")),
        })),
    }
}

fn parse_any_2<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);
    let scnd = parse_any_(tokens, tokens2, None);
    match scnd {
        Err(e) => Err(e),
        Ok(o) => Ok(Some(ASTNode::Dereference {
            value: Box::new(o.expect("expected rval after '*', but got None")),
        })),
    }
}

fn parse_call_args<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Vec<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let mut arguments: Vec<ASTNode> = vec![];
    loop {
        arguments.push(parse_statement(tokens, tokens2).unwrap().unwrap());
        match tokens.peek() {
            None => return Err("Expected ',' or ')', got None".to_string()),
            Some(Tok::Comma) => advance(tokens, tokens2),
            Some(Tok::CBracket) => break,
            Some(_) => {}
        }
    }
    Ok(arguments)
}

fn parse_gate_call_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let name = tokens2.peek().unwrap().value.clone();
    advance(tokens, tokens2);

    match tokens.peek() {
        None => return Err("Expected '(', got None".to_string()),
        Some(Tok::OBracket) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected '(', got {other:?}")),
    }
    let args = parse_call_args(tokens, tokens2);
    let mut arguments = Vec::new();
    match args {
        Err(e) => println!("Error: {e:?}"),
        Ok(a) => arguments = a,
    }
    Ok(Some(ASTNode::GateCall {
        name,
        args: arguments,
    }))
}

fn parse_var_decl<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    println!("1");
    let tok = tokens2.peek().unwrap().token;
    advance(tokens, tokens2);
    let name_ = parse_name(tokens, tokens2);
    let mut name = String::new();
    match name_ {
        Ok(v) => name = v,
        Err(e) => println!("Error: {e:?}"),
    }
    match tokens.peek() {
        None => return Err("Expected ':', got None".to_string()),
        Some(Tok::DoublePoint) => advance(tokens, tokens2),
        Some(Tok::Semicolon) => {
            return Ok(Some(ASTNode::VariableDecl {
                name,
                value: None,
                type_: None,
                token: -1,
            }));
        }
        Some(Tok::Equal) => {
            advance(tokens, tokens2);
            let rval = parse_statement(tokens, tokens2);
            let mut rval_ = ASTNode::Void;
            match rval {
                Ok(Some(v)) => rval_ = v,
                Ok(None) => return Err("Expected rval expression, got None".to_string()),
                Err(e) => println!("Error: {e:?}"),
            }

            match tokens.peek() {
                None => return Err("Expected ';', got None".to_string()),
                Some(Tok::CBracket) => {
                    advance(tokens, tokens2);
                    return Ok(Some(ASTNode::VariableDecl {
                        name,
                        value: Some(Box::new(rval_)),
                        type_: None,
                        token: tok,
                    }));
                }
                Some(Tok::Semicolon) => {
                    return Ok(Some(ASTNode::VariableDecl {
                        name,
                        value: Some(Box::new(rval_)),
                        type_: None,
                        token: tok,
                    }));
                }
                Some(other) => return Err(format!("Expected ';', got {:?}", other)),
            }
        }
        Some(other) => return Err(format!("Expected ':', got {:?}", other)),
    }

    let type_ = parse_type_(tokens, tokens2);
    let mut type__ = ASTNode::Void;
    match type_ {
        Ok(v) => type__ = v.expect("Error: didn't find valid type in function declaration"),
        Err(e) => println!("Error: {e:?}"),
    }

    match tokens.peek() {
        None => return Err("Expected ';' or '=', got None".to_string()),
        Some(Tok::Semicolon) => {
            return Ok(Some(ASTNode::VariableDecl {
                name,
                value: None,
                type_: Some(Box::new(type__)),
                token: -1,
            }))
        }
        Some(Tok::Equal) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected ';' or '=', got {:?}", other)),
    }

    let rval = parse_statement(tokens, tokens2);
    let mut rval_ = ASTNode::Void;
    match rval {
        Ok(Some(v)) => rval_ = v,
        Ok(None) => return Err("Expected rval expression, got None".to_string()),
        Err(e) => println!("Error: {e:?}"),
    }

    match tokens.peek() {
        None => return Err("Expected ';', got None".to_string()),
        Some(Tok::Semicolon) => {
            return Ok(Some(ASTNode::VariableDecl {
                name,
                value: Some(Box::new(rval_)),
                type_: Some(Box::new(type__)),
                token: tok,
            }));
        }
        Some(other) => return Err(format!("Expected ';', got {:?}", other)),
    }
}

fn parse_struct_members<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Vec<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    let mut struct_members: Vec<ASTNode> = Vec::new();
    let is_ = true;
    while is_ {
        if let Ok(Some(node)) = parse_var_decl(tokens, tokens2) {
            struct_members.push(node);
        } else {
            break;
        }

        match tokens.peek() {
            None => return Err("Expected '{{' or var declaration, got None".to_string()),
            Some(Tok::Semicolon) => advance(tokens, tokens2),
            Some(other) => return Err(format!("Expected '{{' or var declaration, got {other:?}")),
        }

        if let Some(Tok::CCBracket) = tokens.peek() {
            return Ok(struct_members);
        }
    }

    Ok(struct_members)
}

fn parse_struct_def_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);
    let name_ = parse_name(tokens, tokens2);
    let mut name = String::new();
    match name_ {
        Ok(v) => name = v,
        Err(e) => println!("1__Error: {e:?}"),
    }

    match tokens.peek() {
        None => return Err("Expected {{, got None".to_string()),
        Some(Tok::OCBracket) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected '{{', got {:?}", other)),
    }

    let mems = parse_struct_members(tokens, tokens2);
    advance(tokens, tokens2);
    let mut mems2 = vec![];
    match mems {
        Ok(ref a) => mems2 = a.to_vec(),
        Err(ref e) => println!("Error: {e:?}"),
    }
    Ok(Some(ASTNode::Struct { name, types: mems2 }))
}

fn parse_for_<I, I2>(
    tokens: &mut Peekable<I>,
    tokens2: &mut Peekable<I2>,
) -> Result<Option<ASTNode>, String>
where
    I: Iterator<Item = Tok>,    // Expecting an iterator of owned `Tok` instances
    I2: Iterator<Item = Token>, // Expecting an iterator of owned `Tok` instances
{
    advance(tokens, tokens2);
    if let Some(Tok::OBracket) = tokens.peek() {
        advance(tokens, tokens2);
    } else {
        return Err("Expected (".to_string());
    }
    let name_ = parse_name(tokens, tokens2);
    let mut name = String::new();
    match name_ {
        Ok(v) => name = v,
        Err(e) => println!("Error: {e:?}"),
    }

    match tokens.peek() {
        Some(Tok::In) => advance(tokens, tokens2),
        Some(another) => return Err(format!("Expected 'in', got {:?}", another)),
        None => return Err("Expected 'in', got None".to_string()),
    }

    let container = parse_statement(tokens, tokens2);
    let mut container_ = ASTNode::Void;
    match container {
        Ok(Some(v)) => container_ = v,
        Ok(None) => return Err("Expected rval expression, got None".to_string()),
        Err(e) => println!("Error: {e:?}"),
    }

    match tokens.peek() {
        None => return Err("Expected ), got None".to_string()),
        Some(Tok::CBracket) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected ), got {:?}", other)),
    }

    match tokens.peek() {
        None => return Err("Expected {, got None".to_string()),
        Some(Tok::OCBracket) => advance(tokens, tokens2),
        Some(other) => return Err(format!("Expected {{, got {:?}", other)),
    }

    let mut body = None;
    let body_ = parse_body_(tokens, tokens2);
    match body_ {
        Ok(Some(v)) => body = Some(Box::new(v)),
        Ok(None) => body = None,
        Err(e) => println!("Error: {e:?}"),
    }
    advance(tokens, tokens2);
    Ok(Some(ASTNode::For {
        container: Box::new(container_),
        alias: name,
        body,
    }))
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

    match tokens.peek() {
        None => return Err("Expected '{', got None".to_string()),
        Some(Tok::OCBracket) => advance(tokens, tokens2),
        Some(o) => return Err(format!("Expected '{{', got {o:?}")),
    }

    let mut body = None;
    let body_ = parse_body_(tokens, tokens2);
    match body_ {
        Ok(Some(v)) => body = Some(Box::new(v)),
        Ok(None) => body = None,
        Err(e) => println!("Error: {e:?}"),
    }

    advance(tokens, tokens2);
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
        Some(Tok::Void) => {
            advance(tokens, tokens2);
            Ok(Some(ASTNode::Void))
        }
        Some(Tok::Qdit) => {
            advance(tokens, tokens2);
            match tokens.peek() {
                None => Ok(Some(ASTNode::Qdit)),
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
                                        type_: Box::new(ASTNode::Qdit),
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
                                                type_: Box::new(ASTNode::Qdit),
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
                                                type_: Box::new(ASTNode::Qdit),
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
                                    type_: Box::new(ASTNode::Qdit),
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
                Some(_) => Ok(Some(ASTNode::Qdit)),
            }
        }
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
            format!("didn't find name, bear in mind, function names must not be defined in current scope, found {:?}", tokens.peek().unwrap()),
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
    advance(tokens, tokens2);
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

        match tokens.peek() {
            None => return Err("Expected ',', or ')', got None".to_string()),
            Some(Tok::CBracket) => break,
            Some(Tok::Comma) => advance(tokens, tokens2),
            Some(other) => return Err(format!("Expected ',', or ')', got {other:?}")),
        }
    }
    advance(tokens, tokens2);
    Ok(nodes)
}

/*
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
*/
