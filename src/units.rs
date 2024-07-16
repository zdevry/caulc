use crate::{
    ast::{EvalError, EvalResult},
    autonum::{AutoNum, AutoNumResult},
};

fn gcd(m: i8, n: i8) -> i8 {
    match n {
        0 => m,
        _ => gcd(n, m % n),
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            .zip(other.exponents)
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

    pub fn no_units(&self) -> bool {
        self.exponents.iter().all(|n| *n == 0)
    }

    const SI_UNIT_NAMES: [&'static str; 7] = ["kg", "m", "s", "A", "K", "mol", "cd"];
    pub fn to_str(&self) -> String {
        self.exponents
            .iter()
            .zip(Dimension::SI_UNIT_NAMES)
            .filter_map(|(&e, u)| {
                if e == 0 {
                    return None;
                }
                let frac_gcd = gcd(e, self.denom);
                let simpl_numerator = e / frac_gcd;
                let simpl_denom = self.denom / frac_gcd;
                match (simpl_numerator, simpl_denom) {
                    (1, 1) => Some(String::from(u)),
                    (_, 1) => Some(format!("{u}^{simpl_numerator}")),
                    _ => Some(format!("{u}^{simpl_numerator}/{simpl_denom}")),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Clone)]
pub struct Quantity {
    pub value: AutoNum,
    pub units: Dimension,
}

impl Quantity {
    pub fn new(value: AutoNum, units: Dimension) -> Quantity {
        Quantity { value, units }
    }

    pub fn dimensionless(value: AutoNum) -> Quantity {
        Quantity {
            value,
            units: Dimension::new(0, 0, 0, 0, 0, 0, 0, 1),
        }
    }

    pub fn to_str(&self) -> String {
        match self.value {
            AutoNum::Int(n) => format!("{n} {} :: int", self.units.to_str()),
            AutoNum::Float(x) => {
                if x >= 1e10 {
                    format!("{x:e} {} :: float", self.units.to_str())
                } else {
                    format!("{x} {} :: float", self.units.to_str())
                }
            }
        }
    }

    pub fn combine_quantity_terms(
        &self,
        other: &Quantity,
        combine_op: fn(&AutoNum, &AutoNum) -> AutoNum,
        error_msg: &str,
    ) -> EvalResult {
        if self.units != other.units {
            Err(EvalError {
                error: String::from(error_msg),
            })
        } else {
            Ok(Quantity::new(
                combine_op(&self.value, &other.value),
                self.units.clone(),
            ))
        }
    }

    pub fn pow_quantity(&self, other: &Quantity) -> EvalResult {
        if !other.units.no_units() {
            return Err(EvalError {
                error: String::from("cannot take the power of a quantity with units"),
            });
        }
        if self.units.no_units() {
            return Ok(Quantity::new(
                self.value.auto_pow(&other.value),
                self.units.clone(),
            ));
        }

        if let AutoNum::Int(n) = other.value {
            if n <= 127 && n >= -128 {
                let units = self.units.pow(n as i8)?;
                Ok(Quantity::new(self.value.auto_pow(&other.value), units))
            } else {
                Err(EvalError {
                    error: String::from(
                        "power too large to raise quantity with units (max: -128 <= n <= 127)",
                    ),
                })
            }
        } else {
            Err(EvalError {
                error: String::from("raising a quantity with units to a non-integer power not supported (power cannot be guaranteed to be an integer)"),
            })
        }
    }

    pub fn unitless_op(&self, op: fn(&AutoNum) -> AutoNumResult, error_msg: &str) -> EvalResult {
        if self.units.no_units() {
            Ok(Quantity::dimensionless(op(&self.value)?))
        } else {
            Err(EvalError {
                error: String::from(error_msg),
            })
        }
    }
}
