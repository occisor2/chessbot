use super::BitBoard;
use crate::{
    board::{Board, Color, Piece},
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

const KNIGHT_ATTACKS: [BitBoard; 64] = gen_knight_attacks();

impl Board {
    pub fn gen_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        self.gen_pawns(&mut moves);
        moves
    }

    fn gen_pawns(&self, moves: &mut Vec<Move>) {
        self.gen_pawn_pushes(moves);
        self.gen_pawn_captures(moves);
    }

    fn gen_pawn_pushes(&self, moves: &mut Vec<Move>) {
        fn extract(moves: &mut Vec<Move>, mut board: BitBoard, offset: u8, color: Color) {
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
        }

        if let Color::White = self.side_to_move {
            let pawns = self.get_pieces(Piece::Pawn, Color::White);
            let pushes = (pawns << 8) & self.empty() & !RANK8;
            let double_pushes = (pushes << 8) & self.empty() & RANK4;
            extract(moves, pushes, 1, Color::White);
            extract(moves, double_pushes, 2, Color::White);
        } else {
            let pawns = self.get_pieces(Piece::Pawn, Color::Black);
            let pushes = (pawns >> 8) & self.empty() & !RANK1;
            let double_pushes = (pushes >> 8) & self.empty() & RANK5;
            extract(moves, pushes, 1, Color::Black);
            extract(moves, double_pushes, 2, Color::Black);
        }
    }

    fn gen_pawn_captures(&self, moves: &mut Vec<Move>) {
        #[derive(Clone, Copy, Debug)]
        enum Direction {
            Right,
            Left,
        }

        fn extract(moves: &mut Vec<Move>, mut board: BitBoard, dir: Direction, color: Color) {
            while board != 0 {
                let index = board.trailing_zeros();
                let start_index = match (dir, color) {
                    (Direction::Left, Color::White) => index - 9,
                    (Direction::Right, Color::White) => index - 7,
                    (Direction::Left, Color::Black) => index + 9,
                    (Direction::Right, Color::Black) => index + 7,
                };
                moves.push(Move::new(index, start_index, None));
                board.clear_bit(index);
            }
        }

        if let Color::White = self.side_to_move {
            let pawns = self.get_pieces(Piece::Pawn, Color::White);
            // from white's pov
            let captures_left = (pawns << 9) & self.black_pieces() & !RANK8 & !FILEH;
            let captures_right = (pawns << 7) & self.black_pieces() & !RANK8 & !FILEA;
            extract(moves, captures_left, Direction::Left, Color::White);
            extract(moves, captures_right, Direction::Right, Color::White);
        } else {
            let pawns = self.get_pieces(Piece::Pawn, Color::Black);
            // from black's pov
            let captures_left = (pawns >> 9) & self.white_pieces() & !RANK1 & !FILEA;
            let captures_right = (pawns >> 7) & self.white_pieces() & !RANK1 & !FILEH;
            extract(moves, captures_left, Direction::Left, Color::Black);
            extract(moves, captures_right, Direction::Right, Color::Black);
        }
    }

    fn gen_pawn_promotions(&self, moves: &mut Vec<Move>) -> Vec<Move> {
        todo!()
    }

    fn gen_knights(&self, moves: &mut Vec<Move>) {
        todo!()
    }
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
