use std::thread;

use rand::seq::SliceRandom;

use crate::entity::{Entity, Mapable};

pub trait Item {
    fn item_type(&self) -> String;
    fn use_item(&self);
    fn name(&self) -> &String;
    fn avatar(&self) -> &char;
}

pub struct Weapon {
    pub data: Entity,
}

impl Mapable for Weapon {
    fn avatar(&self) -> char {
        self.data.avatar
    }

    fn position(&self) -> fundamentals::position::Position {
        self.data.position
    }

    fn previous_position(&self) -> fundamentals::position::Position {
        self.data.previous_position
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn dialogue(&self) -> Vec<String> {
        vec![
            "Pick Up".to_string(),
            "Inspect".to_string(),
            "Drop".to_string(),
        ]
    }

}

impl Item for Weapon {
    fn item_type(&self) -> String {
        "Weapon".to_string()
    }

    fn use_item(&self) {
        todo!()
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn avatar(&self) -> &char {
        &self.data.avatar
    }
}
impl Weapon {
    pub(crate) fn random_weapon_name() -> String {
        let n = [
            "Broadsword",
            "Shortsword",
            "Axe",
            "Boomerang",
            "Dagger",
            "Mace",
        ];

        n.choose(&mut rand::thread_rng()).unwrap().to_string()
    }
}

pub struct Armour {
    pub data: Entity,
}

impl Mapable for Armour {
    fn avatar(&self) -> char {
        self.data.avatar
    }

    fn position(&self) -> fundamentals::position::Position {
        self.data.position
    }

    fn previous_position(&self) -> fundamentals::position::Position {
        self.data.previous_position
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn dialogue(&self) -> Vec<String> {
        vec![
            "Pick Up".to_string(),
            "Inspect".to_string(),
            "Leave".to_string(),
        ]
    }
}

impl Item for Armour {
    fn item_type(&self) -> String {
        "Armour".to_string()
    }

    fn use_item(&self) {
        todo!()
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn avatar(&self) -> &char {
        &self.data.avatar
    }
}

impl Armour {
    pub(crate) fn random_armour_name() -> String {
        let name = [
            "Chainmail",
            "Leather",
            "Plate",
            "Wooden"
        ];

        name.choose(&mut rand::thread_rng()).unwrap().to_string()
    }
}


pub struct Potion {
    pub data: Entity,
}

impl Mapable for Potion {
    fn avatar(&self) -> char {
        self.data.avatar
    }

    fn position(&self) -> fundamentals::position::Position {
        self.data.position
    }

    fn previous_position(&self) -> fundamentals::position::Position {
        self.data.previous_position
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn dialogue(&self) -> Vec<String> {
        todo!()
    }
}

impl Item for Potion {
    fn item_type(&self) -> String {
        "Potion".to_string()
    }

    fn use_item(&self) {
        todo!()
    }

    fn name(&self) -> &String {
        &self.data.name
    }

    fn avatar(&self) -> &char {
        &self.data.avatar
    }
}
impl Potion {
    pub(crate) fn random_potion_name() -> String {
        [
            "Health",
            "Attack",
            "Stamina",
            "Charisma",
        ].choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

