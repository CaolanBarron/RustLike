use std::io::Stdout;
use crossterm::{Result, terminal::{EnterAlternateScreen, enable_raw_mode, Clear, ClearType}, execute, cursor::{MoveTo}, style::Print};

pub struct Map {
    size: (u16, u16),
    pub writer: Stdout,
    
}

impl Map {
   pub fn new(size: (u16,u16) ,writer: Stdout) -> Self { 
        Self { 
            size, 
            writer 
        } 
    }

    // 
    pub fn display_base_map(&mut self,) -> Result<()> {
        execute!(
            self.writer, 
            Clear(ClearType::All),
            EnterAlternateScreen,
            )?;

        for _ in 0..self.size.1 {
            for _ in 0..self.size.0 {
                print!("-");    
            }
            println!();
        }

        enable_raw_mode()?;
        execute!(self.writer, MoveTo(0,0))?;

        Ok(())
    }
    
    fn edit_map(&mut self, x:u16, y:u16, input: String) -> Result<()> {
        execute!(self.writer, MoveTo(x, y), Print(input))?;
        Ok(())
    }    

    pub fn refresh(&mut self) {
        //for e in self.entities.iter() {
        //    self.edit_map(e.position().x.try_into().unwrap(), e.position().y.try_into().unwrap(), e.avatar());
        //}
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn edit_map_test(){
        let mut m = Map{ size: (10,10),writer: std::io::stdout()};
    
        m.display_base_map().expect("Could not display map");

        m.edit_map(2,2,"C").expect("Could not edit map");
    }
}
#[cfg(test)]
mod display_tests {
    use std::io::{stderr, stdout};
    use super::*;

    
    fn test_map() -> Map{

        let p = Player::new("TestPlayer1".to_string(), 10, (3, 5));

        Map {
            size: (10,10), 
            writer: stdout(),
        }
    }

    #[test]
    fn display_base_map() {
        let mut m = test_map();
        m.display_base_map();
    }
}