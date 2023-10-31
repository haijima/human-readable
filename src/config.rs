use super::unit;

pub struct Config {
    pub delimiter: String,
    pub fields: Vec<usize>,
    pub format: Format,
}

pub struct Format {
    pub unit: Option<unit::Unit>,
    pub precision: usize,
}

impl Config {
    pub fn new(
        delimiter: String,
        fields: Vec<usize>,
        unit: Option<unit::Unit>,
        precision: usize,
    ) -> Self {
        Self {
            delimiter,
            fields,
            format: Format::new(unit, precision),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delimiter: "\t".to_string(),
            fields: vec![1],
            format: Default::default(),
        }
    }
}

impl Format {
    pub fn new(unit: Option<unit::Unit>, precision: usize) -> Self {
        Self { unit, precision }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self {
            unit: None,
            precision: 1,
        }
    }
}
