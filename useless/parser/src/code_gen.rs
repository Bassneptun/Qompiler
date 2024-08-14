include!("parser.rs");

use std::collections::HashMap

struct ProgramMemory {
    count: u32,
    current_scope: u32,
    variables: HashMap<String, (usize, (usize, usize))>,
    variable_properties: HashMap<String, (bool, u8, u8)>,
    stack_handler: Vec<String>,
}

fn num(node: ASTNode) -> usize {
    node as usize
}

fn num_info(num: usize, node: ASTNode) -> Vec<usize> {
    match num{
        0 => panic!("why???");
        1 =>
    }
}

fn representation(root: ASTNode) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    match root{
        ASTNode::Program => {
            for child in root.0{
                let mut tmp = String::new();
            }
        },
        _ => panic!("why the fuck");
    }
}

fn code_gen(input: Root) {
    let mut program_code: Vec<String> = vec![];
    let mut program_memory = ProgramMemory {
        count: 0,
        current_scope: 0,
        variables: HashMap::new(),
        variable_properties: HashMap::new(),
        stack_handler: vec![],
    };

    for line in input.children {
        match line.type_ {
            LType::Alloc => {
                if line.children.as_ref().expect("No arguments, syntax error").len() == 2{
                    let num: f32 = unsafe { line.children.as_ref().unwrap()[1].data.as_ref().unwrap().data__.opt2 as f32 };
                    let qbit_count: usize;
                    if num >= 2.0 {
                        qbit_count = (num - 1.0).log2().ceil() as usize;
                    } else if num >= 0.0 {
                        qbit_count = 1;
                    } else {
                        qbit_count = (num.abs() - 1.0).log2().ceil() as usize;
                    }

                    for i in 0..qbit_count {
                        program_code.push(format!(
                            "QAL & 0 $ {}",
                            format!("{}_{}", ManuallyDrop::into_inner(unsafe {
                                line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1.clone()
                            }), i)
                        ));
                    }

                    program_memory.variables.insert(
                        unsafe { &line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1 }
                            .to_string(),
                        (
                            0,
                            (
                                program_memory.count as usize,
                                program_memory.count as usize + qbit_count as usize,
                            ),
                        ),
                    );
                    program_memory.variable_properties.insert(
                        unsafe { &line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1 }
                            .to_string(),
                        (
                            line.data[0] == 1,
                            program_memory.current_scope as u8,
                            (qbit_count > 1) as u8,
                        ),
                    );
                } else {
                    program_code.push(format!(
                            "QAL & 0 $ {}",
                            ManuallyDrop::into_inner(unsafe {
                                line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1.clone()
                            })));
                    program_memory.variables.insert(
                        unsafe { &line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1 }
                            .to_string(),
                        (
                            0,
                            (
                                program_memory.count as usize,
                                program_memory.count as usize,
                            ),
                        ),
                    );
                    program_memory.variable_properties.insert(
                        unsafe { &line.children.as_ref().unwrap()[0].data.as_ref().unwrap().data__.opt1 }
                            .to_string(),
                        (
                            line.data[0] == 1,
                            program_memory.current_scope as u8,
                            0 as u8,
                        ),
                    );
                }
            }
            LType::Expression => {
                program_code.push(format!("{} ${}", __TOKENS[line.data[0] as usize], ManuallyDrop::into_inner(unsafe{line.children.unwrap()[0].data.as_ref().unwrap().data__.opt1.clone()}))); // missing: caching Expression calls
            }

            LType::Process => {}
            _ => todo!(),
        }
    }
}
