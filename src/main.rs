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
    fields: u8,
}

fn main() {
    let cli = Cli::parse();

    let buf_reader: BufReader<Box<dyn std::io::Read>> = match cli.filename {
        None => BufReader::new(Box::new(stdin().lock())),
        Some(f) => match File::open(f) {
            Ok(r) => BufReader::new(Box::new(r)),
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        },
    };
    read(buf_reader, &cli.delimiter, cli.fields);
}

fn read<R: BufRead>(buf_reader: R, delimiter: &str, fields: u8) {
    for line in buf_reader.lines() {
        println!(
            "{}",
            line.unwrap()
                .split(delimiter)
                .enumerate()
                .map(|(i, c)| match c.parse::<u32>() {
                    Ok(n) =>
                        if i as u8 + 1 == fields {
                            human_readable(n)
                        } else {
                            c.to_string()
                        },
                    Err(_) => c.to_string(),
                })
                .collect::<Vec<_>>()
                .join(delimiter)
        );
    }
}

const BASE: f64 = 1024f64;
// const UNITS: [&str; 7] = ["KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];
const UNITS: [&str; 7] = ["K", "M", "G", "T", "P", "E", "Z"];

pub fn human_readable<T: Into<u128>>(bytes: T) -> String {
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
    format!("{:.1}{}", size / BASE, "Y")
}
