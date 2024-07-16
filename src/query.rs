use crate::{
    ast::{EvalError, Expr},
    consts::Definitions,
    error::ParseError,
    lex::Lexer,
    parse::pratt,
    units::Quantity,
};

pub struct Query {
    pub expr: Expr,
    pub unit: Option<(String, Quantity)>,
}

impl Query {
    pub fn get_answer(&self) -> Result<String, EvalError> {
        let answer = self.expr.eval()?;
        Ok(format!(
            "{} {}",
            answer.value.cast().to_string(),
            answer.units.to_str()
        ))
    }
}

pub fn parse<'a>(s: &'a str, defs: &Definitions<'a>) -> Result<Query, ParseError<'a>> {
    let mut lexer = Lexer::new(s);
    let expr = pratt(&mut lexer, defs, 0)?;

    Ok(Query { expr, unit: None })
}
