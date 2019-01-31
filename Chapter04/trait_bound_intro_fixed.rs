// trait_bounds_intro.rs

struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
