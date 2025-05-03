use std::env::Args;
use std::path::Path;
use std::{env, fs};

use rlox_scanner::scanner::Scanner;
use rlox_scanner::token::Token;

fn main() -> std::io::Result<()> {
    let path_string: String = get_path_argument();
    let path: &Path = Path::new(&path_string);
    let source: String = fs::read_to_string(path)?;
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
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
