mod ast;
mod codegen;
mod common;
mod parser;
mod scanner;
mod semantics;
mod tests;

use crate::ast::Expr;
use crate::codegen::interpreter;
use crate::codegen::interpreter::Value;
use crate::common::Token;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::env::Args;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let path_string: String = get_path_argument();
    let path: &Path = Path::new(&path_string);
    let source: String = fs::read_to_string(path)?;

    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    let mut parser: Parser = Parser::new(tokens);
    match parser.parse() {
        Some(expr) => {
            let result: Value = interpreter::evaluate(&expr);
            println!("{}", result);
        }
        None => {
            eprintln!("Failed to parse expression.");
        }
    }

    let expressions: Vec<Expr> = parser.parse_all();

    for expr in expressions {
        let value = interpreter::evaluate(&expr);
        println!("{}", value);
    }

    Ok(())
}

// Should look something like this at some point..?
// fn main() -> io::Result<()> {
//     let src = std::fs::read_to_string("input.rlox")?;
//     let tokens = scanner::tokenize(&src)?;
//     let ast    = parser::parse(tokens)?;
//     semantic::check(&ast)?;
//     codegen::emit(&ast)?;
//     Ok(())
// }

fn get_path_argument() -> String {
    let mut args: Args = env::args();
    while let Some(arg) = args.next() {
        if arg == "--path" {
            return args
                .next()
                .expect("No value provided for `--path` argument!");
        }
    }

    panic!("Required `--path` argument not provided!");
}
