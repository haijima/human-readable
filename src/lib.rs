use std::io::BufRead;

use crate::config::Format;
pub use config::Config;
pub use unit::Unit;

mod config;
mod unit;

pub fn read<R: BufRead>(buf_reader: R, config: Config) {
    log::debug!("{:?}", &config);
    let f = |(i, c): (usize, &str)| match (config.fields.contains(&(i + 1)), c.parse::<u64>()) {
        (true, Ok(n)) => human_readable(n, &config.format),
        (true, Err(e)) => {
            log::warn!("Failed to parse \"{}\": {}", c, e);
            c.to_string()
        }
        (_, _) => c.to_string(),
    };

    for line in buf_reader.lines() {
        if let Err(e) = line {
            log::error!("{}", e);
            continue;
        }
        println!(
            "{}",
            line.unwrap()
                .split(&config.delimiter)
                .enumerate()
                .map(f)
                .collect::<Vec<_>>()
                .join(&config.delimiter)
        );
    }
}

pub fn human_readable<T: Into<u64>>(bytes: T, format: &Format) -> String {
    let size = bytes.into() as f64;
    let u = format.unit.clone().unwrap_or_else(|| Unit::auto(size));
    log::trace!(
        "bytes: {}, unit: {:?}->{:?}, precision: {}",
        size,
        format.unit,
        u,
        format.precision,
    );
    if u == Unit::Byte {
        return format!("{}{}", size, u);
    }
    if format.precision <= 20 {
        return format!("{:.n$}{}", u.apply(size), u, n = format.precision);
    }
    format!("{:.20}{}", u.apply(size), u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable() {
        assert_eq!(human_readable(0u8, &Format::default()), "0B");
        assert_eq!(human_readable(1u8, &Format::default()), "1B");
        assert_eq!(human_readable(1023u32, &Format::default()), "1023B");
        assert_eq!(human_readable(1024u32, &Format::default()), "1.0K");
        assert_eq!(human_readable(1025u32, &Format::default()), "1.0K");
        assert_eq!(human_readable(1048576u32, &Format::default()), "1.0M");
        assert_eq!(human_readable(1073741824u64, &Format::default()), "1.0G");
        assert_eq!(human_readable(1200000000u64, &Format::default()), "1.1G");
        assert_eq!(human_readable(1099511627776u64, &Format::default()), "1.0T");
        assert_eq!(
            human_readable(1125899906842624u64, &Format::default()),
            "1.0P"
        );
        assert_eq!(
            human_readable(1152921504606846975u64, &Format::default()),
            "1.0E"
        );
        assert_eq!(
            human_readable(1152921504606846976u64, &Format::default()),
            "1.0E"
        );
        assert_eq!(human_readable(u64::MAX, &Default::default()), "16.0E");
    }
}
