use std::fmt::Display;

use crate::{board::position::Position, piece::Piece};

pub enum Movement{
    CastleKingSide(Box<dyn Piece>),
    CastleQueenSide(Box<dyn Piece>),
    Move(Box<dyn Piece>, Position, Position, Option<Box<dyn Piece>>),
    Capture(Box<dyn Piece>, Position, Position,Option<Box<dyn Piece>>),
}

impl Movement {
    pub fn new_move(piece: Box<dyn Piece>, from: Position, to: Position, promotion : Option<Box<dyn Piece>>) -> Option<Movement> {
        if !Movement::is_moving(&from, &to) {
            return None;
        }

        return Some(Movement::Move(piece, from, to, promotion));
    }

    pub fn new_capture(piece: Box<dyn Piece>, from: Position, to: Position, promotion : Option<Box<dyn Piece>>) -> Option<Movement> {
        if !Movement::is_moving(&from, &to) {
            return None;
        }

        return Some(Movement::Capture(piece, from, to, promotion));
    }

    pub fn piece(&self) -> &Box<dyn Piece> {
        return match self {
            Movement::Move(piece, _, _, _) => piece,
            Movement::Capture(piece, _, _, _) => piece,
            Movement::CastleKingSide(piece) => piece,
            Movement::CastleQueenSide(piece) => piece,
        }
    }

    pub fn from(&self) -> Option<&Position> {
        return match self {
            Movement::Move(_, from, _, _) => Some(&from),
            Movement::Capture(_, from, _, _) => Some(&from),
            _ => None,
        }
    }

    pub fn to(&self) -> Option<&Position> {
        return match self {
            Movement::Move(_, _, to, _) => Some(&to),
            Movement::Capture(_, _, to, _) => Some(&to),
            _ => None,
        }
    }

    pub fn promotion(&self) -> Option<&Box<dyn Piece>> {
        return match self {
            Movement::Move(_, _, _, promotion) => promotion.as_ref(),
            Movement::Capture(_, _, _, promotion) => promotion.as_ref(),
            _ => None,
        }
    }

    fn is_moving(from : &Position, to: &Position) -> bool {
        return from != to;
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Movement::Move(piece, from, to, promotion) => {
                let promotion = match promotion {
                    Some(promotion) => format!("={}", promotion.prefix()),
                    None => String::from(""),
                };
                write!(f, "{}{}{}{}", piece.prefix(), from, to, promotion)
            },
            Movement::Capture(piece, from, to, promotion) => {
                let promotion = match promotion {
                    Some(promotion) => format!("={}", promotion.prefix()),
                    None => String::from(""),
                };
                write!(f, "{}{}x{}{}", piece.prefix(), from, to, promotion)
            },
            Movement::CastleKingSide(_) => write!(f, "O-O"),
            Movement::CastleQueenSide(_) => write!(f, "O-O-O"),
        }
    }
}