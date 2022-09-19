use fundamentals::{pos, position::Position};

use crate::entity::{Entity, Character, Mapable};

pub struct Enemy {
    data: Entity,
}

impl Enemy {

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

    fn walkable(ground: char) -> bool {
        todo!()
    }

    fn look() -> Vec<Entity> {
        todo!()
    }
}

