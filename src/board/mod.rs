use std::fmt::Display;

use crate::{piece::{Piece, pieces::{rook::Rook, king::King, knight::Knight, bishop::Bishop, queen::Queen, pawn::Pawn}, piece_factory}, color::Color, game::movement::Movement, game::castle_rights::CastleRights};

use self::{tile::Tile, position::Position, relative_position::RelativePosition};

mod tile;
pub mod position;
pub mod relative_position;

pub struct Board{
    tiles: Vec<Tile>,
    white_castle_rights: CastleRights,
    black_castle_rights: CastleRights,
    pawn_shadow: Option<(Position, Position)>,
}

impl Board {
    fn new(white_castle_rights: CastleRights, black_castle_rights: CastleRights) -> Board {
        let mut tiles = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                tiles.push(Tile::new(Position::new(x, y)));
            }
        }
        Board {
            tiles,
            white_castle_rights,
            black_castle_rights,
            pawn_shadow: None,
        }
    }

    pub fn new_classical() -> Board{
        let mut board = Board::new(
            CastleRights::Both,
            CastleRights::Both,
        );
        
        board.set_piece_at(Position::new(0, 0), Box::new(Rook::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(1, 0), Box::new(Knight::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(2, 0), Box::new(Bishop::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(3, 0), Box::new(King::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(4, 0), Box::new(Queen::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(5, 0), Box::new(Bishop::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(6, 0), Box::new(Knight::new(Color::White))).unwrap();
        board.set_piece_at(Position::new(7, 0), Box::new(Rook::new(Color::White))).unwrap();

        board.set_piece_at(Position::new(0, 7), Box::new(Rook::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(1, 7), Box::new(Knight::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(2, 7), Box::new(Bishop::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(3, 7), Box::new(King::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(4, 7), Box::new(Queen::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(5, 7), Box::new(Bishop::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(6, 7), Box::new(Knight::new(Color::Black))).unwrap();
        board.set_piece_at(Position::new(7, 7), Box::new(Rook::new(Color::Black))).unwrap();

        for x in 0..8 {
            board.set_piece_at(Position::new(x, 1), Box::new(Pawn::new(Color::White))).unwrap();
            board.set_piece_at(Position::new(x, 6), Box::new(Pawn::new(Color::Black))).unwrap();
        }

        return board;
    }

    pub fn from_movements(&self, movements : &Vec<Movement>) -> Result<Board, String> {
        let mut board = Board::new_classical();

        fn make_move(from : &Position, to: &Position, movement: &Movement, board: &mut Board) -> Result<(), String> {
            match board.move_piece(from.clone(),to.clone()){
                Ok(_) => {
                    board.check_for_en_passante(&movement);
                },
                Err(e) => {
                    println!("{}", e);
                    return Err(e)
                },
            };

            match movement.promotion() {
                Some(promotion) => {
                    match board.set_piece_at(to.clone(), piece_factory(promotion.prefix(), promotion.color().clone())){
                        Ok(_) => {},
                        Err(e) => return Err(e),
                    };

                    return Ok(());
                },
                None => return Ok(()),
            }
        }
        
        for movement in movements {
            match movement{
                Movement::Move(_, from, to, _) => make_move(from, to, movement, &mut board)?,
                Movement::Capture(_, from, to, _) => make_move(from, to, movement, &mut board)?,
                Movement::CastleKingSide(_) => board.castle_king_side(&movement)?,
                Movement::CastleQueenSide(_) => board.castle_queen_side(&movement)?,
            }
        }

        return Ok(board);
    }

    pub fn get_piece_at(&self, position: &Position) -> &Option<Box<dyn Piece>> {
        return match &self.tiles.iter().find(|tile| tile.position() == position){
            Some(tile) => tile.piece(),
            None => &None,
        }
    }

    pub fn revoke_castle_right(&mut self, color: &Color, castle_right: CastleRights) {
        match color {
            Color::White => self.white_castle_rights = self.white_castle_rights.revoke_right(castle_right),
            Color::Black => self.black_castle_rights = self.black_castle_rights.revoke_right(castle_right),
        }
    }

    pub fn get_castle_rights(&self, color: &Color) -> &CastleRights {
        return match color {
            Color::White => &self.white_castle_rights,
            Color::Black => &self.black_castle_rights,
        }
    }

    fn move_piece(&mut self, from: Position, to: Position) -> Result<(), String> {
        let piece = match self.remove_piece_at(from){
            Ok(piece) => piece,
            Err(e) => return Err(e),
        };

        match self.set_piece_at(to, piece){
            Ok(_) => {},
            Err(e) => return Err(e),
        };

        if let Some((shadow_location, piece_location)) = self.pawn_shadow {
            if shadow_location == to {
                match self.remove_piece_at(piece_location){
                    Ok(_) => {},
                    Err(e) => return Err(e),
                };
            }
        }

        return Ok(());
    }


    fn castle_king_side(&mut self, movement: &Movement) -> Result<(), String> {
        self.has_castle_rights(&movement.piece().color(), &CastleRights::KingSide)?;

        let rank = match movement.piece().color() {
            Color::White => 0,
            Color::Black => 7,
        };

        match self.move_piece(Position::new(4, rank), Position::new(6, rank)){
            Ok(king) => king,
            Err(e) => return Err(e),
        };


        match self.move_piece(Position::new(7, rank), Position::new(5, rank)){
            Ok(rook) => rook,
            Err(e) => return Err(e),
        };

        return Ok(());
    }

    fn castle_queen_side(&mut self, movement: &Movement) -> Result<(), String> {
        self.has_castle_rights(&movement.piece().color(), &CastleRights::QueenSide)?;

        let rank = match movement.piece().color() {
            Color::White => 0,
            Color::Black => 7,
        };

        match self.move_piece(Position::new(4, rank), Position::new(2, rank)){
            Ok(king) => king,
            Err(e) => return Err(e),
        };

        match self.move_piece(Position::new(0, rank), Position::new(3, rank)){
            Ok(rook) => rook,
            Err(e) => return Err(e),
        };

        return Ok(());
    }

    fn set_piece_at(&mut self, position: Position, piece: Box<dyn Piece>) -> Result<(), String> {
        let tile = match self.tiles.iter_mut().find(|tile| tile.position() == &position){
            Some(tile) => tile,
            None => return Err(String::from(format!("No tile at position {}", position))),
        };

        tile.set_piece(piece);

        return Ok(());
    }

    fn remove_piece_at(&mut self, position: Position) -> Result<Box<dyn Piece>, String> {
        let tile = match self.tiles.iter_mut().find(|tile| tile.position() == &position){
            Some(tile) => tile,
            None => return Err(String::from(format!("No tile at position {}", position))),
        };

        return tile.remove_piece();
    }

    fn has_castle_rights(&self, color: &Color, castle_right: &CastleRights) -> Result<(), String> {
        let rights = self.get_castle_rights(color);
        if rights == &CastleRights::None {
            return Err(String::from("No castle rights"));
        }
        if rights != castle_right {
            return Err(String::from("No castle rights"));
        }
        return Ok(());
    }

    fn check_for_en_passante(&mut self, movement: &Movement) {
        if movement.piece().prefix() != Pawn::prefix(){
            self.pawn_shadow = None;
            return;
        }

        let from = match movement.from(){
            Some(from) => from,
            None => return,
        };

        let to = match movement.to(){
            Some(to) => to,
            None => return,
        };

        let relative_position = RelativePosition::from_absolute(&from, &to);

        if Pawn::is_double_move(&relative_position) {

            let rank = match movement.piece().color() {
                Color::White => 2,
                Color::Black => 5,
            };

            let shadow_position = Position::new(to.file(), rank);

            self.pawn_shadow = Some((shadow_position, to.clone()));
        } else {
            self.pawn_shadow = None;
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..8 {
            for x in 0..8 {
                let position = Position::new(x, y);
                let tile = self.tiles.iter().find(|tile| tile.position() == &position).unwrap();
                if (x + y) % 2 == 0 {
                    board.push_str(&format!("{}", tile));
                } else {
                    board.push_str(&format!("{}", tile));
                }
            }
            board.push_str("\n");
        }
        return write!(f, "{}", board);
    }
}