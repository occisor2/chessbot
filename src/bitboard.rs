#![allow(dead_code)]
#![allow(unused)]

use derive_more::derive::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
    Shl, ShlAssign, Shr, ShrAssign,
};
use std::fmt::Display;

#[derive(
    Debug,
    Clone,
    Copy,
    Deref,
    DerefMut,
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
    pub fn set_bit(&mut self, rank: u8, file: u8) {
        let mask = 1 << (rank * 8 + file);
        *self |= mask.into();
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
