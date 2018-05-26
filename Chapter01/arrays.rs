
fn main() { 
  let numbers: [u8; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11]; 
  let floats = [0.1, 0.2, 0.3]; 

  println!("First number is {}", numbers[0]); 

  for number in &numbers { 
    println!("Number is {}", number); 
  } 

  for float_number in &floats { 
    println!("Float is {}", float_number); 
  } 
}
