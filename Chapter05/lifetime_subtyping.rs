// lifetime_subtyping.rs

struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R
}

impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
where 'a: 'b {
    
}

fn main() {
    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![];
    let decoder = Decoder {schema: &a, reader: &b};
}
