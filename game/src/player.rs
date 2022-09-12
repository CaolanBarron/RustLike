extern crate fundamentals;

use fundamentals::position as pos;
use std::fmt;

struct Player {
    name: String,
    health: usize,
    position: pos::Position,
}

impl Player {
    pub fn new(name: String, health: usize, position: (usize, usize)) -> Player {
        Player {
            name,
            health,
            position: pos::Position::new(position.0, position.1),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Player name: {} \nHealth: {} \nPosition: {}",
            self.name, self.health, self.position
        )
    }
}

#[cfg(test)]
mod player_tests {
    use super::*;
    #[test]
    fn create_player() {
        let player = Player::new(String::from("testName"), 10, (5, 5));

        println!("{}", player);

        assert_eq!(player.name, "testName");
        assert_eq!(player.health, 10);
        assert_eq!(player.position.x, 5);
        assert_eq!(player.position.y, 5);
    }
}
