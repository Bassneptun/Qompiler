include!("parser.rs");

use std::collections::HashMap;

#[derive(Debug)]
struct Comptime {
    program: String,
    p_variables: HashMap<String, (i32, (Vec<String>, f32))>, // size, (names, value in b10)
    p_variables_attr: HashMap<String, ((i32, i32), (bool, i32))>, // position, is_const, scope(not
    // yet used)
    i: i32,
}


fn pjfhs(root: ASTNode) -> Vec<String>{
    let mut ret: Vec<String>;
    for node in root {
        let mut app = String::new();
        app.push_str(for);
    }
    
}

fn gen_var_decl(name: String, value: Option<Box<ASTNode>>, type_: Option<Box<ASTNode>>, token: i32, comptime: &mut Comptime) -> (){
    if type_.is_none(){
        if value.is_none() {
            continue;
        } else {
            match *value.unwrap() {
                ASTNode::Num(num) => {
                    let n = num.log(2.0).ceil() as i32;
                    let mut out = String::new();
                    comptime.p_variables.insert(name, (n, (0..n.iter().map(|x| format!("{}_{}", name, x)).collect(), num)));
                    for i in 0..n {
                        let current_num = (x >> i) & 1;
                        out.push(format!("QAL & 0 $ \"{}_{}\"\n", name, i));
                        out.push(format!("SET ${} {} {}\n", format!("{}_{}", name, i), current_num, !current_num));
                    }
                    comptime.program.push_str(out.clone());
                    comptime.p_variables_attr.insert(name, ((comptime.i, 0), (token == 18, n)));
                    comptime.i += n;
                    return
                },
                ASTNode::VariableCall{name_2} => {
                    comptime.p_variables.insert(name, (comptime.p_variables.get(&name_2).unwrap().0, (0..comptime.p_variables.get(&name_2).unwrap().0.iter().map(|x| format!("{}_{}", name, x)).collect(), comptime.p_variables.get(&name_2).unwrap().1.1)));
                    for i in 0..comptime.p_variables.get(&name).unwrap().0 {
                        let current = (comptime.p_variables.get(&name_2).unwrap().1.1 >> i) & 1;
                        comptime.program.push(format!("QAL & 0 $ \"{}_{}\"\n", name, i));
                        comptime.program.push(format!("SET ${} {} {}\n", format!("{}_{}", name, i), current, !current));
                    }
                    comptime.p_variables_attr.insert(name, ((comptime.i, 0), (token == 18, comptime.p_variables.get(&name_2).unwrap().0)));
                    comptime.i += comptime.p_variables.get(&name).unwrap().0;
                    return
                },
                ASTNode::GateCall{

                }
            }
        }
    }
}

fn code_gen(root: ASTNode) {
    let mut comptime = Comptime { program: String::new(), p_variables: HashMap::new() };
    if let ASTNode::Program(prog) = root {
        for node in prog{
            match node {
                ASTNode::VariableDecl{name, value, type_, token} => {
                }
            }
        }
    }
}
