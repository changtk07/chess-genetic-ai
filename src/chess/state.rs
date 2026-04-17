use super::board::*;

#[derive(Clone)]
struct History {
    mv: Move,
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
            mv,
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

    pub fn unmake_move(&mut self) {
        let Some(history) = self.history.pop() else {
            return;
        };

        self.en_passant = history.en_passant;
        self.castling_rights = history.castling_rights;
        self.halfmove_clock = history.halfmove_clock;
        self.turn = self.turn.flip();
        self.fullmove_number -= self.turn as usize;

        let from = history.mv.from();
        let to = history.mv.to();
        let move_type = history.mv.move_type();

        self.board.move_piece(to, from);

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

    // ------------------------------------------------------------------------
    // Move Generation
    // ------------------------------------------------------------------------

    fn generate_legal_moves(&self) -> Vec<Move> {
        // TODO
        let moves = vec![];
        moves
    }

    fn generate_pawn_moves(&self, position: Position, moves: &mut Vec<Move>) {
        self.generate_pawn_push_moves(position, moves);
        // TODO: attack
    }

    fn generate_pawn_push_moves(&self, position: Position, moves: &mut Vec<Move>) {
        let (forward_delta, start_rank_mask, end_rank_mask) = match self.turn {
            Color::White => (8i8, BitBoard::RANK_2, BitBoard::RANK_8),
            Color::Black => (-8i8, BitBoard::RANK_7, BitBoard::RANK_1),
        };

        let single_push_position = position.offset_unchecked(forward_delta);

        if self.board.is_occupied(single_push_position) {
            return;
        }

        if end_rank_mask.is_not_empty(single_push_position) {
            moves.extend([
                Move::new(position, single_push_position, MoveType::PromotionQueen),
                Move::new(position, single_push_position, MoveType::PromotionRook),
                Move::new(position, single_push_position, MoveType::PromotionBishop),
                Move::new(position, single_push_position, MoveType::PromotionKnight),
            ]);
            return;
        }

        moves.push(Move::new(
            position,
            single_push_position,
            MoveType::Standard,
        ));

        if start_rank_mask.is_not_empty(position) {
            let double_push_position = single_push_position.offset_unchecked(forward_delta);

            if self.board.is_not_occupied(double_push_position) {
                moves.push(Move::new(
                    position,
                    double_push_position,
                    MoveType::DoublePush,
                ));
            }
        }
    }
}
