use super::board::*;
use arrayvec::ArrayVec;

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

        self.castling_rights = self.castling_rights.update(from, to);
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

    pub fn generate_moves(&self) -> ArrayVec<Move, 256> {
        let mut moves = ArrayVec::<Move, 256>::new();
        self.board.pieces[Piece::pawn(self.turn)]
            .for_each(|position| self.generate_pawn_moves(position, &mut moves));
        self.board.pieces[Piece::knight(self.turn)]
            .for_each(|position| self.generate_knight_moves(position, &mut moves));
        self.board.pieces[Piece::bishop(self.turn)]
            .for_each(|position| self.generate_bishop_moves(position, &mut moves));
        self.board.pieces[Piece::rook(self.turn)]
            .for_each(|position| self.generate_rook_moves(position, &mut moves));
        self.board.pieces[Piece::queen(self.turn)]
            .for_each(|position| self.generate_queen_moves(position, &mut moves));
        self.board.pieces[Piece::king(self.turn)]
            .for_each(|position| self.generate_king_moves(position, &mut moves));
        // TODO:
        // 1. filter moves that leave king in check
        // 2. generate castling moves
        moves
    }

    fn generate_pawn_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        'push: {
            let (forward_delta, start_rank_mask, end_rank_mask) = match self.turn {
                Color::White => (8i8, BitBoard::RANK_MASKS[1], BitBoard::RANK_MASKS[7]),
                Color::Black => (-8i8, BitBoard::RANK_MASKS[6], BitBoard::RANK_MASKS[0]),
            };

            let single_push_position = position.offset_unchecked(forward_delta);

            if self.board.is_occupied(single_push_position) {
                break 'push;
            }

            if end_rank_mask.is_not_empty(single_push_position) {
                moves.extend([
                    Move::new(position, single_push_position, MoveType::PromotionQueen),
                    Move::new(position, single_push_position, MoveType::PromotionRook),
                    Move::new(position, single_push_position, MoveType::PromotionBishop),
                    Move::new(position, single_push_position, MoveType::PromotionKnight),
                ]);
                break 'push;
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

        let attack_mask = BitBoard::PAWN_ATTACK_MASKS[self.turn][position];

        if let Some(en_passant_position) = self.en_passant {
            if en_passant_position.mask() & attack_mask != BitBoard::EMPTY {
                moves.push(Move::new(
                    position,
                    en_passant_position,
                    MoveType::EnPassant,
                ));
            }
        }

        let opponent_mask = self.board.colors[self.turn.flip()];
        let end_rank_mask = match self.turn {
            Color::White => BitBoard::RANK_MASKS[7],
            Color::Black => BitBoard::RANK_MASKS[0],
        };

        (opponent_mask & attack_mask).for_each(|p| {
            if end_rank_mask.is_not_empty(p) {
                moves.extend([
                    Move::new(position, p, MoveType::PromotionQueen),
                    Move::new(position, p, MoveType::PromotionRook),
                    Move::new(position, p, MoveType::PromotionBishop),
                    Move::new(position, p, MoveType::PromotionKnight),
                ]);
            } else {
                moves.push(Move::new(position, p, MoveType::Standard));
            }
        });
    }

    fn generate_knight_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = BitBoard::KNIGHT_ATTACK_MASKS[position];
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_bishop_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = BitBoard::bishop_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_rook_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = BitBoard::rook_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_queen_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = BitBoard::queen_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_king_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = BitBoard::KING_ATTACK_MASKS[position];
        let friendly_mask = self.board.colors[self.turn];
        let opponent_attack_mask = self.board.opponent_attack_mask(self.turn);

        (attack_mask & !friendly_mask & !opponent_attack_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }
}
