use bitboard::{BitBoard, Square};

mod attacks;
pub mod bitboard;
pub mod fen;
mod magic;
pub mod movegen;

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black = 7,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if matches!(self, Color::White) {
            write!(f, "white")
        } else {
            write!(f, "black")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CastleRights {
    pub king: bool,
    pub queen: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    side_to_move: Color,
    white_castle_rights: CastleRights,
    black_castle_rights: CastleRights,
    valid_en_passant: Option<Square>,
    half_moves: u32,
    full_moves: u32,
    /// First 6 entrys are white's piece bitboards,
    /// the 7th is a mask of all the white pieces on the board.
    /// The next 6 entrys are black's piece bitboards,
    /// and the last is a mask of all of black's pieces.
    pieces: [BitBoard; 14],
}

impl Board {
    pub fn new() -> Self {
        Self {
            side_to_move: Color::White,
            white_castle_rights: CastleRights {
                king: false,
                queen: false,
            },
            black_castle_rights: CastleRights {
                king: false,
                queen: false,
            },
            valid_en_passant: None,
            half_moves: 0,
            full_moves: 0,
            pieces: [0.into(); 14],
        }
    }

    /// Return the bitboard of piece kind `piece` for `color`
    fn piece(&self, piece: Piece, color: Color) -> BitBoard {
        self.pieces[piece as usize + color as usize]
    }

    /// Returns a mask of all white's pieces
    fn white_pieces(&self) -> BitBoard {
        let mut total = 0.into();
        for i in 0..6 {
            total |= self.pieces[i];
        }
        total
    }

    /// Returns a mask of all black's pieces
    fn black_pieces(&self) -> BitBoard {
        let mut total = 0.into();
        for i in 6..self.pieces.len() {
            total |= self.pieces[i];
        }
        total
    }

    /// Returns a mask of all friendly pieces of [side_to_move][Self::side_to_move]
    fn friendly(&self) -> BitBoard {
        self.pieces[self.side_to_move as usize + 6]
    }

    /// Returns a mask of all pieces on the board
    fn occupied(&self) -> BitBoard {
        self.white_pieces() | self.black_pieces()
    }

    /// Returns a mask of all the empty squares on the board
    fn empty(&self) -> BitBoard {
        !self.occupied()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "side: {}", self.side_to_move)?;
        writeln!(
            f,
            "white castle: {}, {}",
            self.white_castle_rights.queen, self.white_castle_rights.king
        )?;
        writeln!(
            f,
            "black castle: {}, {}",
            self.black_castle_rights.queen, self.black_castle_rights.king
        )?;
        writeln!(
            f,
            "en passant: {}",
            // ugly solution but Square doesn't have an empty varaint, and it shouldn't
            if self.valid_en_passant.is_none() {
                "-".to_string()
            } else {
                self.valid_en_passant.unwrap().to_string()
            }
        )?;
        writeln!(f, "half moves: {}", self.half_moves)?;
        writeln!(f, "full moves: {}", self.full_moves)?;
        writeln!(f)?;
        for board in self.pieces {
            writeln!(f, "{board}")?;
        }
        Ok(())
    }
}
