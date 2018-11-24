// closures.rs

fn main() {
    // single line closures don't need braces
    let doubler = |x| x * 2;
    let value = 5;
    let two_times = doubler(value);
    println!("{} doubled is {}", value, two_times);

    // need braces for multiline closures
    let big_closure = |b, c| {
        let z = b + c;
        z * two_times    // references two_time
    };

    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);
}
