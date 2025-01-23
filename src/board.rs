#![allow(dead_code)]
#![allow(unused)]

use crate::bitboard::BitBoard;

#[derive(Debug)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug)]
enum Color {
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
    half_moves: u32,
    full_moves: u32,
    pieces: [BitBoard; 12],
}

impl Board {
    pub fn new() -> Self {
        Self {
            side_to_move: Color::White,
            half_moves: 0,
            full_moves: 0,
            pieces: [0.into(); 12], // even indexes are white, odd black
        }
    }

    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Board::new();
        let mut add_piece = |piece: Piece, color: Color, rank, file| {
            let index = piece as usize + color as usize;
            board.pieces[index].set_bit(rank, file);
        };

        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.len() != 6 {
            return None;
        }

        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return None;
        }

        for (rank, rank_chars) in ranks.iter().rev().enumerate() {
            let mut file: u8 = 0;
            for ch in rank_chars.chars() {
                if file > 7 {
                    return None;
                }
                match ch {
                    'k' => add_piece(Piece::King, Color::Black, rank as u8, file),
                    'q' => add_piece(Piece::Queen, Color::Black, rank as u8, file),
                    'r' => add_piece(Piece::Rook, Color::Black, rank as u8, file),
                    'b' => add_piece(Piece::Bishop, Color::Black, rank as u8, file),
                    'n' => add_piece(Piece::Knight, Color::Black, rank as u8, file),
                    'p' => add_piece(Piece::Pawn, Color::Black, rank as u8, file),
                    'K' => add_piece(Piece::King, Color::White, rank as u8, file),
                    'Q' => add_piece(Piece::Queen, Color::White, rank as u8, file),
                    'R' => add_piece(Piece::Rook, Color::White, rank as u8, file),
                    'B' => add_piece(Piece::Bishop, Color::White, rank as u8, file),
                    'N' => add_piece(Piece::Knight, Color::White, rank as u8, file),
                    'P' => add_piece(Piece::Pawn, Color::White, rank as u8, file),
                    '1'..='8' => file += ch.to_digit(10)? as u8,
                    _ => return None,
                }
                if !ch.is_numeric() {
                    file += 1;
                }
            }
        }

        board.side_to_move = if parts[1].chars().next().unwrap() == 'w' {
            Color::White
        } else {
            Color::Black
        };

        Some(board)
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
