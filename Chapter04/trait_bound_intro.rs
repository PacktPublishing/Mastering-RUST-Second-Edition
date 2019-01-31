// trait_bounds_intro.rs

struct Game;
struct Enemy;
struct Hero;

impl Game {
    fn load<T>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
