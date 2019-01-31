// unsafe_trait_and_impl.rs

struct MyType;

unsafe trait UnsafeTrait { 
   unsafe fn unsafe_function();
}

trait SafeTrait {
    unsafe fn unsafe_method(&self);
}

unsafe impl UnsafeTrait for MyType { 
     unsafe fn unsafe_function() { } 
}

fn main() {}
