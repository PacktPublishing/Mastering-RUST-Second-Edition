// panic_test.rs

// compile in test mode: `rustc --test panic_test.rs`
// run tests using: `./panic_test`

#[test]
#[should_panic]
fn this_panics() {
    panic!("Succeeded in failing!");
}
