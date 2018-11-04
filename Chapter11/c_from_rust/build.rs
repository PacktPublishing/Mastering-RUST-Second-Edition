// c_from_rust/build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("mystrlen.c")
        .static_flag(true)
        .compile("mystrlen");
}
