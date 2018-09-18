// expressions.rs

fn compute() -> i32 {
    234
}

fn main() {
    // using bare blocks to do multiple things at once
    let precompute = {
        let a = (-34i64).abs();
        let (b, _) = 345i64.overflowing_rem(34);
        let c = 3;
        a + b + c
    };

    // match expressions
    let result_msg = match precompute {
        42 => "done",
        a if a % 2 == 0 => "continue",
        _ => panic!("Oh no !")
    };

    // if expressions
    let result = if result_msg == "done" {
        let some_work = compute();
        let stuff = compute();
        compute() + stuff // last expression gets assigned to result
    } else {
        compute()
    };

    println!("{}", result);
}
