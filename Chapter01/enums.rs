
#[derive(Debug)]
enum Direction {
  N,
  NE,
  E,
  SE,
  S,
  SW,
  W,
  NW,
}

enum PlayerAction {
  Move {
    direction: Direction,
    speed: u8,
  },
  Wait,
  Attack(Direction),
}

fn main() {
  let simulated_player_action = PlayerAction::Move {
    direction: Direction::NE,
    speed: 2,
  };

  match simulated_player_action {
    PlayerAction::Wait => println!("Player wants to wait"),
    PlayerAction::Move { direction, speed } => {
      println!("Player wants to move in direction {:?} with speed {}",
                direction, speed)
    }
    PlayerAction::Attack(direction) => {
      println!("Player wants to attack direction {:?}", direction)
    }
  };
}
