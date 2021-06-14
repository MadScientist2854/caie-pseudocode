mod scanner;
use env::Environment;
use parser::Parser;
use scanner::Scanner;
mod parser;

mod token;
mod expr;
mod stmt;
mod pprint;
mod interpreter;
mod env;

use std::io::{Read, Result};
use pprint::PPrint;
use interpreter::Interpreter;
use token::Literal;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        parse_file(args[1].clone())?;
    } else {
        println!(r#"Usage: camps <file name>"#)
    }

    Ok(())
}

fn parse_file(path: String) -> Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::default();

    file.read_to_string(&mut contents)?;

    let mut scanner = Scanner::new(contents);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            // for token in tokens.clone() {
            //     println!("{}", token);
            // }
            let mut parser = Parser::new(tokens);
            let prog = parser.parse();
            // println!("{:?}", stmts);
            let mut env = Environment::new();
            prog.interpret(&mut env);
        },
        // Err((line, message)) => {println!("{}", message)}
        Err(_) => {}
    }

    Ok(())
}