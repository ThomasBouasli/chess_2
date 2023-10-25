use std::fmt::Display;

use crate::{color::Color, piece::{Piece, movement::linear::LinearMovement}, board::relative_position::RelativePosition};

pub struct Rook{
    color: Color,
}

impl Rook {
    pub fn prefix() -> &'static str {
        return "R";
    }
}

impl Piece for Rook {
    fn new(color: Color) -> Self where Self: Sized {
        return Rook{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn icon(&self) -> &str {
        return "♜";
    }

    fn name(&self) -> &str {
        return "Rook";
    }

    fn prefix(&self) -> &str {
        return "R";
    }

    fn value(&self) -> u8 {
        return 5;
    }

    fn is_valid_move(&self, position: &RelativePosition) -> bool {
        return self.is_valid_linear_move(position);
    }

    fn is_valid_capture(&self, position: &RelativePosition) -> bool {
        return self.is_valid_move(position);
    }

    fn is_valid_play(&self, position: &RelativePosition) -> bool {
        return self.is_valid_move(position);
    }

    fn possible_captures(&self) -> Vec<RelativePosition> {
        return self.possible_moves();
    }

    fn possible_moves(&self) -> Vec<RelativePosition> {
        return self.linear_moves();
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl LinearMovement for Rook{}

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "R");
    }
}