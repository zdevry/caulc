use crate::ast::EvalError;

pub fn gcd(m: i8, n: i8) -> i8 {
    match n {
        0 => m,
        _ => gcd(n, m % n),
    }
}

#[derive(Debug, PartialEq)]
pub struct Dimension {
    exponents: [i8; 7],
    denom: i8,
}

impl Dimension {
    pub fn new(
        mass: i8,
        length: i8,
        time: i8,
        current: i8,
        temp: i8,
        mole: i8,
        lum: i8,
        denom: i8,
    ) -> Dimension {
        Dimension {
            exponents: [mass, length, time, current, temp, mole, lum],
            denom,
        }
        .simplify()
    }

    pub fn simplify(&self) -> Dimension {
        let dividing_factor = self
            .exponents
            .iter()
            .fold(self.denom, |acc, n| gcd(acc, n.abs()));
        let mut result_exponents = self.exponents.clone();
        for exponent in &mut result_exponents {
            *exponent /= dividing_factor;
        }
        Dimension {
            exponents: result_exponents,
            denom: self.denom / dividing_factor,
        }
    }

    pub fn combine(&self, other: &Dimension, div_other: bool) -> Result<Dimension, EvalError> {
        let gcd_ab = gcd(self.denom, other.denom);
        let factor_a = other.denom / gcd_ab;
        let factor_b = self.denom / gcd_ab;
        let mut result_exponents = self.exponents.clone();
        let added_correctly = result_exponents
            .iter_mut()
            .zip(other.exponents.iter())
            .map(|(a, b)| -> bool {
                a.checked_mul(factor_a)
                    .zip(b.checked_mul(factor_b))
                    .and_then(|(a_norm, b_norm)| match div_other {
                        true => a_norm.checked_sub(b_norm),
                        false => a_norm.checked_add(b_norm),
                    })
                    .inspect(|x| *a = *x)
                    .is_some()
            })
            .all(|b| b);

        if !added_correctly {
            return Err(EvalError {
                error: String::from("overflow error in the calculation of units"),
            });
        }

        let result_dim = Dimension {
            exponents: result_exponents,
            denom: self.denom * factor_a,
        };
        Ok(result_dim.simplify())
    }

    pub fn root(&self, n: i8) -> Result<Dimension, EvalError> {
        let denom = self.denom.checked_mul(n);
        match denom {
            Some(denom) => Ok(Dimension {
                exponents: self.exponents,
                denom,
            }
            .simplify()),
            None => Err(EvalError {
                error: String::from("overflow error in the calculation of units"),
            }),
        }
    }

    pub fn pow(&self, n: i8) -> Result<Dimension, EvalError> {
        let mut result_exponents = self.exponents.clone();
        for r in &mut result_exponents {
            match r.checked_mul(n) {
                Some(exp) => *r = exp,
                None => {
                    return Err(EvalError {
                        error: String::from("overflow error in the calculation of units"),
                    })
                }
            }
        }

        Ok(Dimension {
            exponents: result_exponents,
            denom: self.denom,
        }
        .simplify())
    }
}
