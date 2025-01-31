use super::bitboard::{BitBoard, Square, FILEA, FILEH, RANK1, RANK8};

pub const KNIGHT_ATTACKS: [BitBoard; 64] = gen_knight_attacks();
pub const KING_ATTACKS: [BitBoard; 64] = gen_king_attacks();

/// Get all moves for a king on `square`
///
/// `occ`, occupancy, should be a [Bitboard] of all pieces on the board.
pub fn get_king_attacks(square: Square, occ: BitBoard) -> BitBoard {
    todo!()
}

/// Get all moves for a queen on `square`
///
/// `occ`, occupancy, should be a [Bitboard] of all pieces on the board.
pub fn get_queen_attacks(square: Square, occ: BitBoard) -> BitBoard {
    todo!()
}

/// Get all moves for a rook on `square`
///
/// `occ`, occupancy, should be a [Bitboard] of all pieces on the board.
pub fn get_rook_attacks(square: Square, occ: BitBoard) -> BitBoard {
    todo!()
}

/// Get all moves for a bishop on `square`
///
/// `occ`, occupancy, should be a [Bitboard] of all pieces on the board.
pub fn get_bishop_attacks(square: Square, occ: BitBoard) -> BitBoard {
    todo!()
}

/// Get all moves for a knight on `square`
///
/// `occ`, occupancy, should be a [Bitboard] of all pieces on the board.
pub fn get_knight_attacks(square: Square, occ: BitBoard) -> BitBoard {
    todo!()
}

const fn gen_knight_attacks() -> [BitBoard; 64] {
    let mut boards: [BitBoard; 64] = [BitBoard::new(0); 64];

    let mut i = 0;
    while i < 64 {
        let knights = 1 << i;
        // This is an implementation based of an algorithm presented on the chessprogramming wiki
        // https://www.chessprogramming.org/Knight_Pattern
        let l1 = (knights >> 1) & (0x7f7f7f7f7f7f7f7f);
        let l2 = (knights >> 2) & (0x3f3f3f3f3f3f3f3f);
        let r1 = (knights << 1) & (0xfefefefefefefefe);
        let r2 = (knights << 2) & (0xfcfcfcfcfcfcfcfc);
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        let board = (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);
        boards[i] = BitBoard::new(board);
        i += 1;
    }

    boards
}

const fn gen_king_attacks() -> [BitBoard; 64] {
    let mut boards: [BitBoard; 64] = [BitBoard::new(0); 64];

    let mut i = 0;
    while i < 64 {
        let king = 1 << i;
        let mut moves = 0;
        moves ^= (king << 1) & !FILEH;
        moves ^= (king >> 1) & !FILEA;
        moves ^= (king << 8) & !RANK1;
        moves ^= (king >> 8) & !RANK8;
        moves ^= (king << 9) & !(RANK1 | FILEH);
        moves ^= (king << 7) & !(RANK1 | FILEA);
        moves ^= (king >> 9) & !(RANK8 | FILEA);
        moves ^= (king >> 7) & !(RANK8 | FILEH);
        boards[i] = BitBoard::new(moves);
        i += 1;
    }

    boards
}
