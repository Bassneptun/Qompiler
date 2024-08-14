use std::collections::HashMap;
use std::string::String;
use std::mem::ManuallyDrop;

// Definitions for your types
#[repr(C)]
union Data_ {
    opt1: ManuallyDrop<String>,
    opt2: i32,
    opt3: (char, ManuallyDrop<String>),
}

enum Type {
    STRING,
    NUM,
    REFERENCE,
}

struct Data {
    data__: Data_,
    type_: Type,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_ {
            Type::NUM => write!(f, "{}", unsafe { self.data__.opt2 }),
            Type::STRING => write!(f, "{:?}", unsafe { &self.data__.opt1 }),
            Type::REFERENCE => write!(
                f,
                "{}{:?}",
                unsafe { self.data__.opt3.0 },
                unsafe { &self.data__.opt3.1 }
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum LType {
    Alloc,
    Expression,
    Process,
    LinePart,
    Unknown,
}

#[derive(Debug, Clone)]
struct LineToken {
    _type: LType,
    tokens: Vec<i32>,
    line: String,
}

#[derive(Debug)]
struct Root {
    children: Vec<LineNode>,
}

#[derive(Debug)]
struct LineNode {
    type_: LType,
    parent: *const Root,
    data: Vec<i32>,
    children: Option<Vec<LeafNode>>,
    priorityChildren: Option<Vec<PriorityChildNode>>,
}

#[derive(Debug)]
struct PriorityChildNode{
    type_: LType,
    data: Vec<i32>,
    children: Option<Vec<LeafNode>>,
    priorityChildren: Option<Vec<PriorityChildNode>>,
}

#[derive(Debug)]
struct LeafNode {
    parent: *mut LineNode,
    data: Option<Data>,
}

fn from_expr(data: String, parent: *mut LineNode) -> LeafNode {
    if data.chars().all(|c| c.is_ascii_alphabetic()) && data.chars().next().unwrap() == '&' {
        LeafNode {
            parent,
            data: Some(Data {
                data__: Data_ {
                    opt3: (data.chars().next().unwrap(), ManuallyDrop::new(data[1..].to_string())),
                },
                type_: Type::REFERENCE,
            }),
        }
    } else if data.chars().all(|c| c.is_ascii_digit()) {
        LeafNode {
            parent,
            data: Some(Data {
                data__: Data_ { opt2: data.parse().unwrap() },
                type_: Type::NUM,
            }),
        }
    } else if data.chars().all(|c| c.is_ascii_alphabetic()) {
        LeafNode {
            parent,
            data: Some(Data {
                data__: Data_ {
                    opt1: ManuallyDrop::new(data),
                },
                type_: Type::STRING,
            }),
        }
    } else {
        LeafNode { parent, data: None }
    }
}

fn get_token_category(tok: i32) -> LType {
    match tok {
        0..=2 => LType::Alloc,
        3..=4 => LType::Expression,
        5..=19 => LType::Process,
        20..=24 => LType::LinePart,
        _ => LType::Unknown,
    }
}

fn from_tokens_(toks: Vec<i32>, s: String) -> LineToken {
    LineToken {
        _type: get_token_category(toks[0]),
        tokens: toks,
        line: s,
    }
}

fn split_tokens(toks: Vec<i32>, find: i32) -> Vec<Vec<i32>>{
    let mut ret: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = vec![];
    for token in toks {
        if token == find {
            ret.push(current);
            current = vec![];
        } else {
            current.push(token);
        }
    }
    ret
}


fn from_line(data: Vec<&str>, parent: *mut LineNode, type_: LType) -> Vec<LeafNode> {
    let mut ret: Vec<LeafNode> = vec![];

    match type_ {
        LType::Alloc => {
            ret.push(from_expr(data[1].to_string(), parent));
            if data[data.len() - 1].chars().all(|c| c.is_ascii_digit()) {
                ret.push(from_expr(data[data.len() - 1].to_string(), parent));
            }
        }
        LType::Expression => {
            for i in 1..data.len(){
                ret.push(from_expr(data[i].to_string(), parent));
            }
        }
        LType::Process => {
            ret.push(from_expr(data[1].to_string(), parent));
        }
        LType::LinePart => {}
        LType::Unknown => {}
    }
    ret
}

fn from_tokens(tokens: Vec<LineToken>) -> Root {
    let mut program = Root { children: vec![] };
    for token in tokens {
        let words: Vec<&str> = token.line.split_whitespace().collect();
        let mut current = LineNode {
            type_: token._type.clone(),
            parent: &program,
            data: token.tokens.clone(),
            children: None,
            priorityChildren: None,
        };
        current.children = Some(from_line(words, &mut current, token._type.clone()));
        program.children.push(current);
    }
    program
}

fn get_priority_children(toks: LineToken, type_: LType) -> Vec<PriorityChildNode> {
    let mut ret: Vec<PriorityChildNode> = vec![];
    match type_ {
        LType::Alloc => {
            for (i, token) in toks.tokens.iter().enumerate() {
                if i != 0 {
                    match get_token_category(*token) {
                        LType::Expression => {
                            let mut index: usize = i + 1;
                            let mut current: PriorityChildNode = PriorityChildNode {
                                type_: LType::Expression,
                                data: vec![],
                                children: Some(vec![]),
                                priorityChildren: None,
                            };
                            while index < toks.tokens.len() && toks.tokens[index] > 1000 && toks.tokens[index] < 10000 {
                                current.data.push(toks.tokens[index]);
                                current.children.clone().expect("Invalid Syntax").push(from_expr(toks.line.split(" ").collect::<Vec<&str>>()[index].to_string(), std::ptr::null_mut()));
                                index += 1;
                            }
                            ret.push(current);
                        }
                        _ => panic!("Invalid Syntax"),
                    }
                }
            }
        },
        LType::Expression => {
            for (i, token) in toks.tokens.iter().enumerate() {
                if i != 0 {
                    match get_token_category(*token) {
                        LType::Expression => {
                            let mut index = i + 1;
                            let mut current: PriorityChildNode = PriorityChildNode {
                                type_: LType::Expression,
                                data: vec![],
                                children: Some(vec![]),
                                priorityChildren: None,
                            };
                            while index < toks.tokens.len() && toks.tokens[index] > 1000 && toks.tokens[index] < 10000 {
                                current.data.push(toks.tokens[index]);
                                current.children.expect("Invalid Syntax").push(from_expr(toks.line.split(" ").collect::<Vec<&str>>()[index].to_string(), std::ptr::null_mut()));
                                index += 1;
                            }
                            ret.push(current);
                        }
                        _ => panic!("Invalid Syntax"),
                    }
                }
            }
        },
        LType::Process => {
            let size: i32;
            let start: i32;
            for (i, token) in toks.tokens.iter().enumerate() {
                if *token as usize == __TOKENS.len() - 5 {
                    start = i as i32;
                } else if *token as usize == __TOKENS.len() - 4 {
                    size = i as i32 - start;
                } else{
                    continue;
                }
            }
            if size > 1 {
                let lines: Vec<Vec<i32>> = split_tokens(toks.tokens, (__TOKENS.len() - 1) as i32);
                let toks: Vec<LineToken> = lines.iter().map(|x| from_tokens_(x.clone(), toks.line.clone())).collect();
                let line_nodes: Vec<LineNode> = from_tokens(toks.clone()).children;
                for line_node in line_nodes {
                    ret.push(PriorityChildNode {
                        type_: line_node.type_,
                        data: line_node.data,
                        children: line_node.children,
                        priorityChildren: None,
                    });
                }
            }
        },
        _ => {},
    }
    ret
}

fn parse(source: Vec<String>) -> Vec<LineToken> {
    let mut tokens: Vec<LineToken> = vec![];
    let mut current = LineToken {
        _type: LType::Unknown,
        tokens: vec![],
        line: String::new(),
    };
    let mut initialized = false;
    let mut mode: u8 = 0;

    for s in source {
        if s.is_empty() && initialized {
            tokens.push(current);
            mode = 0;
            initialized = false;
            current = LineToken {
                _type: LType::Unknown,
                tokens: vec![],
                line: String::new(),
            };
        } else if s.is_empty() && !initialized {
            continue;
        } else if !s.is_empty() {
            match mode {
                0 => {
                    let t: LType;
                    match s.chars().next().unwrap() {
                        '0' => t = LType::Alloc,
                        '1' => t = LType::Expression,
                        '2' => t = LType::Process,
                        '3' => t = LType::LinePart,
                        _ => t = LType::Unknown,
                    }
                    current = LineToken {
                        _type: t,
                        tokens: vec![],
                        line: String::new(),
                    };
                    mode = 1;
                }
                1 => {
                    let mut is_num = false;
                    let mut current_num: i32 = 0;
                    let mut tokens: Vec<i32> = vec![];
                    let mut is_negative = false;
                    for ch in s.chars() {
                        if ch.is_ascii_digit() {
                            is_num = true;
                            current_num = current_num * 10 + ch.to_digit(10).unwrap() as i32;
                        } else if ch == ',' {
                            is_num = false;
                            if is_negative {
                                current_num = -current_num;
                                is_negative = false;
                            }
                            tokens.push(current_num);
                            current_num = 0;
                        } else if ch == '{' {
                            continue;
                        } else if ch == '}' {
                            tokens.push(current_num);
                            current.tokens = tokens;
                            mode = 2;
                            break;
                        } else if ch == '-' {
                            if is_num {
                                panic!("Invalid number: {}", s);
                            } else {
                                is_negative = true;
                                is_num = true;
                            }
                        }
                    }
                }
                2 => {
                    initialized = true;
                    let mut s2 = String::new();
                    let mut in_str = false;
                    for ch in s.chars() {
                        if ch == '"' {
                            match in_str {
                                false => in_str = true,
                                true => {
                                    current.line = s2;
                                    mode = 0;
                                    break;
                                }
                            }
                        } else if in_str {
                            s2.push(ch);
                        } else {
                            panic!("Invalid string: {}", s);
                        }
                    }
                }
                _ => panic!("Invalid mode: {}", mode),
            }
        }
    }
    if initialized {
        tokens.push(current);
    }
    tokens
}

fn from_stdin(s: String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let mut is_nl = false;
    let mut current = String::new();
    for ch in s.chars() {
        if ch == '\n' && !is_nl {
            is_nl = true;
            ret.push(current.clone());
            current = String::new();
        } else if ch == '\n' && is_nl {
            ret.push(String::from(""));
            is_nl = false;
        } else {
            is_nl = false;
            current.push(ch);
        }
    }
    if !current.is_empty() {
        ret.push(current);
    }
    ret
}
