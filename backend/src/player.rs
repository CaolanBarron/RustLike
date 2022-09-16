extern crate fundamentals;

use crate::entity::{Entity, Movement, Avatar};
use fundamentals::position as pos;

// Name: input from the user before the character is created
// Health: A value that starts as a default but changes often
// Position: A custom coordinate type that will change constantly
#[derive(Debug)]
pub struct Player {
    name: String,
    health: usize,
    position: pos::Position,
    avatar: Avatar,
}

impl Player {
    pub fn new(name: String, health: usize, position: (usize, usize)) -> Player {
        Player {
            name,
            health,
            position: pos::Position::new(position.0, position.1),
            avatar: Avatar::P,
        }
    }
}

impl Entity for Player{
    //Getters
    fn avatar(&self) ->  String {
        self.avatar.to_string()
    }
    fn position(&self) -> &pos::Position {
        &self.position
    }
}

impl Movement for Player {
    fn move_up(&mut self) {
        self.position = pos::Position::new(self.position.x, self.position.y - 1);
    }

    fn move_down(&mut self) {
        self.position = pos::Position::new(self.position.x, self.position.y + 1);
    }

    fn move_left(&mut self) {
        self.position = pos::Position::new(self.position.x - 1, self.position.y);
    }

    fn move_right(&mut self) {
        self.position = pos::Position::new(self.position.x + 1, self.position.y);
    }
}


#[cfg(test)]
mod player_tests {
    use super::*;
    #[test]
    fn create_player() {
        let player = Player::new(String::from("testName"), 10, (5, 5));

        println!("{:?}", player);

        assert_eq!(player.name, "testName");
        assert_eq!(player.health, 10);
        assert_eq!(player.position.x, 5);
        assert_eq!(player.position.y, 5);
    }
}

#[cfg(test)]
mod player_move_tests {
    use super::*;

    fn base_player() -> Player {
        Player::new("TestPlayer".to_string(), 10, (5, 5))
    }
    #[test]
    fn player_move_up_test() {
        let mut p = base_player();
        p.move_up();
        assert_eq!(p.position.position(), (5, 4));
    }
    #[test]
    fn player_move_down_test() {
        let mut p = base_player();
        p.move_down();
        assert_eq!(p.position.position(), (5, 6));
    }
    #[test]
    fn player_move_left_test() {
        let mut p = base_player();
        p.move_left();
        assert_eq!(p.position.position(), (4, 5));
    }
    #[test]
    fn player_move_right_test() {
        let mut p = base_player();
        p.move_right();
        assert_eq!(p.position.position(), (6, 5));
    }
}
