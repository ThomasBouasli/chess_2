use regex::Regex;

use crate::{game::{Game, movement::Movement}, color::Color, piece::{pieces::king::King, Piece, piece_factory}, board::position::Position};



pub fn from_pgn(pgn: &str) -> Game {
    unimplemented!()
}

fn clean_string(pgn: &str) -> String {
    let regex = Regex::new(r"\+").unwrap();
    let pgn = regex.replace_all(pgn, "");

    return pgn.to_string();
}

fn get_turn_strings(pgn: &str) -> Vec<String> {
    let regex = Regex::new(r"\d+\.").unwrap();

    let turns = regex.split(pgn).map(|s| s.to_string().trim().to_string()).filter(|s| s.len() > 0).collect();

    return turns;
}

fn get_movement_string(turn: &str) -> (String, String) {
    let regex = Regex::new(r"\s+").unwrap();

    let moves : Vec<String> = regex.split(turn).map(|s| s.to_string().trim().to_string()).collect();

    return (moves[0].clone(), moves[1].clone());
}

fn get_movement(game: &mut Game,movement_string: &str, color: Color) -> Option<Movement> {
    let mut disambiguation : Option<&str> = None;
    let mut prefix : Option<&str> = None;
    let mut destination : Option<&str> = None;
    let mut capture : Option<&str> = None;
    let mut promotion : Option<&str> = None;

    if movement_string == "O-O" {
        return Some(Movement::CastleKingSide(Box::new(King::new(color))))
    }

    if movement_string == "O-O-O" {
        return Some(Movement::CastleQueenSide(Box::new(King::new(color))))
    }


    if movement_string.contains("=") {
        promotion = Some(&movement_string[movement_string.len() - 1..movement_string.len()]);
    }

    if let Some(c) = movement_string.chars().nth(0) {
        if c.is_uppercase() {
            prefix = Some(&movement_string[0..1]);
            disambiguation = Some(&movement_string[1..movement_string.len()]);
        }else{
            prefix = Some("P");
        }
    }


    if movement_string.chars().nth(movement_string.len() - 1).is_some(){

        if promotion.is_some(){
            destination = Some(&movement_string[movement_string.len() - 4..movement_string.len() - 2]);
            match disambiguation{
                Some(prev) => {
                    disambiguation = Some(&prev[0..prev.len() - 4]);
                },
                None => {
                    disambiguation = Some(&movement_string[0..movement_string.len() - 4]);
                },
            }
        }else{
            destination = Some(&movement_string[movement_string.len() - 2..movement_string.len()]);
            match disambiguation{
                Some(prev) => {
                    disambiguation = Some(&prev[0..prev.len() - 2]);
                },
                None => {
                    disambiguation = Some(&movement_string[0..movement_string.len() - 2]);
                },
            }
        }
    }

    if movement_string.contains("x") {
        capture = Some("x");
        match disambiguation{
            Some(prev) => {
                disambiguation = Some(&prev[0..prev.len() - 1]);
            },
            None => {
                destination = Some(&movement_string[0..movement_string.len() - 1]);
            },
        }
    }



    let mut movement_gen : Option<Movement> = None;


    let disambiguation_char : Option<char> = match disambiguation {
        Some(disambiguation) => {
            match disambiguation.chars().nth(0){
                Some(c) => Some(c),
                None => None,
            }
        },
        None => None,
    };

    let to = Position::from_string(destination.unwrap()).unwrap();
    let from = game.board().get_position_by_movement(game ,&piece_factory(prefix.unwrap(), color), &to, capture.is_some(), disambiguation_char, movement_string);



    match capture {
        Some(_) => movement_gen = Movement::new_capture(piece_factory(prefix.unwrap(), color),from, to, None),
        None => movement_gen = Movement::new_move(piece_factory(prefix.unwrap(), color),from, to, None),
    }


    return movement_gen;
}


#[test]
fn test_pgn(){
    let pgn = String::from("1.e4 e5 2.Nf3 Nc6 3.Bb5 a6 4.Ba4 Nf6 5.O-O Be7 6.d4 exd4 7.e5 Ne4 8.Nxd4 O-O
    9.Nf5 d5 10.Bxc6 bxc6 11.Nxe7+ Qxe7 12.Re1 Re8 13.f3 Nd6 14.Bf4 Nf5 15.Qd2 Rb8
    16.b3 Rb4 17.c3 Rb6 18.Qf2 c5 19.Nd2 Bb7 20.Nf1 d4 21.Ng3 Nh4 22.Ne4 Bxe4
    23.Rxe4 Ng6 24.Bd2 Re6 25.f4 Qd7 26.cxd4 f5 27.d5 fxe4 28.dxe6 Qxe6 29.Qxc5 Rd8
    30.Be3 Rd3 31.Re1 Kh8 32.Rf1 Qe7 33.e6 Kg8 34.f5 Qxc5 35.Bxc5 Ne5 36.f6 gxf6
    37.e7 Kf7 38.e8=Q Kg7");

    let cleaned = clean_string(&pgn);

    let turns = get_turn_strings(&cleaned);

    let moves_per_turn = turns.iter().map(|turn| get_movement_string(turn)).collect::<Vec<(String, String)>>();

    let mut game = Game::new_classical();

    for (index, (white_move, black_move)) in moves_per_turn.iter().enumerate() {
        let m1 = match get_movement(&mut game, &white_move, Color::White){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        
        assert_eq!(game.play(m1).is_ok(), true, "Invalid movement {} on turn {}", white_move, index + 1);

        // println!("{}", game.board());

        let m2 = match get_movement(&mut game, &black_move, Color::Black){
            Some(movement) => movement,
            None => panic!("Invalid movement"),
        };

        assert_eq!(game.play(m2).is_ok(), true, "Invalid movement {} on turn {}", black_move, index + 1);

        // println!("{}", game.board());
    }

    // assert_eq!(1,2);
}