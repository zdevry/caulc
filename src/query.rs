use crate::{
    ast::{EvalError, Expr},
    autonum::AutoNum,
    consts::Definitions,
    error::{get_token_str, ParseError},
    lex::{Lexer, Token, TokenData},
    parse::{parse_expr, parse_units},
    units::Quantity,
};

const QUERY_KEYWORDS: [&'static str; 9] = [
    "in",
    "round",
    "fixed",
    "scientific",
    "hideunits",
    "rd",
    "fd",
    "sc",
    "hide",
];
pub fn is_query_keyword(s: &str) -> bool {
    QUERY_KEYWORDS.contains(&s)
}

#[derive(Clone)]
enum ScientificDisplay {
    Never,
    Always(bool),
    Exceeds(f64, f64),
}

pub struct Query {
    expr: Expr,
    unit: Option<(String, Quantity)>,
    round: Option<(usize, bool)>,
    scientific: Option<ScientificDisplay>,
    nounits: bool,
}

fn get_scientific(x: f64, n: usize, fixed: bool) -> String {
    if !fixed {
        return format!("{x:.n$e}");
    }

    let num_chars_before_exponent = match n {
        0 => 2,
        _ => n + 3,
    };

    if !x.is_normal() {
        return match x {
            f64::INFINITY => format!("{}inf", " ".repeat(num_chars_before_exponent + 2)),
            f64::NEG_INFINITY => format!("{}-inf", " ".repeat(num_chars_before_exponent + 1)),
            _ => format!("{}NaN", " ".repeat(num_chars_before_exponent + 2)),
        };
    }

    let unfixed = format!("{x:+.n$e}");
    let mut exponent = unfixed
        .chars()
        .skip(num_chars_before_exponent)
        .collect::<Vec<char>>();

    // exponents will always have at least 2 characters, example: e4
    // target is 5 chars, example: ____e+012
    if exponent[1] != '-' {
        exponent.insert(1, '+');
    }
    while exponent.len() < 5 {
        exponent.insert(2, '0');
    }

    format!(
        "{}{}",
        String::from_iter(unfixed.chars().take(num_chars_before_exponent)),
        String::from_iter(exponent)
    )
}

impl Query {
    fn requires_scientific_notation(&self, x: f64) -> (bool, bool) {
        match self
            .scientific
            .clone()
            .unwrap_or(ScientificDisplay::Exceeds(1e-5, 1e10))
        {
            ScientificDisplay::Never => (false, false),
            ScientificDisplay::Always(fixed) => (true, fixed),
            ScientificDisplay::Exceeds(lo, hi) => {
                let y = x.abs();
                (y >= hi || y <= lo && y != 0.0, false)
            }
        }
    }

    fn get_float_str(&self, x: f64) -> String {
        let (scientific, scientific_fixed) = self.requires_scientific_notation(x);
        let (rounding, fixed) =
            self.round
                .unwrap_or(if scientific { (4, false) } else { (8, false) });
        if scientific {
            get_scientific(x, rounding, scientific_fixed)
        } else if rounding == 0 {
            format!("{x:.0}")
        } else {
            let rounded = format!("{x:.rounding$}");
            if fixed {
                rounded
            } else {
                rounded
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        }
    }

    fn get_autonum_str(&self, num: &AutoNum) -> String {
        match num {
            AutoNum::Int(n) => {
                let casted_value = *n as f64;
                let (scientific, scientific_fixed) =
                    self.requires_scientific_notation(casted_value);
                if scientific {
                    let (rounding, _) = self.round.unwrap_or((4, false));
                    get_scientific(casted_value, rounding, scientific_fixed)
                } else {
                    n.to_string()
                }
            }
            AutoNum::Float(x) => self.get_float_str(*x),
        }
    }

    pub fn get_answer(&self) -> Result<String, EvalError> {
        let answer = self.expr.eval()?;
        if let Some((unit_str, unit_val)) = &self.unit {
            if answer.units != unit_val.units {
                return Err(EvalError {
                    error: format!(
                        "cannot convert to given units: {} -> {}",
                        answer.units.to_si_units_str(),
                        unit_str
                    ),
                });
            }

            let converted = answer.value.cast() / unit_val.value.cast();
            let converted_str = self.get_float_str(converted);
            if self.nounits {
                Ok(format!("{converted_str}"))
            } else {
                Ok(format!("{converted_str} {unit_str}"))
            }
        } else {
            let number_str = self.get_autonum_str(&answer.value);
            if answer.units.no_units() || self.nounits {
                Ok(number_str)
            } else {
                Ok(format!("{number_str} {}", answer.units.to_si_units_str()))
            }
        }
    }
}

fn parse_in_query<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
    query: &mut Query,
    query_token: &Token<'a>,
) -> Result<(), ParseError<'a>> {
    if query.unit.is_some() {
        return Err(ParseError::from_token(
            String::from("'in' query already specified"),
            &query_token,
            &lexer.original,
        ));
    }

    let units = parse_units(lexer, defs)?;
    query.unit = Some(units);
    Ok(())
}

fn parse_round_query<'a>(
    lexer: &mut Lexer<'a>,
    query: &mut Query,
    query_token: &Token<'a>,
    fixed: bool,
) -> Result<(), ParseError<'a>> {
    if query.round.is_some() {
        return Err(ParseError::from_token(
            String::from("'round' query already specified"),
            &query_token,
            &lexer.original,
        ));
    }

    let round_token = lexer.next_token()?;
    if let TokenData::Int(n) = round_token.data {
        query.round = Some((n as usize, fixed));
        Ok(())
    } else {
        Err(ParseError::from_token(
            String::from("expected integer for round query"),
            &round_token,
            lexer.original,
        ))
    }
}

fn parse_scientific_if_query<'a>(
    lexer: &mut Lexer<'a>,
    query: &mut Query,
    query_token: &Token<'a>,
) -> Result<(), ParseError<'a>> {
    let mut limit_hi: Option<f64> = None;
    let mut limit_lo: Option<f64> = None;

    loop {
        let subquery_token = lexer.next_token()?;
        let (limit_ref, keyword) = match subquery_token.data {
            TokenData::Word("over") | TokenData::Sym('>') => (&mut limit_hi, "over"),
            TokenData::Word("under") | TokenData::Sym('<') => (&mut limit_lo, "under"),
            _ => {
                break;
            }
        };

        if limit_ref.is_some() {
            return Err(ParseError::from_token(
                format!("'{keyword}' subquery already specified"),
                &subquery_token,
                lexer.original,
            ));
        }

        let num_token = lexer.next_token()?;
        if let TokenData::Num(x) = num_token.data {
            *limit_ref = Some(x);
        } else {
            return Err(ParseError::from_token(
                format!("expected floating point number for '{keyword}' subquery"),
                &num_token,
                lexer.original,
            ));
        }
    }

    if limit_lo.is_none() && limit_hi.is_none() {
        return Err(ParseError::from_token(
            format!("expected 'more'/'less' for 'scientific if' subquery"),
            &query_token,
            lexer.original,
        ));
    }

    query.scientific = Some(ScientificDisplay::Exceeds(
        limit_lo.unwrap_or(1e-5),
        limit_hi.unwrap_or(1e10),
    ));
    Ok(())
}

fn parse_scientific_query<'a>(
    lexer: &mut Lexer<'a>,
    query: &mut Query,
    query_token: &Token<'a>,
) -> Result<(), ParseError<'a>> {
    if query.scientific.is_some() {
        return Err(ParseError::from_token(
            String::from("'scientific' query already specified"),
            &query_token,
            lexer.original,
        ));
    }

    let subquery_token = lexer.next_token()?;
    match subquery_token.data {
        TokenData::Word("never" | "ne") => {
            query.scientific = Some(ScientificDisplay::Never);
            Ok(())
        }
        TokenData::Word("always" | "al") => {
            let token = lexer.peek_token()?;
            if matches!(token.data, TokenData::Word("fixed" | "fd")) {
                let _ = lexer.next_token();
                query.scientific = Some(ScientificDisplay::Always(true));
                Ok(())
            } else {
                query.scientific = Some(ScientificDisplay::Always(false));
                Ok(())
            }
        }
        TokenData::Word("if") => parse_scientific_if_query(lexer, query, &subquery_token),
        _ => Err(ParseError::from_token(
            String::from("expected 'never'/'always'/'if' for 'scientific' query"),
            &subquery_token,
            lexer.original,
        )),
    }
}

fn parse_query<'a>(
    lexer: &mut Lexer<'a>,
    defs: &Definitions<'a>,
    query: &mut Query,
    query_word: &'a str,
    query_token: &Token<'a>,
) -> Result<(), ParseError<'a>> {
    match query_word {
        "in" => parse_in_query(lexer, defs, query, query_token),
        "round" | "rd" => parse_round_query(lexer, query, query_token, false),
        "fixed" | "fd" => parse_round_query(lexer, query, query_token, true),
        "scientific" | "sc" => parse_scientific_query(lexer, query, query_token),
        "hideunits" | "hide" => {
            if query.nounits {
                Err(ParseError::from_token(
                    String::from("'hideunits' query already specified"),
                    query_token,
                    lexer.original,
                ))
            } else {
                query.nounits = true;
                Ok(())
            }
        }
        _ => Err(ParseError::from_token(
            format!("unknown query keyword '{}'", query_word),
            &query_token,
            lexer.original,
        )),
    }
}

pub fn parse<'a>(s: &'a str, defs: &Definitions<'a>) -> Result<Query, ParseError<'a>> {
    let mut lexer = Lexer::new(s);
    let expr = parse_expr(&mut lexer, defs)?;

    let mut query = Query {
        expr,
        unit: None,
        round: None,
        scientific: None,
        nounits: false,
    };

    loop {
        let query_token = lexer.next_token()?;
        match query_token.data {
            TokenData::Word(w) => parse_query(&mut lexer, defs, &mut query, w, &query_token)?,
            TokenData::EndOfInput => {
                break;
            }
            _ => {
                return Err(ParseError::from_token(
                    format!("unexpected {}", get_token_str(&query_token)),
                    &query_token,
                    lexer.original,
                ));
            }
        }
    }

    Ok(query)
}
