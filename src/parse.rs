use crate::{
    ast,
    error::{self, ParseError},
    lex::{self, Sym, TokenData},
};
use std::iter::Peekable;

pub struct Precedence {
    value: u8,
    reverse: bool,
}

pub type PeekLexer<'a> = Peekable<lex::Lexer<'a>>;
pub type ParseResult<'a> = Result<ast::Expr, error::ParseError<'a>>;

fn get_operator(s: &Sym) -> Option<(ast::BinaryOp, Precedence)> {
    match s {
        Sym::Add => Some((
            ast::BinaryOp::Add,
            Precedence {
                value: 10,
                reverse: false,
            },
        )),
        Sym::Sub => Some((
            ast::BinaryOp::Sub,
            Precedence {
                value: 10,
                reverse: false,
            },
        )),
        Sym::Mul => Some((
            ast::BinaryOp::Mul,
            Precedence {
                value: 20,
                reverse: false,
            },
        )),
        Sym::Div => Some((
            ast::BinaryOp::Div,
            Precedence {
                value: 20,
                reverse: false,
            },
        )),
        _ => None,
    }
}

fn try_get_operator(data: &TokenData) -> Option<(ast::BinaryOp, Precedence)> {
    match data {
        TokenData::Sym(s) => get_operator(s),
        _ => None,
    }
}

fn atom<'a>(lexer: &mut PeekLexer<'a>) -> ParseResult<'a> {
    match lexer.next() {
        Some(token) => match token.data {
            lex::TokenData::Num(x) => Ok(ast::Expr::Num(x)),
            lex::TokenData::LBrack => {
                let expr = pratt(lexer, 0)?;
                match lexer.next() {
                    Some(rbrack) => match &rbrack.data {
                        lex::TokenData::RBrack => Ok(expr),
                        _ => Err(error::ParseError {
                            error: format!("Unexpected '{}', expected ')'", rbrack.substr),
                            wrong_token: Some(rbrack),
                        }),
                    },
                    None => Err(error::ParseError {
                        error: String::from("Unexpected end of input, expected ')'"),
                        wrong_token: None,
                    }),
                }
            }
            _ => Err(error::ParseError {
                error: format!("'{}' is not a number or '('", token.substr),
                wrong_token: Some(token),
            }),
        },
        None => Err(error::ParseError {
            wrong_token: None,
            error: String::from("Unexpected end of input, expected number or '('"),
        }),
    }
}

fn pratt<'a>(lexer: &mut PeekLexer<'a>, prev_prec: u8) -> ParseResult<'a> {
    let mut lhs = atom(lexer)?;

    while let Some((op, prec)) = lexer.peek().and_then(|token| try_get_operator(&token.data)) {
        if prec.value < prev_prec || (prec.value == prev_prec && !prec.reverse) {
            break;
        }
        lexer.next();
        let rhs = pratt(lexer, prec.value)?;
        lhs = ast::Expr::Binary(Box::new(ast::Binary { lhs, op, rhs }));
    }

    Ok(lhs)
}

pub fn parse<'a>(s: &'a str) -> ParseResult<'a> {
    let mut lexer = lex::Lexer::new(s).peekable();
    pratt(&mut lexer, 0)
}
