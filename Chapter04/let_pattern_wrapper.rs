// let_pattern_wrapper.rs

fn main() {
    let mut return_val = Some(0u64);
    let success = if let Some(0) = return_val {
        true
    } else {
        false
    };
}
