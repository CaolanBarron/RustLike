use std::io::stdout;

use crossterm::{execute, cursor::{MoveTo, Hide}, style::Print, Result};
use fundamentals::position::Position;

use crate::UiElement;

pub struct CharacterPortrait {
    start_position: Position,
    end_position: Position,

} 

impl CharacterPortrait {
     pub fn draw_portrait(&self, portrait: &char) {
        match portrait {
            'H' => self.draw_h().expect("Failed while drawing H"),
            'O' => self.draw_o().expect("Failed while drawing O"),
            'B' => self.draw_b().expect("Failed while drawing B"),
            'Z' => self.draw_z().expect("Failed while drawing Z"),
            'D' => self.draw_d().expect("Failed while drawing D"),
            'W' => self.draw_w().expect("Failed while drawing W"),
            'S' => self.draw_s().expect("Failed while drawing S"),
            _ => self.clear_portrait(),
        }
    }
    fn draw_w(&self) -> Result<()>{
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print("WW      WW      WW"))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print("WW      WW      WW"))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print(" WW   WW  WW   WW "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print(" WW   WW  WW   WW "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print("  WW WW    WW WW  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print("  WW WW    WW WW  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print("   WWW      WWW   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print("   WWW      WWW   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print("    W        W    "))?;

        Ok(())
    }

    fn draw_s(&self) -> Result<()> {
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 3) as u16),
            Print("       SSSS       "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 4) as u16),
            Print("     SS    SS     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print("   SS        SS   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print("  S            S  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print("  S            S  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print("   SS             "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print("     SS           "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print("       SSSS       "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print("           SS     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print("             SS   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print("  S            S  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print("  S            S  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print("   SS        SS   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print("     SS    SS     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 17) as u16),
            Print("       SSSS       "))?;

        Ok(())
    }

    fn draw_z(&self) -> Result<()>{
        
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 3) as u16),
            Print(" ZZZZZZZZZZZZZZZZ "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 4) as u16),
            Print("                Z "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print("               Z  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print("             ZZ   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print("            Z     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print("           Z      "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print("         ZZ       "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print("        Z         "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print("       Z          "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print("     ZZ           "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print("    Z             "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print("   Z              "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print("  Z               "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print(" ZZ               "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 17) as u16),
            Print(" ZZZZZZZZZZZZZZZZ "))?;

        Ok(())
    }

    fn draw_d(&self) -> Result<()>{
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 2) as u16),
            Print(" DDDDDDDDD        "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 3) as u16),
            Print(" DDDDDDDDDD       "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 4) as u16),
            Print(" DD       DD      "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print(" DD        DD     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print(" DD         DD    "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print(" DD          DD   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print(" DD           DD  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print(" DD            DD "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print(" DD            DD "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print(" DD            DD "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print(" DD           DD  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print(" DD          DD   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print(" DD         DD    "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print(" DD        DD     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print(" DD       DD      "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 17) as u16),
            Print(" DDDDDDDDDD       "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 18) as u16),
            Print(" DDDDDDDDD        "))?;

        Ok(())
    }

    fn draw_h(&self) -> Result<()> {
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 2) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 3) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 4) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print(" HHHHHHHHHHHHHHHH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print(" HHHHHHHHHHHHHHHH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print(" HH            HH "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 17) as u16),
            Print(" HH            HH "))?;

        Ok(())
    }

    fn draw_b(&self) -> Result<()> {
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print(" BBBBBBBBBBBBBB   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print(" BB           BB  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print(" BBBBBBBBBBBBBBB  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print(" BB             B "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print(" BB           BB  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print(" BBBBBBBBBBBBBB   "))?;

        Ok(())
    }

    fn draw_o(&self) -> Result<()> {
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 3) as u16),
            Print("      OOOOOO      "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 4) as u16),
            Print("     O      O     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 5) as u16),
            Print("    O        O    "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 6) as u16),
            Print("   O          O   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 7) as u16),
            Print("  O            O  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 8) as u16),
            Print(" O              O "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 9) as u16),
            Print("O                O"))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 10) as u16),
            Print("O                O"))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 11) as u16),
            Print(" O              O "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 12) as u16),
            Print("  O            O  "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 13) as u16),
            Print("   O          O   "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 14) as u16),
            Print("    O        O    "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 15) as u16),
            Print("     O      O     "))?;
        execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (self.start_position.y + 16) as u16),
            Print("      OOOOOO      "))?;

        Ok(())
    }
    
    pub fn clear_portrait(&self) {
        for i in self.start_position.y+1..self.end_position.y-1 {
            execute!(stdout(), MoveTo((self.start_position.x + 1) as u16, (i).try_into().unwrap()), 
                Print("                  ".to_string())).expect("Failed while clearing portrait");
        }
    }
}



impl UiElement for CharacterPortrait {
    fn build(start_position: Position, end_position: Position) -> Self {
        CharacterPortrait { start_position, end_position}
    }


    fn edit_ui(&self, x: u16, y: u16, input: char, writer: &mut std::io::Stdout) -> crossterm::Result<()> {
        execute!(writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }

    fn draw_frame(&self, start: Position, end: Position) {
        for y in start.y .. end.y {
            for x in start.x .. end.x {

                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{250F}', &mut std::io::stdout()).expect("Error editing UI");
                }
                // If X is max and Y is 0: print top right
                else if x == end.x - 1 && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2513}', &mut std::io::stdout()).expect("Error editing UI");
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2517}', &mut std::io::stdout()).expect("Error editing UI");
                }
                // If X and Y is max: print bottom Right
                else if x == end.x - 1 && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{251B}', &mut std::io::stdout()).expect("Error editing UI");
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x - 1) && (y == start.y || y == end.y - 1) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2501}', &mut std::io::stdout()).expect("Error editing UI");
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x - 1) && (y != start.y && y != end.y) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2503}', &mut std::io::stdout()).expect("Error editing UI");
                }
            }
        }
 
    }

    fn start_position(&self) -> Position {
       self.start_position.clone() 
    }

    fn end_position(&self) -> Position {
        self.end_position.clone()
    }
}


#[cfg(test)]
mod character_portrait_tests {
    use fundamentals::pos;

    use super::*;

    #[test]
    fn portrait_display_frame() {
        let cp = CharacterPortrait::build(pos!(0,0), pos!(5,5));
        cp.draw_frame(cp.start_position, cp.end_position);
    }
}