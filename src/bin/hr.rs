use std::fs::File;
use std::io::{stdin, BufReader, Write};

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use log::Level;
use shadow_rs::shadow;

use crate::build::CLAP_LONG_VERSION;
use hrdbl::Unit;

shadow!(build);

#[derive(Debug, Parser)]
#[command(name = "hr", author, version, about, long_about = None, long_version = CLAP_LONG_VERSION)]
struct Cli {
    /// File to read, if empty read from stdin
    #[arg(value_name = "FILE")]
    filename: Option<String>,

    /// Use <DELIMITER> as the field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    /// Specify which fields to convert to human-readable format
    #[arg(short, long, default_value = "1", value_delimiter = ',')]
    fields: Vec<usize>,

    /// Specify which unit to use
    #[arg(short, long, value_enum)]
    unit: Option<Unit>,

    /// Decimal precision of the output
    #[arg(short, long, default_value = "1")]
    precision: usize,

    #[clap(flatten)]
    verbose: Verbosity,
}

fn main() {
    let cli = Cli::parse();

    init_logger(cli.verbose.log_level_filter());
    log::debug!("{:?}", cli);

    let buf_reader: BufReader<Box<dyn std::io::Read>> = match cli.filename {
        Some(f) => match File::open(&f) {
            Ok(r) => {
                log::info!("Open file: \"{}\"", &f);
                BufReader::new(Box::new(r)) // Open file and read from it
            }
            Err(err) => return log::error!("Failed to open file: \"{}\": {}", &f, err), // Print error and exit if file cannot be opened
        },
        None => {
            log::info!("Read from stdin...");
            BufReader::new(Box::new(stdin().lock())) // Use stdin if no file is specified
        }
    };

    hrdbl::read(
        buf_reader,
        hrdbl::Config::new(cli.delimiter, cli.fields, cli.unit, cli.precision),
    );
}

fn init_logger(level_filter: log::LevelFilter) {
    env_logger::Builder::new()
        .filter_level(level_filter)
        .format(|buf, record| match record.level() {
            Level::Error | Level::Warn | Level::Info => {
                writeln!(
                    buf,
                    "{:>5}: {}",
                    buf.default_level_style(record.level())
                        .value(record.level().to_string().to_lowercase()),
                    record.args(),
                )
            }
            Level::Debug | Level::Trace => {
                writeln!(
                    buf,
                    "{:>5}: {} [{}:{}]",
                    buf.default_level_style(record.level())
                        .value(record.level().to_string().to_lowercase()),
                    record.args(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                )
            }
        })
        .init();
}
