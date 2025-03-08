use super::board::Position;
use super::piece::{Color, Piece};

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

pub enum Move {
    Normal(NormalMove),
    DoubleAdvance(DoubleAdvanceMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
    Castle(CastleMove),
}

pub struct NormalMove {
    pub from: Position,
    pub to: Position,
}

pub struct DoubleAdvanceMove {
    pub from: Position,
    pub to: Position,
}

pub struct EnPassantMove {
    pub from: Position,
    pub to: Position,
}

pub struct PromotionMove {
    pub pawn: NormalMove,
    pub promotion: Piece,
}

pub enum CastleMove {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}
