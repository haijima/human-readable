use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE", help = "File to read, if empty read from stdin")]
    filename: Option<String>,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Use <DELIMITER> as the field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    /// Specify which fields to convert to human-readable format
    #[arg(short, long, default_value = "1")]
    fields: usize,
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
    read(buf_reader, &cli.delimiter, cli.fields);
}

fn read<R: BufRead>(buf_reader: R, delimiter: &str, fields: usize) {
    for line in buf_reader.lines() {
        println!(
            "{}",
            line.unwrap()
                .split(delimiter)
                .enumerate()
                .map(|(i, c)| {
                    if i + 1 == fields {
                        match c.parse::<u64>() {
                            Ok(n) => human_readable(n),
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

const BASE: f64 = 1024_f64;
const UNITS: [&str; 5] = ["K", "M", "G", "T", "P"];

pub fn human_readable<T: Into<u64>>(bytes: T) -> String {
    let mut size = bytes.into() as f64;
    if size < BASE {
        return format!("{}{}", size, "B");
    }
    for unit in UNITS {
        size /= BASE;
        if size < BASE {
            return format!("{:.1}{}", size, unit);
        }
    }
    format!("{:.1}{}", size / BASE, "E")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable() {
        assert_eq!(human_readable(0u8), "0B");
        assert_eq!(human_readable(1u8), "1B");
        assert_eq!(human_readable(1023u32), "1023B");
        assert_eq!(human_readable(1024u32), "1.0K");
        assert_eq!(human_readable(1025u32), "1.0K");
        assert_eq!(human_readable(1048576u32), "1.0M");
        assert_eq!(human_readable(1073741824u64), "1.0G");
        assert_eq!(human_readable(1200000000u64), "1.1G");
        assert_eq!(human_readable(1099511627776u64), "1.0T");
        assert_eq!(human_readable(1125899906842624u64), "1.0P");
        assert_eq!(human_readable(1152921504606846976u64), "1.0E");
        assert_eq!(human_readable(u64::MAX), "16.0E");
    }
}
