use crate::lex::{Sym, TokenData};

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Precedence {
    pub value: u8,
    pub reverse: bool,
}

fn get_operator(s: &Sym) -> Option<(BinaryOp, u8, bool)> {
    match s {
        Sym::Add => Some((BinaryOp::Add, 10, false)),
        Sym::Sub => Some((BinaryOp::Sub, 10, false)),
        Sym::Mul => Some((BinaryOp::Mul, 20, false)),
        Sym::Div => Some((BinaryOp::Div, 20, false)),
        _ => None,
    }
}

pub fn try_get_operator(data: &TokenData) -> Option<(BinaryOp, u8, bool)> {
    match data {
        TokenData::Sym(s) => get_operator(s),
        _ => None,
    }
}
