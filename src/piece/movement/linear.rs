use crate::board::relative_position::RelativePosition;

/// Trait for pieces that can move linearly.
pub trait LinearMovement{
    /// Returns a vector of all possible linear moves for a piece.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use chess::board::relative_position::RelativePosition;
    /// use chess::piece::movement::linear::LinearMovement;
    /// 
    /// struct DummyPiece;
    /// 
    /// impl LinearMovement for DummyPiece{}
    /// 
    /// let dummy_piece = DummyPiece{};
    /// 
    /// let moves = dummy_piece.linear_moves();
    /// 
    /// 
    /// // There are 28 possible linear moves for a piece.
    /// // 7 moves for each of the 4 lines.
    /// 
    /// assert_eq!(moves.len(), 28);
    /// ``` 
    /// 
    fn linear_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();
        for file in -7..=7 {
            for rank in -7..=7 {
                let position = match RelativePosition::new(file, rank){
                    Ok(position) => position,
                    Err(_) => panic!("Invalid position")
                };

                if self.is_valid_linear_move(&position) {
                    moves.push(position);
                }
            }
        }
        return moves;
    }

    /// Returns true if the given position is a valid linear move for a piece.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chess::board::relative_position::RelativePosition;
    /// use chess::piece::movement::linear::LinearMovement;
    /// 
    /// struct DummyPiece;
    /// 
    /// impl LinearMovement for DummyPiece{}
    /// 
    /// let dummy_piece = DummyPiece{};
    /// 
    /// let movement = RelativePosition::new(1, 0).unwrap();
    /// 
    /// assert_eq!(dummy_piece.is_valid_linear_move(&movement), true);
    /// ```
    fn is_valid_linear_move(&self, position: &RelativePosition) -> bool {
        return (position.file() == 0 && position.rank() != 0) || (position.file() != 0 && position.rank() == 0);
    }
}