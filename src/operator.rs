use crate::lex::TokenData;

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub fn try_get_binary_operator(data: &TokenData) -> Option<(BinaryOp, u8, bool)> {
    match data {
        TokenData::Add => Some((BinaryOp::Add, 10, false)),
        TokenData::Sub => Some((BinaryOp::Sub, 10, false)),
        TokenData::Mul => Some((BinaryOp::Mul, 20, false)),
        TokenData::Div => Some((BinaryOp::Div, 20, false)),
        _ => None,
    }
}

pub enum UnaryOp {
    Positive,
    Negative,
    Percent,
    Factorial,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Log,
}

pub fn try_get_prefix_operator(data: &TokenData) -> Option<UnaryOp> {
    match data {
        TokenData::Add => Some(UnaryOp::Positive),
        TokenData::Sub => Some(UnaryOp::Negative),
        _ => None,
    }
}

pub fn try_get_postfix_operator(data: &TokenData) -> Option<UnaryOp> {
    match data {
        TokenData::Percent => Some(UnaryOp::Percent),
        TokenData::Factorial => Some(UnaryOp::Factorial),
        _ => None,
    }
}

pub fn try_get_function(w: &str) -> Option<UnaryOp> {
    match w {
        "sqrt" => Some(UnaryOp::Sqrt),
        "sin" => Some(UnaryOp::Sin),
        "cos" => Some(UnaryOp::Cos),
        "tan" => Some(UnaryOp::Tan),
        "exp" => Some(UnaryOp::Exp),
        "ln" => Some(UnaryOp::Ln),
        "log" => Some(UnaryOp::Log),
        _ => None,
    }
}
