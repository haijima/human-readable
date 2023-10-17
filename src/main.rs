use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Use <DELIMITER> as the field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    /// Specify which fields to convert to human readable format
    #[arg(short, long, default_value = "1")]
    fields: u8,
}

fn main() {
    let cli = Cli::parse();
    read(BufReader::new(stdin().lock()), &cli.delimiter, cli.fields);
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
// const UNITS: [&str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];
const UNITS: [&str; 8] = ["B", "K", "M", "G", "T", "P", "E", "Z"];

pub fn human_readable<T: Into<u32>>(bytes: T) -> String {
    let mut size = bytes.into() as f64;
    for unit in UNITS {
        if size < BASE {
            let s = format!("{:.1}", size).trim_end_matches(".0").to_string();
            return format!("{}{}", s, unit);
        }
        size /= BASE;
    }
    format!("{:.1}{}", size, "Y")
}
