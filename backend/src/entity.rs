use std::rc::Rc;

use fundamentals::{pos, position::Position};

use crate::{
    enemy::Enemy,
    item::{Armour, Item, Potion, Weapon},
    player::Player,
};

/*
Entity is a struct that contains base data
for all dynamic features of the game
*/
#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub position: Position,
    pub previous_position: Position,
    pub avatar: char,
}

impl Entity {
    pub fn name(self) -> String {
        self.name
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }

    pub fn previous_position(&self) -> Position {
        self.previous_position.clone()
    }

    pub fn update_position(&mut self) {
        self.previous_position = self.position;
    }

    pub fn move_position(&mut self, add_pos: Position) {
        self.position = self.position + add_pos
    }
}

pub trait Mapable {
    fn avatar(&self) -> char;
    fn position(&self) -> Position;
    fn previous_position(&self) -> Position;
    fn name(&self) -> &String;
    fn dialogue(&self) -> Vec<String>;
}

pub trait Character {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);

    fn walkable(ground: Option<&char>) -> bool {
        match ground {
            Some(ground) => match ground {
                '\u{2591}' => true,
                '\u{2592}' => true,
                '\u{2593}' => true,
                _ => false,
            },
            None => false,
        }
    }
}

#[derive(Default)]
pub struct EntityBuilder;

impl EntityBuilder {
    pub fn new() -> EntityBuilder {
        Default::default()
    }
    #[allow(dead_code)]
    pub fn build_player() -> Player {
        todo!()
    }
    #[allow(dead_code)]
    pub fn build_enemy(self, pos: Position) -> Enemy {
        let n = Enemy::random_enemy_name();
        Enemy {
            data: Entity {
                name: n.0,
                position: pos,
                previous_position: pos,
                avatar: n.1,
            },
        }
    }

    pub fn build_armour(self, pos: Position) -> Armour {
        let n = Armour::random_armour_name();

        Armour {
            data: Entity {
                name: n,
                position: pos,
                previous_position: pos,
                avatar: '\u{25DB}',
            },
        }
    }

    pub fn build_weapon(self, pos: Position) -> Weapon {
        let n = Weapon::random_weapon_name();

        Weapon {
            data: Entity {
                name: n,
                position: pos,
                previous_position: pos,
                avatar: '\u{2694}',
            },
        }
    }

    pub fn build_potion(self, pos: Position) -> Potion {
        let n = Potion::random_potion_name();

        Potion {
            data: Entity {
                name: n,
                position: pos,
                previous_position: pos,
                avatar: '\u{10C4}',
            },
        }
    }
}

#[cfg(test)]
mod entity_tests {
    use super::*;

    #[test]
    fn build_enemy_test() {
        let eb = EntityBuilder::new();

        let e1 = eb.build_enemy(pos!(2, 2));

        print!("{:?}", e1)
    }
}
