use std::io::{self, Write};
use crate::parser;
use crate::runtime::Runtime;

pub fn repl() {
    let mut runtime = Runtime::new();
    loop {
        print!("Plain> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        match parser::parse(input) {
            Ok(stmt) => {
                if let Err(e) = runtime.exec_stmt(&stmt) {
                    println!("{}", e);
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
}
