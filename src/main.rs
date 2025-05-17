mod ast;
mod codegen;
mod common;
mod parser;
mod scanner;
mod semantics;
mod tests;

use crate::codegen::interpreter::Interpreter;
use crate::parser::parser::Parser;
use crate::scanner::Scanner;
use std::env::Args;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    if let Some(path_string) = get_path_argument() {
        let source = fs::read_to_string(Path::new(&path_string))?;
        run(&source);
    } else {
        println!("RLOX REPL - press Ctrl+D to exit");
        let stdin = io::stdin();

        loop {
            let mut buffer = String::new();

            loop {
                print!("> ");
                io::stdout().flush()?;
                let mut line = String::new();

                if stdin.read_line(&mut line)? == 0 {
                    return Ok(()); // EOF
                }

                if line.trim().is_empty() {
                    break;
                }

                buffer.push_str(&line);
            }

            if !buffer.trim().is_empty() {
                run(&buffer);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn get_path_argument() -> Option<String> {
    let mut args: Args = env::args();
    while let Some(arg) = args.next() {
        if arg == "--path" {
            return Some(
                args.next()
                    .expect("No value provided for `--path` argument!"),
            );
        }
    }

    None
}

fn run(source: impl Into<String>) {
    let tokens = Scanner::new(source).tokenize().unwrap_or_else(|| exit(65));

    let statements = Parser::new(tokens).parse().unwrap_or_else(|| exit(65));

    Interpreter::new(statements)
        .interpret()
        .unwrap_or_else(|_| exit(70));
}
