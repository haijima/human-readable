use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// byte size
    number: u32,
}

fn main() {
    let cli = Cli::parse();
    println!("{}", human_readable(cli.number));
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
