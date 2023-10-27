use std::fmt::Display;

use super::relative_position::RelativePosition;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position{
    file: u8,
    rank: u8,
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Result<Position, String> {
        //file and rank have to be between 1 and 8

        if file > 8 || file < 1 {
            return Err(format!("File out of bounds: {}", file));
        }

        if rank > 8 || file < 1 {
            return Err(format!("Rank out of bounds: {}", rank));
        }
        

        return Ok(Position {
            file,
            rank,
        })
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn file_char(&self) -> char {
        match self.file {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("Invalid file")
        }
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn from_relative(absolute_position: Position, relative_position: RelativePosition) -> Result<Position, String> {
        return Position::new((absolute_position.file() as i8 + relative_position.file()) as u8, (absolute_position.rank() as i8 + relative_position.rank()) as u8);
    }

    pub fn from_string(string: &str) -> Result<Position, String> {
        let chars: Vec<char> = string.chars().collect();

        if chars.len() != 2 {
            return Err(format!("Cannot parse position from more than 2 characters: {}", string));
        }

        let file = match chars[0] {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => return Err(format!("First character is not a valid file: {}!", chars[0]))
        };

        let rank = match chars[1] {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => return Err(format!("Second character is not a valid rank: {}!", chars[1]))
        };

        return Position::new(file, rank);
    }
}

impl Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self.file {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("Invalid file")
        };

        let rank = match self.rank {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            _ => panic!("Invalid rank")
        };

        write!(f, "{}{}", file, rank)
    }
}