use std::fmt::Display;

use super::position::Position;

#[derive(Debug)]
pub struct RelativePosition{
    file: i8,
    rank: i8,
}

impl RelativePosition {
    pub fn new(file: i8, rank: i8) -> RelativePosition {
        RelativePosition {
            file,
            rank,
        }
    }

    pub fn file(&self) -> i8 {
        self.file
    }

    pub fn rank(&self) -> i8 {
        self.rank
    }

    pub fn from_absolute(from: &Position, to: &Position) -> RelativePosition {
        RelativePosition {
            file: (to.file() as i8 - from.file() as i8) as i8,
            rank: (to.rank() as i8 - from.rank() as i8) as i8,
        }
    }
}

impl Display for RelativePosition{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}