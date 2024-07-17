use std::collections::HashMap;

use crate::{
    autonum::AutoNum,
    units::{Dimension, Quantity},
};

pub struct Definitions<'a> {
    pub constants: HashMap<&'a str, Quantity>,
    units: HashMap<&'a str, Quantity>,
}

impl<'a> Definitions<'a> {
    pub fn get_default() -> Definitions<'a> {
        Definitions {
            constants: get_default_constants(),
            units: get_default_units(),
        }
    }

    pub fn get_unit(&self, unit: &str) -> Option<Quantity> {
        if let Some(u) = self.units.get(unit) {
            return Some(u.clone());
        }

        let prefix_factor = get_metric_prefix(unit.chars().nth(0)?)?;
        let base_unit = &unit[1..];
        let unit_quantity = self.units.get(base_unit)?;
        Some(Quantity::new(
            unit_quantity.value.auto_mul(&AutoNum::Float(prefix_factor)),
            unit_quantity.units.clone(),
        ))
    }
}

fn get_metric_prefix(c: char) -> Option<f64> {
    match c {
        'T' => Some(1e12),
        'G' => Some(1e9),
        'M' => Some(1e6),
        'k' => Some(1e3),
        'd' => Some(1e-1),
        'c' => Some(1e-2),
        'm' => Some(1e-3),
        'u' => Some(1e-6),
        'n' => Some(1e-9),
        _ => None,
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
    result.insert(
        "G",
        qconst(6.6743015e-11, Dimension::new(-1, 3, -2, 0, 0, 0, 0, 1)),
    );

    result
}

fn get_default_units<'a>() -> HashMap<&'a str, Quantity> {
    let mut result = HashMap::new();

    result.insert("g", qconst(0.001, Dimension::new(1, 0, 0, 0, 0, 0, 0, 1)));
    result.insert("m", qconst(1.0, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)));
    result.insert("s", qconst(1.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)));
    result.insert("A", qconst(1.0, Dimension::new(0, 0, 0, 1, 0, 0, 0, 1)));
    result.insert("K", qconst(1.0, Dimension::new(0, 0, 0, 0, 1, 0, 0, 1)));
    result.insert("mol", qconst(1.0, Dimension::new(0, 0, 0, 0, 0, 1, 0, 1)));
    result.insert("cd", qconst(1.0, Dimension::new(0, 0, 0, 0, 0, 0, 1, 1)));

    result.insert("deg", uconst(std::f64::consts::PI / 180.0));
    result.insert("rad", uconst(1.0));

    result.insert("L", qconst(0.001, Dimension::new(0, 3, 0, 0, 0, 0, 0, 1)));

    result.insert("N", qconst(1.0, Dimension::new(1, 1, -2, 0, 0, 0, 0, 1)));

    result
}
