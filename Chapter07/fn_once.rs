// fn_once.rs

fn main() {
    let a = Box::new(23);
    let call_me = || {
        let _c = a;
    };

    call_me();
    call_me();
}
