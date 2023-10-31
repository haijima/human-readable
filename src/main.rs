use std::fs::File;
use std::io::{stdin, BufReader, Write};

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use hr::Unit;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
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

    let buf_reader: BufReader<Box<dyn std::io::Read>> = match cli.filename {
        Some(f) => match File::open(f) {
            Ok(r) => BufReader::new(Box::new(r)), // Open file and read from it
            Err(err) => return eprintln!("{}", err), // Print error and exit if file cannot be opened
        },
        None => BufReader::new(Box::new(stdin().lock())), // Use stdin if no file is specified
    };

    hr::read(
        buf_reader,
        hr::Config::new(cli.delimiter, cli.fields, cli.unit, cli.precision),
    );
}

fn init_logger(level_filter: log::LevelFilter) {
    env_logger::Builder::new()
        .filter_level(level_filter)
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}",
                buf.default_styled_level(record.level()),
                record.args(),
            )
        })
        .init();
}
