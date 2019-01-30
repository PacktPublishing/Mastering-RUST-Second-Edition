// const_fns.rs

const fn salt(a: u32) -> u32 {
    0xDEADBEEF ^ a
}

const CHECKSUM: u32 = salt(23);

fn main() {
    println!("{}", CHECKSUM);
}
