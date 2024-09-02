use crate::parser::ASTNode;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Comptime {
    pub program: String,
    pub functions: HashMap<String, String>,
    pub function_info: HashMap<String, (Vec<String>, String)>, // signature
    pub vars: HashMap<String, (usize, usize)>,                 // position in memory
    pub var_info: HashMap<String, (bool, usize, String)>,      // is_const, size, type
    pub tmp_vars: HashMap<String, (usize, usize)>,             // position in memory
    pub tmp_var_info: HashMap<String, (usize, String)>,        // size, type
    pub types: HashMap<String, usize>,                         // name to size of type
    pub structs: HashMap<(String, String), (usize, String)>, // name of type + field to size of field and type
    pub i: i32,
}

pub fn code_gen(ast: ASTNode) -> Result<Comptime, String> {
    let mut nodes_iterator;
    if let ASTNode::Program(nodes) = ast {
        nodes_iterator = nodes.into_iter().peekable();
    } else {
        return Err("AST_ERROR: Expected Program node, got something else.".to_string());
    }

    code_gen_nodes(&mut nodes_iterator)
}

pub fn code_gen_nodes<I>(iterator: &mut Peekable<I>) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    let mut c: Comptime = Comptime {
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
    let mut ret: Vec<_> = vec![];
    while let Ok(compt_) = code_gen_node(iterator, &mut c) {
        ret.push(compt_);
        iterator.next();
    }
    Ok(c)
}

pub fn code_gen_node<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("Expected Node, got None".to_owned()),
        Some(ASTNode::VariableDecl { .. }) => generate_var_decl(iterator, cmptime),
        //Some(ASTNode::FunctionDef { .. }) => generate_func_decl(iterator, cmptime),
        //Some(ASTNode::For { .. }) => generate_for(iterator, cmptime),
        //Some(ASTNode::GateCall { .. }) => generate_gate_call(iterator, cmptime),
        _ => Err("".to_string()),
    }
}

pub fn generate_var_decl<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    let _ = match iterator.peek() {
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl {
            name: _,
            value,
            type_,
            token: _,
        }) => match type_ {
            None => {
                if let Some(_) = value {
                    return Ok(generate_var_decl_td(iterator, cmptime)?.unwrap());
                } else {
                    return Err("temporary".to_string());
                }
            }
            Some(_) => return Err("temporary".to_string()),
        },
        Some(other) => {
            return Err(
                format!("BACKEND_ERROR: Expected ASTNode::VariableDecl, got {other:?}").to_owned(),
            );
        }
    };
}

fn str_mul(s: String, num: i32) -> String {
    let mut s_ret: String = String::new();
    for _ in 0..num {
        s_ret.push_str(s.clone().as_str());
    }
    s_ret
}

pub fn generate_var_decl_td<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    let _ = match iterator.peek() {
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl { value, .. }) => match *value.clone().unwrap() {
            ASTNode::Num(_num) => gen_var_decl_num(iterator, cmptime),
            ASTNode::VariableCall { name: _ } => gen_var_decl_cpy(iterator, cmptime),
            ASTNode::FunctionCall { name, args } => {
                gen_func_cal(iterator, cmptime);
                gen_var_decl_cpy(iterator, cmptime)
            }
            _ => Ok(Some(cmptime.clone())),
        },
        Some(other) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::VariableDecl, got {:?}",
            other
        )),
    };
    Ok(Some(cmptime.clone()))
}

pub fn gen_var_decl_num<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl {
            value, name, token, ..
        }) => match *(value.clone().unwrap()) {
            ASTNode::Num(num) => {
                let mut n_qbits = if num > 1 {
                    (num.to_owned() as f32).log2().ceil() as u32
                } else {
                    1
                };

                if num == 2 {
                    n_qbits = 2;
                }

                cmptime
                    .vars
                    .insert(name.to_string(), (cmptime.i as usize, 0));
                cmptime.i += n_qbits as i32;
                cmptime.var_info.insert(
                    name.to_string(),
                    (*token == 13, n_qbits as usize, format!("Array[Qbit]")),
                );

                for i in 0..n_qbits as i32 {
                    cmptime
                        .program
                        .push_str(format!("QAL & 0 $ \"{}\"\n", format!("{name}_{i}")).as_str())
                }
                let qbits_bin: Vec<String> = (0..n_qbits)
                    .map(|n| ((num.to_owned() >> n) & 1))
                    .map(|num| format!("{num}"))
                    .collect();
                for (i, s) in qbits_bin.iter().enumerate() {
                    cmptime.program.push_str(
                        format!(
                            "SET ${} {}\n",
                            format!("{}_{}", name, i),
                            if s == "0" { "1 0" } else { "0 1" }
                        )
                        .as_str(),
                    );
                }
                Ok(None)
            }
            _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
        },
        _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
    }
}

pub fn generate_var_decl_<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    Err("".to_string())
}

pub fn gen_var_decl_cpy<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl {
            value, name, token, ..
        }) => match *(value.clone().unwrap()) {
            ASTNode::VariableCall { name } => {
                let info = cmptime.var_info.get(&name).unwrap();
                let n_qbits = info.1;
                cmptime
                    .vars
                    .insert(name.to_string(), (cmptime.i as usize, 0));
                cmptime.i += n_qbits as i32;
                cmptime
                    .var_info
                    .insert(name.to_string(), (*token == 13, n_qbits, info.2.clone()));

                for i in 0..n_qbits as i32 {
                    cmptime
                        .program
                        .push_str(format!("QAL & 0 $ \"{}\"", format!("{name}_{i}")).as_str())
                }
                for i in 0..n_qbits {
                    cmptime.program.push_str(
                        format!(
                            "CPY ${} ${}",
                            format!("{}_{}", name, i),
                            format!("{}_{}", "tmp", i)
                        )
                        .as_str(),
                    );
                }
                Ok(None)
            }
            _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
        },
        _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
    }
}

pub fn gen_func_cal<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    let _ = match iterator.peek() {
        Some(ASTNode::FunctionCall { name, args }) => {
            // allocate arguments
            for (i, arg) in args.iter().enumerate() {
                match arg {
                    ASTNode::VariableDecl { name, .. } => {
                        println!("1");
                        let info = cmptime.var_info.get(name).unwrap();
                        println!("1");
                        let n_qbits = info.1;
                        println!("1");
                        cmptime
                            .vars
                            .insert(format!("arg{i}"), (cmptime.i as usize, 0));
                        println!("1");
                        cmptime.i += n_qbits as i32;
                        println!("1");
                        cmptime
                            .var_info
                            .insert(format!("arg{i}"), (false, n_qbits, info.2.clone()));
                        println!("1");
                        for i in 0..n_qbits as i32 {
                            cmptime.program.push_str(
                                format!("QAL & 0 $ \"{}\"", format!("{}_{i}", format!("arg{i}")))
                                    .as_str(),
                            )
                        }
                        println!("1");
                        for i in 0..n_qbits {
                            cmptime.program.push_str(
                                format!(
                                    "CPY ${} ${}",
                                    format!("arg{i}"),
                                    format!("{}_{}", "tmp", i)
                                )
                                .as_str(),
                            );
                        }
                        println!("1");
                    }
                    _ => return Err("BACKEND_ERROR: Expected VariableDecl ".to_string()),
                }
            }
        }
        None => return Err("BACKEND_ERROR: Expected ASTNode::FunctionCall, got None".to_string()),
        _ => return Err("BACKEND_ERROR: Expected ASTNode::FunctionCall".to_string()),
    };
    Err("".to_string())
}

/*
pub fn code_gen_node<I>(iterator: &mut Peekable<I>, cmptime: &mut Comptime) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
}

*/
