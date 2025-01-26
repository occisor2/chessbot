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

const FILEA: u64 = 0x8080808080808080;
const FILEH: u64 = FILEA >> 7;

impl Board {
    pub fn gen_moves(&self) -> Vec<Move> {
        self.gen_pawns()
    }

    fn gen_pawns(&self) -> Vec<Move> {
        //self.gen_pawn_pushes()
        self.gen_pawn_captures()
    }

    fn gen_pawn_pushes(&self) -> Vec<Move> {
        fn extract_moves(mut board: BitBoard, offset: u8, color: Color) -> Vec<Move> {
            let mut moves = Vec::new();
            while board != 0 {
                let index = board.trailing_zeros();
                let start_index = if let Color::White = color {
                    index - 8 * offset
                } else {
                    index + 8 * offset
                };
                moves.push(Move::new(index, start_index, None));
                board.clear_bit(index);
            }
            moves
        }

        let mut moves = Vec::new();
        if let Color::White = self.side_to_move {
            let pawns = self.get_pieces(Piece::Pawn, Color::White);
            let pushes = (pawns << 8) & self.empty() & !RANK8;
            let double_pushes = (pushes << 8) & self.empty() & RANK4;
            moves.extend(extract_moves(pushes, 1, Color::White));
            moves.extend(extract_moves(double_pushes, 2, Color::White));
        } else {
            let pawns = self.get_pieces(Piece::Pawn, Color::Black);
            let pushes = (pawns >> 8) & self.empty() & !RANK1;
            let double_pushes = (pushes >> 8) & self.empty() & RANK5;
            moves.extend(extract_moves(pushes, 1, Color::Black));
            moves.extend(extract_moves(double_pushes, 2, Color::Black));
        }

        moves
    }

    fn gen_pawn_captures(&self) -> Vec<Move> {
        enum Direction {
            Right,
            Left,
        }

        fn extract_moves(mut board: BitBoard, dir: Direction, color: Color) -> Vec<Move> {
            let mut moves = Vec::new();
            while board != 0 {
                let index = board.trailing_zeros();
                let start_index = match (&dir, &color) {
                    (Direction::Left, Color::White) => index - 9,
                    (Direction::Right, Color::White) => index - 7,
                    (Direction::Left, Color::Black) => index + 9,
                    (Direction::Right, Color::Black) => index + 7,
                };
                moves.push(Move::new(index, start_index, None));
                board.clear_bit(index);
            }
            moves
        }

        let mut moves = Vec::new();
        if let Color::White = self.side_to_move {
            let pawns = self.get_pieces(Piece::Pawn, Color::White);
            // from white's pov
            let captures_left = (pawns << 9) & self.black_pieces() & !RANK8 & !FILEH;
            let captures_right = (pawns << 7) & self.black_pieces() & !RANK8 & !FILEA;
            moves.extend(extract_moves(captures_left, Direction::Left, Color::White));
            moves.extend(extract_moves(
                captures_right,
                Direction::Right,
                Color::White,
            ));
        } else {
            let pawns = self.get_pieces(Piece::Pawn, Color::Black);
            // from black's pov
            let captures_left = (pawns >> 9) & self.white_pieces() & !RANK1 & !FILEA;
            let captures_right = (pawns >> 7) & self.white_pieces() & !RANK1 & !FILEH;
            moves.extend(extract_moves(captures_left, Direction::Left, Color::Black));
            moves.extend(extract_moves(
                captures_right,
                Direction::Right,
                Color::Black,
            ));
        }

        moves
    }

    fn gen_pawn_promotions(&self) -> Vec<Move> {
        todo!()
    }
}
