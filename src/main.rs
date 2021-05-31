mod scanner;
use parser::Parser;
use scanner::Scanner;
mod parser;

mod token;
use token::{Token, TokenType};
mod expr;
mod pprint;

use std::io::{Read, Result};
use expr::Expr;
use pprint::PPrint;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        parse_file(args[1].clone())?;
    } else {
        println!(r#"Usage: camps <file name>"#)
    }

    // let e = Expr::Grouping(Box::new(
    //     Expr::Binary(Box::new(
    //         Expr::Literal(Token::new(TokenType::Int(5), "5".to_string(), 0))
    //     ),
    //     Token::new(TokenType::Plus, "+".to_string(), 0),
    //     Box::new(Expr::Unary(
    //         Token::new(TokenType::Minus, "-".to_string(), 0),
    //         Box::new(
    //             Expr::Literal(Token::new(TokenType::Int(5), "5".to_string(), 0))
    //         )
    //     )))
    // ));
    // println!("{}", e.prettify());

    Ok(())
}

fn parse_file(path: String) -> Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::default();

    file.read_to_string(&mut contents)?;

    let mut scanner = Scanner::new(contents);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in tokens.clone() {
                println!("{}", token.to_string());
            }
            let mut parser = Parser::new(tokens);
            println!("{}", parser.expr().prettify())
        },
        // Err((line, message)) => {println!("{}", message)}
        Err(_) => {}
    }

    Ok(())
}