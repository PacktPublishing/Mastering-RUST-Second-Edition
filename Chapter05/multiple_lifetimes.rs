// multiple_lifetimes.rs

struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R
}

fn main() {}
