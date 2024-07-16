use std::collections::HashMap;

use crate::{
    autonum::AutoNum,
    units::{Dimension, Quantity},
};

pub struct Definition<'a> {
    pub constants: HashMap<&'a str, Quantity>,
}

pub fn qconst(value: f64, units: Dimension) -> Option<Quantity> {
    Some(Quantity::new(AutoNum::Float(value), units))
}

pub fn uconst(value: f64) -> Option<Quantity> {
    Some(Quantity::dimensionless(AutoNum::Float(value)))
}

pub fn try_get_constant(w: &str) -> Option<Quantity> {
    match w {
        "pi" => uconst(std::f64::consts::PI),
        "e" => uconst(std::f64::consts::E),
        "g" => qconst(9.80665, Dimension::new(0, 1, -2, 0, 0, 0, 0, 1)),
        _ => None,
    }
}
