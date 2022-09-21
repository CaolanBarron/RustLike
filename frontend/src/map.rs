use backend::entity::{Mapable};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    Result,
};
use fundamentals::{pos, position::{Position}};
use indexmap::IndexMap;
use std::{collections::HashMap, io::{Stdout, stdout}};
use crate::UiElement;
pub struct Map {
    pub start_position: Position,
    pub end_position: Position,
    pub blocks: HashMap<String, char>,
}

impl UiElement for Map {
    fn build(start_position: Position, end_position: Position) -> Self {
        Self {
            start_position,
            end_position,
            blocks: HashMap::from([
                ("blank".to_string(), ('\u{2592}')),
                ("vert_wall".to_string(), '\u{2551}'),
                ("horiz_wall".to_string(), '\u{2550}'),
                ("top_left_wall".to_string(), '\u{2554}'),
                ("bottom_left_wall".to_string(), '\u{255A}'),
                ("top_right_wall".to_string(), '\u{2557}'),
                ("bottom_right_wall".to_string(), '\u{255D}'),
            ]),
        }
    }


    fn edit_ui(&self, x: u16, y: u16, input: char, writer: &mut Stdout) -> Result<()> {
        execute!(writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }

    fn start_position(&self) -> Position {
        self.start_position.clone()
    }

    fn end_position(&self) -> Position {
        self.end_position.clone()
    }

    fn draw_frame(&self, start: Position, end: Position) {
        for y in start.y .. end.y {
            for x in start.x .. end.x {

                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{250F}', &mut stdout()).expect("Error editing UI");
                }
                // If X is max and Y is 0: print top right
                else if x == end.x - 1 && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2513}', &mut stdout()).expect("Error editing UI");
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2517}', &mut stdout()).expect("Error editing UI");
                }
                // If X and Y is max: print bottom Right
                else if x == end.x - 1 && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{251B}', &mut stdout()).expect("Error editing UI");
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x - 1) && (y == start.y || y == end.y - 1) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2501}', &mut stdout()).expect("Error editing UI");
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x - 1) && (y != start.y && y != end.y) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2503}', &mut stdout()).expect("Error editing UI");
                }
                else {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2592}', &mut stdout()).expect("Error editing UI");
                }
            }
        }
 
    }

   
}

impl Map {
    //
    pub fn display_base_level(&self, level: &IndexMap<Position, char>, writer: &mut Stdout) -> Result<()> {

        level.iter().for_each(|input| {
            self.edit_ui(
                input.0.x.try_into().unwrap(),
                input.0.y.try_into().unwrap(),
                *input.1,
                writer
            ).expect("Failed lol");
        });

        Ok(())
    }

    

    pub fn refresh_entities(&self, entities: &Vec<Box<dyn Mapable>>, writer: &mut Stdout) {
        entities.iter().for_each(|entity| {
            self.edit_ui(
                entity.position().x.try_into().unwrap(), 
                entity.position().y.try_into().unwrap(), 
                entity.avatar(),
                writer).expect("Error while refreshing entities");
        });

    }

    pub fn refresh_player(&self, player: &backend::player::Player, previous_char: char, writer: &mut Stdout) {

        self.edit_ui(
            player.previous_position().x.try_into().unwrap(),
            player.previous_position().y.try_into().unwrap(),
            previous_char,
            writer
        )
        .expect("Refresh failed on player");
       
       self.edit_ui(
            player.position().x.try_into().unwrap(),
            player.position().y.try_into().unwrap(),
            player.avatar(),
            writer
        )
        .expect("Refresh failed on player");
    }
}


#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn display_map_frame() {
        let m = Map::build(pos!(0, 0), pos!(20, 20));
        execute!(stdout(),Clear(ClearType::All), EnterAlternateScreen);
        enable_raw_mode();
        m.draw_frame(m.start_position, m.end_position);
        loop{}
    }
}