use crate::lex::Token;

#[derive(Clone)]
pub struct ParseError {
    pub start_pos: usize,
    pub end_pos: usize,
    pub error: String,
}

pub fn expected_certain(token: &Token, expected: &str) -> ParseError {
    ParseError {
        start_pos: token.start_pos,
        end_pos: token.end_pos,
        error: format!("{} is not {}", token.substr, expected),
    }
}

pub fn unexpected(token: &Token) -> ParseError {
    ParseError {
        start_pos: token.start_pos,
        end_pos: token.end_pos,
        error: format!("unexpected {}", token.substr),
    }
}

pub fn unknown_symbol(c: char, pos: usize) -> ParseError {
    ParseError {
        start_pos: pos,
        end_pos: pos,
        error: format!("{c} is an unknown symbol"),
    }
}
