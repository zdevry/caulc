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
    result.insert("tau", uconst(std::f64::consts::TAU));
    result.insert("e", uconst(std::f64::consts::E));
    result.insert("golden", uconst(1.61803398875));
    result.insert(
        "c",
        qconst(299792458.0, Dimension::new(0, 1, -1, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "h",
        qconst(6.62607e-34, Dimension::new(1, 2, -1, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "mu_no",
        qconst(1.256637e-6, Dimension::new(1, 1, -2, -2, 0, 0, 0, 1)),
    );
    result.insert(
        "eps_no",
        qconst(8.854188e-12, Dimension::new(-1, -3, 4, 2, 0, 0, 0, 1)),
    );
    result.insert(
        "kB",
        qconst(1.38065e-23, Dimension::new(1, 2, -2, 0, -1, 0, 0, 1)),
    );
    result.insert(
        "g",
        qconst(9.80665, Dimension::new(0, 1, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "G",
        qconst(6.6743e-11, Dimension::new(-1, 3, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "eC",
        qconst(1.602e-19, Dimension::new(-1, 3, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "Mp",
        qconst(1.672622e-27, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "Mn",
        qconst(1.674928e-27, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "Me",
        qconst(9.109384e-31, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "Ryd",
        qconst(10973731.6, Dimension::new(0, -1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "RydH",
        qconst(10967758.3, Dimension::new(0, -1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "avogadro",
        qconst(6.02214e23, Dimension::new(0, 0, 0, 0, 0, -1, 0, 1)),
    );
    result.insert(
        "R",
        qconst(8.314463, Dimension::new(1, 2, -2, 0, -1, -1, 0, 1)),
    );
    result.insert(
        "faraday",
        qconst(96485.332, Dimension::new(0, 0, 1, 1, 0, -1, 0, 1)),
    );

    result
}

fn get_default_units<'a>() -> HashMap<&'a str, Quantity> {
    let mut result = HashMap::new();

    // SI base units
    result.insert("g", qconst(0.001, Dimension::new(1, 0, 0, 0, 0, 0, 0, 1)));
    result.insert("m", qconst(1.0, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)));
    result.insert("s", qconst(1.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)));
    result.insert("A", qconst(1.0, Dimension::new(0, 0, 0, 1, 0, 0, 0, 1)));
    result.insert("K", qconst(1.0, Dimension::new(0, 0, 0, 0, 1, 0, 0, 1)));
    result.insert("mol", qconst(1.0, Dimension::new(0, 0, 0, 0, 0, 1, 0, 1)));
    result.insert("cd", qconst(1.0, Dimension::new(0, 0, 0, 0, 0, 0, 1, 1)));

    // SI derived units
    result.insert("Hz", qconst(1.0, Dimension::new(0, 0, -1, 0, 0, 0, 0, 1)));

    result.insert("N", qconst(1.0, Dimension::new(1, 1, -2, 0, 0, 0, 0, 1)));
    result.insert("Pa", qconst(1.0, Dimension::new(1, -1, -2, 0, 0, 0, 0, 1)));
    result.insert("J", qconst(1.0, Dimension::new(1, 2, -2, 0, 0, 0, 0, 1)));
    result.insert("W", qconst(1.0, Dimension::new(1, 2, -3, 0, 0, 0, 0, 1)));

    result.insert("C", qconst(1.0, Dimension::new(0, 0, 1, 1, 0, 0, 0, 1)));
    result.insert("V", qconst(1.0, Dimension::new(1, 2, -3, -1, 0, 0, 0, 1)));
    result.insert("F", qconst(1.0, Dimension::new(-1, -2, 4, 2, 0, 0, 0, 1)));
    result.insert("ohm", qconst(1.0, Dimension::new(1, 2, -3, -2, 0, 0, 0, 1)));
    result.insert("T", qconst(1.0, Dimension::new(1, 0, -2, -1, 0, 0, 0, 1)));
    result.insert("Wb", qconst(1.0, Dimension::new(1, 2, -2, -1, 0, 0, 0, 1)));
    result.insert("H", qconst(1.0, Dimension::new(1, 2, -2, -2, 0, 0, 0, 1)));

    // Non SI Units
    // mass
    result.insert(
        "ton",
        qconst(1000.0, Dimension::new(1, 0, 0, 0, 0, 0, 0, 1)),
    );
    // length
    result.insert(
        "ansgtrom",
        qconst(1e-10, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert("ft", qconst(0.3048, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)));
    result.insert("NM", qconst(1852.0, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)));
    result.insert(
        "AU",
        qconst(1.496e11, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "ly",
        qconst(9.46e15, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "pc",
        qconst(3.09e16, Dimension::new(0, 1, 0, 0, 0, 0, 0, 1)),
    );
    // volume
    result.insert("L", qconst(0.001, Dimension::new(0, 3, 0, 0, 0, 0, 0, 1)));
    // time
    result.insert("min", qconst(60.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)));
    result.insert("h", qconst(3600.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)));
    result.insert("d", qconst(86400.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)));
    result.insert(
        "yr",
        qconst(365.2425 * 86400.0, Dimension::new(0, 0, 1, 0, 0, 0, 0, 1)),
    );
    // force
    result.insert(
        "gf",
        qconst(0.009807, Dimension::new(1, 1, -2, 0, 0, 0, 0, 1)),
    );
    // energy
    result.insert(
        "eV",
        qconst(1.602e-19, Dimension::new(1, 2, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "cal",
        qconst(4.184, Dimension::new(1, 2, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "Cal",
        qconst(4184.0, Dimension::new(1, 2, -2, 0, 0, 0, 0, 1)),
    );
    // pressure
    result.insert("bar", qconst(1e5, Dimension::new(1, -1, -2, 0, 0, 0, 0, 1)));
    result.insert(
        "atm",
        qconst(101325.0, Dimension::new(1, -1, -2, 0, 0, 0, 0, 1)),
    );
    result.insert(
        "mHg",
        qconst(133322.0, Dimension::new(1, -1, -2, 0, 0, 0, 0, 1)),
    );
    // Other units
    result.insert("deg", uconst(std::f64::consts::PI / 180.0));
    result.insert("am", uconst(std::f64::consts::PI / 180.0 / 60.0));
    result.insert("as", uconst(std::f64::consts::PI / 180.0 / 3600.0));
    result.insert("rad", uconst(1.0));

    result
}
