use super::board::Position;
use super::piece::Piece;

#[derive(Clone)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
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
    pub to: Piece,
}

pub struct CastleMove {
    pub king: NormalMove,
    pub rook: NormalMove,
}
