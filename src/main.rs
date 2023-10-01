use std::io::{BufRead, BufReader, stdin};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    Cli::parse();
    read(BufReader::new(stdin().lock()));
}

fn read<R: BufRead>(buf_reader: R) {
    for line in buf_reader.lines() {
        println!("{}", line
            .unwrap()
            .split('\t')
            .enumerate()
            .map(|(i, c)|
                match (i, c.parse::<u32>()) {
                    (0, Ok(n)) => human_readable(n),
                    (0, Err(_)) => c.to_string(),
                    (_, _) => c.to_string(),
                }
            )
            .collect::<Vec<_>>()
            .join("\t"));
    }
}

const BASE: f64 = 1024f64;
const UNITS: [&str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];

pub fn human_readable<T: Into<u32>>(bytes: T) -> String {
    let mut size = bytes.into() as f64;
    for unit in UNITS {
        if size < BASE {
            let s = format!("{:.1}", size)
                .trim_end_matches(".0")
                .to_string();
            return format!("{} {}", s, unit);
        }
        size /= BASE;
    }
    format!("{:.1} {}", size, "YiB")
}
