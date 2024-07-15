use crate::autonum::AutoNum;
use crate::operator::{BinaryOp, UnaryOp};
use crate::units::{Dimension, Quantity};

pub struct EvalError {
    pub error: String,
}

pub type EvalResult = Result<Quantity, EvalError>;

pub struct Binary {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

pub struct Unary {
    pub op: UnaryOp,
    pub operand: Expr,
}

pub enum Expr {
    Num(f64),
    Int(i64),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
}

impl Expr {
    pub fn eval(&self) -> EvalResult {
        match self {
            Expr::Num(x) => Ok(Quantity::new(
                AutoNum::Float(*x),
                Dimension::new(0, 0, 0, 0, 0, 0, 0, 1),
            )),
            Expr::Int(n) => Ok(Quantity::new(
                AutoNum::Int(*n),
                Dimension::new(0, 0, 0, 0, 0, 0, 0, 1),
            )),
            Expr::Binary(b) => b.eval(),
            Expr::Unary(u) => u.eval(),
        }
    }

    pub fn binary(op: BinaryOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary(Box::new(Binary { op, lhs, rhs }))
    }

    pub fn unary(op: UnaryOp, operand: Expr) -> Expr {
        Expr::Unary(Box::new(Unary { op, operand }))
    }
}

impl Binary {
    pub fn eval(&self) -> EvalResult {
        let left = self.lhs.eval()?;
        let right = self.rhs.eval()?;
        match self.op {
            BinaryOp::Add => left.add(&right),
            BinaryOp::Sub => left.sub(&right),
            BinaryOp::Mul => left.mul(&right),
            BinaryOp::Div => left.div(&right),
            BinaryOp::Pow => left.pow(&right),
        }
    }
}

impl Unary {
    pub fn eval(&self) -> EvalResult {
        let operand_result = self.operand.eval()?;
        match self.op {
            UnaryOp::Negative => Ok(operand_result.negative()),
            UnaryOp::Positive => Ok(operand_result),
            UnaryOp::Percent => operand_result.percent(),
            UnaryOp::Factorial => operand_result.factorial(),
            UnaryOp::RootN(n) => operand_result.root_n(n),
            UnaryOp::Sin => operand_result.sin(),
            UnaryOp::Cos => operand_result.cos(),
            UnaryOp::Tan => operand_result.tan(),
            UnaryOp::Exp => operand_result.exp(),
            UnaryOp::Ln => operand_result.ln(),
            UnaryOp::Log => operand_result.log(),
        }
    }
}
