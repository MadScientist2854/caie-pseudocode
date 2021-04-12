mod scanner;
use scanner::Scanner;

mod token;

use std::io::{Read, Result};

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
            for token in tokens {
                println!("{}", token.to_string());
            }
        },
        // Err((line, message)) => {println!("{}", message)}
        Err(_) => {}
    }

    Ok(())
}