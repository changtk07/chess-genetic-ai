use super::board::*;

pub struct State {
    board: Board,
    turn: Color,
    en_passant: Option<(Position, Position)>,
    castling_rights: CastlingRights,
    fullmove_number: usize,
    halfmove_clock: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            en_passant: None,
            castling_rights: CastlingRights::new(),
            fullmove_number: 1,
            halfmove_clock: 0,
        }
    }

    pub fn make_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let move_type = mv.move_type();

        let (moved, captured) = self.board.move_piece(from, to);

        match move_type {
            MoveType::DoublePush => self.make_move_double_push(from, to),
            MoveType::EnPassant => self.make_move_en_passant(),
            MoveType::PromotionRook => self.make_move_promotion_rook(to),
            MoveType::PromotionKnight => self.make_move_promotion_knight(to),
            MoveType::PromotionBishop => self.make_move_promotion_bishop(to),
            MoveType::PromotionQueen => self.make_move_promotion_queen(to),
            MoveType::KingSideCastling => self.make_move_king_side_castling(),
            MoveType::QueenSideCastling => self.make_move_queen_side_castling(),
            MoveType::Standard => self.make_move_standard(moved, captured),
        }

        self.castling_rights.update(from, to);
        self.fullmove_number += self.turn as usize;
        self.turn = self.turn.flip();
    }

    fn make_move_double_push(&mut self, from: Position, to: Position) {
        self.en_passant = Some((Position::middle_of(from, to), to));
        self.halfmove_clock = 0;
    }

    fn make_move_en_passant(&mut self) {
        if let Some((_, captured)) = self.en_passant {
            self.board.unset_piece(captured);
        }
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_rook(&mut self, to: Position) {
        let promotion = match self.turn {
            Color::White => Piece::WhiteRook,
            Color::Black => Piece::BlackRook,
        };
        self.board.set_piece(to, promotion);
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_knight(&mut self, to: Position) {
        let promotion = match self.turn {
            Color::White => Piece::WhiteKnight,
            Color::Black => Piece::BlackKnight,
        };
        self.board.set_piece(to, promotion);
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_bishop(&mut self, to: Position) {
        let promotion = match self.turn {
            Color::White => Piece::WhiteBishop,
            Color::Black => Piece::BlackBishop,
        };
        self.board.set_piece(to, promotion);
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_queen(&mut self, position: Position) {
        let promotion = match self.turn {
            Color::White => Piece::WhiteQueen,
            Color::Black => Piece::BlackQueen,
        };
        self.board.set_piece(position, promotion);
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_king_side_castling(&mut self) {
        let (rook_from, rook_to) = match self.turn {
            Color::White => (Position::H1, Position::F1),
            Color::Black => (Position::H8, Position::F8),
        };
        self.board.move_piece(rook_from, rook_to);
        self.en_passant = None;
        self.halfmove_clock += 1;
    }

    fn make_move_queen_side_castling(&mut self) {
        let (rook_from, rook_to) = match self.turn {
            Color::White => (Position::A1, Position::D1),
            Color::Black => (Position::A8, Position::D8),
        };
        self.board.move_piece(rook_from, rook_to);
        self.en_passant = None;
        self.halfmove_clock += 1;
    }

    fn make_move_standard(&mut self, moved: Option<Piece>, captured: Option<Piece>) {
        self.en_passant = None;

        if matches!(moved, Some(Piece::WhitePawn | Piece::BlackPawn)) || captured.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }
}
