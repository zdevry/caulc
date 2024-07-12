use crate::lex;

pub struct ParseError<'a> {
    pub wrong_token: Option<lex::Token<'a>>,
    pub error: String,
}
