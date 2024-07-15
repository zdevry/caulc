use crate::{
    ast::Expr,
    error::{get_token_str, ParseError},
    lex::{Lexer, TokenData},
    operator::{
        try_get_binary_operator, try_get_function, try_get_postfix_operator,
        try_get_prefix_operator, BinaryOp,
    },
};

pub type ParseResult<'a> = Result<Expr, ParseError<'a>>;

fn bracketed<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let expr = pratt(lexer, 0)?;
    let rbrack = lexer.next_token()?;
    match rbrack.data {
        TokenData::RBracket => Ok(expr),
        _ => Err(ParseError::from_token(
            format!("unexpected {}", get_token_str(&rbrack)),
            &rbrack,
            lexer.original,
        )),
    }
}

fn atom<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let token = lexer.next_token()?;
    match token.data {
        TokenData::Num(x) => Ok(Expr::Num(x)),
        TokenData::Int(n) => Ok(Expr::Int(n)),
        TokenData::LBracket => bracketed(lexer),
        TokenData::Word(w) => match try_get_function(w) {
            Some(op) => {
                let token = lexer.next_token()?;
                match token.data {
                    TokenData::LBracket => {
                        let inner = bracketed(lexer)?;
                        Ok(Expr::unary(op, inner))
                    }
                    _ => Err(ParseError::from_token(
                        String::from("expected left bracket '(' after function"),
                        &token,
                        lexer.original,
                    )),
                }
            }
            None => Err(ParseError::from_token(
                format!("'{w}' is not a valid function"),
                &token,
                lexer.original,
            )),
        },
        _ => Err(ParseError::from_token(
            format!("unexpected {}", get_token_str(&token)),
            &token,
            lexer.original,
        )),
    }
}

fn postfixed<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let mut operand = atom(lexer)?;
    loop {
        let token = lexer.peek_token()?;
        if let Some(op) = try_get_postfix_operator(&token.data) {
            let _ = lexer.next_token();
            operand = Expr::unary(op, operand);
            continue;
        }
        match token.data {
            TokenData::Pow => {
                let _ = lexer.next_token();
                let power = prefixed(lexer)?;
                operand = Expr::binary(BinaryOp::Pow, operand, power);
            }
            _ => {
                return Ok(operand);
            }
        }
    }
}

fn prefixed<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let token = lexer.peek_token()?;
    match try_get_prefix_operator(&token.data) {
        Some(op) => {
            let _ = lexer.next_token();
            let operand = prefixed(lexer)?;
            Ok(Expr::unary(op, operand))
        }
        None => postfixed(lexer),
    }
}

fn pratt<'a>(lexer: &mut Lexer<'a>, prev_prec: u8) -> ParseResult<'a> {
    let mut lhs = prefixed(lexer)?;

    while let Some((op, prec, r_assoc)) = try_get_binary_operator(&lexer.peek_token()?.data) {
        if prec < prev_prec || (prec == prev_prec && !r_assoc) {
            break;
        }
        let _ = lexer.next_token();
        let rhs = pratt(lexer, prec)?;
        lhs = Expr::binary(op, lhs, rhs);
    }

    Ok(lhs)
}

pub fn parse<'a>(s: &'a str) -> ParseResult<'a> {
    let mut lexer = Lexer::new(s);
    let expr = pratt(&mut lexer, 0)?;
    let final_token = lexer.peek_token()?;
    match final_token.data {
        TokenData::EndOfInput => Ok(expr),
        _ => Err(ParseError::from_token(
            format!("unexpected {}", get_token_str(&final_token)),
            &final_token,
            lexer.original,
        )),
    }
}
