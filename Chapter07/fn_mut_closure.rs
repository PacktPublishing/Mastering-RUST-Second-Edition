// fn_mut_closure.rs

fn main() {
    let mut a = String::from("FnMut closure");
    let mut mut_closure = || {
        a.push('!');
        println!("{:?}", a);    
    };
    mut_closure();
}
