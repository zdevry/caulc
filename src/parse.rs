use crate::{
    ast::Expr,
    autonum::AutoNum,
    consts::Definitions,
    error::{get_token_str, ParseError},
    lex::{Lexer, Token, TokenData},
    operator::{
        try_get_binary_operator, try_get_function, try_get_postfix_operator,
        try_get_prefix_operator, BinaryOp, UnaryOp,
    },
    query::is_query_keyword,
    units::Quantity,
};

pub type ParseResult<'a> = Result<Expr, ParseError<'a>>;

fn bracketed<'a>(lexer: &mut Lexer<'a>, defs: &Definitions<'a>) -> ParseResult<'a> {
    let expr = parse_expr(lexer, defs)?;
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

fn root_n<'a>(lexer: &mut Lexer<'a>, defs: &Definitions<'a>) -> ParseResult<'a> {
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

        let inner = bracketed(lexer, defs)?;
        Ok(Expr::unary(UnaryOp::RootN(n as i8), inner))
    } else {
        Err(ParseError::from_pos(
            String::from("Missing integer for root function"),
            n_token.start_pos,
            lexer.original,
        ))
    }
}

fn parse_word_at_start_of_atom<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
    word: &'a str,
    word_token: &Token<'a>,
) -> ParseResult<'a> {
    if word == "root" {
        root_n(lexer, defs)
    } else if let Some(op) = try_get_function(word) {
        let token = lexer.next_token()?;
        match token.data {
            TokenData::LBracket => {
                let inner = bracketed(lexer, defs)?;
                Ok(Expr::unary(op, inner))
            }
            _ => Err(ParseError::from_token(
                String::from("expected left bracket '(' after function"),
                &token,
                lexer.original,
            )),
        }
    } else if let Some(c) = defs.constants.get(word) {
        Ok(Expr::Quantity(c.clone()))
    } else {
        Err(ParseError::from_token(
            format!("'{word}' is not a valid function or constant"),
            word_token,
            lexer.original,
        ))
    }
}

fn atom<'a>(lexer: &mut Lexer<'a>, defs: &Definitions<'a>) -> ParseResult<'a> {
    let token = lexer.next_token()?;
    match token.data {
        TokenData::Num(x) => Ok(Expr::Quantity(Quantity::dimensionless(AutoNum::Float(x)))),
        TokenData::Int(n) => Ok(Expr::Quantity(Quantity::dimensionless(AutoNum::Int(n)))),
        TokenData::LBracket => bracketed(lexer, defs),
        TokenData::Word(w) => parse_word_at_start_of_atom(lexer, defs, w, &token),
        _ => Err(ParseError::from_token(
            format!("unexpected {}", get_token_str(&token)),
            &token,
            lexer.original,
        )),
    }
}

fn parse_unit_exponent<'a>(
    lexer: &mut Lexer<'a>,
    units_str: &mut String,
) -> Result<i8, ParseError<'a>> {
    let token = lexer.next_token()?;
    let exponent = match token.data {
        TokenData::Int(n) => {
            units_str.push_str(token.substr);
            n
        }
        TokenData::Sym('-') => {
            units_str.push('-');
            let num_token = lexer.next_token()?;
            match num_token.data {
                TokenData::Int(n) => {
                    units_str.push_str(num_token.substr);
                    -n
                }
                _ => {
                    return Err(ParseError::from_token(
                        format!(
                            "unexpected {}, expected integer in exponent",
                            get_token_str(&num_token)
                        ),
                        &num_token,
                        lexer.original,
                    ));
                }
            }
        }
        _ => {
            return Err(ParseError::from_token(
                format!(
                    "unexpected {}, expected integer in exponent",
                    get_token_str(&token)
                ),
                &token,
                lexer.original,
            ));
        }
    };

    if exponent > 127 || exponent < -128 {
        return Err(ParseError::from_token(
            format!(
                "magnitude of '{}' exceeds maximum (max: -127 to 128)",
                token.substr
            ),
            &token,
            &lexer.original,
        ));
    }

    Ok(exponent as i8)
}

pub fn parse_units<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
) -> Result<(String, Quantity), ParseError<'a>> {
    let mut units_quantity = Quantity::dimensionless(AutoNum::Float(1.0));
    let mut units_str = String::new();
    let mut first = true;

    while let (TokenData::Word(w), token) = {
        let token = lexer.peek_token()?;
        (token.data.clone(), token)
    } {
        if is_query_keyword(w) || w == "per" {
            return Ok((units_str, units_quantity));
        }

        if !first {
            units_str.push(' ');
        }

        let _ = lexer.next_token();
        let base_unit = match defs.get_unit(w) {
            Some(u) => {
                units_str.push_str(w);
                u
            }
            None => {
                return Err(ParseError::from_token(
                    format!("'{w}' is not a valid unit"),
                    &token,
                    lexer.original,
                ));
            }
        };
        if !matches!(lexer.peek_token()?.data, TokenData::Sym('^')) {
            units_quantity = units_quantity
                .mul_quantity(&base_unit)
                .map_err(|e| ParseError::from_token(e.error, &token, lexer.original))?;
            continue;
        }

        units_str.push('^');
        let _ = lexer.next_token();
        let exponent = parse_unit_exponent(lexer, &mut units_str)?;
        let exponentiated_unit = base_unit.units.pow(exponent);
        let exponentiated_value = base_unit.value.auto_pow(&AutoNum::Int(exponent as i64));
        units_quantity = exponentiated_unit
            .and_then(|d| units_quantity.mul_quantity(&Quantity::new(exponentiated_value, d)))
            .map_err(|e| ParseError::from_token(e.error, &token, lexer.original))?;
        first = false;
    }
    Ok((units_str, units_quantity))
}

fn postfixed<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
    consume_postfix_words: bool,
) -> ParseResult<'a> {
    let mut operand = atom(lexer, defs)?;
    loop {
        let token = lexer.peek_token()?;
        match token.data {
            TokenData::Sym(c) => {
                if let Some(op) = try_get_postfix_operator(c) {
                    let _ = lexer.next_token();
                    operand = Expr::unary(op, operand);
                    continue;
                } else if c == '^' {
                    let _ = lexer.next_token();
                    let power = prefixed(lexer, defs, false)?;
                    operand = Expr::binary(BinaryOp::Pow, operand, power);
                } else {
                    return Ok(operand);
                }
            }
            TokenData::Word(w) if consume_postfix_words && !is_query_keyword(w) => {
                let (_, units) = parse_units(lexer, defs)?;
                return Ok(Expr::with_units(operand, units));
            }
            _ => {
                return Ok(operand);
            }
        }
    }
}

fn prefixed<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
    consume_postfix_words: bool,
) -> ParseResult<'a> {
    let token = lexer.peek_token()?;
    match token.data {
        TokenData::Sym(c) => match try_get_prefix_operator(c) {
            Some(op) => {
                let _ = lexer.next_token();
                let operand = prefixed(lexer, defs, consume_postfix_words)?;
                Ok(Expr::unary(op, operand))
            }
            None => postfixed(lexer, defs, consume_postfix_words),
        },
        _ => postfixed(lexer, defs, consume_postfix_words),
    }
}

fn pratt<'a>(lexer: &mut Lexer<'a>, defs: &Definitions<'a>, prev_prec: u8) -> ParseResult<'a> {
    let mut lhs = prefixed(lexer, defs, true)?;

    while let Some((op, prec, r_assoc)) = {
        let token = lexer.peek_token()?;
        match token.data {
            TokenData::Sym(c) => try_get_binary_operator(c),
            _ => None,
        }
    } {
        if prec < prev_prec || (prec == prev_prec && !r_assoc) {
            break;
        }
        let _ = lexer.next_token();
        let rhs = pratt(lexer, defs, prec)?;
        lhs = Expr::binary(op, lhs, rhs);
    }

    Ok(lhs)
}

pub fn parse_expr<'a>(lexer: &mut Lexer<'a>, defs: &Definitions<'a>) -> ParseResult<'a> {
    let has_undim_prefix = match lexer.peek_token()?.data {
        TokenData::Sym(':') => {
            let _ = lexer.next_token();
            true
        }
        _ => false,
    };
    let inner_expr = pratt(lexer, defs, 0)?;
    let postfixed_expr = match lexer.peek_token()?.data {
        TokenData::Word("per") => {
            let _ = lexer.next_token();
            let (_, unit) = parse_units(lexer, defs)?;
            Expr::binary(BinaryOp::Div, inner_expr, Expr::Quantity(unit))
        }
        _ => inner_expr,
    };

    if has_undim_prefix {
        Ok(Expr::unary(UnaryOp::Undim, postfixed_expr))
    } else {
        Ok(postfixed_expr)
    }
}
