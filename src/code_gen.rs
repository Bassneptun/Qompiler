use crate::parser::ASTNode;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Comptime {
    pub program: String,
    pub functions: HashMap<String, ASTNode>,
    pub function_info: HashMap<String, (Vec<ASTNode>, ASTNode)>, // signature
    pub function_args: HashMap<String, Vec<String>>,             // signature
    pub vars: HashMap<String, (usize, usize)>,                   // position in memory
    pub var_info: HashMap<String, (bool, usize, ASTNode)>,       // is_const, size, type
    pub iterators: HashMap<String, usize>,                       // name to size of type
    //pub tmp_vars: HashMap<String, (usize, usize)>,             // position in memory
    //pub tmp_var_info: HashMap<String, (usize, String)>,        // size, type
    //pub types: HashMap<String, usize>,                         // name to size of type
    //pub structs: HashMap<(String, String), (usize, String)>, // name of type + field to size of field and type
    pub aliass: HashMap<String, String>,
    pub i: i32,
}

pub fn code_gen(ast: ASTNode) -> Result<Comptime, String> {
    let mut nodes_iterator;
    if let ASTNode::Program(nodes) = ast {
        nodes_iterator = nodes.into_iter().peekable();
    } else if let ASTNode::Block(nodes) = ast {
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
        function_args: HashMap::new(),
        vars: HashMap::new(),
        var_info: HashMap::new(),
        iterators: HashMap::new(),
        //tmp_vars: HashMap::new(),
        //tmp_var_info: HashMap::new(),
        //types: HashMap::new(),
        //structs: HashMap::new(),
        aliass: HashMap::new(),
        i: 0,
    };
    while let Ok(_) = code_gen_node(iterator, &mut c) {
        iterator.next();
    }

    match code_gen_node(iterator, &mut c) {
        Err(e) => println!("Error: {}", e),
        Ok(_thing_else) => {}
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
        None => Err("Expected Node, got None1".to_owned()),
        Some(ASTNode::VariableDecl { .. }) => generate_var_decl(iterator, cmptime),
        Some(ASTNode::FunctionDef { .. }) => gen_func_decl(iterator, cmptime),
        Some(ASTNode::For { .. }) => generate_for(iterator, cmptime),
        Some(ASTNode::Assignment { .. }) => generate_assignment(iterator, cmptime),
        Some(ASTNode::GateCall { .. }) => generate_gate_call(iterator, cmptime),
        Some(ASTNode::Return(..)) => generate_return(iterator, cmptime),
        _ => Err("1".to_string()),
    }
}

pub fn generate_return<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::Return, got None".to_string()),
        Some(ASTNode::Return(value)) => match *value.clone() {
            ASTNode::VariableCall { name } => {
                let target_size = if !cmptime.aliass.contains_key(&name) {
                    println!("{:#?}", cmptime.aliass);
                    cmptime.var_info.get(&name).unwrap().1
                } else {
                    cmptime
                        .var_info
                        .get(&cmptime.aliass.get(&name).unwrap().clone())
                        .unwrap()
                        .1
                };
                for i in 0..target_size {
                    cmptime
                        .program
                        .push_str(format!("QAL & 0 $ \"TMP_{}\"\n", i).as_str());
                }
                for i in 0..target_size {
                    cmptime
                        .program
                        .push_str(format!("CPY $TMP_{} ${}\n", i, name).as_str());
                }
                Ok(cmptime.clone())
            }
            _ => Err(format!(
                "BACKEND_ERROR: Expected ASTNode::VariableCall, got {value:?}"
            )),
        },
        Some(other) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::Return, got {other:?}"
        )),
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
                    return gen_var_alloc(iterator, cmptime);
                }
            }
            Some(_) => return gen_var_alloc(iterator, cmptime),
        },
        Some(other) => {
            return Err(
                format!("BACKEND_ERROR: Expected ASTNode::VariableDecl, got {other:?}").to_owned(),
            );
        }
    };
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
            ASTNode::FunctionCall { name: _, args: _ } => {
                let _ = gen_func_call(iterator, cmptime);
                func_cpy(iterator, cmptime)
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
                    (
                        *token == 13,
                        n_qbits as usize,
                        ASTNode::ArrayType {
                            type_: Box::new(ASTNode::Qbit),
                            size: Box::new(ASTNode::Num(num)),
                        },
                    ),
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

/*
pub fn generate_var_decl_<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    Err("".to_string())
}
*/

pub fn gen_func_decl<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::FuncDef, got None".to_string()),
        Some(ASTNode::FunctionDef {
            name,
            ret_type,
            in_type,
            body,
        }) => {
            let mut input_types: Vec<ASTNode> = vec![];
            let mut input_names: Vec<String> = vec![];

            for t in in_type.clone() {
                match t {
                    ASTNode::VariableDecl { type_, name, .. } => {
                        input_types.push(*type_.unwrap());
                        input_names.push(name.clone());
                    }
                    _ => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl".to_string()),
                }
            }
            cmptime
                .function_info
                .insert(name.clone(), (input_types.clone(), *ret_type.clone()));

            cmptime.function_args.insert(name.clone(), input_names);

            if body.is_none() {
                cmptime.functions.insert(name.clone(), ASTNode::Void);
                return Ok(cmptime.clone());
            } else {
                cmptime
                    .functions
                    .insert(name.clone(), *body.clone().unwrap());
            }
            Ok(cmptime.clone())
        }
        _ => Err("BACKEND_ERROR: Expected ASTNode::FuncDef".to_string()),
    }
}

pub fn generate_assignment<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::Assignment, got None".to_string()),
        Some(ASTNode::Assignment { lval, value }) => match *value.clone() {
            ASTNode::VariableCall { name } => {
                let info = cmptime.var_info.get(&name).unwrap();
                let n_qbits = info.1;

                let mut name_ = match *lval.clone() {
                    ASTNode::VariableCall { name } => name,
                    _ => return Err("BACKEND_ERROR: Expected ASTNode::VariableCall".to_string()),
                };
                name_ = if !cmptime.aliass.contains_key(&name_) {
                    name_.clone()
                } else {
                    cmptime.aliass.get(&name_).unwrap().clone()
                };

                for i in 0..n_qbits {
                    cmptime.program.push_str(
                        format!(
                            "CPY ${} ${}\n",
                            format!("{}_{}", name, i),
                            format!("{}_{}", name_, i)
                        )
                        .as_str(),
                    );
                }
                Ok(cmptime.clone())
            }
            ASTNode::Num(num) => {
                let mut name_ = match *lval.clone() {
                    ASTNode::VariableCall { name } => name,
                    _ => return Err("BACKEND_ERROR: Expected ASTNode::VariableCall".to_string()),
                };
                name_ = if !cmptime.aliass.contains_key(&name_) {
                    name_.clone()
                } else {
                    cmptime.aliass.get(&name_).unwrap().clone()
                };
                let mut n_qbits = if num > 1 {
                    (num.to_owned() as f32).log2().ceil() as u32
                } else {
                    1
                };
                if num == 2 {
                    n_qbits = 2;
                }
                let qbits_bin: Vec<String> = (0..n_qbits)
                    .map(|n| ((num.to_owned() >> n) & 1))
                    .map(|num| format!("{num}"))
                    .collect();
                if qbits_bin.len() == 1 {
                    cmptime.program.push_str(
                        format!(
                            "SET ${} {}\n",
                            format!("{}", name_),
                            if qbits_bin[0] == "0" { "1 0" } else { "0 1" }
                        )
                        .as_str(),
                    );
                } else {
                    for (i, s) in qbits_bin.iter().enumerate() {
                        cmptime.program.push_str(
                            format!(
                                "SET ${} {}\n",
                                format!("{}_{}", name_, i),
                                if s == "0" { "1 0" } else { "0 1" }
                            )
                            .as_str(),
                        );
                    }
                }
                Ok(cmptime.clone())
            }
            _ => Err("BACKEND_ERROR: Expected ASTNode::VariableCall".to_string()),
        },

        Some(thing_else) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::Assignment, got {thing_else:?}"
        )),
    }
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
            value,
            name: _,
            token,
            ..
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
                        .push_str(format!("QAL & 0 $ \"{}\"\n", format!("{name}_{i}")).as_str())
                }
                for i in 0..n_qbits {
                    cmptime.program.push_str(
                        format!(
                            "CPY ${} ${}\n",
                            format!("{}_{}", name, i),
                            format!("TMP_{}", i)
                        )
                        .as_str(),
                    );
                }

                for i in 0..n_qbits {
                    cmptime
                        .program
                        .push_str(format!("FRE & $ \"{}\"\n", format!("TMP_{}", i)).as_str());
                }
                Ok(None)
            }
            _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
        },
        _ => return Err("BACKEND_ERROR: Expected Num, as Num was found earlier".to_string()),
    }
}

pub fn gen_func_call<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    let _ = match iterator.peek() {
        Some(ASTNode::VariableDecl { value, .. }) => {
            match *value.clone().unwrap() {
                ASTNode::FunctionCall {
                    name: func_name,
                    args,
                } => {
                    // allocate arguments
                    for (i, arg) in args.iter().enumerate() {
                        let _ = match arg {
                            ASTNode::VariableCall { name } => {
                                println!("{:?}", name);
                                cmptime.aliass.insert(
                                    cmptime.function_args.get(&func_name).unwrap()[i].clone(),
                                    name.clone(),
                                );
                                println!("{:?}", cmptime.aliass);
                            }
                            _ => return Err("BACKEND_ERROR: Expected VariableDecl ".to_string()),
                        };
                    }

                    println!("{:?}", cmptime.aliass);

                    match cmptime.functions.get(&func_name.clone()).unwrap().clone() {
                        ASTNode::Block(b) => {
                            let mut s = b.into_iter().peekable();
                            while let Ok(_) = code_gen_node(&mut s, cmptime) {
                                s.next();
                            }
                            match code_gen_node(&mut s, cmptime) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }
                        }
                        _ => return Err("BACKEND_ERROR: Expected ASTNode::Block".to_string()),
                    }
                }
                _ => return Err("BACKEND_ERROR: Expected ASTNode::FunctionCall".to_string()),
            };
        }
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        _ => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl".to_string()),
    };
    Ok(Some(cmptime.clone()))
}
pub fn gen_var_alloc<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => return Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl {
            value: _,
            name,
            token,
            type_,
        }) => match type_ {
            None => {
                return Err("Error: variable declarations need either a type or a value".to_string())
            }
            Some(other) => match *other.clone() {
                ASTNode::ArrayType { size, .. } => {
                    cmptime
                        .vars
                        .insert(name.to_string(), (cmptime.i as usize, 0));
                    let s;
                    if let ASTNode::Num(num) = *size.clone() {
                        s = num;
                    } else {
                        return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string());
                    }
                    cmptime.i += s as i32;
                    cmptime.var_info.insert(
                        name.to_string(),
                        (*token == 13, s as usize, *type_.clone().unwrap()),
                    );

                    for i in 0..s as i32 {
                        cmptime
                            .program
                            .push_str(format!("QAL & 0 $ \"{}\"\n", format!("{name}_{i}")).as_str())
                    }
                    Ok(cmptime.clone())
                }
                _ => todo!(),
            },
        },
        Some(other) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::VariableDecl, got {:?}",
            other
        )),
    }
}

pub fn generate_for<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::For, got None".to_string()),
        Some(ASTNode::For { container, .. }) => match *container.clone() {
            ASTNode::VariableCall { name } => {
                let type_ = cmptime.var_info.get(&name.clone()).unwrap().2.clone();
                println!("Type: {type_:?}");
                match type_ {
                    ASTNode::ArrayType { .. } => {
                        return gen_for_array(iterator, cmptime);
                    }
                    _ => todo!(),
                }
            }
            ASTNode::Range { .. } => {
                return gen_it_for(iterator, cmptime);
            }
            _ => Err("".to_string()),
        },
        Some(thing_else) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::For, got {thing_else:?}"
        )),
    }
}

pub fn gen_it_for<I>(iterator: &mut Peekable<I>, cmptime: &mut Comptime) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    let _ = match iterator.peek() {
        None => Err::<I, String>("BACKEND_ERROR: Expected ASTNode::For, got None".to_string()),
        Some(ASTNode::For {
            container,
            alias,
            body,
            ..
        }) => match *container.clone() {
            ASTNode::Range { start, end } => {
                let _s = match *start.clone() {
                    ASTNode::Num(num) => num,
                    _ => return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string()),
                };
                let _e = match *end.clone() {
                    ASTNode::Num(num) => num,
                    _ => return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string()),
                };
                if body.is_some() {
                    match *body.clone().unwrap() {
                        ASTNode::Block(b) => {
                            let mut s = b.into_iter().peekable();
                            for i in _s.._e {
                                cmptime.iterators.insert(alias.clone(), i as usize);
                                let _ = code_gen_node(&mut s, cmptime).unwrap().program;
                            }
                        }
                        _ => return Err("BACKEND_ERROR: Expected ASTNode::Block".to_string()),
                    }
                }
                return Ok(cmptime.clone());
            }
            _ => return Err("BACKEND_ERROR: Expected ASTNode::Range".to_string()),
        },
        Some(thing_else) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::For, got {thing_else:?}"
        )),
    };
    Ok(cmptime.clone())
}

pub fn gen_for_array<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::For, got None".to_string()),
        Some(ASTNode::For {
            container,
            alias,
            body,
            ..
        }) => match *container.clone() {
            ASTNode::VariableCall { name } => {
                let name = if !cmptime.aliass.contains_key(&name) {
                    name.clone()
                } else {
                    cmptime.aliass.get(&name).unwrap().clone()
                };
                let type_ = cmptime.var_info.get(&name.clone()).unwrap().2.clone();
                match type_ {
                    ASTNode::ArrayType { type_, size } => {
                        let _s;
                        match *size.clone() {
                            ASTNode::Num(num) => _s = num,
                            _ => {
                                return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string());
                            }
                        }
                        cmptime.vars.insert(alias.clone(), (cmptime.i as usize, 0));
                        let incr = match *type_.clone() {
                            ASTNode::Type { name: _, specifier } => match *specifier {
                                ASTNode::Qbit => 1,
                                _ => 0,
                            },
                            ASTNode::Qbit => 1,
                            _ => 0,
                        };

                        cmptime.i += incr as i32;
                        cmptime
                            .var_info
                            .insert(alias.clone(), (true, incr, *type_.clone()));

                        if body.is_none() {
                            return Ok(cmptime.clone());
                        }

                        let size_ = match *size.clone() {
                            ASTNode::Num(num) => num,
                            _ => {
                                return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string());
                            }
                        };

                        match body.clone().unwrap().deref() {
                            ASTNode::Block(nodes) => {
                                for i__ in 0..size_ {
                                    cmptime
                                        .aliass
                                        .insert(alias.clone(), format!("{}_{i__}", name.clone()));
                                    let mut it = nodes.clone().into_iter().peekable();
                                    let _ = code_gen_node(&mut it, cmptime);
                                }
                            }
                            _ => return Err("BACKEND_ERROR: Expected ASTNode::Block".to_string()),
                        };
                        Ok(cmptime.clone())
                    }
                    _ => Err(format!(
                        "BACKEND_ERROR: Expected ASTNode::ArrayType, got {type_:?}"
                    )),
                }
            }
            _ => Err("".to_string()),
        },
        Some(thing_else) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::For, got {thing_else:?}"
        )),
    }
}

pub fn fuck_join(s: Vec<ASTNode>, cmptime: &mut Comptime) -> String {
    let mut ret: String = String::new();
    for s_ in s {
        let _ = match s_ {
            ASTNode::VariableCall { name } => {
                ret.push('$');
                if !cmptime.aliass.contains_key(&name) {
                    ret.push_str(name.as_str())
                } else {
                    ret.push_str(cmptime.aliass.get(&name).unwrap().as_str())
                }
            }
            ASTNode::ExternArg { idx } => {
                ret.push_str("??");
                let _ = match *idx {
                    ASTNode::Num(num) => ret.push_str(num.to_string().as_str()),
                    ASTNode::IntCall { name } => {
                        ret.push_str(&cmptime.iterators.get(&name).unwrap().to_string())
                    }
                    _ => return "".to_string(),
                };
            }
            ASTNode::ArrayAccess { name, index } => {
                ret.push('$');
                if let ASTNode::VariableCall { name } = *name {
                    if !cmptime.aliass.contains_key(&name) {
                        ret.push_str(name.as_str())
                    } else {
                        ret.push_str(cmptime.aliass.get(&name).unwrap().as_str())
                    }
                }
                ret.push_str("_");
                let _ = match *index {
                    ASTNode::Num(num) => ret.push_str(num.to_string().as_str()),
                    ASTNode::IntCall { name: n } => {
                        ret.push_str(&cmptime.iterators.get(&n).unwrap().to_string())
                    }
                    _ => return "".to_string(),
                };
            }
            _ => return "".to_string(),
        };
        ret.push_str(" ");
    }
    ret
}

pub fn generate_gate_call<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Comptime, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected Node, got None2".to_string()),
        Some(ASTNode::GateCall { name, args }) => {
            let var_name = &format!("{name} {}\n", fuck_join(args.clone(), cmptime));
            cmptime.program.push_str(var_name.as_str());
            Ok(cmptime.clone())
        }
        Some(other) => Err(format!("BACKEND_ERROR: Expected Node, got {other:?}")),
    }
}

pub fn func_cpy<I>(
    iterator: &mut Peekable<I>,
    cmptime: &mut Comptime,
) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
    match iterator.peek() {
        None => Err("BACKEND_ERROR: Expected ASTNode::VariableDecl, got None".to_string()),
        Some(ASTNode::VariableDecl {
            name,
            value,
            type_: _,
            token: _,
        }) => match value.clone().unwrap().deref().clone() {
            ASTNode::FunctionCall { name: n, args: _ } => match cmptime.function_info.get(&n) {
                None => Err("BACKEND_ERROR: Function is not properly registered".to_string()),
                Some(f) => match &f.1 {
                    ASTNode::ArrayType { type_, size } => {
                        cmptime
                            .vars
                            .insert(name.to_string(), (cmptime.i as usize, 0));
                        let s = match *size.clone() {
                            ASTNode::Num(num) => num,
                            _ => {
                                return Err("BACKEND_ERROR: Expected ASTNode::Num".to_string());
                            }
                        };
                        cmptime
                            .var_info
                            .insert(name.to_string(), (false, s as usize, *type_.clone()));
                        cmptime.i += s as i32;
                        for i__ in 0..s {
                            cmptime
                                .program
                                .push_str(format!("CPY ${name}_{i__} $TMP_{i__}\n").as_str());
                        }
                        Ok(Some(cmptime.clone()))
                    }
                    _ => todo!(),
                },
            },
            other => Err(format!("BACKEND_ERROR: Expected Node, got {other:?}")),
        },
        Some(other) => Err(format!(
            "BACKEND_ERROR: Expected ASTNode::VariableDecl, got {other:?}"
        )),
    }
}

/*
pub fn code_gen_node<I>(iterator: &mut Peekable<I>, cmptime: &mut Comptime) -> Result<Option<Comptime>, String>
where
    I: Iterator<Item = ASTNode>,
{
}

*/
