use std::{ops, fmt};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x , y }
    }
}

impl ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x = {}, y = {}", self.x, self.y)
    }
}


#[cfg(test)]
mod position_tests {
    use super::Position;

    #[test]
    fn add_positions() {
        let pos_one = Position {x: 3, y: 3};
        let pos_two = Position {x: 4, y: 4};
        let new_pos = pos_one + pos_two;

        assert_eq!(new_pos.x, 7);
        assert_eq!(new_pos.y, 7);
    }

    #[test]
    fn sub_positions() {
        let pos_one = Position { x: 5, y: 5};
        let pos_two = Position { x: 1, y : 1};
        let new_pos = pos_one - pos_two;

        assert_eq!(new_pos.x, 4);
        assert_eq!(new_pos.y, 4);
    }

    #[test]
    fn display_position() {
        let displ_pos = Position { x:7, y:3}; 
        println!("Displays as: {}",displ_pos);
    }
} 
