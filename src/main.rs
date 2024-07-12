#![allow(warnings)]
use std::process::ExitCode;

mod ast;
mod error;
mod lex;
mod parse;

fn main() -> ExitCode {
    if let Some(s) = std::env::args().nth(1) {
        match parse::parse(s.as_str()) {
            Ok(expr) => {
                println!("{}", expr.eval());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("Error in parsing: {}", e.error);
                ExitCode::FAILURE
            }
        }
    } else {
        eprintln!("Provide an expression");
        ExitCode::FAILURE
    }
}
