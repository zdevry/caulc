use crate::ast::EvalError;

#[derive(Clone, Debug)]
pub enum AutoNum {
    Int(i64),
    Float(f64),
}

pub type AutoNumResult = Result<AutoNum, EvalError>;

impl AutoNum {
    pub fn cast(&self) -> f64 {
        match self {
            &AutoNum::Int(n) => n as f64,
            &AutoNum::Float(x) => x,
        }
    }

    pub fn cast_then<F: Fn(&f64) -> f64>(&self, f: F) -> AutoNum {
        AutoNum::Float(f(&self.cast()))
    }

    pub fn auto_checked_binary_op(
        &self,
        other: &AutoNum,
        checked_op: fn(&i64, &i64) -> Option<i64>,
        fallback: fn(&f64, &f64) -> f64,
    ) -> AutoNum {
        match (self, other) {
            (AutoNum::Int(left), AutoNum::Int(right)) => match checked_op(left, right) {
                Some(result) => AutoNum::Int(result),
                None => AutoNum::Float(fallback(&(*left as f64), &(*right as f64))),
            },
            _ => AutoNum::Float(fallback(&self.cast(), &other.cast())),
        }
    }

    pub fn auto_mul(&self, other: &AutoNum) -> AutoNum {
        self.auto_checked_binary_op(other, |a, b| a.checked_mul(*b), |a, b| *a * *b)
    }

    pub fn auto_div(&self, other: &AutoNum) -> AutoNumResult {
        match (self, other) {
            (&AutoNum::Int(left), &AutoNum::Int(right)) => {
                if right == 0 {
                    Err(EvalError {
                        error: String::from("Division by 0"),
                    })
                } else if left % right == 0 {
                    Ok(AutoNum::Int(left / right))
                } else {
                    Ok(AutoNum::Float((left as f64) / (right as f64)))
                }
            }
            _ => {
                let denom = other.cast();
                if denom == 0.0 {
                    Err(EvalError {
                        error: String::from("Division by 0"),
                    })
                } else {
                    Ok(self.cast_then(|x| x / denom))
                }
            }
        }
    }

    pub fn auto_pow(&self, other: &AutoNum) -> AutoNum {
        match other {
            &AutoNum::Int(n) => {
                if n < 64 && n > -64 {
                    let negexp = n < 0;
                    let product = std::iter::repeat(self)
                        .take(n.abs() as usize)
                        .fold(AutoNum::Int(1), |acc, x| acc.auto_mul(x));

                    if negexp {
                        product.cast_then(|x| 1.0 / x)
                    } else {
                        product
                    }
                } else {
                    self.cast_then(|x| x.powf(n as f64))
                }
            }
            &AutoNum::Float(x) => self.cast_then(|y| y.powf(x)),
        }
    }

    pub fn auto_factorial(&self) -> AutoNumResult {
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

    pub fn auto_positive_only(
        &self,
        f: fn(&f64) -> f64,
        can_equals_zero: bool,
        error_msg: &str,
    ) -> AutoNumResult {
        let val = self.cast();
        if val < 0.0 || (val == 0.0 && !can_equals_zero) {
            Err(EvalError {
                error: String::from(error_msg),
            })
        } else {
            Ok(AutoNum::Float(f(&val)))
        }
    }

    pub fn auto_root_n(&self, n: i8) -> AutoNumResult {
        match n {
            2 => self.auto_positive_only(
                |x| x.sqrt(),
                true,
                "Cannot take the square root of a negative number",
            ),
            3 => Ok(self.cast_then(|x| x.cbrt())),
            _ => {
                let val = self.cast();
                if n % 2 == 0 && val < 0.0 {
                    Err(EvalError {
                        error: format!("Cannot take the {n}-root of a negative number"),
                    })
                } else {
                    Ok(self.cast_then(|x| x.powf(1.0 / (n as f64))))
                }
            }
        }
    }
}
