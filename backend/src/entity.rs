use fundamentals::position::Position;

use crate::{player::Player, enemy::Enemy};

/*
Entity is a struct that contains base data 
for all dynamic features of the game
*/
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

    pub fn position(self) -> Position {
        self.position.clone()
    }

    pub fn previous_position(self) -> Position {
        self.previous_position.clone()
    }

    pub fn update_position(&mut self) {
        self.previous_position = self.position;
    }

    pub fn move_position(&mut self, add_pos:Position) {
        self.position = self.position + add_pos
    }
}

pub trait Mapable {
    fn avatar(&self) -> char;
    fn position(&self) -> Position;
    fn previous_position(&self) -> Position;
}

pub trait Character {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);

    fn walkable(ground: char) -> bool;
    fn look() -> Vec<Entity>;
}

struct EntityBuilder;

impl EntityBuilder {
    pub fn build_player() -> Player {
        todo!()
    }
    pub fn build_enemy() -> Enemy {
        todo!()
    }
    pub fn build_item() -> ! {
        todo!()
    }
}