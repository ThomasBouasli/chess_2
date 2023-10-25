use std::fmt::Display;

use crate::{color::Color, board::{relative_position::RelativePosition, Board, position::Position}};

pub mod pieces;
pub mod movement;

pub fn piece_factory(prefix : &str, color : Color) -> Box<dyn Piece>{
    return match prefix {
        "P" => Box::new(pieces::pawn::Pawn::new(color)),
        "R" => Box::new(pieces::rook::Rook::new(color)),
        "N" => Box::new(pieces::knight::Knight::new(color)),
        "B" => Box::new(pieces::bishop::Bishop::new(color)),
        "Q" => Box::new(pieces::queen::Queen::new(color)),
        "K" => Box::new(pieces::king::King::new(color)),
        _ => panic!("Invalid piece prefix"),
    }
}

pub trait Piece : Display{
    fn new(color: Color) -> Self where Self: Sized;
    fn color(&self) -> &Color;
    fn name(&self) -> &str;
    fn prefix(&self) -> &str;
    fn icon(&self) -> &str;
    fn value(&self) -> u8;
    fn is_valid_move(&self, position: &RelativePosition) -> bool;
    fn is_valid_capture(&self, position: &RelativePosition) -> bool;
    fn is_valid_play(&self, position: &RelativePosition) -> bool;
    fn possible_plays(&self) -> Vec<RelativePosition>;
    fn possible_moves(&self) -> Vec<RelativePosition>;
    fn possible_captures(&self) -> Vec<RelativePosition>;
    fn will_colide(&self, board: &Board, from : &Position, to: &Position) -> bool {

        let relative_position = RelativePosition::from_absolute(from, to);
        
        let is_diagonal = relative_position.file().abs() == relative_position.rank().abs();

        if is_diagonal{
            for i in from.file()+1..to.file() {
                let position = Position::new(i, i);
                let piece = board.get_piece_at(&position);
                if piece.is_some() {
                    return true;
                }
            }
        }

        let is_linear_rank = relative_position.file() == 0;

        if is_linear_rank {
            for i in from.rank()+1..to.rank() {
                let position = Position::new(from.file(), i);
                let piece = board.get_piece_at(&position);
                if piece.is_some() {
                    return true;
                }
            }
        }

        let is_linear_file = relative_position.rank() == 0;

        if is_linear_file {
            for i in from.file()+1..to.file() {
                let position = Position::new(i, from.rank());
                let piece = board.get_piece_at(&position);
                if piece.is_some() {
                    return true;
                }
            }
        }

        return false;
    }
}