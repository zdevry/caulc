use crate::lex;

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Binary {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

pub enum Expr {
    Num(f64),
    Binary(Box<Binary>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(x) => *x,
            Expr::Binary(b) => b.eval(),
        }
    }
}

impl Binary {
    pub fn eval(&self) -> f64 {
        let left = self.lhs.eval();
        let right = self.rhs.eval();
        match self.op {
            BinaryOp::Add => left + right,
            BinaryOp::Sub => left - right,
            BinaryOp::Mul => left * right,
            BinaryOp::Div => left / right,
        }
    }
}
