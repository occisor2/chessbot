#![allow(dead_code)]
#![allow(unused)]

use bitboard::BitBoard;

pub mod bitboard;
pub mod fen;
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
    Black = 6,
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

#[derive(Debug)]
pub struct Board {
    side_to_move: Color,
    white_castle_rights: (bool, bool), // (queen side, king side)
    black_castle_rights: (bool, bool), // (queen side, king side)
    valid_en_passant: Option<u8>,      // index on board of valid square
    half_moves: u32,
    full_moves: u32,
    pieces: [BitBoard; 12],
}

impl Board {
    pub fn new() -> Self {
        Self {
            side_to_move: Color::White,
            white_castle_rights: (false, false),
            black_castle_rights: (false, false),
            valid_en_passant: None,
            half_moves: 0,
            full_moves: 0,
            pieces: [0.into(); 12], // even indexes are white, odd black
        }
    }

    fn get_pieces(&self, piece: Piece, color: Color) -> BitBoard {
        self.pieces[piece as usize + color as usize]
    }

    fn white_pieces(&self) -> BitBoard {
        let mut total = 0.into();
        for i in 0..6 {
            total |= self.pieces[i];
        }
        total
    }

    fn black_pieces(&self) -> BitBoard {
        let mut total = 0.into();
        for i in 6..self.pieces.len() {
            total |= self.pieces[i];
        }
        total
    }

    fn occupied(&self) -> BitBoard {
        self.white_pieces() | self.black_pieces()
    }

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
        writeln!(f, "{}", self.side_to_move)?;
        writeln!(f)?;
        for board in self.pieces {
            writeln!(f, "{board}")?;
        }
        Ok(())
    }
}
