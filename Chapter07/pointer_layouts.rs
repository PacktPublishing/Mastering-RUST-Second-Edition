// pointer_layouts.rs

trait Position {}

struct Coordinates(f64, f64);

impl Position for Coordinates {}

fn main() {
    let val = Coordinates(1.0, 2.0);
    let ref_: &Coordinates = &val;
    let pos_ref: &Position = &val as &Position;
    let ptr:       *const Coordinates = &val as *const Coordinates;
    let pos_ptr: *const Position  = &val as *const Position;
    
    println!("ref_: {}", std::mem::size_of_val(&ref_));
    println!("ptr: {}", std::mem::size_of_val(&ptr));
    println!("val: {}", std::mem::size_of_val(&val));
    println!("pos_ref: {}", std::mem::size_of_val(&pos_ref));
    println!("pos_ptr: {}", std::mem::size_of_val(&pos_ptr));
}
