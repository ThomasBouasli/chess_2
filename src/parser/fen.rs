use crate::board::Board;

pub struct Fen{
    fen: String,
}

impl Fen {
    pub fn new(fen: String) -> Fen {
        Fen {
            fen,
        }
    }

    pub fn to_board(&self) -> Board{
        Board::new()
    }
}