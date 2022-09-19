extern crate fundamentals;
use fundamentals::{pos,position::Position};
    
    
use crate::entity::{Entity, Character, Mapable};


pub struct Player {
    data: Entity
}

impl Player {

    pub fn new(name: String, x: isize, y: isize) -> Self {
        Player { 
            data: Entity {
                name,
                position: pos!(x, y),
                previous_position: pos!(x,y),
                avatar: '\u{263A}',
            } 
        }
    }
    pub fn position(&self) -> Position {
       self.data.position.clone() 
    }

    pub fn previous_position(&self) -> Position {
        self.data.previous_position.clone()
    }

}

impl Mapable for Player {
    fn avatar(&self) -> char {
        self.data.avatar.clone()
    }

    fn position(&self) -> Position {
        self.data.position.clone()
    }

    fn previous_position(&self) -> Position {
        self.data.previous_position.clone()
    }
}

impl Character for Player {
    fn move_up(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0, -1));
    }
    fn move_down(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(0, 1));
    }
    fn move_left(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(-1, 0));
    }
    fn move_right(&mut self) {
        self.data.update_position();
        self.data.move_position(pos!(1, 0));
    }
    
    fn walkable(ground: char) -> bool {
        todo!()
    }
    fn look() -> Vec<Entity> {
        todo!()
    }
}
