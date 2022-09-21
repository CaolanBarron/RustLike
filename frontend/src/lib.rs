use std::io::{Stdout, stdout};

use character_portrait::CharacterPortrait;
use crossterm::{execute, cursor::{MoveTo, Hide}, style::Print, Result, terminal::{Clear, ClearType, enable_raw_mode, EnterAlternateScreen}};
use dialogue_box::DialogueBox;
use fundamentals::{position::Position, pos};
use inventory_display::InventoryDisplay;
use options_display::OptionsDisplay;
use map::Map;
pub mod map;
pub mod dialogue_box;
pub mod character_portrait;
pub mod options_display;
pub mod inventory_display;

pub struct UI {
    output: Stdout,
    inventory_display: InventoryDisplay,
    map: Map,
    dialogue_box: DialogueBox,
    character_portrait: CharacterPortrait,
    options_display: OptionsDisplay,
}

impl UI {


    pub fn inventory_display(&self) -> &InventoryDisplay {
        &self.inventory_display
    }
    pub fn map(&self) -> &Map {
        &self.map
    }
    pub fn dialogue_box(&self) -> &DialogueBox {
        &self.dialogue_box
    }
    pub fn character_portrait(&self) -> &CharacterPortrait {
        &self.character_portrait
    }
    pub fn options_display(&self) -> &OptionsDisplay {
        &self.options_display
    }

    pub fn build(size: (isize, isize), ratios: isize) -> Self {
/*         let id = 
            InventoryDisplay::build(pos!(0 ,0), pos!(size.0/ratios, size.1 - size.1/ratios));

        let m = 
            Map::build(pos!(size.0/ratios,0), pos!(size.0, size.1 - size.1/ratios));

        let db = 
            DialogueBox::build(pos!(0, size.1 - size.1/ratios), pos!(size.0 - size.0/ratios, size.1));
        
        let cp = 
            CharacterPortrait::build(
                pos!(size.0 - size.0 - ratios, size.1 - size.1 - ratios), 
                pos!( size.0, size.1));

        let od = 
            OptionsDisplay::build(pos!(0, size.1 + 1), pos!(size.0, size.1 + 2));
*/
        let id = 
            InventoryDisplay::build(pos!(30 ,0), pos!(50, 50));

        let m = 
            Map::build(pos!(50,0), pos!(100,50));

        let db = 
            DialogueBox::build(pos!(30, 50), pos!(80, 70));
        
        let cp = 
            CharacterPortrait::build(
                pos!(80, 50), 
                pos!( 100, 70));

        let od = 
            OptionsDisplay::build(pos!(30, 70), pos!(100, 74));


        UI {
            output: stdout(),
            inventory_display: id,
            map: m,
            dialogue_box: db,
            character_portrait: cp,
            options_display: od,
        }
    }

    pub fn init_term(&mut self) {
        execute!(self.output, Clear(ClearType::All), Hide, EnterAlternateScreen);
        enable_raw_mode();

        self.inventory_display.draw_frame(self.inventory_display.start_position(), self.inventory_display.end_position());
        self.map.draw_frame(self.map.start_position(), self.map.end_position());
        self.dialogue_box.draw_frame(self.dialogue_box.start_position(), self.dialogue_box.end_position());
        self.character_portrait.draw_frame(self.character_portrait.start_position(), self.character_portrait.end_position());
        self.options_display.draw_frame(self.options_display.start_position(), self.options_display.end_position());
    }

}

trait UiElement {
    fn build(start_position: Position, end_position: Position) -> Self;

    fn start_position(&self) -> Position;
    fn end_position(&self) -> Position;
    fn draw_frame(&self, start: Position, end: Position) {
        for y in start.y .. end.y {
            for x in start.x .. end.x {

                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{250F}', &mut stdout());
                }
                // If X is max and Y is 0: print top right
                else if x == end.x - 1 && y == start.y {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2513}', &mut stdout());
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2517}', &mut stdout());
                }
                // If X and Y is max: print bottom Right
                else if x == end.x - 1 && y == end.y - 1 {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{251B}', &mut stdout());
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x - 1) && (y == start.y || y == end.y - 1) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2501}', &mut stdout());
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x - 1) && (y != start.y && y != end.y) {
                    self.edit_ui(x.try_into().unwrap(), y.try_into().unwrap(), '\u{2503}', &mut stdout());
                }
            }
        }
 
    }

    fn edit_ui(&self, x: u16, y: u16, input: char, writer: &mut Stdout) -> Result<()> {
        execute!(writer, MoveTo(x, y), Hide, Print(input))?;
        Ok(())
    }
}

#[cfg(test)]
mod ui_tests {
    use crossterm::{event::{read, Event, KeyEvent, KeyCode}, terminal::disable_raw_mode};
    use super::*;

    fn test_input() {
        let event = read().unwrap();

        if let Event::Key(KeyEvent {
            code, 
            modifiers: _, 
            kind: _, 
            state: _ 
        }) = event {
            match code {
                KeyCode::Esc => disable_raw_mode().expect("msg"),
                _=> (),
            }
        }
    }

    #[test]
    fn display_ui_frame() {
        let mut ui = UI::build((150, 100), 4);
        ui.init_term();
        loop{
            test_input();
        }
    }
}