use crate::{ast::EvalError, autonum::AutoNum};

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

    pub fn no_units(&self) -> bool {
        self.exponents.iter().all(|n| *n == 0)
    }

    pub fn get_units_str(&self) -> String {
        String::from("[TODO]") // TODO: implement get_dimension_str
    }
}

pub struct Quantity {
    pub value: AutoNum,
    pub units: Dimension,
}

impl Quantity {
    pub fn new(value: AutoNum, units: Dimension) -> Quantity {
        Quantity { value, units }
    }

    pub fn to_str(&self) -> String {
        match self.value {
            AutoNum::Int(n) => format!("{n} {} :: int", self.units.get_units_str()),
            AutoNum::Float(x) => {
                if x >= 1e10 {
                    format!("{x:e} {} :: float", self.units.get_units_str())
                } else {
                    format!("{x} {} :: float", self.units.get_units_str())
                }
            }
        }
    }

    pub fn add(&self, other: &Quantity) -> Result<Quantity, EvalError> {
        if self.units != other.units {
            Err(EvalError {
                error: format!(
                    "cannot add two quantities with different units: {} and {}",
                    self.units.get_units_str(),
                    other.units.get_units_str()
                ),
            })
        } else {
            let result = self.value.auto_add(&other.value);
            Ok(Quantity::new(result, self.units.clone()))
        }
    }

    pub fn sub(&self, other: &Quantity) -> Result<Quantity, EvalError> {
        if self.units != other.units {
            Err(EvalError {
                error: format!(
                    "cannot subtract two quantities with different units: {} and {}",
                    self.units.get_units_str(),
                    other.units.get_units_str()
                ),
            })
        } else {
            let result = self.value.auto_sub(&other.value);
            Ok(Quantity::new(result, self.units.clone()))
        }
    }

    pub fn mul(&self, other: &Quantity) -> Result<Quantity, EvalError> {
        let units = self.units.combine(&other.units, false)?;
        let value = self.value.auto_mul(&other.value);
        Ok(Quantity::new(value, units))
    }

    pub fn div(&self, other: &Quantity) -> Result<Quantity, EvalError> {
        let units = self.units.combine(&other.units, true)?;
        let value = self.value.auto_div(&other.value)?;
        Ok(Quantity::new(value, units))
    }

    pub fn pow(&self, other: &Quantity) -> Result<Quantity, EvalError> {
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

    pub fn negative(&self) -> Quantity {
        Quantity::new(self.value.auto_negative(), self.units.clone())
    }

    pub fn percent(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from(
                    "cannot use percentage on quantity with units, consider using x / 100 instead",
                ),
            });
        }

        Ok(Quantity::new(
            AutoNum::Float(self.value.cast() / 100.0),
            self.units.clone(),
        ))
    }

    pub fn factorial(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from("cannot take the factorial of quantity with units"),
            });
        }

        Ok(Quantity::new(
            self.value.auto_factorial()?,
            self.units.clone(),
        ))
    }

    pub fn root_n(&self, n: i8) -> Result<Quantity, EvalError> {
        let units = self.units.root(n)?;
        let value = self.value.auto_root_n(n)?;

        Ok(Quantity::new(value, units))
    }

    pub fn sin(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from(
                    "cannot take the sine of quantity with units (degrees are dimensionless)",
                ),
            });
        }

        Ok(Quantity::new(
            AutoNum::Float(self.value.cast().sin()),
            self.units.clone(),
        ))
    }

    pub fn cos(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from(
                    "cannot take the cosine of quantity with units (degrees are dimensionless)",
                ),
            });
        }

        Ok(Quantity::new(
            AutoNum::Float(self.value.cast().cos()),
            self.units.clone(),
        ))
    }

    pub fn tan(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from(
                    "cannot take the tangent of quantity with units (degrees are dimensionless)",
                ),
            });
        }

        Ok(Quantity::new(
            AutoNum::Float(self.value.cast().tan()),
            self.units.clone(),
        ))
    }

    pub fn exp(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from("cannot take the exponentiation of quantity with units"),
            });
        }

        Ok(Quantity::new(
            AutoNum::Float(self.value.cast().exp()),
            self.units.clone(),
        ))
    }

    pub fn ln(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from("cannot take the natural log of quantity with units"),
            });
        }

        Ok(Quantity::new(self.value.auto_ln()?, self.units.clone()))
    }

    pub fn log(&self) -> Result<Quantity, EvalError> {
        if !self.units.no_units() {
            return Err(EvalError {
                error: String::from("cannot take the log of quantity with units"),
            });
        }

        Ok(Quantity::new(self.value.auto_log()?, self.units.clone()))
    }
}
