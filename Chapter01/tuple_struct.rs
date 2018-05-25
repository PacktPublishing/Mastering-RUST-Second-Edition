// tuple_struct.rs 

struct Color(u8, u8, u8);

fn main() {
    let white = Color(255, 255, 255);
    
    // or you can pull them out by index
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}\n", blue);

    let orange = Color(255, 165, 0);

    // you can also destructure the fields directly, that's so cool
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    // can also ignore fields while destructuring
    let Color(r, _, b) = orange;
}