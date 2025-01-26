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
        let from_rank = self.from / 8 + 1;
        let from_file = self.from % 8;
        let to_rank = self.to / 8 + 1;
        let to_file = self.to % 8;
        format!(
            "{}{}{}{}",
            (from_file + b'a') as char,
            from_rank,
            (to_file + b'a') as char,
            to_rank
        )
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lan_str())
    }
}
