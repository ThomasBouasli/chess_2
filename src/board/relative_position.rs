use std::fmt::Display;

use super::position::Position;

#[derive(Debug, Clone)]
pub struct RelativePosition{
    file: i8,
    rank: i8,
}

impl RelativePosition {
    pub fn new(file: i8, rank: i8) -> Result<RelativePosition, String> {
        // min is -7 and max is 7
        if file > 7 || file < -7 {
            return Err(format!("Invalid position: File out of bounds: {}", file));
        }

        if rank > 7 || rank < -7 {
            return Err(format!("Invalid position: Rank out of bounds: {}", rank));
        }

        return Ok(RelativePosition {file, rank})
    }

    pub fn file(&self) -> i8 {
        self.file
    }

    pub fn rank(&self) -> i8 {
        self.rank
    }

    pub fn from_absolute(from: &Position, to: &Position) -> Result<RelativePosition, String> {
        return RelativePosition::new((to.file() as i8 - from.file() as i8) as i8, (to.rank() as i8 - from.rank() as i8) as i8);
    }
}

impl Display for RelativePosition{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank: {} File: {}", self.rank, self.file)
    }
}