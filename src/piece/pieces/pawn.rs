use std::fmt::Display;

use colored::Colorize;

use crate::{color::Color, piece::Piece, board::relative_position::RelativePosition};

pub struct Pawn{
    color: Color,
}

impl Pawn {
    fn rank_multiplier(&self) -> i8 {
        return match self.color {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    pub fn is_double_move(to : &RelativePosition) -> bool {
        return to.rank().abs() == 2;
    }

    pub fn prefix() -> &'static str {
        return "P";
    }
}

impl Piece for Pawn {

    fn new(color: Color) -> Self where Self: Sized {
        return Pawn{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn icon(&self) -> &str {
        return "♟";
    }

    fn name(&self) -> &str {
        return "Pawn";
    }

    fn prefix(&self) -> &str {
        return "P";
    }

    fn value(&self) -> u8 {
        return 1;
    }

    fn is_valid_move(&self, position: &RelativePosition) -> bool {
        if position.file() == 0 && position.rank() * self.rank_multiplier() <= 2 {
            return true;
        }

        return false;
    }

    fn is_valid_capture(&self, position: &RelativePosition) -> bool {
        if position.file().abs() == 1 && position.rank() * self.rank_multiplier() == 1 {
            return true;
        }

        return false;
    }

    fn is_valid_play(&self, position: &RelativePosition) -> bool {
        return self.is_valid_move(position) || self.is_valid_capture(position);
    }

    fn possible_captures(&self) -> Vec<RelativePosition> {
        let mut captures = Vec::new();

        captures.push(RelativePosition::new(1, self.rank_multiplier()).unwrap());
        captures.push(RelativePosition::new(-1, self.rank_multiplier()).unwrap());

        return captures;
    }

    fn possible_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();
        
        moves.push(RelativePosition::new(0, self.rank_multiplier()).unwrap());
        moves.push(RelativePosition::new(0, self.rank_multiplier() * 2).unwrap());

        return moves;
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("P");

        match self.color() {
            Color::White => string = string.green().to_string(),
            Color::Black => string = string.red().to_string(),
        };


        return write!(f, "{}", string);
    }
}