use crate::{
    ast::Expr,
    error,
    lex::{TokenData, TokenLexer},
    operator::try_get_binary_operator,
};

pub type ParseResult<'a> = Result<Expr, error::ParseError<'a>>;

fn atom<'a>(lexer: &mut TokenLexer<'a>) -> ParseResult<'a> {
    let token = lexer.next_token()?;
    match token.data {
        TokenData::Num(x) => Ok(Expr::Num(x)),
        TokenData::Int(n) => Ok(Expr::Num(n as f64)),
        TokenData::LBrack => {
            let expr = pratt(lexer, 0)?;
            let rbrack = lexer.next_token()?;
            match rbrack.data {
                TokenData::RBrack => Ok(expr),
                _ => Err(error::expected_certain(
                    &rbrack,
                    "closing bracket ')'",
                    lexer.original,
                )),
            }
        }
        _ => Err(error::unexpected(&token, lexer.original)),
    }
}

fn pratt<'a>(lexer: &mut TokenLexer<'a>, prev_prec: u8) -> ParseResult<'a> {
    let mut lhs = atom(lexer)?;

    while let Some((op, prec, r_assoc)) = try_get_binary_operator(&lexer.peek_token()?.data) {
        if prec < prev_prec || (prec == prev_prec && !r_assoc) {
            break;
        }
        lexer.next_token();
        let rhs = pratt(lexer, prec)?;
        lhs = Expr::binary(op, lhs, rhs);
    }

    Ok(lhs)
}

pub fn parse<'a>(s: &'a str) -> ParseResult<'a> {
    let mut lexer = TokenLexer::new(s);
    let expr = pratt(&mut lexer, 0)?;
    let final_token = lexer.peek_token()?;
    match final_token.data {
        TokenData::EndOfInput => Ok(expr),
        _ => Err(error::ParseError {
            error: String::from("Input should end here"),
            start_pos: final_token.start_pos,
            end_pos: final_token.start_pos,
            original: lexer.original,
        }),
    }
}
