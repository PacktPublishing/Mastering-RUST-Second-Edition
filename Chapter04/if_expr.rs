// if_expr.rs

fn compute(i: i32) -> i32 {
    2 * i
}

fn main() {
    let result_msg = "done";
    
    // if expression assignments
    let result = if result_msg == "done" {
        let some_work = compute(8);
        let stuff = compute(4);
        compute(2) + stuff // last expression gets assigned to result
    } else {
        compute(1)
    };

    println!("{}", result);
}
