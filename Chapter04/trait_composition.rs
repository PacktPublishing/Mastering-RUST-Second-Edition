// traits_composition.rs

trait Eat {
    fn eat(&self) {
        println!("eat");
    }
}
trait Code {
    fn code(&self) {
        println!("code");
    }
}
trait Sleep {
    fn sleep(&self) {
        println!("sleep");
    }
}

trait Programmer : Eat + Code + Sleep {
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
        println!("repeat!");
    }
}

struct Bob;
impl Programmer for Bob {}
impl Eat for Bob {}
impl Code for Bob {}
impl Sleep for Bob {}

fn main() {
    Bob.animate();
}
