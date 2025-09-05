use clap::{Arg, Command};
use std::fs;

mod ast;
mod lexer;
mod parser;
mod runtime;
mod repl;

fn main() {
    let app = Command::new("plain")
        .subcommand(
            Command::new("run")
                .about("Run a Plain file")
                .arg(Arg::new("file").help("Path to the .plain file").required(true)),
        )
        .subcommand(Command::new("repl").about("Start the REPL"));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            run_file(file);
        }
        _ => repl::repl(),
    }
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).unwrap();
    let mut runtime = runtime::Runtime::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line == "," {
            continue;
        }
        match parser::parse(line) {
            Ok(stmt) => {
                if let Err(e) = runtime.exec_stmt(&stmt) {
                    println!("{}", e);
                }
            }
            Err(e) => println!("Parse error in: {} - {}", line, e),
        }
    }
}
