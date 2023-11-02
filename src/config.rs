use super::unit;

#[derive(Debug, Clone)]
pub struct Config {
    /// delimiter to use
    pub delimiter: String,
    /// index of fields to convert
    pub fields: Vec<usize>,
    /// config for output format
    pub format: Format,
}

#[derive(Debug, Clone)]
pub struct Format {
    /// unit to use
    pub unit: Option<unit::Unit>,
    /// number of digits after the decimal point.
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
    /// Create a new `Config` with default values
    ///
    /// default values:
    /// * delimiter: "\t"
    /// * fields: vec![1]
    /// * format: Default::default()
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
    /// Create a new `Format` with default values
    ///
    /// default values:
    /// * unit: None
    /// * precision: 1
    fn default() -> Self {
        Self {
            unit: None,
            precision: 1,
        }
    }
}
