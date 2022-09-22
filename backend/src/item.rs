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


pub struct Armour {
    pub data: Entity,
}

impl Mapable for Armour {
    fn avatar(&self) -> char {
        todo!()
    }

    fn position(&self) -> fundamentals::position::Position {
        todo!()
    }

    fn previous_position(&self) -> fundamentals::position::Position {
        todo!()
    }

    fn name(&self) -> &String {
        todo!()
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


pub struct Potion {
    pub data: Entity,
}

impl Mapable for Potion {
    fn avatar(&self) -> char {
        todo!()
    }

    fn position(&self) -> fundamentals::position::Position {
        todo!()
    }

    fn previous_position(&self) -> fundamentals::position::Position {
        todo!()
    }

    fn name(&self) -> &String {
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