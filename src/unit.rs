use std::fmt::{Display, Formatter, Result};

use clap::ValueEnum;

#[derive(Debug, Clone, Default, PartialEq, ValueEnum)]
pub enum Unit {
    #[default]
    Auto,
    Byte,
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
