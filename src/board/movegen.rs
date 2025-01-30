use super::{
    attacks::{KING_ATTACKS, KNIGHT_ATTACKS},
    bitboard::{BitBoard, FILEA, FILEH, RANK1, RANK4, RANK5, RANK8},
    Board,
};
use crate::{
    board::{Color, Piece},
    moves::Move,
};

fn extract_moves(moves: &mut Vec<Move>, mut board: BitBoard, start: u8) {
    while board != 0 {
        let index = board.trailing_zeros();
        moves.push(Move::new(index.into(), start.into(), None));
        board.clear_bit(index);
    }
}

impl Board {
    pub fn gen_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        self.gen_king(&mut moves);
        self.gen_knights(&mut moves);
        self.gen_pawns(&mut moves);
        moves
    }

    fn gen_king(&self, moves: &mut Vec<Move>) {
        let mut king = self.piece(Piece::King, self.side_to_move);
        while king != 0 {
            let index = king.trailing_zeros();
            let attacks = KING_ATTACKS[index as usize];
            let valid_attacks = attacks & !self.friendly();
            extract_moves(moves, valid_attacks, index);
            king.clear_bit(index);
        }
    }

    fn gen_knights(&self, moves: &mut Vec<Move>) {
        let mut knights = self.piece(Piece::Knight, self.side_to_move);
        while knights != 0 {
            let index = knights.trailing_zeros();
            let attacks = KNIGHT_ATTACKS[index as usize];
            let valid_attacks = attacks & !self.friendly();
            extract_moves(moves, valid_attacks, index);
            knights.clear_bit(index);
        }
    }

    fn gen_pawns(&self, moves: &mut Vec<Move>) {
        self.gen_pawn_pushes(moves);
        self.gen_pawn_captures(moves);
    }

    fn gen_pawn_pushes(&self, moves: &mut Vec<Move>) {
        fn extract_moves(moves: &mut Vec<Move>, mut board: BitBoard, offset: u8, color: Color) {
            while board != 0 {
                let index = board.trailing_zeros();
                let start_index = if let Color::White = color {
                    index - 8 * offset
                } else {
                    index + 8 * offset
                };
                moves.push(Move::new(index.into(), start_index.into(), None));
                board.clear_bit(index);
            }
        }

        if let Color::White = self.side_to_move {
            let pawns = self.piece(Piece::Pawn, Color::White);
            let pushes = (pawns << 8) & self.empty() & !RANK8;
            let double_pushes = (pushes << 8) & self.empty() & RANK4;
            extract_moves(moves, pushes, 1, Color::White);
            extract_moves(moves, double_pushes, 2, Color::White);
        } else {
            let pawns = self.piece(Piece::Pawn, Color::Black);
            let pushes = (pawns >> 8) & self.empty() & !RANK1;
            let double_pushes = (pushes >> 8) & self.empty() & RANK5;
            extract_moves(moves, pushes, 1, Color::Black);
            extract_moves(moves, double_pushes, 2, Color::Black);
        }
    }

    fn gen_pawn_captures(&self, moves: &mut Vec<Move>) {
        #[derive(Clone, Copy, Debug)]
        enum Direction {
            Right,
            Left,
        }

        fn extract_moves(moves: &mut Vec<Move>, mut board: BitBoard, dir: Direction, color: Color) {
            while board != 0 {
                let index = board.trailing_zeros();
                let start_index = match (dir, color) {
                    (Direction::Left, Color::White) => index - 9,
                    (Direction::Right, Color::White) => index - 7,
                    (Direction::Left, Color::Black) => index + 9,
                    (Direction::Right, Color::Black) => index + 7,
                };
                moves.push(Move::new(index.into(), start_index.into(), None));
                board.clear_bit(index);
            }
        }

        if let Color::White = self.side_to_move {
            let pawns = self.piece(Piece::Pawn, Color::White);
            // from white's pov
            let captures_left = (pawns << 9) & self.black_pieces() & !RANK8 & !FILEH;
            let captures_right = (pawns << 7) & self.black_pieces() & !RANK8 & !FILEA;
            extract_moves(moves, captures_left, Direction::Left, Color::White);
            extract_moves(moves, captures_right, Direction::Right, Color::White);
        } else {
            let pawns = self.piece(Piece::Pawn, Color::Black);
            // from black's pov
            let captures_left = (pawns >> 9) & self.white_pieces() & !RANK1 & !FILEA;
            let captures_right = (pawns >> 7) & self.white_pieces() & !RANK1 & !FILEH;
            extract_moves(moves, captures_left, Direction::Left, Color::Black);
            extract_moves(moves, captures_right, Direction::Right, Color::Black);
        }
    }

    fn gen_pawn_promotions(&self, moves: &mut Vec<Move>) -> Vec<Move> {
        todo!()
    }
}
