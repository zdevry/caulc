use std::process::ExitCode;

mod ast;
mod autonum;
mod consts;
mod error;
mod lex;
mod operator;
mod parse;
mod query;
mod units;

fn display_evaluation(query: &query::Query) -> ExitCode {
    match query.get_answer() {
        Ok(answer) => {
            println!("{answer}");
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
        match query::parse(s.as_str(), &consts::Definitions::get_default()) {
            Ok(query) => display_evaluation(&query),
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
