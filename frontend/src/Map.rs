use backend::entity::Entity;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    Result,
};
use fundamentals::position::Position;
use std::{ascii::AsciiExt, collections::HashMap, io::Stdout, vec};

enum BuildingBlocks {}
pub struct Map {
    pub size: (u16, u16),
    pub writer: Stdout,
    pub blocks: HashMap<String, char>,
}

impl Map {
    pub fn new(size: (u16, u16), writer: Stdout) -> Self {
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
    pub fn display_base_level(&mut self, level: Vec<(char, Position)>) -> Result<()> {
        execute!(self.writer, Clear(ClearType::All), EnterAlternateScreen,)?;

        level.iter().for_each(|input| {
            self.edit_map(
                input.1.x.try_into().unwrap(),
                input.1.y.try_into().unwrap(),
                input.0,
            )
            .expect("Error while displaying map");
        });

        enable_raw_mode()?;
        execute!(self.writer, MoveTo(0, 0))?;

        Ok(())
    }

    fn edit_map(&mut self, x: u16, y: u16, input: char) -> Result<()> {
        execute!(self.writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }

    pub fn refresh(&mut self, entities: &Vec<Box<dyn Entity>>) {
        for e in entities.iter() {
            self.edit_map(
                e.position().x.try_into().unwrap(),
                e.position().y.try_into().unwrap(),
                e.avatar(),
            )
            .expect("Refresh failed");
        }
    }

    pub fn refresh_player(&mut self, player: &backend::player::Player) {
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
#[cfg(test)]
mod display_tests {
    use super::*;
    use std::io::{stderr, stdout};

    fn test_map() -> Map {
        let p = backend::player::Player::new("TestPlayer1".to_string(), (3, 5));

        Map::new((10, 10), stdout())
    }
}
