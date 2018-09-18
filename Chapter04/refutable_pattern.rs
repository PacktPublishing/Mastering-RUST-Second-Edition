// refutable_pattern.rs

fn main() {
    let mut a: Option<u32> = Some(56);
    let Some(b) = a;
}
