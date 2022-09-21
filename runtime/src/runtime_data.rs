use std::{collections::HashMap, ops::Index};

use backend::{entity::{Entity, Mapable, EntityBuilder, Character}, player::Player, enemy::Enemy};
use frontend::map::Map;
use fundamentals::{position::Position, pos};
use indexmap::IndexMap;
use rand::{Rng, seq::IteratorRandom};

//  Data concerning what entities are active in the game
pub(crate) struct RuntimeData {
    pub(crate) player: Player,
    pub(crate) entities: Vec<Box<dyn Mapable>>,
    pub(crate) level: IndexMap<Position, char>,
}

impl RuntimeData {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            entities: vec![],
            level: IndexMap::new()
        }
    }

    pub fn add_enemy(&mut self) {
        loop{
            let pos = self.level.keys().choose(&mut rand::thread_rng());
            if Enemy::walkable(self.level.get(pos.unwrap())) {
                let eb = EntityBuilder::new();
                self.entities.push(Box::new(eb.build_enemy(*pos.unwrap())));
                break;
            }
        }
    }

    pub(crate) fn generate_level(&mut self, map: &Map, start: Position, end: Position) {

        let start = pos!(52,5);
        let end = pos!(80,15);
        let mut insert = |character: &str, x: isize, y: isize| {
            self.level.insert(
                Position::new(x, y),
                map.blocks.get(&character.to_string()).unwrap().clone(),
            );
        };
        for y in start.y..end.y {
            for x in start.x..end.x {
                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    insert("top_left_wall", x, y)
                }
                // If X is max and Y is 0: print top right
                else if x == end.x - 1 && y == start.y {
                    insert("top_right_wall", x, y)
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y - 1 {
                    insert("bottom_left_wall", x, y)
                }
                // If X and Y is max: print bottom Right
                else if x == end.x - 1 && y == end.y - 1 {
                    insert("bottom_right_wall", x, y)
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x - 1) && (y == start.y || y == end.y - 1) {
                    insert("horiz_wall", x, y)
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x - 1) && (y != start.y && y != end.y) {
                    insert("vert_wall", x, y)
                }
                // Else: Print blank
                else {
                    insert("blank", x, y)
                }
            }
        }
    }
}


#[cfg(test)]
mod runtime_data_tests {
    use frontend::UI;

    use super::*;

    #[test]
    fn add_enemy_test() {
        let mut rd = RuntimeData::new( Player::new("namJose".to_string(), 55, 5) );

        let ui = UI::build((2,2), 2);

        rd.generate_level(ui.map(), pos!(50,4), pos!(100,50));

        rd.add_enemy();

        print!("FOund");
        print!("{:?}", rd.entities.index(0).name());
    } 
}