use crate::{color::Color, board::{Board, relative_position::RelativePosition, position::Position}, piece::{pieces::{king::King, rook::Rook, pawn::Pawn}, Piece}};

use self::{movement::Movement, castle_rights::CastleRights};

pub mod movement;
pub mod castle_rights;

pub struct Game{
    turn: Color,
    board: Board,
    movements: Vec<Movement>,
}

impl Game{
    pub fn new_classical() -> Game{
        return Game{
            turn: Color::White,
            board: Board::new_classical(),
            movements: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board{
        &self.board
    }

    pub fn play(&mut self, movement: Movement) -> Result<(), String> {

        match movement {
            Movement::Move(_, from, to, _) => self.move_piece(from, to)?,
            Movement::Capture(_, from, to, _) => self.capture_piece(from, to)?,
            Movement::CastleKingSide(_) => self.castle(&movement)?,
            Movement::CastleQueenSide(_) => self.castle(&movement)?,
        }

        self.movements.push(movement);

        match self.board.from_movements(&self.movements){
            Ok(board) => self.board = board,
            Err(e) => {
                self.movements.pop();
                return Err(e)
            },
        }

        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.on_move();

        return Ok(());
    }

    pub fn is_legal(&self, movement: Movement) -> bool {

        match movement {
            Movement::Move(_, from, to, _) => match self.move_piece(from, to){
                Err(_) => return false,
                _ => (),
            },
            Movement::Capture(_, from, to, _) => match self.capture_piece(from, to){
                Err(_) => return false,
                _ => (),
            },
            Movement::CastleKingSide(_) => match self.castle(&movement){
                Err(_) => return false,
                _ => (),
            },
            Movement::CastleQueenSide(_) => match self.castle(&movement){
                Err(_) => return false,
                _ => (),
            },
        }

        return true;
    }

    fn move_piece(&self, from: Position, to: Position) -> Result<(), String>{
        let piece = match self.board.get_piece_at(&from){
            Some(piece) => piece,
            None => return Err(String::from(format!("Cannot move from an empty tile! There is no piece at {}", from))),
        };

        match self.board.get_piece_at(&to){
            Some(piece) => return Err(String::from(format!("Cannot move to an occupied tile! There is already a {} {} at {}", piece.color(), piece.name(), to))),
            None => (),
        };

        if piece.color() != &self.turn {
            return Err(String::from(format!("Cannot move opponent's {}. It's {} turn and the selected piece is {}", piece.name(), self.turn, piece.color())));
        }

        let position = match RelativePosition::from_absolute(&from, &to){
            Ok(position) => position,
            Err(e) => return Err(e),
        };

        if !piece.is_valid_move(&position){
            return Err(String::from(format!("Invalid move for {} {}, this type of piece cannot move like that!", piece.color(), piece.name())));
        }

        match piece.will_colide(&self.board, &from, &to){
            Err(e) => return Err(e),
            _ => (),
        }

        return Ok(());
    }

    fn capture_piece(&self, from: Position, to: Position) -> Result<(), String>{
        let from_piece = match self.board.get_piece_at(&from){
            Some(piece) => piece,
            None => return Err(String::from(format!("Cannot move from an empty tile! There is no piece at {}", from))),
        };

        match self.board.get_piece_at(&to){
            Some(piece) => {
                if piece.color() == &self.turn {
                    return Err(String::from(format!("Cannot capture your own piece! There is a {} {} at {}", piece.color(), piece.name(), to)));
                }
            },
            None => return Err(String::from(format!("Cannot capture an empty tile! There is no piece at {}", to))),
        };

        if from_piece.color() != &self.turn {
            return Err(String::from(format!("Cannot move opponent's {}. It's {} turn and the selected piece is {}", from_piece.name(), self.turn, from_piece.color())));
        }

        let position = match RelativePosition::from_absolute(&from, &to){
            Ok(position) => position,
            Err(e) => return Err(e),
        };

        if !from_piece.is_valid_capture(&position){
            return Err(String::from(format!("Invalid capture for {} {}, this type of piece cannot capture like that!", from_piece.color(), from_piece.name())));
        }

        match from_piece.will_colide(&self.board, &from, &to){
            Err(e) => return Err(e),
            _ => (),
        }

        return Ok(());
    }

    fn castle(&self, movement: &Movement) -> Result<(), String>{
        let rank = match movement.piece().color() {
            Color::White => 1,
            Color::Black => 8,
        };

        match movement {
            Movement::CastleKingSide(_) => {
                let from = Position::new(5, rank).unwrap();
                let to = Position::new(7, rank).unwrap();

                match movement.piece().will_colide(&self.board, &from, &to){
                    Err(e) => return Err(e),
                    _ => (),
                }
            },
            Movement::CastleQueenSide(_) => {
                let from = Position::new(5, rank).unwrap();
                let to = Position::new(3, rank).unwrap();

                match movement.piece().will_colide(&self.board, &from, &to){
                    Err(e) => return Err(e),
                    _ => (),
                }
            },
            _ => return Err(String::from(format!("Invalid castle movement {}", movement))),
        }

        return Ok(());
    }

    fn on_move(&mut self){
        self.change_castle_rights();
    }

    fn change_castle_rights(&mut self){
        let last_movement = match self.movements.last(){
            Some(movement) => movement,
            None => return,
        };

        if last_movement.piece().prefix() == King::prefix() {
            self.board.revoke_castle_right(&Color::opposite(&self.turn), CastleRights::Both);
        }

        if last_movement.piece().prefix() == Rook::prefix(){
            let is_king_side = match last_movement.from() {
                Some(from) => from.file() == 8,
                None => false,
            };

            if is_king_side {
                self.board.revoke_castle_right(&Color::opposite(&self.turn), CastleRights::KingSide);
            }

            let is_queen_side = match last_movement.from() {
                Some(from) => from.file() == 1,
                None => false,
            };

            if is_queen_side {
                self.board.revoke_castle_right(&Color::opposite(&self.turn), CastleRights::QueenSide);
            }
        }
    }

    fn check_promotion(piece : &Box<dyn Piece>, to : &Position, promotion: Option<&Box<dyn Piece>>) -> Result<(), String>{
        match promotion {
            Some(promotion) => {
                if piece.prefix() != Pawn::prefix() {
                    return Err(String::from(format!("A {} cannot be promoted! Only a {}", piece.name(), Pawn::prefix())));
                }

                if promotion.color() != piece.color() {
                    return Err(String::from(format!("Cannot promote to a different color! {} is {} and {} is {}", piece, piece.color(), promotion, promotion.color())));
                }

                if promotion.prefix() == King::prefix() || promotion.prefix() == Pawn::prefix() {
                    return Err(String::from(format!("Cannot promote to a {}! Only Rooks, Knights, Bishops and Queens are allowed", promotion.name())));
                }

                if to.rank() != 8 && to.rank() != 1 {
                    return Err(String::from(format!("Cannot promote {} in this location! Only on rank 1 or 8", piece.name())));
                }

                return Ok(());
            },
            None => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::{piece::{pieces::pawn::Pawn, Piece}, board::position::Position};

    use super::*;

    #[test]
    fn can_play(){
        let mut game = Game::new_classical();

        let movement = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(1, 2).unwrap(), Position::new(1, 3).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let result = game.play(movement);

        assert_eq!(result, Ok(()), "\n{}", game.board);
    }

    #[test]
    fn cannot_castle_through_pieces_play(){
        let mut game = Game::new_classical();

        let movement = Movement::CastleKingSide(Box::new(King::new(Color::White)));

        let result = game.play(movement);

        assert!(result.is_err(), "\n{}", game.board);
        assert_eq!(game.board.get_castle_rights(&Color::White), &CastleRights::Both, "\n{}", game.board);
    }

    #[test]
    fn can_en_passante(){
        let mut game = Game::new_classical();

        let m1 = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(4, 2).unwrap(), Position::new(4, 4).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };
        
        let m2 = match Movement::new_move(Box::new(Pawn::new(Color::Black)), Position::new(5, 7).unwrap(), Position::new(5, 5).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m3 = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(4, 4).unwrap(), Position::new(4, 5).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m4 = match Movement::new_move(Box::new(Pawn::new(Color::Black)), Position::new(3, 7).unwrap(), Position::new(3, 5).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m5 = match Movement::new_capture(Box::new(Pawn::new(Color::White)), Position::new(4, 5).unwrap(), Position::new(3, 6).unwrap(), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };


        let result = game.play(m1);
        assert_eq!(result, Ok(()), "\n{}", game.board);
        let result = game.play(m2);
        assert_eq!(result, Ok(()), "\n{}", game.board);
        let result = game.play(m3);
        assert_eq!(result, Ok(()), "\n{}", game.board);
        let result = game.play(m4);
        assert_eq!(result, Ok(()), "\n{}", game.board);
        let result = game.play(m5);
        assert_eq!(result, Ok(()), "\n{}", game.board);
    }
}