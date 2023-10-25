use std::fmt::Display;

use crate::{color::Color, piece::{Piece, movement::{diagonal::DiagonalMovement, linear::LinearMovement}}, board::relative_position::RelativePosition};

pub struct King{
    color: Color,
}

impl King {
    pub fn prefix() -> &'static str {
        return "K";
    }
}

impl Piece for King {

    fn new(color: Color) -> Self where Self: Sized {
        return King{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn name(&self) -> &str {
        return "King";
    }

    fn value(&self) -> u8 {
        return 0;
    }

    fn prefix(&self) -> &str {
        return "K";
    }

    fn icon(&self) -> &str {
        return "♚";
    }

    fn is_valid_move(&self, position: &RelativePosition) -> bool {
        return self.is_valid_diagonal_move(position) || self.is_valid_linear_move(position);
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
        let mut moves = Vec::new();
        
        moves.append(&mut self.diagonal_moves());
        moves.append(&mut self.linear_moves());

        return moves;
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl DiagonalMovement for King{
    fn is_valid_diagonal_move(&self, position: &RelativePosition) -> bool {
        return position.file().abs() == position.rank().abs() && position.file() == 1;
    }
}
impl LinearMovement for King {
    fn is_valid_linear_move(&self, position: &RelativePosition) -> bool {
        return (position.file() == 0 && position.rank() == 1) || (position.file() == 1 && position.rank() == 0);
    }
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "K");
    }
}