use std::fmt::Display;

use crate::piece::Piece;

use super::position::Position;

pub struct Tile{
    position: Position,
    piece: Option<Box<dyn Piece>>,
}

impl Tile {
    pub fn new(position: Position) -> Tile {
        Tile {
            position,
            piece: None,
        }
    }

    pub fn position(&self) -> &Position {
        return &self.position;
    }

    pub fn piece(&self) -> &Option<Box<dyn Piece>> {
        return &self.piece;
    }

    pub fn set_piece(&mut self, piece: Box<dyn Piece>) {
        self.piece = Some(piece);
    }

    pub fn remove_piece(&mut self) -> Result<Box<dyn Piece>, String> {
        return match self.piece.take() {
            Some(piece) => Ok(piece),
            None => Err(String::from("Tile is empty")),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self.piece {
            Some(piece) => write!(f, "{}", piece),
            None => write!(f, " "),
        }
    }
}