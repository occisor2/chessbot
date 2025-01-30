#![allow(dead_code)]
#![allow(unused)]

use derive_more::derive::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
    Shl, ShlAssign, Shr, ShrAssign,
};
use std::{
    fmt::Display,
    ops::{BitAnd, BitOrAssign, BitXorAssign, Shl},
};

/// Converts the name of board square to an index number.
///
/// `square` must be a valid square name (e.g. e4) or `None` will be
/// returned.
pub fn square_to_index(square: &str) -> Option<u8> {
    if square.chars().count() != 2 {
        return None;
    }

    let file_char = square.chars().nth(0)?;
    if let 'a'..='h' = file_char {
        let file = file_char as u8 - b'a';
        let rank = square.chars().nth(1)?.to_digit(10)?;
        if let 1..=8 = rank {
            let index = (rank - 1) * 8 + file as u32;
            return Some(index as u8);
        }
    }

    None
}

/// Converts a board index number into a board square name.
///
/// Squares are mapped using Little-Endian Rank-File mapping, so for
/// example, a1 would be at index 0 and a2 index 8.
pub fn index_to_square(index: u8) -> String {
    let rank = index / 8 + 1;
    let file = index % 8;
    format!("{}{}", (b'a' + file) as char, rank)
}

#[derive(
    Debug,
    Clone,
    Copy,
    From,
    Not,
    BitOr,
    BitOrAssign,
    BitAnd,
    BitAndAssign,
    BitXor,
    BitXorAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(number: u64) -> Self {
        Self(number)
    }

    pub fn set_index(&mut self, index: u8) {}

    pub fn set_rank_file(&mut self, rank: u8, file: u8) {
        let mask = 1 << (rank * 8 + file);
        *self |= mask;
    }

    pub fn trailing_zeros(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = 8 * rank + file;
                let bit = (self.0 >> index) & 1;
                if file == 0 {
                    write!(f, "{} ", rank + 1)?;
                }
                write!(f, "{} ", if bit == 1 { '1' } else { '0' })?;
            }
            writeln!(f)?;

            if rank == 0 {
                write!(f, "  ")?;
                for rank in 0..8 {
                    write!(f, "{} ", (b'a' + rank) as char)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl BitAnd<u64> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: u64) -> Self::Output {
        self & BitBoard::from(rhs)
    }
}

impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs
    }
}

impl BitXorAssign<u64> for BitBoard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}
