use crate::lex::{Token, TokenData};

#[derive(Clone)]
pub struct ParseError<'a> {
    pub start_pos: usize,
    pub end_pos: usize,
    pub error: String,
    pub original: &'a str,
}

pub fn get_token_str(token: &Token) -> String {
    match token.data {
        TokenData::Num(_) => format!("number '{}'", token.substr),
        TokenData::Int(_) => format!("integer '{}'", token.substr),
        TokenData::Add => String::from("symbol '+'"),
        TokenData::Sub => String::from("symbol '-'"),
        TokenData::Mul => String::from("symbol '*'"),
        TokenData::Div => String::from("symbol '/'"),
        TokenData::LBrack => String::from("symbol '('"),
        TokenData::RBrack => String::from("symbol ')'"),
        TokenData::EndOfInput => String::from("end of input"),
    }
}

pub fn expected_certain<'a>(
    token: &Token<'a>,
    expected: &str,
    original: &'a str,
) -> ParseError<'a> {
    ParseError {
        start_pos: token.start_pos,
        end_pos: token.end_pos,
        error: format!("{} is not {}", get_token_str(token), expected),
        original,
    }
}

pub fn unexpected<'a>(token: &Token<'a>, original: &'a str) -> ParseError<'a> {
    ParseError {
        start_pos: token.start_pos,
        end_pos: token.end_pos,
        error: format!("unexpected {}", get_token_str(token)),
        original,
    }
}

pub fn unknown_symbol<'a>(c: char, start_pos: usize, original: &'a str) -> ParseError<'a> {
    ParseError {
        start_pos,
        end_pos: start_pos,
        error: format!("{c} is an unknown symbol"),
        original,
    }
}

pub fn display_error<'a>(e: ParseError<'a>) {
    eprintln!("\x1B[mError in parsing: {}", e.error);
    eprintln!(" | {}", e.original);
    let token_len = (e.end_pos - e.start_pos).max(1);
    if e.start_pos == 0 {
        eprintln!(" | \x1B[31m{}", "^".repeat(token_len));
    } else {
        eprintln!(" | \x1B[{}C\x1B[31m{}", e.start_pos, "^".repeat(token_len));
    }
}
