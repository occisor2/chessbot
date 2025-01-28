use crate::board;

#[derive(Debug)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug)]
pub struct Move {
    pub to: u8,   // board index
    pub from: u8, // board index
    pub promotion: Option<Promotion>,
}

impl Move {
    pub fn new(to: u8, from: u8, promotion: Option<Promotion>) -> Self {
        Move {
            to,
            from,
            promotion,
        }
    }

    pub fn lan_str(&self) -> String {
        let from_square = board::index_to_square(self.from);
        let to_square = board::index_to_square(self.to);
        format!("{}{}", from_square, to_square)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lan_str())
    }
}
