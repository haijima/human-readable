use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE", help = "File to read, if empty read from stdin")]
    filename: Option<String>,

    /// Use <DELIMITER> as the field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    /// Specify which fields to convert to human-readable format
    #[arg(short, long, default_value = "1")]
    fields: usize,

    /// Specify which unit to use
    #[arg(short, long, value_enum, default_value_t = Unit::Auto)]
    unit: Unit,

    /// Decimal precision of the output
    #[arg(short, long, default_value = "1")]
    precision: usize,
}

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

fn main() {
    let cli = Cli::parse();

    let buf_reader: BufReader<Box<dyn std::io::Read>> = match cli.filename {
        None => BufReader::new(Box::new(stdin().lock())), // Use stdin if no file is specified
        Some(f) => match File::open(f) {
            Ok(r) => BufReader::new(Box::new(r)), // Open file and read from it
            Err(err) => return eprintln!("{}", err), // Print error and exit if file cannot be opened
        },
    };

    read(
        buf_reader,
        &cli.delimiter,
        cli.fields,
        &cli.unit,
        cli.precision,
    );
}

fn read<R: BufRead>(buf_reader: R, delimiter: &str, fields: usize, unit: &Unit, precision: usize) {
    for line in buf_reader.lines() {
        println!(
            "{}",
            line.unwrap()
                .split(delimiter)
                .enumerate()
                .map(|(i, c)| {
                    if i + 1 == fields {
                        match c.parse::<u64>() {
                            Ok(n) => human_readable(n, unit, precision),
                            Err(_) => c.to_string(),
                        }
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(delimiter)
        );
    }
}

const UNITS: [&str; 7] = ["B", "K", "M", "G", "T", "P", "E"];

pub fn human_readable<T: Into<u64>>(bytes: T, unit: &Unit, precision: usize) -> String {
    let size = bytes.into() as f64;
    let i = match unit {
        Unit::Byte => 0,
        Unit::Kilo => 1,
        Unit::Mega => 2,
        Unit::Giga => 3,
        Unit::Tera => 4,
        Unit::Peta => 5,
        Unit::Exa => 6,
        Unit::Auto => size.log(1024_f64).floor() as usize,
    };

    if i == 0 {
        return format!("{}{}", size, "B");
    }
    format!(
        "{:.prec$}{}",
        size / (1u64 << (10 * i)) as f64,
        UNITS[i],
        prec = precision
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable() {
        assert_eq!(human_readable(0u8, &Unit::Auto, 1), "0B");
        assert_eq!(human_readable(1u8, &Unit::Auto, 1), "1B");
        assert_eq!(human_readable(1023u32, &Unit::Auto, 1), "1023B");
        assert_eq!(human_readable(1024u32, &Unit::Auto, 1), "1.0K");
        assert_eq!(human_readable(1025u32, &Unit::Auto, 1), "1.0K");
        assert_eq!(human_readable(1048576u32, &Unit::Auto, 1), "1.0M");
        assert_eq!(human_readable(1073741824u64, &Unit::Auto, 1), "1.0G");
        assert_eq!(human_readable(1200000000u64, &Unit::Auto, 1), "1.1G");
        assert_eq!(human_readable(1099511627776u64, &Unit::Auto, 1), "1.0T");
        assert_eq!(human_readable(1125899906842624u64, &Unit::Auto, 1), "1.0P");
        assert_eq!(
            human_readable(1152921504606846975u64, &Unit::Auto, 1),
            "1.0E"
        );
        assert_eq!(
            human_readable(1152921504606846976u64, &Unit::Auto, 1),
            "1.0E"
        );
        assert_eq!(human_readable(u64::MAX, &Unit::Auto, 1), "16.0E");
    }
}
