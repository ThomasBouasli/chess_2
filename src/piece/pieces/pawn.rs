use std::fmt::Display;

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
        return self.possible_moves();
    }

    fn possible_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();
        
        for file in -2i8..=2 {
            for rank in -2i8..=2 {
                if file.abs() != rank.abs() && file != 0 && rank != 0 {
                    moves.push(RelativePosition::new(file, rank));
                }
            }
        }

        return moves;
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "P");
    }
}