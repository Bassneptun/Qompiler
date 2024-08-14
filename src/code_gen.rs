use crate::parser::ASTNode;
use core::panicking::panic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Comptime {
    program: String,
    functions: HashMap<String, String>,
    function_info: HashMap<String, (Vec<String>, String)>, // signature
    vars: HashMap<String, (usize, usize)>,                 // position in memory
    var_info: HashMap<String, (bool, usize, String)>,      // is_const, size, type
    tmp_vars: HashMap<String, (usize, usize)>,             // position in memory
    tmp_var_info: HashMap<String, (usize, String)>,        // size, type
    types: HashMap<String, usize>,                         // name to size of type
    structs: HashMap<(String, String), (usize, String)>, // name of type + field to size of field and type
    i: i32,
}

// Assign unique numbers to each node type
pub fn get_discriminant(node: &ASTNode) -> u32 {
    match node {
        ASTNode::Program(_) => 1,          // done
        ASTNode::FunctionDef { .. } => 2,  // done
        ASTNode::VariableDecl { .. } => 3, // done... maybe
        ASTNode::Block(_) => 4,            // done
        ASTNode::For { .. } => 5,
        ASTNode::Return(_) => 6,
        ASTNode::FunctionCall { .. } => 7,
        ASTNode::GateCall { .. } => 8,
        ASTNode::Gate { .. } => 9,
        ASTNode::Struct { .. } => 10,
        ASTNode::Array { .. } => 11,
        ASTNode::ArrayIndex(_) => 12,
        ASTNode::ArrayAccess { .. } => 13,
        ASTNode::Reference { .. } => 14,
        ASTNode::Dereference { .. } => 15,
        ASTNode::Break => 16,
        ASTNode::Void => 17,
        ASTNode::Qbit => 18,
        ASTNode::Qdit => 19,
        ASTNode::Custom => 20,
        ASTNode::Num(_) => 21,
        ASTNode::Type { .. } => 22,
        ASTNode::Range { .. } => 23,
        ASTNode::VariableCall { .. } => 24,
        ASTNode::Assignment { .. } => 25,
        ASTNode::Pointer { .. } => 26,
        ASTNode::StructAccess { .. } => 27,
    }
}

pub fn convert_to_string(node: &ASTNode) -> String {
    let discriminant = get_discriminant(node);
    match node {
        ASTNode::Program(children) => {
            let children_str = children
                .iter()
                .map(|child| convert_to_string(child))
                .collect::<Vec<String>>()
                .join(".");
            format!("{}.{}", discriminant, children_str)
        }
        ASTNode::FunctionDef { body, .. } => {
            let mut children_str = String::new();
            if let Some(body_node) = body {
                children_str = convert_to_string(body_node);
            }
            format!("{}.{}", discriminant, children_str)
        }
        ASTNode::VariableDecl { value, type_, .. } => {
            let mut children_str = vec![];
            if let Some(value_node) = value {
                children_str.push(if convert_to_string(value_node).starts_with("8") {
                    "8".to_string()
                } else {
                    convert_to_string(value_node)
                });
            }
            if let Some(type_node) = type_ {
                children_str.push(convert_to_string(type_node));
            }
            if children_str.is_empty() {
                format!("{}", discriminant)
            } else {
                format!("{}.{}", discriminant, children_str.join("."))
            }
        }
        ASTNode::Block(_children) => "4".to_string(),
        ASTNode::For {
            container, body, ..
        } => {
            let mut children_str = vec![convert_to_string(container)];
            if let Some(body_node) = body {
                children_str.push(convert_to_string(body_node));
            }
            format!("{}:{}", discriminant, children_str.join("."))
        }
        ASTNode::Return(child) => {
            format!("{}:{}", discriminant, convert_to_string(child))
        }
        ASTNode::FunctionCall { args, .. } => {
            let children_str = args
                .iter()
                .map(|child| convert_to_string(child))
                .collect::<Vec<String>>()
                .join(".");
            format!("{}:{}", discriminant, children_str)
        }
        ASTNode::GateCall { args, .. } => {
            let children_str = args
                .iter()
                .map(|child| convert_to_string(child))
                .collect::<Vec<String>>()
                .join(".");
            format!("{}:{}", discriminant, children_str)
        }
        ASTNode::Gate { args, .. } => {
            let children_str = args
                .iter()
                .map(|child| convert_to_string(child))
                .collect::<Vec<String>>()
                .join(".");
            format!("{}:{}", discriminant, children_str)
        }
        ASTNode::Struct { types, .. } => {
            let children_str = types
                .iter()
                .map(|child| convert_to_string(child))
                .collect::<Vec<String>>()
                .join(".");
            format!("{}:{}", discriminant, children_str)
        }
        ASTNode::Array {
            type_,
            elements,
            size,
            ..
        } => {
            let mut children_str = vec![convert_to_string(type_)];
            children_str.extend(elements.iter().map(|child| convert_to_string(child)));
            children_str.push(convert_to_string(size));
            format!("{}:{}", discriminant, children_str.join("."))
        }
        ASTNode::ArrayIndex(_) => format!("{}", discriminant),
        ASTNode::ArrayAccess { index, .. } => {
            format!("{}:{}", discriminant, convert_to_string(index))
        }
        ASTNode::Reference { .. } => format!("{}", discriminant),
        ASTNode::Dereference { .. } => format!("{}", discriminant),
        ASTNode::Break => format!("{}", discriminant),
        ASTNode::Void => format!("{}", discriminant),
        ASTNode::Qbit => format!("{}", discriminant),
        ASTNode::Qdit => format!("{}", discriminant),
        ASTNode::Custom => format!("{}", discriminant),
        ASTNode::Num(_) => format!("{}", discriminant),
        ASTNode::Type { specifier, .. } => {
            format!("{}:{}", discriminant, convert_to_string(specifier))
        }
        ASTNode::Range { start, end } => {
            format!(
                "{}:{}.{}",
                discriminant,
                convert_to_string(start),
                convert_to_string(end)
            )
        }
        ASTNode::VariableCall { .. } => format!("{}", discriminant),
        ASTNode::Assignment {
            arr_index, value, ..
        } => {
            let mut children_str = vec![];
            if let Some(index_node) = arr_index {
                children_str.push(convert_to_string(index_node));
            }
            children_str.push(convert_to_string(value));
            format!("{}:{}", discriminant, children_str.join("."))
        }
        ASTNode::Pointer { value, type_, .. } => {
            let mut children_str = vec![];
            if let Some(value_node) = value {
                children_str.push(convert_to_string(value_node));
            }
            children_str.push(convert_to_string(type_));
            format!("{}:{}", discriminant, children_str.join("."))
        }
        ASTNode::StructAccess { .. } => format!("{}", discriminant),
    }
}

pub fn extract_body(body: &ASTNode) -> Vec<ASTNode> {
    match body {
        ASTNode::Block(children) => children.clone(),
        _ => panic!("expected block"),
    }
}

pub fn generate_code2(nodes: Vec<ASTNode>, comptime: Comptime) -> String {
    let converted = ASTNode::Program(nodes);
    let copy = comptime;
    generate_code(&converted, copy).program
}

pub fn to_name(s: ASTNode) -> String {
    match s {
        ASTNode::Type { name, specifier: _ } => name,
        _ => panic!("s"),
    }
}

pub fn to_names_(signature: (Box<ASTNode>, Vec<ASTNode>)) -> (Vec<String>, String) {
    (
        signature.1.iter().map(|s| to_name(s.clone())).collect(),
        to_name(*signature.0.clone()),
    )
}

pub fn generate_code(root: &ASTNode, comptime: Comptime) -> Comptime {
    let mut comptime_ = comptime.clone();
    if let ASTNode::Program(children) = root {
        let extracted = children;

        let iter = extracted.iter();
        for child in iter {
            let match_str = convert_to_string(child);
            match match_str.as_str() {
                "2.4" => {
                    if let ASTNode::FunctionDef {
                        name,
                        body,
                        signature,
                        ..
                    } = child
                    {
                        comptime_.functions.insert(
                            name.clone(),
                            generate_code2(extract_body(&body.clone().unwrap()), comptime_.clone()),
                        );
                        comptime_
                            .function_info
                            .insert(name.clone(), to_names_(signature.clone()));
                    }
                }
                "3" => {
                    continue;
                }
                "3.7" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        let name_ = name.clone();
                        if let ASTNode::FunctionCall { name, args: _ } = *value.clone().unwrap() {
                            comptime_.vars.insert(
                                name_.clone(),
                                (
                                    comptime_.i as usize,
                                    comptime_.i as usize
                                        + *comptime_
                                            .types
                                            .get(comptime_.functions.get(&name).unwrap())
                                            .unwrap(),
                                ),
                            );
                            comptime_.var_info.insert(
                                name_.clone(),
                                (
                                    *token == 13,
                                    *comptime_
                                        .types
                                        .get(comptime_.functions.get(&name).unwrap())
                                        .unwrap(),
                                    comptime_.functions.get(&name).unwrap().clone(),
                                ),
                            );
                            comptime_.i += *comptime
                                .types
                                .get(comptime_.functions.get(&name).unwrap())
                                .unwrap() as i32;
                            comptime_
                                .program
                                .push_str(comptime_.functions.get(&name).unwrap());
                            for i in 0..*comptime_
                                .types
                                .get(comptime_.functions.get(&name).expect("should exist"))
                                .unwrap()
                            {
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 0 $ \"{name__}_{index}\"\nCPY {name__} tmp_{index}\n",
                                        name__ = name_,
                                        index = i
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                }
                "3.8" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        comptime_.vars.insert(
                            name.clone(),
                            (
                                comptime_.i.try_into().unwrap(),
                                comptime_.i.try_into().unwrap(),
                            ),
                        );
                        comptime_
                            .var_info
                            .insert(name.clone(), (*token == 13, 1, "Qubit".to_string()));
                        comptime_.i += 1;
                        comptime_.program.push_str(
                            format!(
                                "QAL & 0 $ \"{name}\"\n{code}CPY {name} tmp\n",
                                name = name,
                                code = generate_code2(
                                    vec![*value.clone().unwrap().clone()],
                                    comptime_.clone()
                                )
                            )
                            .as_str(),
                        );
                    }
                }
                "3.13.12" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        let tmp = name;
                        if let ASTNode::ArrayAccess { index, name } = *value.clone().unwrap() {
                            let mut im: u32 = 0;
                            if let ASTNode::ArrayIndex(n) = *index {
                                im = n;
                            }
                            if (*comptime_.var_info.get(&name).unwrap()).2 == "Qubit".to_string() {
                                comptime_.vars.insert(
                                    tmp.clone(),
                                    (
                                        comptime_.i.try_into().unwrap(),
                                        comptime_.i.try_into().unwrap(),
                                    ),
                                );
                                comptime_
                                    .var_info
                                    .insert(tmp.clone(), (*token == 13, 1, "Qubit".to_string()));
                                comptime_.i += 1;
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 0 $ \"{name}\"\nCPY {name} {place}\n",
                                        name = tmp,
                                        place = format!("{}_{}", name, im)
                                    )
                                    .as_str(),
                                );
                            } else {
                                comptime_.vars.insert(
                                    tmp.clone(),
                                    (
                                        comptime_.i.try_into().unwrap(),
                                        comptime_.i as usize + *comptime.types.get(&name).unwrap(),
                                    ),
                                );
                                comptime_.var_info.insert(
                                    tmp.clone(),
                                    (
                                        *token == 13,
                                        *comptime_.types.get(&name).unwrap(),
                                        name.clone(),
                                    ),
                                );
                                comptime_.i += *comptime.types.get(&name).unwrap() as i32;
                                for i in
                                    0..(*comptime_.types.get(&name).unwrap()).try_into().unwrap()
                                {
                                    comptime_.program.push_str(
                                        format!(
                                            "QAL & 0 $ \"{name}\"\nCPY {name} {place}\n",
                                            name = format!("{}_{}", tmp, i),
                                            place = format!("{}_{}_{}", name, im, i)
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                        }
                    }
                }
                "3.14" => {
                    if let ASTNode::VariableDecl {
                        name, value, token, ..
                    } = child
                    {
                        let tmp = name;
                        if let ASTNode::Reference { name } = *value.clone().unwrap() {
                            comptime_
                                .vars
                                .insert(tmp.clone(), comptime_.vars.get(&name).unwrap().clone());
                            comptime_.var_info.insert(
                                tmp.clone(),
                                (
                                    *token == 13,
                                    0,
                                    "*".to_string()
                                        + comptime_.var_info.get(name.as_str()).unwrap().2.as_str(),
                                ),
                            );
                        }
                    }
                }
                "3.15" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        let tmp = name;
                        if let ASTNode::Dereference { name } = *value.clone().unwrap() {
                            comptime_.vars.insert(
                                tmp.clone(),
                                (
                                    comptime_.i.try_into().unwrap(),
                                    comptime_.i as usize
                                        + comptime_.var_info.get(name.as_str()).unwrap().1,
                                ),
                            );
                            comptime_.var_info.insert(
                                tmp.clone(),
                                (
                                    *token == 13,
                                    comptime_.var_info.get(name.as_str()).unwrap().1,
                                    comptime_.var_info.get(name.as_str()).unwrap().2.clone(),
                                ),
                            );
                            comptime_.i += comptime.var_info.get(name.as_str()).unwrap().1 as i32;
                            for i in 0..comptime_.var_info.get(name.as_str()).unwrap().1 {
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 0 $ \"{name}\"\nCPY {name} {place}\n",
                                        name = format!("{}_{}", tmp, i),
                                        place = format!("{}_{}", name, i)
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                }
                "3.24" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        let tmp = name;
                        if let ASTNode::VariableCall { name } = *value.clone().unwrap() {
                            comptime_.vars.insert(
                                tmp.clone(),
                                (
                                    comptime_.i.try_into().unwrap(),
                                    comptime_.i as usize
                                        + comptime_.var_info.get(name.as_str()).unwrap().1,
                                ),
                            );
                            comptime_.var_info.insert(
                                tmp.clone(),
                                (
                                    *token == 13,
                                    comptime_.var_info.get(name.as_str()).unwrap().1,
                                    comptime_.var_info.get(name.as_str()).unwrap().2.clone(),
                                ),
                            );
                            comptime_.i += comptime.var_info.get(name.as_str()).unwrap().1 as i32;
                            for i in 0..comptime_.var_info.get(name.as_str()).unwrap().1 {
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 0 $ \"{name}\"\nCPY {name} {place}\n",
                                        name = format!("{}_{}", tmp, i),
                                        place = format!("{}_{}", name, i)
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                }
                "3.27" => {
                    if let ASTNode::VariableDecl {
                        name,
                        value,
                        type_: _,
                        token,
                    } = child
                    {
                        let tmp = name;
                        if let ASTNode::StructAccess { name, member } = *value.clone().unwrap() {
                            comptime_.vars.insert(
                                tmp.clone(),
                                (
                                    comptime_.i.try_into().unwrap(),
                                    comptime_.i as usize
                                        + comptime_
                                            .structs
                                            .get(&(name.clone(), member.clone()))
                                            .unwrap()
                                            .0,
                                ),
                            );
                            comptime_.var_info.insert(
                                tmp.clone(),
                                (
                                    *token == 13,
                                    comptime_
                                        .structs
                                        .get(&(name.clone(), member.clone()))
                                        .unwrap()
                                        .0,
                                    comptime_
                                        .structs
                                        .get(&(name.clone(), member.clone()))
                                        .unwrap()
                                        .1
                                        .clone(),
                                ),
                            );
                            comptime_.i += comptime
                                .structs
                                .get(&(name.clone(), member.clone()))
                                .unwrap()
                                .0 as i32;
                            for i in 0..comptime_
                                .structs
                                .get(&(name.clone(), member.clone()))
                                .unwrap()
                                .0
                            {
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 0 $ \"{name}\"\nCPY {name} {place}\n",
                                        name = format!("{}_{}", tmp, i),
                                        place = format!("{}_{}", name, i)
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                }
                // here should be the implementations for not deduced variables, but I decided to
                // ignore them for now. why would you even annotate when I specificaly add type
                // deduction...
                "4" => {
                    // now that I think about it, there is no scenario where this should be
                    // called. if it ever is kys
                    if let ASTNode::Block(b) = child {
                        comptime_
                            .program
                            .push_str(&generate_code2((*b.clone()).to_vec(), comptime_.clone()));
                    }
                }
                "5.7.4" => {
                    if let ASTNode::For {
                        container,
                        alias,
                        body,
                    } = child
                    {
                        let func_out: String;
                        if let ASTNode::FunctionCall { name, args: _ } = *container.clone() {
                            comptime_.tmp_vars.insert(
                                alias.clone(),
                                (
                                    comptime_.i as usize,
                                    comptime_.i as usize
                                        + *comptime_
                                            .types
                                            .get(comptime_.functions.get(&name).unwrap())
                                            .unwrap(),
                                ),
                            );
                            comptime_.tmp_var_info.insert(
                                alias.clone(),
                                (
                                    *comptime_
                                        .types
                                        .get(comptime_.functions.get(&name).unwrap())
                                        .unwrap(),
                                    comptime_.functions.get(&name).unwrap().clone(),
                                ),
                            );
                            comptime_.i += *comptime
                                .types
                                .get(comptime_.functions.get(&name).unwrap())
                                .unwrap() as i32;
                            comptime_
                                .program
                                .push_str(comptime_.functions.get(&name).unwrap());
                            for i in 0..*comptime_
                                .types
                                .get(comptime_.functions.get(&name).expect("should exist"))
                                .unwrap()
                            {
                                comptime_.program.push_str(
                                    format!(
                                        "QAL & 1 $ \"{alias}_{index}\"\nCPY {alias} tmp_{index}\n",
                                        alias = alias,
                                        index = i
                                    )
                                    .as_str(),
                                );
                            }
                            func_out = comptime_.function_info.get(&name).unwrap().1.clone()
                        }
                        if let ASTNode::Block(nodes) = *(body.clone().unwrap()) {
                            match func_out {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    return comptime_;
}
