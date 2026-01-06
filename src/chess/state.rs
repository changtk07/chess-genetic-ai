use super::board::*;

#[derive(Clone)]
struct History {
    captured: Option<Piece>,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    halfmove_clock: usize,
}

pub struct State {
    board: Board,
    turn: Color,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    fullmove_number: usize,
    halfmove_clock: usize,
    history: Vec<History>,
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
            history: Vec::with_capacity(64),
        }
    }

    // ------------------------------------------------------------------------
    // Move Making
    // ------------------------------------------------------------------------

    pub fn make_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let move_type = mv.move_type();

        let (moved, captured) = self.board.move_piece(from, to);

        self.history.push(History {
            captured,
            en_passant: self.en_passant,
            castling_rights: self.castling_rights,
            halfmove_clock: self.halfmove_clock,
        });

        match move_type {
            MoveType::DoublePush => self.make_move_double_push(from, to),
            MoveType::EnPassant => self.make_move_en_passant(from, to),
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

    pub fn unmake_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let move_type = mv.move_type();

        self.turn = self.turn.flip();
        self.fullmove_number -= self.turn as usize;

        self.board.move_piece(to, from);

        let Some(history) = self.history.pop() else {
            return;
        };

        self.en_passant = history.en_passant;
        self.castling_rights = history.castling_rights;
        self.halfmove_clock = history.halfmove_clock;

        if let Some(captured) = history.captured {
            self.board.set_piece(to, captured);
        }

        match move_type {
            MoveType::EnPassant => {
                let captured = Position::en_passant_captured(from, to);
                self.board
                    .set_piece(captured, Piece::pawn(self.turn.flip()));
            }
            MoveType::PromotionRook
            | MoveType::PromotionKnight
            | MoveType::PromotionBishop
            | MoveType::PromotionQueen => {
                self.board.set_piece(from, Piece::pawn(self.turn));
            }
            MoveType::KingSideCastling => {
                let (rook_from, rook_to) = match self.turn {
                    Color::White => (Position::H1, Position::F1),
                    Color::Black => (Position::H8, Position::F8),
                };
                self.board.move_piece(rook_to, rook_from);
            }
            MoveType::QueenSideCastling => {
                let (rook_from, rook_to) = match self.turn {
                    Color::White => (Position::A1, Position::D1),
                    Color::Black => (Position::A8, Position::D8),
                };
                self.board.move_piece(rook_to, rook_from);
            }
            _ => (),
        }
    }

    fn make_move_double_push(&mut self, from: Position, to: Position) {
        self.en_passant = Some(Position::middle_of(from, to));
        self.halfmove_clock = 0;
    }

    fn make_move_en_passant(&mut self, from: Position, to: Position) {
        let captured = Position::en_passant_captured(from, to);
        self.board.unset_piece(captured);
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_rook(&mut self, to: Position) {
        self.board.set_piece(to, Piece::rook(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_knight(&mut self, to: Position) {
        self.board.set_piece(to, Piece::knight(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_bishop(&mut self, to: Position) {
        self.board.set_piece(to, Piece::bishop(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
    }

    fn make_move_promotion_queen(&mut self, to: Position) {
        self.board.set_piece(to, Piece::queen(self.turn));
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
