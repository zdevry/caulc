use std::process::ExitCode;

mod ast;
mod autonum;
mod consts;
mod error;
mod functions;
mod lex;
mod operator;
mod parse;
mod units;

fn display_evaluation(expr: &ast::Expr) -> ExitCode {
    match expr.eval() {
        Ok(result) => {
            println!("{}", result.to_str());
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
                e.display_error_to_stderr();
                ExitCode::FAILURE
            }
        }
    } else {
        eprintln!("Provide an expression");
        ExitCode::FAILURE
    }
}
