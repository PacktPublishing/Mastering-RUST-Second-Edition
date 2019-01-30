// loop_expr.rs

fn main() {
    let mut i = 0;
    let counter = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    
    println!("{}", counter);
}
