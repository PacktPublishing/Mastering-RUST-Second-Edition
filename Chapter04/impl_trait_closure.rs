// impl_trait_closure.rs

fn lazy_adder(a:u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

fn main() {
    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());
}
