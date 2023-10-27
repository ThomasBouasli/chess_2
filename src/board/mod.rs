use std::fmt::Display;

use colored::Colorize;

use crate::{piece::{Piece, pieces::{rook::Rook, king::King, knight::Knight, bishop::Bishop, queen::Queen, pawn::Pawn}, piece_factory}, color::Color, game::{movement::Movement, Game}, game::castle_rights::CastleRights};

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
        for x in 1..=8 {
            for y in 1..=8 {
                tiles.push(Tile::new(Position::new(x, y).unwrap()));
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
        
        board.set_piece_at(Position::new(1, 1).unwrap(), Box::new(Rook::new(Color::White)));
        board.set_piece_at(Position::new(2, 1).unwrap(), Box::new(Knight::new(Color::White)));
        board.set_piece_at(Position::new(3, 1).unwrap(), Box::new(Bishop::new(Color::White)));
        board.set_piece_at(Position::new(4, 1).unwrap(), Box::new(Queen::new(Color::White)));
        board.set_piece_at(Position::new(5, 1).unwrap(), Box::new(King::new(Color::White)));
        board.set_piece_at(Position::new(6, 1).unwrap(), Box::new(Bishop::new(Color::White)));
        board.set_piece_at(Position::new(7, 1).unwrap(), Box::new(Knight::new(Color::White)));
        board.set_piece_at(Position::new(8, 1).unwrap(), Box::new(Rook::new(Color::White)));

        board.set_piece_at(Position::new(1, 8).unwrap(), Box::new(Rook::new(Color::Black)));
        board.set_piece_at(Position::new(2, 8).unwrap(), Box::new(Knight::new(Color::Black)));
        board.set_piece_at(Position::new(3, 8).unwrap(), Box::new(Bishop::new(Color::Black)));
        board.set_piece_at(Position::new(4, 8).unwrap(), Box::new(Queen::new(Color::Black)));
        board.set_piece_at(Position::new(5, 8).unwrap(), Box::new(King::new(Color::Black)));
        board.set_piece_at(Position::new(6, 8).unwrap(), Box::new(Bishop::new(Color::Black)));
        board.set_piece_at(Position::new(7, 8).unwrap(), Box::new(Knight::new(Color::Black)));
        board.set_piece_at(Position::new(8, 8).unwrap(), Box::new(Rook::new(Color::Black)));

        for x in 1..=8 {
            board.set_piece_at(Position::new(x, 2).unwrap(), Box::new(Pawn::new(Color::White)));
            board.set_piece_at(Position::new(x, 7).unwrap(), Box::new(Pawn::new(Color::Black)));
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
                    board.set_piece_at(to.clone(), piece_factory(promotion.prefix(), promotion.color().clone()));
                },
                _ => {},
            }

            return Ok(())
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
        if let Some((shadow_location, piece_location)) = self.pawn_shadow {
            if shadow_location == *position {
                return self.get_piece_at(&piece_location);
            }
        }

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

    pub fn get_position_by_movement(&self, game: &Game, piece: &Box<dyn Piece>, to: &Position, is_capture : bool, rank_or_file : Option<char>, movement_string : &str) -> Position{

        enum AmbiguityChar{
            File(char),
            Rank(u8),
        }

        let ambiguity_char : Option<AmbiguityChar> = match rank_or_file{
            Some(c) => {
                match c.is_digit(10){
                    true => Some(AmbiguityChar::Rank(c.to_digit(10).unwrap() as u8)),
                    false => Some(AmbiguityChar::File(c)),
                }
            },
            _ => None,
        };

        let pieces = self.tiles.iter().filter(|tile| {
            match tile.piece() {
                Some(p) => {
                    if p.prefix() == piece.prefix() && p.color() == piece.color() {
                        return true;
                    }
                },
                None => {},
            }
            return false;
        });

        let mut deduced_from = None;

        for tile in pieces {
            let found_piece = match tile.piece(){
                Some(piece) => {
                    if piece.prefix() == piece.prefix() && piece.color() == piece.color() {
                        piece
                    }else{
                        continue;
                    }
                },
                None => continue,
            };

            let possible_moves = match is_capture{
                true => found_piece.possible_captures(),
                false => found_piece.possible_moves(),
            };

            for relative_position in possible_moves {
                let position = match Position::from_relative(tile.position().clone(), relative_position.clone()){
                    Ok(position) => position,
                    Err(_) => continue,
                };
  
                if position == *to {
                    let would_be_movement = match is_capture{
                        true => Movement::Capture(piece_factory(found_piece.prefix(), found_piece.color().clone()), tile.position().clone(), position.clone(), None),
                        false => Movement::Move(piece_factory(found_piece.prefix(), found_piece.color().clone()), tile.position().clone(), position.clone(), None),
                    };


                    if !game.is_legal(would_be_movement){
                        continue;
                    }

                    let currently_deduced_from = tile.position().clone();

                    match ambiguity_char{
                        Some(AmbiguityChar::File(file)) => {
                            if currently_deduced_from.file_char() != file {
                                continue;
                            }
                        },
                        Some(AmbiguityChar::Rank(rank)) => {
                            if currently_deduced_from.rank() != rank {
                                continue;
                            }
                        },
                        None => {},
                    }

                    if deduced_from.is_some() {
                        panic!("Ambiguous move: {}! Both {} and {} can move to {}", movement_string, deduced_from.unwrap(), currently_deduced_from, to);
                    }

                    deduced_from = Some(currently_deduced_from);
                }
            }
        }

        return match deduced_from{
            Some(from) => from,
            None => panic!("No piece found for move"),
        }
    }

    fn move_piece(&mut self, from: Position, to: Position) -> Result<(), String> {
        let piece = match self.remove_piece_at(from){
            Ok(piece) => piece,
            Err(e) => return Err(e),
        };

        self.set_piece_at(to, piece);

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
            Color::White => 1,
            Color::Black => 8,
        };

        let king_from = Position::new(5, rank).unwrap();
        let king_to = Position::new(7, rank).unwrap();

        match self.move_piece(king_from, king_to){
            Ok(king) => king,
            Err(e) => return Err(e),
        };


        let rook_from = Position::new(8, rank).unwrap();
        let rook_to = Position::new(6, rank).unwrap();

        match self.move_piece(rook_from, rook_to){
            Ok(rook) => rook,
            Err(e) => return Err(e),
        };

        return Ok(());
    }

    fn castle_queen_side(&mut self, movement: &Movement) -> Result<(), String> {
        self.has_castle_rights(&movement.piece().color(), &CastleRights::QueenSide)?;

        let rank = match movement.piece().color() {
            Color::White => 1,
            Color::Black => 8,
        };

        let king_from = Position::new(5, rank).unwrap();
        let king_to = Position::new(3, rank).unwrap();

        match self.move_piece(king_from, king_to){
            Ok(king) => king,
            Err(e) => return Err(e),
        };

        let rook_from = Position::new(1, rank).unwrap();
        let rook_to = Position::new(4, rank).unwrap();

        match self.move_piece(rook_from, rook_to){
            Ok(rook) => rook,
            Err(e) => return Err(e),
        };

        return Ok(()); 
    }

    fn set_piece_at(&mut self, position: Position, piece: Box<dyn Piece>){
        let tile = match self.tiles.iter_mut().find(|tile| tile.position() == &position){
            Some(tile) => tile,
            None => panic!("Cannot set piece on a non existing tile! Could not find tile at {}", position),
        };

        tile.set_piece(piece);
    }

    fn remove_piece_at(&mut self, position: Position) -> Result<Box<dyn Piece>, String> {
        let tile = match self.tiles.iter_mut().find(|tile| tile.position() == &position){
            Some(tile) => tile,
            None => panic!("Cannot remove piece on a non existing tile! Could not find tile at {}", position),
        };

        return tile.remove_piece();
    }

    fn has_castle_rights(&self, color: &Color, castle_right: &CastleRights) -> Result<(), String> {
        let rights = self.get_castle_rights(color);

        if rights == &CastleRights::None {
            return Err(String::from(format!("No castle rights for {}", color)));
        }

        if rights != castle_right  && rights != &CastleRights::Both {
            return Err(String::from(format!("{} does not have castle rights for {}! Only {} castle rights.", color, castle_right, rights)));
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

        let relative_position = match RelativePosition::from_absolute(&from, &to){
            Ok(relative_position) => relative_position,
            Err(_) => return,
        };

        if Pawn::is_double_move(&relative_position) {

            let rank = match movement.piece().color() {
                Color::White => 3,
                Color::Black => 6,
            };

            let shadow_position = match Position::new(to.file(), rank){
                Ok(position) => position,
                Err(_) => return,
            };

            self.pawn_shadow = Some((shadow_position, to.clone()));
        } else {
            self.pawn_shadow = None;
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        board.push_str("  abcdefgh\n");

        for file in (1..=8).rev() {
            board.push_str(&format!("{} ", file));

            for rank in 1..=8{
                let position = match Position::new(rank, file){
                    Ok(position) => position,
                    Err(_) => continue,
                };

                let tile = match self.tiles.iter().find(|tile| tile.position() == &position){
                    Some(tile) => tile,
                    None => continue,
                };

                if (rank + file) % 2 == 0 {
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