// block_expr.rs

fn main() {
    // using bare blocks to do multiple things at once
    let precompute = {
        let a = (-34i64).abs();
        let b = 345i64.pow(3);
        let c = 3;
        a + b + c
    };

    // match expressions
    let result_msg = match precompute {
        42 => "done",
        a if a % 2 == 0 => "continue",
        _ => panic!("Oh no !")
    };

    println!("{}", result_msg);
}
