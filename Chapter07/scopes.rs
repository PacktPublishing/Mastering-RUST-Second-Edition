// scopes.rs

fn main() {
    let mut b = 4;
    {
        let mut a = 34 + b;
        a += 1;
    }

    b = a;   
}
