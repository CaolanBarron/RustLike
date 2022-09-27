extern crate fundamentals;
use std::rc::Rc;

use crate::{
    entity::{Character, Entity, Mapable},
    item::Item,
};
use fundamentals::{pos, position::Position};

pub struct Player {
    data: Entity,

    inventory: Vec<Rc<dyn Item>>,
    pub inventory_choice: u16,
}

impl Player {
    pub fn new(name: String, x: isize, y: isize) -> Self {
        Player {
            data: Entity {
                name,
                position: pos!(x, y),
                previous_position: pos!(x, y),
                avatar: '\u{263A}',
            },
            inventory: vec![],
            inventory_choice: 0,
        }
    }

    pub fn position(&self) -> Position {
        self.data.position.clone()
    }

    pub fn previous_position(&self) -> Position {
        self.data.previous_position.clone()
    }

    pub fn pick_up(&mut self, item: Rc<dyn Item>) {
        self.inventory.push(item)
    }

    pub fn drop(item: Box<dyn Item>) -> Box<dyn Item> {
        todo!()
    }

    pub fn look(&self, entities: &Vec<Rc<dyn Mapable>>) -> Option<Vec<Rc<dyn Mapable>>> {
        let mut player_prox: Vec<Position> = vec![];
        for y in self.position().y - 1..self.position().y + 2 {
            for x in self.position().x - 1..self.position().x + 2 {
                player_prox.push(pos!(x, y));
            }
        }

        let mut new_vec = vec![];
        for e in entities {
            if player_prox.contains(&e.position()) {
                new_vec.push(e.clone())
            }
        }
        if new_vec.is_empty() {
            None
        } else {
            Some(new_vec)
        }
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

    fn name(&self) -> &String {
        &self.data.name
    }

    fn dialogue(&self) -> Vec<String> {
        todo!()
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

    fn walkable(_ground: Option<&char>) -> bool {
        match _ground {
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
