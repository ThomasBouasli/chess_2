use std::fmt::Display;

use colored::Colorize;

use crate::{color::Color, piece::{Piece, movement::{diagonal::DiagonalMovement, linear::LinearMovement}}, board::relative_position::RelativePosition};

pub struct Queen{
    color: Color,
}

impl Piece for Queen {

    fn new(color: Color) -> Self where Self: Sized {
        return Queen{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn icon(&self) -> &str {
        return "♛";
    }

    fn name(&self) -> &str {
        return "Queen";
    }

    fn prefix(&self) -> &str {
        return "Q";
    }

    fn value(&self) -> u8 {
        return 9;
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

impl DiagonalMovement for Queen{}
impl LinearMovement for Queen {}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("Q");

        match self.color() {
            Color::White => string = string.green().to_string(),
            Color::Black => string = string.red().to_string(),
        };


        return write!(f, "{}", string);
    }
}