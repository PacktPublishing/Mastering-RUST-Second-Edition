// slog_demo/player.rs

use slog::Logger;

use weapon::PlasmaCannon;
use PlayingCharacter;

pub struct Player {
    name: String,
    logger: Logger,
    weapon: PlasmaCannon
}

impl Player {
    pub fn new(logger: &Logger, name: &str) -> Self {
        let player_log = logger.new(o!("Player" => format!("{}", name)));
        let weapon_log = player_log.new(o!("PlasmaCannon" => "M435"));
        Self {
            name: name.to_string(),
            logger: player_log,
            weapon: PlasmaCannon(weapon_log),
        }
    }
}

impl PlayingCharacter for Player {
    fn shoot(&self) {
        info!(self.logger, "Player shooting with {}", self.weapon);
        self.weapon.fire();
    }
}