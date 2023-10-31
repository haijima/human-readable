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
