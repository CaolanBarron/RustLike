use crate::entity::{Entity, Movement};
use fundamentals::position::{self as pos, Position};

// Name: name of the entity. Randomly generated
// Health: Value that changes often 
// Attach: used to determine probability out of 10
// Damage: the amount of damage dealt if the attack succeeds
#[derive(Debug)]
struct Enemy {
    name: String,
    health: usize,
    attack: usize,
    damage: usize,
    position: pos::Position,
    avatar: char,
}

impl Enemy {
    pub fn new(
        name: String,
        health: usize,
        attack: usize,
        damage: usize,
        position: (usize, usize),
    ) -> Enemy {
        Enemy {
            name,
            health,
            attack,
            damage,
            position: Position::new(position.0, position.1),
            avatar: 'E',
        }
    }
}

impl Entity for Enemy {
    // Getters
    fn avatar(&self) -> char {
        self.avatar
    }
    fn position(&self) -> &pos::Position {
        &self.position
    }
}

impl Movement for Enemy {
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
mod enemy_tests {
    use super::*;
    #[test]
    fn create_enemy() {
        let enemy = Enemy::new("TestEnemy".to_string(), 10, 4, 2, (5, 5));

        println!("{:?}", enemy);

        assert_eq!(enemy.name, "TestEnemy");
        assert_eq!(enemy.health, 10);
        assert_eq!(enemy.position.x, 5);
        assert_eq!(enemy.position.y, 5);
    }
}
#[cfg(test)]
mod enemy_move_tests {
    use super::*;

    fn base_enemy() -> Enemy {
        Enemy::new("TestEnemy".to_string(), 10, 4, 2, (5, 5))
    }
    #[test]
    fn enemy_move_up_test() {
        let mut p = base_enemy();
        p.move_up();
        assert_eq!(p.position.position(), (5, 4));
    }
    #[test]
    fn enemy_move_down_test() {
        let mut p = base_enemy();
        p.move_down();
        assert_eq!(p.position.position(), (5, 6));
    }
    #[test]
    fn enemy_move_left_test() {
        let mut p = base_enemy();
        p.move_left();
        assert_eq!(p.position.position(), (4, 5));
    }
    #[test]
    fn enemy_move_right_test() {
        let mut p = base_enemy();
        p.move_right();
        assert_eq!(p.position.position(), (6, 5));
    }
}
