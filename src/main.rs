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
    let q = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    if q.is_empty() {
        eprintln!("Provide an expression");
        return ExitCode::FAILURE;
    }

    match query::parse(q.as_str(), &consts::Definitions::get_default()) {
        Ok(query) => display_evaluation(&query),
        Err(e) => {
            e.display_error_to_stderr();
            ExitCode::FAILURE
        }
    }
}
