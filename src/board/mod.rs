#![allow(dead_code)]
#![allow(unused)]

use bitboard::BitBoard;

pub mod bitboard;
pub mod fen;
pub mod movegen;

pub fn square_to_index(square: &str) -> Option<u8> {
    if square.chars().count() != 2 {
        return None;
    }

    let file_char = square.chars().nth(0)?;
    if let 'a'..='h' = file_char {
        let file = file_char as u8 - b'a';
        let rank = square.chars().nth(1)?.to_digit(10)? - 1;
        if let 0..7 = rank {
            let index = rank * 8 + file as u32;
            return Some(index as u8);
        }
    }

    None
}

pub fn index_to_square(square: u8) -> String {
    let rank = square / 8 + 1;
    let file = square % 8;
    format!("{}{}", (file + b'a') as char, rank)
}

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
        writeln!(f, "side {}", self.side_to_move)?;
        writeln!(
            f,
            "white castle {}, {}",
            self.white_castle_rights.0, self.white_castle_rights.1
        )?;
        writeln!(
            f,
            "black castle {}, {}",
            self.black_castle_rights.0, self.black_castle_rights.1
        )?;
        writeln!(
            f,
            "en passant {}",
            if self.valid_en_passant.is_none() {
                "-".to_string()
            } else {
                index_to_square(self.valid_en_passant.unwrap())
            }
        )?;
        writeln!(f, "half moves {}", self.half_moves)?;
        writeln!(f, "full moves {}", self.full_moves)?;
        writeln!(f)?;
        for board in self.pieces {
            writeln!(f, "{board}")?;
        }
        Ok(())
    }
}
