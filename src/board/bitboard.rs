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
    pub fn set_index(&mut self, index: u8) {}

    pub fn set_square(&mut self, rank: u8, file: u8) {
        let mask = 1 << (rank * 8 + file);
        *self |= mask;
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
                write!(f, "{} ", if bit == 1 { '1' } else { '0' })?;
            }
            writeln!(f)?;
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
