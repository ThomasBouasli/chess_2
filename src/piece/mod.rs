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
    fn will_colide(&self, board: &Board, from : &Position, to: &Position) -> Result<(), String>{

        let relative_position = match RelativePosition::from_absolute(from, to){
            Ok(relative_position) => relative_position,
            Err(_) => panic!("Invalid relative position")
        };
        
        let is_diagonal = relative_position.file().abs() == relative_position.rank().abs();

        if is_diagonal{
            let file_signum = relative_position.file().signum();
            let rank_signum = relative_position.rank().signum();

            for i in 1..relative_position.file().abs() {

                let file : u8 = (from.file() as i8 + i * file_signum) as u8;
                let rank : u8 = (from.rank() as i8 + i * rank_signum) as u8;

                let position = match Position::new(file, rank){
                    Ok(position) => position,
                    Err(e) => return Err(e)
                };

                let piece = board.get_piece_at(&position);
                
                match piece {
                    Some(piece) => return Err(format!("Piece {} at {} will colide with {} at {} to move to {}", self.name(), position, piece.name(), from, to)),
                    None => {}
                }
            }
        }

        let is_linear_rank = relative_position.file() == 0 && relative_position.rank() != 0 || relative_position.file() != 0 && relative_position.rank() == 0;

        if is_linear_rank{
            let file_signum = relative_position.file().signum();
            let rank_signum = relative_position.rank().signum();

            for i in 1..relative_position.file().abs() + relative_position.rank().abs() {

                let file : u8 = (from.file() as i8 + i * file_signum) as u8;
                let rank : u8 = (from.rank() as i8 + i * rank_signum) as u8;

                let position = match Position::new(file, rank){
                    Ok(position) => position,
                    Err(e) => return Err(e)
                };

                let piece = board.get_piece_at(&position);
                
                match piece {
                    Some(piece) => return Err(format!("{} at {} will colide with {} at {} to move to {}", self.name(), from, piece.name(), position, to)),
                    None => {}
                }
            }
        }
        
        return Ok(());
    }
}