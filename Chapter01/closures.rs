// closures.rs

fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };

    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);
}
