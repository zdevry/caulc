use crate::autonum::AutoNum;
use crate::operator::{BinaryOp, UnaryOp};

pub struct EvalError {
    pub error: String,
}

pub type EvalResult = Result<AutoNum, EvalError>;

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
            Expr::Num(x) => Ok(AutoNum::Float(*x)),
            Expr::Int(n) => Ok(AutoNum::Int(*n)),
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
            BinaryOp::Add => Ok(left.auto_add(&right)),
            BinaryOp::Sub => Ok(left.auto_sub(&right)),
            BinaryOp::Mul => Ok(left.auto_mul(&right)),
            BinaryOp::Div => left.auto_div(&right),
            BinaryOp::Pow => Ok(left.auto_pow(&right)),
        }
    }
}

impl Unary {
    pub fn eval(&self) -> EvalResult {
        let operand_result = self.operand.eval()?;
        match self.op {
            UnaryOp::Negative => Ok(operand_result.negative()),
            UnaryOp::Positive => Ok(operand_result),
            UnaryOp::Percent => Ok(AutoNum::Float(operand_result.cast() / 100.0)),
            UnaryOp::Factorial => operand_result.factorial(),
        }
    }
}
