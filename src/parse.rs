use crate::{
    ast::Expr,
    autonum::AutoNum,
    consts::try_get_constant,
    error::{get_token_str, ParseError},
    lex::{Lexer, Token, TokenData},
    operator::{
        try_get_binary_operator, try_get_function, try_get_postfix_operator,
        try_get_prefix_operator, BinaryOp, UnaryOp,
    },
    units::Quantity,
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

fn function_or_const<'a>(
    lexer: &mut Lexer<'a>,
    word: &'a str,
    word_token: &Token<'a>,
) -> ParseResult<'a> {
    if let Some(op) = try_get_function(word) {
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
    } else if let Some(c) = try_get_constant(word) {
        Ok(Expr::Quantity(c))
    } else {
        Err(ParseError::from_token(
            format!("'{word}' is not a valid function"),
            word_token,
            lexer.original,
        ))
    }
}

fn root_n<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let n_token = lexer.next_token()?;
    if let TokenData::Int(n) = n_token.data {
        if n <= 0 {
            return Err(ParseError::from_token(
                format!("cannot take {n}-root"),
                &n_token,
                lexer.original,
            ));
        } else if n > 127 {
            return Err(ParseError::from_token(
                format!("{n}-root exceeds maximum supported for the program (max: 127). Use x^(1/y) instead"),
                &n_token,
                lexer.original,
            ));
        }

        let lbrack_token = lexer.next_token()?;
        if !matches!(lbrack_token.data, TokenData::LBracket) {
            return Err(ParseError::from_token(
                String::from("expected left bracket '(' after function"),
                &lbrack_token,
                lexer.original,
            ));
        }

        let inner = bracketed(lexer)?;
        Ok(Expr::unary(UnaryOp::RootN(n as i8), inner))
    } else {
        Err(ParseError::from_pos(
            String::from("Missing integer for root function"),
            n_token.start_pos,
            lexer.original,
        ))
    }
}

fn atom<'a>(lexer: &mut Lexer<'a>) -> ParseResult<'a> {
    let token = lexer.next_token()?;
    match token.data {
        TokenData::Num(x) => Ok(Expr::Quantity(Quantity::dimensionless(AutoNum::Float(x)))),
        TokenData::Int(n) => Ok(Expr::Quantity(Quantity::dimensionless(AutoNum::Int(n)))),
        TokenData::LBracket => bracketed(lexer),
        TokenData::Word(w) => match w {
            "root" => root_n(lexer),
            _ => function_or_const(lexer, w, &token),
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
