use std::fmt::Display;

use colored::Colorize;

use crate::{color::Color, piece::Piece, board::relative_position::RelativePosition};

pub struct Knight{
    color: Color,
}

impl Piece for Knight {

    fn new(color: Color) -> Self where Self: Sized {
        return Knight{
            color,
        };
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn icon(&self) -> &str {
        return "♞";
    }

    fn name(&self) -> &str {
        return "Knight";
    }

    fn prefix(&self) -> &str {
        return "N";
    }

    fn value(&self) -> u8 {
        return 3;
    }

    fn is_valid_move(&self, position: &RelativePosition) -> bool {
        return (position.file().abs() == 2 && position.rank().abs() == 1) || (position.file().abs() == 1 && position.rank().abs() == 2);
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
        
        for file in -2i8..=2 {
            for rank in -2i8..=2 {
                if file.abs() != rank.abs() && file != 0 && rank != 0 {
                    let position = match RelativePosition::new(file, rank){
                        Ok(position) => position,
                        Err(_) => panic!("Invalid position")
                    };
                    moves.push(position);
                }
            }
        }

        return moves;
    }

    fn possible_plays(&self) -> Vec<RelativePosition> {
        return self.possible_moves()
    }
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("N");

        match self.color() {
            Color::White => string = string.green().to_string(),
            Color::Black => string = string.red().to_string(),
        };


        return write!(f, "{}", string);
    }
}