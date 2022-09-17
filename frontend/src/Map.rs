use std::{io::Stdout, vec, collections::HashMap, ascii::AsciiExt};
use backend::entity::Entity;
use crossterm::{Result, terminal::{EnterAlternateScreen, enable_raw_mode, Clear, ClearType}, execute, cursor::{MoveTo, Hide}, style::Print};

pub struct Map {
    pub size: (u16, u16),
    pub writer: Stdout,
    blocks: HashMap<String, char>,
}

impl Map {
   pub fn new(size: (u16,u16) ,writer: Stdout) -> Self { 
        Self { 
            size, 
            writer, 
            blocks: HashMap::from([
                ("blank".to_string(), '\u{2592}'),
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
    pub fn display_base_map(&mut self,) -> Result<()> {
        execute!(
            self.writer, 
            Clear(ClearType::All),
            EnterAlternateScreen,
            )?;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                // If X and Y is 0: print top left
                if x == 0 && y == 0{print!{"{}",self.blocks.get(&"top_left_wall".to_string()).unwrap()}}
                // If X is max and Y is 0: print top right
                else if x == self.size.0 -1 && y == 0 {print!{"{}",self.blocks.get(&"top_right_wall".to_string()).unwrap()}}
                // If X is 0 and Y is max: print bottom left
                else if x == 0 && y == self.size.1 -1{print!{"{}",self.blocks.get(&"bottom_left_wall".to_string()).unwrap()}}
                // If X and Y is max: print bottom Right
                else if x == self.size.0 -1 && y == self.size.1 -1 {print!{"{}",self.blocks.get(&"bottom_right_wall".to_string()).unwrap()}}
                // If X is not 0 and max and y is 0 or max: print Horizontal wall 
                else if (x != 0 && x != self.size.0 -1) && (y == 0 || y == self.size.1 -1) {print!{"{}",self.blocks.get(&"horiz_wall".to_string()).unwrap()}} 
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == 0 || x == self.size.0 -1) && (y != 0 && y != self.size.1 -1) {print!{"{}",self.blocks.get(&"vert_wall".to_string()).unwrap()}}
                // Else: Print blank
                else {print!{"{}",self.blocks.get(&"blank".to_string()).unwrap()}}
            }
            println!();
        }

        enable_raw_mode()?;
        execute!(self.writer, MoveTo(0,0))?;

        Ok(())
    }
    
    fn edit_map(&mut self, x:u16, y:u16, input: String) -> Result<()> {
        execute!(self.writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }    

    pub fn refresh(&mut self, entities: &Vec<Box<dyn Entity>>) {
        for e in entities.iter() {
            self.edit_map(e.position().x.try_into().unwrap(), e.position().y.try_into().unwrap(), e.avatar()).expect("Refresh failed");
        }
    }

    pub fn refresh_player(&mut self, player: &backend::player::Player) {
        self.edit_map(player.position().x.try_into().unwrap(), player.position().y.try_into().unwrap(), player.avatar()).expect("Refresh failed on player");
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn edit_map_test(){
        let mut m = Map::new((10,10), std::io::stdout());
    
        m.display_base_map().expect("Could not display map");

        m.edit_map(2,2, "C".to_string()).expect("Could not edit map");
    }

    #[test]
    fn display_building_blocks(){
        let mut m = Map::new((10,10), std::io::stdout());

        println!("{}",m.blocks.get(&"blank".to_string()).unwrap());
        println!("{}",m.blocks.get(&"horiz_wall".to_string()).unwrap());
        println!("{}",m.blocks.get(&"vert_wall".to_string()).unwrap());
        println!("{}",m.blocks.get(&"top_left_wall".to_string()).unwrap());
        println!("{}",m.blocks.get(&"bottom_left_wall".to_string()).unwrap());
        println!("{}",m.blocks.get(&"top_right_wall".to_string()).unwrap());
        println!("{}",m.blocks.get(&"bottom_right_wall".to_string()).unwrap());
    }

}
#[cfg(test)]
mod display_tests {
    use std::io::{stderr, stdout};
    use super::*;

    
    fn test_map() -> Map{

        let p = backend::player::Player::new("TestPlayer1".to_string(), (3, 5));

        Map::new((10,10), stdout())
    }

    #[test]
    fn display_base_map() {
        let mut m = test_map();
        m.display_base_map().expect("Map Display test failed");
    }
}