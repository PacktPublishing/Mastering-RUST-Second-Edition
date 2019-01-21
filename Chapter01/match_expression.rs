// match_expression.rs

fn req_status() -> u32 {
    200
}

fn main() {
    let status = req_status();
    match status {
        200 => println!("Success"),
        404 => println!("Not Found"),
        other => {
            println!("Request failed with code: {}", other);
            // get response from cache
        }
    }
}
