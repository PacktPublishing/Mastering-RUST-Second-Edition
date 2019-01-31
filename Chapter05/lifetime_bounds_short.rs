// lifetime_bounds_short.rs

enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T) where T: Send + 'static {
    // configure the logger here
}

fn main() {
    let other = String::from("Local");
    let log2 = Logger(&other, Level::Error);
    configure_logger(&log2);
}
