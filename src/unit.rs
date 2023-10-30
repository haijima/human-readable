use std::fmt::{Display, Formatter, Result};

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
#[repr(u8)]
pub enum Unit {
    Byte = 0,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let raw = format!("{:?}", self);
        write!(f, "{}", raw.to_uppercase())
    }
}
