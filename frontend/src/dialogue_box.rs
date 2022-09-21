use crossterm::{execute, cursor::{MoveTo, Hide}, style::Print};
use fundamentals::position::Position;

use crate::UiElement;

pub struct DialogueBox {
    start_position: Position,
    end_position: Position,
}

impl UiElement for DialogueBox {
    fn build(start_position: Position, end_position: Position) -> Self {
        DialogueBox { start_position, end_position }
    }


    fn edit_ui(&self, x: u16, y: u16, input: char, writer: &mut std::io::Stdout) -> crossterm::Result<()> {
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
}

#[cfg(test)]
mod dialogue_box_tests {
    use std::io::stdout;

    use crossterm::terminal::{Clear, EnterAlternateScreen, enable_raw_mode};
    use fundamentals::pos;

    use super::*;

    #[test]
    fn dialogue_display_frame() {
        let db = DialogueBox::build(pos!(0,20), pos!(30, 50));
        db.draw_frame(db.start_position, db.end_position);
    }
}