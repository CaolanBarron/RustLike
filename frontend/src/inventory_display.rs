use std::io::stdout;

use backend::item::Item;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
};
use fundamentals::{pos, position::Position};

use crate::UiElement;

pub struct InventoryDisplay {
    start_position: Position,
    end_position: Position,
}

impl InventoryDisplay {
    fn draw_inventory_title(&self) {
        let x = self.start_position.x as u16;
        let y = self.start_position.y as u16 + 1;
        execute!(stdout(), MoveTo(x + 6, y), Print("INVENTORY"))
            .expect("Error while writing inventory title");
        for i in x + 1..self.end_position.x as u16 - 1 {
            self.edit_ui(i, y + 1, '\u{2504}', &mut stdout())
                .expect("Error while drawing inventory line");
        }
    }

    fn draw_inventory_content(&self, items: &Vec<Box<dyn Item>>) {
        let x = self.start_position.x as u16;
        let y = self.start_position.y as u16 + 3;
        let end_x = self.end_position.x as u16;

        for (i, item) in items.iter().enumerate() {
            execute!(
                stdout(),
                MoveTo(x + 3, y + (2 * i) as u16),
                Print(item.name())
            );
            execute!(
                stdout(),
                MoveTo(end_x - 3, y + (2 * i) as u16),
                Print(item.avatar())
            );
        }
    }
}

impl UiElement for InventoryDisplay {
    fn build(start_position: Position, end_position: Position) -> Self {
        InventoryDisplay {
            start_position,
            end_position,
        }
    }

    fn edit_ui(
        &self,
        x: u16,
        y: u16,
        input: char,
        writer: &mut std::io::Stdout,
    ) -> crossterm::Result<()> {
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
        for y in start.y..end.y {
            for x in start.x..end.x {
                // If X and Y is 0: print top left
                if x == start.x && y == start.y {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{250F}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
                // If X is max and Y is 0: print top right
                else if x == end.x - 1 && y == start.y {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{2513}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
                // If X is 0 and Y is max: print bottom left
                else if x == start.x && y == end.y - 1 {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{2517}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
                // If X and Y is max: print bottom Right
                else if x == end.x - 1 && y == end.y - 1 {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{251B}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
                // If X is not 0 and max and y is 0 or max: print Horizontal wall
                else if (x != start.x && x != end.x - 1) && (y == start.y || y == end.y - 1) {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{2501}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
                // If X is 0 or max and y is not 0 and max: print Vertical wall
                else if (x == start.x || x == end.x - 1) && (y != start.y && y != end.y) {
                    self.edit_ui(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        '\u{2503}',
                        &mut stdout(),
                    )
                    .expect("Error editing UI");
                }
            }
        }
        self.draw_inventory_title();
    }
}

#[cfg(test)]
mod inventory_display_tests {
    use super::*;
    use backend::{
        entity::Entity,
        item::{Armour, Potion, Weapon},
    };
    use crossterm::{
        event::{read, Event, KeyCode, KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    };
    use fundamentals::pos;

    fn test_input() {
        let event = read().unwrap();

        if let Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                KeyCode::Esc => disable_raw_mode().expect("msg"),
                _ => (),
            }
        }
    }

    #[test]
    fn display_inventory_frame() {
        let id = InventoryDisplay::build(pos!(0, 0), pos!(20, 50));
        execute!(stdout(), Clear(ClearType::All), EnterAlternateScreen);
        enable_raw_mode();
        id.draw_frame(id.start_position, id.end_position);
        id.draw_inventory_title();

        loop {
            test_input();
        }
    }

    #[test]
    fn display_inventory_content() {
        let id = InventoryDisplay::build(pos!(0, 0), pos!(20, 50));
        execute!(stdout(), Clear(ClearType::All), EnterAlternateScreen);
        enable_raw_mode();
        id.draw_frame(id.start_position, id.end_position);
        id.draw_inventory_title();

        let items: Vec<Box<dyn Item>> = vec![
            Box::new(Weapon {
                data: Entity {
                    name: "TEST 1".to_string(),
                    position: pos!(21, 4),
                    previous_position: pos!(30, 8),
                    avatar: 'W',
                },
            }),
            Box::new(Armour {
                data: Entity {
                    name: "TEST 2".to_string(),
                    position: pos!(21, 4),
                    previous_position: pos!(30, 8),
                    avatar: 'A',
                },
            }),
            Box::new(Potion {
                data: Entity {
                    name: "TEST 3".to_string(),
                    position: pos!(21, 4),
                    previous_position: pos!(30, 8),
                    avatar: 'P',
                },
            }),
        ];

        id.draw_inventory_content(&items);

        loop {
            test_input();
        }
    }
}
