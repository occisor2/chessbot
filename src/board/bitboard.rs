#![allow(dead_code)]
#![allow(unused)]

use derive_more::derive::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
    Shl, ShlAssign, Shr, ShrAssign,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    fmt::Display,
    ops::{BitAnd, BitOrAssign, BitXorAssign, Shl},
    str::FromStr,
};

pub const RANK8: u64 = 0xFF << 7 * 8;
pub const RANK7: u64 = 0xFF << 6 * 8;
pub const RANK6: u64 = 0xFF << 5 * 8;
pub const RANK5: u64 = 0xFF << 4 * 8;
pub const RANK4: u64 = 0xFF << 3 * 8;
pub const RANK3: u64 = 0xFF << 2 * 8;
pub const RANK2: u64 = 0xFF << 1 * 8;
pub const RANK1: u64 = 0xFF;

pub const FILEA: u64 = 0x8080808080808080;
pub const FILEH: u64 = FILEA >> 7;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let index = *self as u8;
        let rank = index / 8 + 1;
        let file = index % 8;
        write!(f, "{}{}", (b'a' + file) as char, rank)
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            return Err(());
        }

        let file_char = s.chars().nth(0).ok_or(())?;
        if let 'a'..='h' = file_char {
            let file = file_char as u8 - b'a';
            let rank = s.chars().nth(1).ok_or(())?.to_digit(10).ok_or(())?;
            if let 1..=8 = rank {
                let index = (rank - 1) * 8 + file as u32;
                return FromPrimitive::from_u8(index as u8).ok_or(());
            }
        }

        Err(())
    }
}

impl From<u8> for Square {
    fn from(value: u8) -> Self {
        // cannot fail since u8 has only 64 possible values (same as enum variants)
        FromPrimitive::from_u8(value).unwrap()
    }
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

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= (1 << index);
    }

    pub fn set_square(&mut self, square: Square) {
        self.set_bit(square as u8);
    }

    pub fn set_rank_file(&mut self, rank: u8, file: u8) {
        let mask = 1 << (rank * 8 + file);
        *self |= mask;
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn clear_square(&mut self, square: Square) {
        self.0 &= !(1 << square as u8);
    }

    pub fn trailing_zeros(&self) -> u8 {
        self.0.trailing_zeros() as u8
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
