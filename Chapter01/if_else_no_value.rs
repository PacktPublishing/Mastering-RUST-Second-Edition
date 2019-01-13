// if_else_no_value.rs

fn main() { 
  let result = if 1 == 2 { 
    "Nothing makes sense"; 
  } else { 
    "Sanity reigns"; 
  };

  println!("Result of computation: {:?}", result);
}
