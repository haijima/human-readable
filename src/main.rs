use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// separator
    #[arg(short, long, default_value = "\t")]
    separator: String,

    /// target column
    #[arg(short, long, default_value = "1")]
    target: u8,
}

fn main() {
    let cli = Cli::parse();
    read(BufReader::new(stdin().lock()), &cli.separator, cli.target);
}

fn read<R: BufRead>(buf_reader: R, separator: &str, t: u8) {
    for line in buf_reader.lines() {
        println!(
            "{}",
            line.unwrap()
                .split(separator)
                .enumerate()
                .map(|(i, c)| match c.parse::<u32>() {
                    Ok(n) =>
                        if i as u8 + 1 == t {
                            human_readable(n)
                        } else {
                            c.to_string()
                        },
                    Err(_) => c.to_string(),
                })
                .collect::<Vec<_>>()
                .join(separator)
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
