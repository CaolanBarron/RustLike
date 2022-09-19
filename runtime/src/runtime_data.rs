use std::collections::HashMap;

use backend::{entity::Entity, player::Player};
use frontend::map::Map;
use fundamentals::position::Position;

//  Data concerning what entities are active in the game
pub(crate) struct RuntimeData {
    pub(crate) player: Player,
    pub(crate) entities: Vec<Box<dyn Entity>>,
    pub(crate) level: HashMap<Position, char>,
}

impl RuntimeData {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            entities: vec![],
            level: HashMap::new(),
        }
    }

    pub(crate) fn generate_level(&mut self, map: &Map, start: Position, end: Position) {
        let mut insert = |character: &str, x: usize, y: usize| {
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
