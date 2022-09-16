use fundamentals::position::Position;
use strum_macros::{EnumString, Display};

/*
An Entity is any non static character that will be displayed on the screen.
The avatar will be the physical representation that is printed to the terminal
The position is the X and Y coordinates of the entity on the terminal.
*/
pub trait Entity {
    fn avatar(&self) -> String;
    fn position(&self) -> &Position;
}

/*
Movement is a trait that will only be implemented alongisde entities that require the ability to
move on a round by round basis if possible.
*/
pub trait Movement {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
}

#[derive(Display,Debug)]
pub enum Avatar {
    P,
    E,
}