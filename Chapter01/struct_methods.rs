// struct_methods.rs

struct Player {
  name: String,
  iq: u8,
  friends: u8,
  score: u8,
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
            score: 100
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }
}

fn main() {
    let player = Player::with_name("Dave");
    println!("{}'s friends count: {}", player.name, player.get_friends());
}