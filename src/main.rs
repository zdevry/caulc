#![allow(warnings)]
use std::{io::Write, process::ExitCode};

mod ast;
mod autonum;
mod error;
mod lex;
mod operator;
mod parse;

fn display_evaluation(expr: &ast::Expr) -> ExitCode {
    match expr.eval() {
        Ok(result) => {
            match result {
                autonum::AutoNum::Int(n) => {
                    println!("{}", n);
                    eprintln!(" -> int")
                }
                autonum::AutoNum::Float(x) => {
                    if x >= 1e10 {
                        println!("{x:e}");
                    } else {
                        println!("{x}")
                    }
                    eprintln!(" -> float")
                }
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Calculation error: {}", e.error);
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    if let Some(s) = std::env::args().nth(1) {
        match parse::parse(s.as_str()) {
            Ok(expr) => display_evaluation(&expr),
            Err(e) => {
                error::display_error(e);
                ExitCode::FAILURE
            }
        }
    } else {
        eprintln!("Provide an expression");
        ExitCode::FAILURE
    }
}
