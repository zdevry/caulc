use crate::lex::{Token, TokenData};

#[derive(Clone)]
pub struct ParseError<'a> {
    pub start_pos: usize,
    pub end_pos: usize,
    pub error: String,
    pub original: &'a str,
}

impl<'a> ParseError<'a> {
    pub fn from_token(error: String, token: &Token<'a>, original: &'a str) -> ParseError<'a> {
        ParseError {
            error,
            start_pos: token.start_pos,
            end_pos: token.end_pos,
            original,
        }
    }

    pub fn from_pos(error: String, pos: usize, original: &'a str) -> ParseError<'a> {
        ParseError {
            error,
            start_pos: pos,
            end_pos: pos,
            original,
        }
    }

    pub fn display_error_to_stderr(&self) {
        eprintln!("\x1B[mError in parsing: {}", self.error);
        eprintln!(" | {}", self.original);
        let token_len = (self.end_pos - self.start_pos).max(1);
        if self.start_pos == 0 {
            eprintln!(" | \x1B[31m{}", "^".repeat(token_len));
        } else {
            eprintln!(
                " | \x1B[{}C\x1B[31m{}",
                self.start_pos,
                "^".repeat(token_len)
            );
        }
    }
}

pub fn get_token_str(token: &Token) -> String {
    match token.data {
        TokenData::Num(_) => format!("number '{}'", token.substr),
        TokenData::Int(_) => format!("integer '{}'", token.substr),
        TokenData::LBracket => String::from("left bracket '('"),
        TokenData::RBracket => String::from("right bracket ')'"),
        TokenData::Sym(c) => format!("symbol '{c}'"),
        TokenData::Word(s) => format!("word '{s}'"),
        TokenData::EndOfInput => String::from("end of input"),
    }
}
