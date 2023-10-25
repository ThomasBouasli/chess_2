use crate::board::relative_position::RelativePosition;

pub trait DiagonalMovement{
    fn diagonal_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();
        for file in -7..8 {
            for rank in -7..8 {
                if self.is_valid_diagonal_move(&RelativePosition::new(file, rank)) {
                    moves.push(RelativePosition::new(file, rank));
                }
            }
        }
        return moves;
    }

    fn is_valid_diagonal_move(&self, position: &RelativePosition) -> bool {
        return position.file().abs() == position.rank().abs() && position.file() != 0;
    }
}