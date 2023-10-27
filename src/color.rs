use std::fmt::Display;

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

impl Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}