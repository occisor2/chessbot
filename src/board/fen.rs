use super::{Board, Color, Piece};

impl Board {
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Board::new();
        let mut add_piece = |piece: Piece, color: Color, rank, file| {
            let index = piece as usize + color as usize;
            board.pieces[index].set_square(rank, file);
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
            let mut file = 0;
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
