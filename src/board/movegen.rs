use super::{BitBoard, Board};
use crate::{
    board::{Color, Piece},
    moves::Move,
};

const RANK8: u64 = 0xFF << 7 * 8;
const RANK7: u64 = 0xFF << 6 * 8;
const RANK6: u64 = 0xFF << 5 * 8;
const RANK5: u64 = 0xFF << 4 * 8;
const RANK4: u64 = 0xFF << 3 * 8;
const RANK3: u64 = 0xFF << 2 * 8;
const RANK2: u64 = 0xFF << 1 * 8;
const RANK1: u64 = 0xFF;

impl Board {
    pub fn gen_moves(&self) -> Vec<Move> {
        self.gen_pawns()
    }

    fn gen_pawns(&self) -> Vec<Move> {
        self.gen_pawn_pushes()
    }

    fn gen_pawn_pushes(&self) -> Vec<Move> {
        fn extract_moves(mut to: BitBoard, mut from: BitBoard) -> Vec<Move> {
            let mut moves = Vec::new();
            while to.trailing_zeros() < 64 {
                let to_index = to.trailing_zeros();
                let from_index = from.trailing_zeros();
                moves.push(Move::new(to_index, from_index, None));
                // Zero out these indexes to move onto the next pawn
                // position in the next loop iteration.
                to ^= 1 << to_index;
                from ^= 1 << from_index;
            }
            moves
        }

        let mut moves = Vec::new();

        if let Color::White = self.side_to_move {
            let pawns = self.get_pieces(Piece::Pawn, Color::White);
            let pushes = (pawns << 8) & self.empty() & !RANK8;
            let double_pushes = (pushes << 8) & self.empty() & RANK4;
            moves.extend(extract_moves(pushes, pawns));
            moves.extend(extract_moves(double_pushes, pawns));
        } else {
            let pawns = self.get_pieces(Piece::Pawn, Color::Black);
            let pushes = (pawns >> 8) & self.empty() & !RANK1;
            let double_pushes = (pushes >> 8) & self.empty() & RANK5;
            moves.extend(extract_moves(pushes, pawns));
            moves.extend(extract_moves(double_pushes, pawns));
        }

        moves
    }
}
