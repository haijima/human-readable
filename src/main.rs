fn main() {
    println!("{}", human_readable(0));
    println!("{}", human_readable(550));
    println!("{}", human_readable(563_200));
    println!("{}", human_readable(681_574_400));
    println!("{}", human_readable(123_456_789));
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
