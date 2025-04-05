use super::board::Position;
use super::piece::*;

#[derive(Clone)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl CastlingRights {
    pub fn new() -> CastlingRights {
        CastlingRights {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }

    pub fn disable_king_side(&mut self, color: &Color) {
        match color {
            Color::White => self.white_king = false,
            Color::Black => self.black_king = false,
        }
    }

    pub fn disable_queen_side(&mut self, color: &Color) {
        match color {
            Color::White => self.white_queen = false,
            Color::Black => self.black_queen = false,
        }
    }

    pub fn disable_both_sides(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.white_king = false;
                self.white_queen = false;
            }
            Color::Black => {
                self.black_king = false;
                self.black_queen = false;
            }
        }
    }
}

#[derive(Clone)]
pub enum Move {
    Standard(StandardMove),
    PawnDoubleAdvance(PawnDoubleAdvanceMove),
    PawnEnPassant(PawnEnPassantMove),
    PawnPromotion(PawnPromotionMove),
    Castling(CastlingMove),
}

impl Move {
    pub fn all_pawn_promotions(pawn: &StandardMove, color: &Color) -> Vec<Move> {
        vec![
            Move::PawnPromotion(PawnPromotionMove {
                pawn: pawn.clone(),
                promotion: Piece(color.clone(), PieceType::Rook),
            }),
            Move::PawnPromotion(PawnPromotionMove {
                pawn: pawn.clone(),
                promotion: Piece(color.clone(), PieceType::Knight),
            }),
            Move::PawnPromotion(PawnPromotionMove {
                pawn: pawn.clone(),
                promotion: Piece(color.clone(), PieceType::Bishop),
            }),
            Move::PawnPromotion(PawnPromotionMove {
                pawn: pawn.clone(),
                promotion: Piece(color.clone(), PieceType::Queen),
            }),
        ]
    }
}

#[derive(Clone)]
pub struct StandardMove {
    pub from: Position,
    pub to: Position,
}

#[derive(Clone)]
pub struct PawnDoubleAdvanceMove {
    pub from: Position,
    pub to: Position,
}

#[derive(Clone)]
pub struct PawnEnPassantMove {
    pub from: Position,
    pub to: Position,
}

#[derive(Clone)]
pub struct PawnPromotionMove {
    pub pawn: StandardMove,
    pub promotion: Piece,
}

#[derive(Clone)]
pub enum CastlingMove {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}
