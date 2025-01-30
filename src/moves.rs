use crate::board::bitboard::Square;

#[derive(Clone, Copy, Debug)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub to: Square,
    pub from: Square,
    pub promotion: Option<Promotion>,
}

impl Move {
    pub fn new(to: Square, from: Square, promotion: Option<Promotion>) -> Self {
        Move {
            to,
            from,
            promotion,
        }
    }

    pub fn lan_str(&self) -> String {
        format!("{}{}", self.from, self.to)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lan_str())
    }
}
