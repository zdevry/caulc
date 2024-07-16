use std::collections::HashMap;

use crate::{
    autonum::AutoNum,
    units::{Dimension, Quantity},
};

pub struct Definitions<'a> {
    pub constants: HashMap<&'a str, Quantity>,
}

impl<'a> Definitions<'a> {
    pub fn get_default() -> Definitions<'a> {
        Definitions {
            constants: get_default_constants(),
        }
    }

    pub fn get_unit(&self, unit: &str) -> Option<Quantity> {
        if unit == "m" {
            Some(qconst(1.0, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)))
        } else {
            None
        }
    }
}

fn qconst(value: f64, units: Dimension) -> Quantity {
    Quantity::new(AutoNum::Float(value), units)
}

fn uconst(value: f64) -> Quantity {
    Quantity::dimensionless(AutoNum::Float(value))
}

fn get_default_constants<'a>() -> HashMap<&'a str, Quantity> {
    let mut result = HashMap::new();

    result.insert("pi", uconst(std::f64::consts::PI));
    result.insert("e", uconst(std::f64::consts::E));
    result.insert(
        "g",
        qconst(9.80665, Dimension::new(0, 1, -2, 0, 0, 0, 0, 1)),
    );

    result
}
