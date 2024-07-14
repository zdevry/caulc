use crate::lex::TokenData;

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
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
