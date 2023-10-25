use std::fmt::Display;

use crate::{color::Color, piece::{Piece, movement::diagonal::DiagonalMovement}, board::relative_position::RelativePosition};

pub struct Bishop{
    color: Color,
}

impl Piece for Bishop {

    fn new(color: Color) -> Self where Self: Sized {
        return Bishop{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn name(&self) -> &str {
        return "Bishop";
    }

    fn prefix(&self) -> &str {
        return "B";
    }

    fn icon(&self) -> &str {
        return "♝";
    }

    fn value(&self) -> u8 {
        return 3;
    }

    fn is_valid_move(&self, position: &RelativePosition) -> bool {
        return self.is_valid_diagonal_move(position);
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
        return self.diagonal_moves();
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl DiagonalMovement for Bishop{}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "B");
    }
}