use std::{array, vec};

use fundamentals::{pos, position::Position};
use rand::seq::SliceRandom;

use crate::entity::{Entity, Character, Mapable};

#[derive(Debug)]
pub struct Enemy {
    pub(crate) data: Entity,
}

impl Enemy {

    pub fn random_enemy_name() -> (String, char) {
        let name = [
            "Harpy",
            "Orc",
            "Bandit",
            "Zombie",
            "Dinosaur",
            "Skeleton",
            "Wizard",
        ];

        let chosen_name = name.choose(&mut rand::thread_rng()).unwrap();
        (chosen_name.to_string(), chosen_name.chars().next().unwrap())
    }
}

impl Mapable for Enemy {
    fn avatar(&self) -> char {
        self.data.avatar.clone()
    }

    fn position(&self) -> Position {
        self.data.position.clone()
    }

    fn previous_position(&self) -> Position {
        self.data.previous_position.clone()
    }

    fn name(&self) -> &String {
       &self.data.name 
    }

    fn dialogue(&self) -> Vec<String> {
        vec!["Talk".to_string(), "Fight".to_string(), "Leave".to_string()]

    }

}

impl Character for Enemy {
    fn move_up(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0,-1));
    }

    fn move_down(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0,-1));
    }

    fn move_left(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0,-1));
    }

    fn move_right(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0,-1));
    }



    fn walkable(ground: Option<&char>) -> bool {
        match ground {
            Some(ground) =>{ 
                match ground {
                    '\u{2591}' => true,
                    '\u{2592}' => true,
                    '\u{2593}' => true,
                    _ => false,
                }
            }
            None => false,
        }
    }
}

