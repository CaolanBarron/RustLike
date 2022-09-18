use backend::{player::Player, entity::Entity};
use frontend::map::Map;
use fundamentals::position::Position;

//  Data concerning what entities are active in the game
pub(crate) struct RuntimeData {
    pub(crate) player: Player,
    pub(crate) entities: Vec<Box<dyn Entity>>,
    pub(crate) level: Vec<(char, Position)>,
}

impl RuntimeData {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            entities: vec![],
            level: vec![],
        }
    }

    pub(crate) fn generate_level(&mut self, map: &Map, start: Position, end: Position) {
        let mut insert = |character: &str, x: usize, y: usize| {
            self.level.push((
                map.blocks.get(&character.to_string()).unwrap().clone(),
                Position::new(x, y),
            ))
        };
        for y in start.y..end.y {
            for x in start.x..end.x {
                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    insert("top_left_wall", x, y)
                }
                // If X is max and Y is 0: print top right
                else if x == end.x && y == start.y {
                    insert("top_right_wall", x, y)
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y {
                    insert("bottom_left_wall", x, y)
                }
                // If X and Y is max: print bottom Right
                else if x == end.x && y == end.y {
                    insert("bottom_right_wall", x, y)
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x) && (y == start.y || y == end.y) {
                    insert("horiz_wall", x, y)
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x) && (y != start.y && y != end.y) {
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
