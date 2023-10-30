use std::io::BufRead;

pub use unit::Unit;

mod unit;

pub fn read<R: BufRead>(
    buf_reader: R,
    delimiter: &str,
    fields: usize,
    unit: Option<Unit>,
    precision: usize,
) {
    for line in buf_reader.lines() {
        println!(
            "{}",
            line.unwrap()
                .split(delimiter)
                .enumerate()
                .map(|(i, c)| {
                    if i + 1 == fields {
                        match c.parse::<u64>() {
                            Ok(n) => human_readable(n, unit.clone(), precision),
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

pub fn human_readable<T: Into<u64>>(bytes: T, unit: Option<Unit>, precision: usize) -> String {
    let size = bytes.into() as f64;
    let i = match unit {
        Some(u) => u as usize,
        None => size.log(1024_f64).floor() as usize,
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
        assert_eq!(human_readable(0u8, None, 1), "0B");
        assert_eq!(human_readable(1u8, None, 1), "1B");
        assert_eq!(human_readable(1023u32, None, 1), "1023B");
        assert_eq!(human_readable(1024u32, None, 1), "1.0K");
        assert_eq!(human_readable(1025u32, None, 1), "1.0K");
        assert_eq!(human_readable(1048576u32, None, 1), "1.0M");
        assert_eq!(human_readable(1073741824u64, None, 1), "1.0G");
        assert_eq!(human_readable(1200000000u64, None, 1), "1.1G");
        assert_eq!(human_readable(1099511627776u64, None, 1), "1.0T");
        assert_eq!(human_readable(1125899906842624u64, None, 1), "1.0P");
        assert_eq!(human_readable(1152921504606846975u64, None, 1), "1.0E");
        assert_eq!(human_readable(1152921504606846976u64, None, 1), "1.0E");
        assert_eq!(human_readable(u64::MAX, None, 1), "16.0E");
    }
}
