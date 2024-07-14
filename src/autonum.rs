use crate::ast::{EvalError, EvalResult};

pub enum AutoNum {
    Int(i64),
    Float(f64),
}

impl AutoNum {
    pub fn cast(&self) -> f64 {
        match self {
            &AutoNum::Int(n) => n as f64,
            &AutoNum::Float(x) => x,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            &AutoNum::Int(n) => n == 0,
            &AutoNum::Float(x) => x == 0.0,
        }
    }

    pub fn auto_add(&self, other: &AutoNum) -> AutoNum {
        match (self, other) {
            (AutoNum::Int(left), AutoNum::Int(right)) => match left.checked_add(*right) {
                Some(result) => AutoNum::Int(result),
                None => AutoNum::Float(self.cast() + other.cast()),
            },
            _ => AutoNum::Float(self.cast() + other.cast()),
        }
    }

    pub fn auto_sub(&self, other: &AutoNum) -> AutoNum {
        match (self, other) {
            (AutoNum::Int(left), AutoNum::Int(right)) => match left.checked_sub(*right) {
                Some(result) => AutoNum::Int(result),
                None => AutoNum::Float(self.cast() - other.cast()),
            },
            _ => AutoNum::Float(self.cast() - other.cast()),
        }
    }

    pub fn auto_mul(&self, other: &AutoNum) -> AutoNum {
        match (self, other) {
            (AutoNum::Int(left), AutoNum::Int(right)) => match left.checked_mul(*right) {
                Some(result) => AutoNum::Int(result),
                None => AutoNum::Float(self.cast() * other.cast()),
            },
            _ => AutoNum::Float(self.cast() * other.cast()),
        }
    }

    pub fn auto_div(&self, other: &AutoNum) -> EvalResult {
        if other.is_zero() {
            return Err(EvalError {
                error: String::from("Division by 0"),
            });
        }

        match (self, other) {
            (&AutoNum::Int(left), &AutoNum::Int(right)) => {
                if left % right == 0 {
                    Ok(AutoNum::Int(left / right))
                } else {
                    Ok(AutoNum::Float((left as f64) / (right as f64)))
                }
            }
            _ => Ok(AutoNum::Float(self.cast() / other.cast())),
        }
    }

    pub fn auto_pow(&self, other: &AutoNum) -> AutoNum {
        match other {
            &AutoNum::Int(n) => {
                if n < 64 && n > -64 {
                    let negexp = n < 0;
                    let mut product = AutoNum::Int(1);
                    let product = std::iter::repeat(self)
                        .take(n.abs() as usize)
                        .fold(AutoNum::Int(1), |acc, x| acc.auto_mul(x));

                    if negexp {
                        AutoNum::Float(1.0 / product.cast())
                    } else {
                        product
                    }
                } else {
                    AutoNum::Float(self.cast().powf(n as f64))
                }
            }
            &AutoNum::Float(x) => AutoNum::Float(self.cast().powf(x)),
        }
    }

    pub fn negative(&self) -> AutoNum {
        match self {
            &AutoNum::Int(n) => match n.checked_neg() {
                Some(result) => AutoNum::Int(result),
                None => AutoNum::Float(-(n as f64)),
            },
            &AutoNum::Float(x) => AutoNum::Float(-x),
        }
    }

    pub fn factorial(&self) -> EvalResult {
        match self {
            &AutoNum::Int(n) => {
                if n < 0 {
                    Err(EvalError {
                        error: String::from("Cannot take the factorial of a negative number"),
                    })
                } else {
                    let product = (1..=n)
                        .map(|n| AutoNum::Int(n))
                        .fold(AutoNum::Int(1), |acc, x| acc.auto_mul(&x));
                    Ok(product)
                }
            }
            AutoNum::Float(_) => Err(EvalError {
                error: String::from("Cannot take the factorial of a floating point value"),
            }),
        }
    }
}
