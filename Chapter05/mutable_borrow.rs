// mutable_borrow.rs

fn main() {
    let a = String::from("Owned string");
    let a_ref = &mut a;
    a_ref.push('!');
}
