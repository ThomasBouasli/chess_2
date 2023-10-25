#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color{
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        return match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}