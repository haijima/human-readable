use std::fmt::{Display, Formatter};

use clap::ValueEnum;

#[derive(Debug, Default, Clone, PartialEq, ValueEnum)]
pub enum Unit {
    #[default]
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let raw = format!("{:?}", self);
        write!(f, "{}", raw.chars().nth(0).unwrap().to_uppercase())
    }
}

impl Unit {
    pub fn apply(&self, byte_size: f64) -> f64 {
        let base = 1024_u64.pow(match &self {
            Unit::Byte => 0,
            Unit::Kilo => 1,
            Unit::Mega => 2,
            Unit::Giga => 3,
            Unit::Tera => 4,
            Unit::Peta => 5,
            Unit::Exa => 6,
        });
        byte_size / base as f64
    }

    pub fn auto(byte_size: f64) -> Self {
        if byte_size < 0f64 {
            log::error!("byte_size shoud be positive. but: {}", byte_size);
            return Unit::default();
        }
        match byte_size.log(1024_f64).floor() as u8 {
            0 => Unit::Byte,
            1 => Unit::Kilo,
            2 => Unit::Mega,
            3 => Unit::Giga,
            4 => Unit::Tera,
            5 => Unit::Peta,
            6 => Unit::Exa,
            _ => {
                log::warn!("unexpected byte size: {}", byte_size);
                Unit::default() // Unit::Byte
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_apply() {
        assert_eq!(Unit::Byte.apply(1073741824.0), 1073741824.0);
        assert_eq!(Unit::Kilo.apply(1073741824.0), 1048576.0);
        assert_eq!(Unit::Mega.apply(1073741824.0), 1024.0);
        assert_eq!(Unit::Giga.apply(1073741824.0), 1.0);
        assert_eq!(Unit::Tera.apply(1073741824.0), 0.0009765625);
        assert_eq!(Unit::Peta.apply(1073741824.0), 0.00000095367431640625);
        assert_eq!(Unit::Exa.apply(1073741824.0), 0.0000000009313225746154785);
    }

    #[test]
    fn test_unit_auto() {
        assert_eq!(Unit::auto(0_f64), Unit::Byte);
        assert_eq!(Unit::auto(1023_f64), Unit::Byte);
        assert_eq!(Unit::auto(1024_f64), Unit::Kilo);
        assert_eq!(Unit::auto((1u64 << 20) as f64), Unit::Mega);
        assert_eq!(Unit::auto((1u64 << 30) as f64), Unit::Giga);
        assert_eq!(Unit::auto((1u64 << 40) as f64), Unit::Tera);
        assert_eq!(Unit::auto((1u64 << 50) as f64), Unit::Peta);
        assert_eq!(Unit::auto((1u64 << 60) as f64), Unit::Exa);
        assert_eq!(Unit::auto(f64::MAX), Unit::default());
    }
}
