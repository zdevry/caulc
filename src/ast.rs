use crate::autonum::AutoNum;
use crate::operator::{BinaryOp, UnaryOp};
use crate::units::Quantity;

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
    Quantity(Quantity),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
}

impl Expr {
    pub fn eval(&self) -> EvalResult {
        match self {
            Expr::Quantity(x) => Ok(x.clone()),
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
            BinaryOp::Add => left.combine_quantity_terms(
                &right,
                |a, b| a.auto_checked_binary_op(b, |x, y| x.checked_add(*y), |x, y| *x + *y),
                "cannot add two quantities with different units",
            ),
            BinaryOp::Sub => left.combine_quantity_terms(
                &right,
                |a, b| a.auto_checked_binary_op(b, |x, y| x.checked_sub(*y), |x, y| *x - *y),
                "cannot subtract two quantities with different units",
            ),
            BinaryOp::Mul => Ok(Quantity::new(
                left.value.auto_mul(&right.value),
                left.units.combine(&right.units, false)?,
            )),
            BinaryOp::Div => Ok(Quantity::new(
                left.value.auto_div(&right.value)?,
                left.units.combine(&right.units, true)?,
            )),
            BinaryOp::Pow => left.pow_quantity(&right),
        }
    }
}

impl Unary {
    pub fn eval(&self) -> EvalResult {
        let operand_result = self.operand.eval()?;
        match self.op {
            UnaryOp::Positive => Ok(operand_result),
            UnaryOp::Negative => Ok(Quantity::new(
                operand_result.value.auto_checked_binary_op(
                    &AutoNum::Int(0),
                    |x, _| x.checked_neg(),
                    |x, _| -x,
                ),
                operand_result.units.clone(),
            )),
            UnaryOp::RootN(n) => Ok(Quantity::new(
                operand_result.value.auto_root_n(n)?,
                operand_result.units.root(n)?,
            )),
            UnaryOp::Percent => operand_result.unitless_op(
                |x| Ok(x.cast_then(|y| y / 100.0)),
                "cannot use percentage on quantity with units, consider using x / 100 instead",
            ),
            UnaryOp::Factorial => operand_result.unitless_op(
                |x| x.auto_factorial(),
                "cannot take the factorial of quantity with units",
            ),
            UnaryOp::Sin => operand_result.unitless_op(
                |x| Ok(x.cast_then(|y| y.sin())),
                "cannot take the sine of quantity with units (degrees are dimensionless)",
            ),
            UnaryOp::Cos => operand_result.unitless_op(
                |x| Ok(x.cast_then(|y| y.cos())),
                "cannot take the cosine of quantity with units (degrees are dimensionless)",
            ),
            UnaryOp::Tan => operand_result.unitless_op(
                |x| Ok(x.cast_then(|y| y.tan())),
                "cannot take the cosine of quantity with units (degrees are dimensionless)",
            ),
            UnaryOp::Exp => operand_result.unitless_op(
                |x| Ok(x.cast_then(|y| y.exp())),
                "cannot take the exponentiation of a quantity with units",
            ),
            UnaryOp::Ln => operand_result.unitless_op(
                |x| {
                    x.auto_positive_only(
                        |y| y.ln(),
                        false,
                        "Cannot take the natural log of a non-positive number",
                    )
                },
                "cannot take the natural log of a quantity with units",
            ),
            UnaryOp::Log => operand_result.unitless_op(
                |x| {
                    x.auto_positive_only(
                        |y| y.log10(),
                        false,
                        "Cannot take the natural log of a non-positive number",
                    )
                },
                "cannot take the natural log of a quantity with units",
            ),
        }
    }
}
