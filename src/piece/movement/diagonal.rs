use crate::board::relative_position::RelativePosition;

pub trait DiagonalMovement{
    fn diagonal_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();
        for file in -7..=7 {
            for rank in -7..=7 {
                let position = match RelativePosition::new(file, rank){
                    Ok(position) => position,
                    Err(_) => panic!("Invalid position")
                };

                if self.is_valid_diagonal_move(&position) {
                    moves.push(position);
                }
            }
        }
        return moves;
    }

    fn is_valid_diagonal_move(&self, position: &RelativePosition) -> bool {
        return position.file().abs() == position.rank().abs() && position.file() != 0;
    }
}