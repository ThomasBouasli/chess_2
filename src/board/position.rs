use std::fmt::Display;

use super::relative_position::RelativePosition;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position{
    file: u8,
    rank: u8,
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Position {
        Position {
            file,
            rank,
        }
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn from_relative(absolute_position: Position, relative_position: RelativePosition) -> Position {
        Position {
            file: (absolute_position.file as i8 + relative_position.file()) as u8,
            rank: (absolute_position.rank as i8 + relative_position.rank()) as u8,
        }
    }
}

impl Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file + 97) as char, self.rank + 1)
    }
}