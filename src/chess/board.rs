use super::piece::{Color, Piece, Type};
use super::r#move::{CastleMove, ChessMove, EnPassantMove, NormalMove, PromotionMove};

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub u8, pub u8);

impl Position {
    pub fn is_valid(&self) -> bool {
        self.0 < 8 && self.1 < 8
    }
}

#[derive(Clone)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().rev().enumerate() {
            write!(f, "{} | ", 8 - i)?;
            for piece in row {
                match piece {
                    Some(p) => write!(f, "{} ", p)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   ----------------")?;
        writeln!(f, "    a b c d e f g h")?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        const INITIAL_BOARD: [[Option<Piece>; 8]; 8] = [
            [
                Some(Piece(Color::White, Type::Rook)),
                Some(Piece(Color::White, Type::Knight)),
                Some(Piece(Color::White, Type::Bishop)),
                Some(Piece(Color::White, Type::Queen)),
                Some(Piece(Color::White, Type::King)),
                Some(Piece(Color::White, Type::Bishop)),
                Some(Piece(Color::White, Type::Knight)),
                Some(Piece(Color::White, Type::Rook)),
            ],
            [Some(Piece(Color::White, Type::Pawn)); 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [Some(Piece(Color::Black, Type::Pawn)); 8],
            [
                Some(Piece(Color::Black, Type::Rook)),
                Some(Piece(Color::Black, Type::Knight)),
                Some(Piece(Color::Black, Type::Bishop)),
                Some(Piece(Color::Black, Type::Queen)),
                Some(Piece(Color::Black, Type::King)),
                Some(Piece(Color::Black, Type::Bishop)),
                Some(Piece(Color::Black, Type::Knight)),
                Some(Piece(Color::Black, Type::Rook)),
            ],
        ];
        Board(INITIAL_BOARD)
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        let Position(x, y) = position;
        self.0[x as usize][y as usize]
    }

    fn set_piece(&mut self, position: Position, piece: Option<Piece>) {
        let Position(x, y) = position;
        self.0[x as usize][y as usize] = piece;
    }

    pub fn is_position_empty(&self, position: Position) -> bool {
        self.get_piece(position).is_none()
    }

    pub fn is_position_color(&self, position: Position, color: Color) -> bool {
        self.get_piece(position)
            .map_or(false, |piece| piece.0 == color)
    }

    pub fn is_position_empty_or_color(&self, position: Position, color: Color) -> bool {
        self.is_position_empty(position) || self.is_position_color(position, color)
    }

    pub fn apply_move(&mut self, mv: &ChessMove) {
        match mv {
            ChessMove::Normal(mv) => self.apply_normal_move(mv),
            ChessMove::EnPassant(mv) => self.apply_en_passant_move(mv),
            ChessMove::Promotion(mv) => self.apply_promotion_move(mv),
            ChessMove::Castle(mv) => self.apply_castle_move(mv),
        }
    }

    pub fn apply_move_copy(&self, mv: &ChessMove) -> Board {
        let mut new_board = self.clone();
        new_board.apply_move(mv);
        new_board
    }

    fn apply_normal_move(&mut self, mv: &NormalMove) {
        self.set_piece(mv.to, self.get_piece(mv.from));
        self.set_piece(mv.from, None);
    }

    fn apply_en_passant_move(&mut self, en_passant: &EnPassantMove) {
        self.apply_normal_move(&en_passant.pawn);
        self.set_piece(en_passant.position, None);
    }

    fn apply_promotion_move(&mut self, promotion: &PromotionMove) {
        self.set_piece(promotion.pawn.from, Some(promotion.to));
        self.apply_normal_move(&promotion.pawn);
    }

    fn apply_castle_move(&mut self, castle: &CastleMove) {
        self.apply_normal_move(&castle.king);
        self.apply_normal_move(&castle.rook);
    }
}
