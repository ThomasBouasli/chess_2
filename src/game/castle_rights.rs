use std::fmt::Display;


#[derive(PartialEq, Eq, Debug)]
pub enum CastleRights{
    QueenSide,
    KingSide,
    Both,
    None,
}

impl CastleRights {
    pub fn revoke_right(&self, right: CastleRights) -> CastleRights {
        match self {
            CastleRights::QueenSide => {
                match right {
                    CastleRights::QueenSide => return CastleRights::None,
                    CastleRights::KingSide => return CastleRights::KingSide,
                    CastleRights::Both => return CastleRights::KingSide,
                    CastleRights::None => return CastleRights::QueenSide,
                }
            },
            CastleRights::KingSide => {
                match right {
                    CastleRights::QueenSide => return CastleRights::QueenSide,
                    CastleRights::KingSide => return CastleRights::None,
                    CastleRights::Both => return CastleRights::QueenSide,
                    CastleRights::None => return CastleRights::KingSide,
                }
            },
            CastleRights::Both => {
                match right {
                    CastleRights::QueenSide => return CastleRights::KingSide,
                    CastleRights::KingSide => return CastleRights::QueenSide,
                    CastleRights::Both => return CastleRights::None,
                    CastleRights::None => return CastleRights::Both,
                }
            },
            CastleRights::None => return CastleRights::None,
        }
    }
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastleRights::QueenSide => write!(f, "QueenSide"),
            CastleRights::KingSide => write!(f, "KingSide"),
            CastleRights::Both => write!(f, "Both"),
            CastleRights::None => write!(f, "None"),
        }
    }
}