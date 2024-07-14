use crate::{
    ast::Expr,
    error,
    lex::{TokenData, TokenLexer},
    operator::try_get_operator,
};

pub type ParseResult = Result<Expr, error::ParseError>;

fn atom<'a>(lexer: &mut TokenLexer<'a>) -> ParseResult {
    let token = lexer.next_token()?;
    match token.data {
        TokenData::Num(x) => Ok(Expr::Num(x)),
        TokenData::LBrack => {
            let expr = pratt(lexer, 0)?;
            let rbrack = lexer.next_token()?;
            match rbrack.data {
                TokenData::RBrack => Ok(expr),
                _ => Err(error::expected_certain(&rbrack, ")")),
            }
        }
        _ => Err(error::expected_certain(&token, "number or '('")),
    }
}

fn pratt<'a>(lexer: &mut TokenLexer<'a>, prev_prec: u8) -> ParseResult {
    let mut lhs = atom(lexer)?;

    while let Some((op, prec, r_assoc)) = try_get_operator(&lexer.peek_token()?.data) {
        if prec < prev_prec || (prec == prev_prec && !r_assoc) {
            break;
        }
        lexer.next_token();
        let rhs = pratt(lexer, prec)?;
        lhs = Expr::binary(op, lhs, rhs);
    }

    Ok(lhs)
}

pub fn parse<'a>(s: &'a str) -> ParseResult {
    let mut lexer = TokenLexer::new(s);
    pratt(&mut lexer, 0)
}
