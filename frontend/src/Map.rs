use backend::entity::{Entity, Mapable};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    Result,
};
use fundamentals::position::Position;
use std::{collections::HashMap, io::Stdout};

pub struct Map {
    pub size: (isize, isize),
    pub writer: Stdout,
    pub blocks: HashMap<String, char>,
}

impl Map {
    pub fn new(size: (isize, isize), writer: Stdout) -> Self {
        Self {
            size,
            writer,
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

    //
    pub fn display_base_level(&mut self, level: &HashMap<Position, char>) -> Result<()> {
        execute!(self.writer, Clear(ClearType::All), EnterAlternateScreen,)?;

        level.iter().for_each(|input| {
            self.edit_map(
                input.0.x.try_into().unwrap(),
                input.0.y.try_into().unwrap(),
                *input.1,
            )
            .expect("Error while displaying map");
        });

        enable_raw_mode()?;
        execute!(self.writer, MoveTo(0, 0))?;

        Ok(())
    }

    pub fn edit_map(&mut self, x: u16, y: u16, input: char) -> Result<()> {
        execute!(self.writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }

    pub fn refresh_entities(&mut self, entities: &Vec<Box<dyn Mapable>>) {
        entities.iter().for_each(|entity| {
            self.edit_map(
                entity.position().x.try_into().unwrap(), 
                entity.position().y.try_into().unwrap(), 
                entity.avatar()).expect("Error while refreshinh entities");
        });

    }

    pub fn refresh_player(&mut self, player: &backend::player::Player, previous_char: char) {

        self.edit_map(
            player.previous_position().x.try_into().unwrap(),
            player.previous_position().y.try_into().unwrap(),
            previous_char,
        )
        .expect("Refresh failed on player");
       
       self.edit_map(
            player.position().x.try_into().unwrap(),
            player.position().y.try_into().unwrap(),
            player.avatar(),
        )
        .expect("Refresh failed on player");
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn display_building_blocks() {
        let mut m = Map::new((10, 10), std::io::stdout());

        println!("{}", m.blocks.get(&"blank".to_string()).unwrap());
        println!("{}", m.blocks.get(&"horiz_wall".to_string()).unwrap());
        println!("{}", m.blocks.get(&"vert_wall".to_string()).unwrap());
        println!("{}", m.blocks.get(&"top_left_wall".to_string()).unwrap());
        println!("{}", m.blocks.get(&"bottom_left_wall".to_string()).unwrap());
        println!("{}", m.blocks.get(&"top_right_wall".to_string()).unwrap());
        println!(
            "{}",
            m.blocks.get(&"bottom_right_wall".to_string()).unwrap()
        );
    }
}