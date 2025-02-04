use super::{Board, CastleRights, Color, Piece};

impl Board {
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Board::new();
        let mut add_piece = |piece: Piece, color: Color, rank, file| {
            let index = piece as usize + color as usize;
            board.pieces[index].set_rank_file(rank, file);
        };
        // Split apart the fen string
        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.len() != 6 {
            return None;
        }
        // Parse piece positions
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

        board.pieces[Color::White as usize + 6] = board.white_pieces();
        board.pieces[Color::Black as usize + 6] = board.black_pieces();

        // Parse side to move
        board.side_to_move = if parts[1].chars().next().unwrap() == 'w' {
            Color::White
        } else {
            Color::Black
        };
        // Parse castling rights
        let mut white_castle_rights = CastleRights {
            king: false,
            queen: false,
        };
        let mut black_castle_rights = white_castle_rights;

        if parts[2] != "-" {
            for right in parts[2].chars() {
                match right {
                    'q' => black_castle_rights.queen = true,
                    'k' => black_castle_rights.king = true,
                    'Q' => white_castle_rights.queen = true,
                    'K' => white_castle_rights.king = true,
                    _ => return None,
                }
            }
        }

        board.white_castle_rights = white_castle_rights;
        board.black_castle_rights = black_castle_rights;
        // Parse En Passant target
        board.valid_en_passant = if parts[3] == "-" {
            None
        } else {
            Some(parts[3].parse().ok()?)
        };
        // Parse half and full time
        board.half_moves = parts[4].parse().ok()?;
        board.full_moves = parts[5].parse().ok()?;

        Some(board)
    }
}
