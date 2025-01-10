pub mod code_gen;
pub mod parser;
pub mod tokenizer;

use std::process::exit;

use code_gen::code_gen;
use parser::from_tokens;
use tokenizer::filter_all;

use crate::parser::parse_;
use crate::tokenizer::{filter50s, tokenize};

fn main() {
    if std::env::args().count() == 2 {
        let path = std::env::args().nth(1).unwrap();
        let code = std::fs::read_to_string(path.clone()).unwrap();
        //println!("{code}\n\n");
        let mut tokens = tokenize(&code);
        tokens = filter50s(tokens.clone());
        tokens = filter_all(tokens.clone());
        //println!("{:#?}\n\n", tokens);

        let tokens1 = from_tokens(tokens.clone());
        let tokens2 = tokens.clone();

        std::fs::write(
            format!("tokens_{}.txt", path),
            format!("{:#?}{:#?}", tokens1, tokens2),
        )
        .unwrap();

        let ast = parse_(tokens1, tokens2);

        let ast_ = match ast {
            Ok(ast) => ast,
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };

        std::fs::write(format!("ast_{}.txt", path), format!("{:#?}", ast_)).unwrap();

        let out = code_gen(ast_);
        let o;
        match out {
            Ok(out) => {
                o = out;
            }
            Err(e) => {
                //println!("{:?}", e);
                exit(1);
            }
        }

        /*
            let mut buffer = String::new();
            while let Ok(_) = io::stdin().read_line(&mut buffer) {
                if buffer.contains("q\n") {
                    buffer = buffer.trim_end_matches("q\n").to_string();
                    buffer.pop();
                    break;
                }
            }

            buffer = buffer.replace("qb", "%cmb");
            buffer = buffer.replace("CX", "CNT $bit $controll %cmb");
            o.program = o.program.replace(
                "CNT $bit $controll %cmb \nDPX %cmb \nTR %cmb $bit 0 \n",
                buffer.as_str(),
            );

        */
        std::fs::write("out.txt", o.program.clone()).unwrap();
        std::fs::write(format!("comptime_{}.txt", path), format!("{:#?}", o)).unwrap();

        let executer = std::process::Command::new("qbackend")
            .arg("out.txt")
            .arg("| cat args.txt")
            .output()
            .expect("Failed to execute qbackend");

        let a = String::from_utf8_lossy(&executer.stdout);

        println!("QBACKEND output:\n\t{}", a);

        /*
            unsafe {
                println!(
                    "QBACKEND output:\n\t{}",
                    a.get_unchecked(a.find("ctrl+z").unwrap() + 8..a.len() - 1)
                        .replace("\n", "\n\t")
                );
            }
        */
    }
    println!("\t\t  Dieses Programm compiliert und führt zwei Programme in der beschriebenen Hochsprache aus.
              Die Algorithmen können im Hauptordner unter 'deutsch.qel' und 'deutsch-jozsa.qel' gefunden werden.
              Der Deutsch-Jozsa-Algorithmus wird mit 10 qubits ausgeführt, dies ist nur innerhalb der gennanten Dateien änderbar,
              indem man alle '10' mit der gewünschten Zahl ersetzt. Der Output beinhaltet lediglich den der Testing-Umgebung,
              die Informationen wie die Tokens, der AST und die Compile-time sind nach dem Ausführen dieser Executable in entsprechenden Dateien zu finden");
    let paths = ["t.qel", "t2.qel"];
    let messages = [
        "Deutsch-Algorithmus: ",
        "Deutsch-Jozsa-Algorithmus mit 10 qubits: ",
    ];
    for (path, message) in paths.iter().zip(messages.iter()) {
        println!("{}", message);
        let code = std::fs::read_to_string(path).unwrap();
        //println!("{code}\n\n");
        let mut tokens = tokenize(&code);
        tokens = filter50s(tokens.clone());
        tokens = filter_all(tokens.clone());
        //println!("{:#?}\n\n", tokens);

        let tokens1 = from_tokens(tokens.clone());
        let tokens2 = tokens.clone();

        std::fs::write(
            format!("tokens_{}.txt", path),
            format!("{:#?}{:#?}", tokens1, tokens2),
        )
        .unwrap();

        let ast = parse_(tokens1, tokens2);

        let ast_ = match ast {
            Ok(ast) => ast,
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };

        std::fs::write(format!("ast_{}.txt", path), format!("{:#?}", ast_)).unwrap();

        let out = code_gen(ast_);
        let o;
        match out {
            Ok(out) => {
                o = out;
            }
            Err(e) => {
                //println!("{:?}", e);
                exit(1);
            }
        }

        /*
            let mut buffer = String::new();
            while let Ok(_) = io::stdin().read_line(&mut buffer) {
                if buffer.contains("q\n") {
                    buffer = buffer.trim_end_matches("q\n").to_string();
                    buffer.pop();
                    break;
                }
            }

            buffer = buffer.replace("qb", "%cmb");
            buffer = buffer.replace("CX", "CNT $bit $controll %cmb");
            o.program = o.program.replace(
                "CNT $bit $controll %cmb \nDPX %cmb \nTR %cmb $bit 0 \n",
                buffer.as_str(),
            );

        */
        std::fs::write("out.txt", o.program.clone()).unwrap();
        std::fs::write(format!("comptime_{}.txt", path), format!("{:#?}", o)).unwrap();

        let executer = std::process::Command::new("qbackend")
            .arg("out.txt")
            .arg("| cat args.txt")
            .output()
            .expect("Failed to execute qbackend");

        let a = String::from_utf8_lossy(&executer.stdout);

        println!("QBACKEND output:\n\t{}", a);

        /*
            unsafe {
                println!(
                    "QBACKEND output:\n\t{}",
                    a.get_unchecked(a.find("ctrl+z").unwrap() + 8..a.len() - 1)
                        .replace("\n", "\n\t")
                );
            }
        */
    }
}
