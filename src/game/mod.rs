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

    fn move_piece(&mut self, from: Position, to: Position) -> Result<(), String>{
        let piece = match self.board.get_piece_at(&from){
            Some(piece) => piece,
            None => return Err(String::from("No piece at that position")),
        };

        match self.board.get_piece_at(&to){
            Some(_) => return Err(String::from("Cannot move to a position with a piece")),
            None => (),
        };

        if piece.color() != &self.turn {
            return Err(String::from("Cannot move opponent's piece"));
        }

        if !piece.is_valid_play(&&RelativePosition::from_absolute(&from, &to)){
            return Err(String::from("Invalid move"));
        }

        if piece.will_colide(&self.board, &from, &to){
            return Err(String::from("Invalid move"));
        }

        return Ok(());
    }

    fn capture_piece(&mut self, from: Position, to: Position) -> Result<(), String>{
        let from_piece = match self.board.get_piece_at(&from){
            Some(piece) => piece,
            None => return Err(String::from("No piece at that position")),
        };

        match self.board.get_piece_at(&to){
            Some(piece) => {
                if piece.color() == &self.turn {
                    return Err(String::from("Cannot capture your own piece"));
                }
            },
            None => return Err(String::from("No piece at that position")),
        };

        if from_piece.color() != &self.turn {
            return Err(String::from("Cannot move opponent's piece"));
        }

        if !from_piece.is_valid_play(&&RelativePosition::from_absolute(&from, &to)){
            return Err(String::from("Invalid move"));
        }

        if from_piece.will_colide(&self.board, &from, &to){
            return Err(String::from("Invalid move"));
        }

        return Ok(());
    }

    fn castle(&mut self, movement: &Movement) -> Result<(), String>{

        let rank = match movement.piece().color() {
            Color::White => 1,
            Color::Black => 8,
        };

        match movement {
            Movement::CastleKingSide(_) => {
                if movement.piece().will_colide(&self.board, &Position::new(4, rank), &Position::new(6, rank)){
                    return Err(String::from("Invalid move"));
                }
            },
            Movement::CastleQueenSide(_) => {
                if movement.piece().will_colide(&self.board, &Position::new(5, rank), &Position::new(4, rank)){
                    return Err(String::from("Invalid move"));
                }
            },
            _ => return Err(String::from("Invalid castle")),
        }

        return Ok(());
    }

    fn on_move(&mut self){
        self.change_castle_rights();
    }

    fn change_castle_rights(&mut self){
        let last_movement = self.movements.last().unwrap();

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
                    return Err(String::from("Invalid promotion"));
                }

                if promotion.color() != piece.color() {
                    return Err(String::from("Invalid promotion"));
                }

                if promotion.prefix() == King::prefix() || promotion.prefix() == Pawn::prefix() {
                    return Err(String::from("Invalid promotion"));
                }

                if to.rank() != 7 && to.rank() != 0 {
                    return Err(String::from("Invalid promotion"));
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

        let movement = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(1, 1), Position::new(1, 2), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let result = game.play(movement);

        println!("{}", game.board);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn cannot_castle_through_pieces_play(){
        let mut game = Game::new_classical();

        let movement = Movement::CastleKingSide(Box::new(King::new(Color::White)));

        let result = game.play(movement);

        println!("{}", game.board);

        assert_eq!(result, Err(String::from("Invalid move")));
        assert_eq!(game.board.get_castle_rights(&Color::White), &CastleRights::Both);
    }

    #[test]
    fn can_en_passante(){
        let mut game = Game::new_classical();

        let m1 = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(1, 1), Position::new(1, 3), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };
        
        let m2 = match Movement::new_move(Box::new(Pawn::new(Color::Black)), Position::new(7, 6), Position::new(7, 4), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m3 = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(1, 3), Position::new(1, 4), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m4 = match Movement::new_move(Box::new(Pawn::new(Color::Black)), Position::new(2, 6), Position::new(2, 4), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        let m5 = match Movement::new_move(Box::new(Pawn::new(Color::White)), Position::new(1, 4), Position::new(2, 5), None){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };


        let result = game.play(m1);
        assert_eq!(result, Ok(()));
        let result = game.play(m2);
        assert_eq!(result, Ok(()));
        let result = game.play(m3);
        assert_eq!(result, Ok(()));
        let result = game.play(m4);
        assert_eq!(result, Ok(()));
        let result = game.play(m5);
        assert_eq!(result, Ok(()));
    }
}