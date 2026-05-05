use super::bitmask::*;
use super::board::*;
use super::moves::*;
use super::prng::*;
use super::types::*;
use arrayvec::ArrayVec;

#[derive(Clone)]
struct UndoRecord {
    mv: Move,
    captured: Option<Piece>,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    halfmove_clock: usize,
    hash: u64,
}

pub struct State {
    board: Board,
    turn: Color,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    fullmove_number: usize,
    halfmove_clock: usize,
    hash: u64,
    history: Vec<UndoRecord>,
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            board: Board::new(),
            turn: Color::White,
            en_passant: None,
            castling_rights: CastlingRights::new(),
            fullmove_number: 1,
            halfmove_clock: 0,
            hash: 0,
            history: Vec::with_capacity(64),
        };
        state.generate_hash();
        state
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts: ArrayVec<&str, 6> = fen.split_whitespace().collect();
        let mut state = Self {
            board: Board::from_fen(parts[0]),
            turn: Color::from_fen(parts[1]),
            castling_rights: CastlingRights::from_fen(parts[2]),
            en_passant: Position::from_fen(parts[3]),
            halfmove_clock: parts[4].parse().unwrap(),
            fullmove_number: parts[5].parse().unwrap(),
            hash: 0,
            history: Vec::with_capacity(64),
        };
        state.generate_hash();
        state
    }

    // ------------------------------------------------------------------------
    // Move Making
    // ------------------------------------------------------------------------

    pub fn make_move(&mut self, mv: Move) {
        let (from, to, move_type) = mv.unpack();
        let (moved, captured) = self.board.move_piece(from, to);

        self.history.push(UndoRecord {
            mv,
            captured,
            en_passant: self.en_passant,
            castling_rights: self.castling_rights,
            halfmove_clock: self.halfmove_clock,
            hash: self.hash,
        });

        if let Some(piece) = moved {
            self.hash ^= RAND_PLACEMENT[piece][from];
            self.hash ^= RAND_PLACEMENT[piece][to];
        }
        if let Some(piece) = captured {
            self.hash ^= RAND_PLACEMENT[piece][to];
        }
        if let Some(pos) = self.en_passant {
            self.hash ^= RAND_EN_PASSANT[pos.file() as usize];
        }
        self.hash ^= RAND_CASTLING[self.castling_rights];

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
        self.hash ^= RAND_COLOR[Color::White];
        self.hash ^= RAND_COLOR[Color::Black];
        self.hash ^= RAND_CASTLING[self.castling_rights];
    }

    pub fn unmake_move(&mut self) {
        let Some(history) = self.history.pop() else {
            return;
        };

        self.hash = history.hash;
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
                let (rook_from, rook_to) = Position::KS_CASTLE_ROOK[self.turn];
                self.board.move_piece(rook_to, rook_from);
            }
            MoveType::QueenSideCastling => {
                let (rook_from, rook_to) = Position::QS_CASTLE_ROOK[self.turn];
                self.board.move_piece(rook_to, rook_from);
            }
            _ => (),
        }
    }

    fn make_move_double_push(&mut self, from: Position, to: Position) {
        let en_passant = Position::middle_of(from, to);
        self.en_passant = Some(en_passant);
        self.halfmove_clock = 0;
        self.hash ^= RAND_EN_PASSANT[en_passant.file() as usize];
    }

    fn make_move_en_passant(&mut self, from: Position, to: Position) {
        let captured = Position::en_passant_captured(from, to);
        self.board.unset_piece(captured);
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.hash ^= RAND_PLACEMENT[Piece::pawn(self.turn.flip())][captured];
    }

    fn make_move_promotion_rook(&mut self, to: Position) {
        self.board.set_piece(to, Piece::rook(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.hash ^= RAND_PLACEMENT[Piece::pawn(self.turn)][to];
        self.hash ^= RAND_PLACEMENT[Piece::rook(self.turn)][to];
    }

    fn make_move_promotion_knight(&mut self, to: Position) {
        self.board.set_piece(to, Piece::knight(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.hash ^= RAND_PLACEMENT[Piece::pawn(self.turn)][to];
        self.hash ^= RAND_PLACEMENT[Piece::knight(self.turn)][to];
    }

    fn make_move_promotion_bishop(&mut self, to: Position) {
        self.board.set_piece(to, Piece::bishop(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.hash ^= RAND_PLACEMENT[Piece::pawn(self.turn)][to];
        self.hash ^= RAND_PLACEMENT[Piece::bishop(self.turn)][to];
    }

    fn make_move_promotion_queen(&mut self, to: Position) {
        self.board.set_piece(to, Piece::queen(self.turn));
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.hash ^= RAND_PLACEMENT[Piece::pawn(self.turn)][to];
        self.hash ^= RAND_PLACEMENT[Piece::queen(self.turn)][to];
    }

    fn make_move_king_side_castling(&mut self) {
        let (rook_from, rook_to) = Position::KS_CASTLE_ROOK[self.turn];
        self.board.move_piece(rook_from, rook_to);
        self.en_passant = None;
        self.halfmove_clock += 1;
        self.hash ^= RAND_PLACEMENT[Piece::rook(self.turn)][rook_from];
        self.hash ^= RAND_PLACEMENT[Piece::rook(self.turn)][rook_to];
    }

    fn make_move_queen_side_castling(&mut self) {
        let (rook_from, rook_to) = Position::QS_CASTLE_ROOK[self.turn];
        self.board.move_piece(rook_from, rook_to);
        self.en_passant = None;
        self.halfmove_clock += 1;
        self.hash ^= RAND_PLACEMENT[Piece::rook(self.turn)][rook_from];
        self.hash ^= RAND_PLACEMENT[Piece::rook(self.turn)][rook_to];
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
        let masks = self.board.move_gen_masks(self.turn);

        if !masks.is_double_check() {
            self.board.pieces[Piece::pawn(self.turn)]
                .for_each(|position| self.generate_pawn_moves(position, &masks, &mut moves));
            self.board.pieces[Piece::knight(self.turn)]
                .for_each(|position| self.generate_knight_moves(position, &masks, &mut moves));
            self.board.pieces[Piece::bishop(self.turn)]
                .for_each(|position| self.generate_bishop_moves(position, &masks, &mut moves));
            self.board.pieces[Piece::rook(self.turn)]
                .for_each(|position| self.generate_rook_moves(position, &masks, &mut moves));
            self.board.pieces[Piece::queen(self.turn)]
                .for_each(|position| self.generate_queen_moves(position, &masks, &mut moves));
        }

        self.board.pieces[Piece::king(self.turn)]
            .for_each(|position| self.generate_king_moves(position, &mut moves));

        moves
    }

    fn generate_pawn_moves(
        &self,
        position: Position,
        masks: &MoveGenMasks,
        moves: &mut ArrayVec<Move, 256>,
    ) {
        let legal_mask = masks.pin_rays[position] & masks.check_mask;

        'push: {
            let (forward_delta, start_rank_mask, end_rank_mask) = match self.turn {
                Color::White => (8i8, Bitmask::RANKS[1], Bitmask::RANKS[7]),
                Color::Black => (-8i8, Bitmask::RANKS[6], Bitmask::RANKS[0]),
            };

            let single_push_position = position.offset_unchecked(forward_delta);

            if self.board.is_occupied(single_push_position) {
                break 'push;
            }

            if legal_mask.contains(single_push_position) {
                if end_rank_mask.contains(single_push_position) {
                    moves.extend([
                        Move::new(position, single_push_position, MoveType::PromotionQueen),
                        Move::new(position, single_push_position, MoveType::PromotionRook),
                        Move::new(position, single_push_position, MoveType::PromotionBishop),
                        Move::new(position, single_push_position, MoveType::PromotionKnight),
                    ]);
                } else {
                    moves.push(Move::new(
                        position,
                        single_push_position,
                        MoveType::Standard,
                    ));
                }
            }

            if start_rank_mask.contains(position) {
                let double_push_position = single_push_position.offset_unchecked(forward_delta);

                if !self.board.is_occupied(double_push_position)
                    && legal_mask.contains(double_push_position)
                {
                    moves.push(Move::new(
                        position,
                        double_push_position,
                        MoveType::DoublePush,
                    ));
                }
            }
        }

        let attack_mask = Bitmask::PAWN_ATTACK_MASKS[self.turn][position];

        if let Some(en_passant) = self.en_passant {
            if attack_mask.contains(en_passant)
                && self.is_en_passant_legal(position, en_passant, masks)
            {
                moves.push(Move::new(position, en_passant, MoveType::EnPassant));
            }
        }

        let opponent_mask = self.board.colors[self.turn.flip()];
        let end_rank_mask = match self.turn {
            Color::White => Bitmask::RANKS[7],
            Color::Black => Bitmask::RANKS[0],
        };

        (attack_mask & opponent_mask & legal_mask).for_each(|p| {
            if end_rank_mask.contains(p) {
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

    fn generate_knight_moves(
        &self,
        position: Position,
        masks: &MoveGenMasks,
        moves: &mut ArrayVec<Move, 256>,
    ) {
        let legal_mask = masks.pin_rays[position] & masks.check_mask;
        let attack_mask = Bitmask::KNIGHT_ATTACK_MASKS[position];
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask & legal_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_bishop_moves(
        &self,
        position: Position,
        masks: &MoveGenMasks,
        moves: &mut ArrayVec<Move, 256>,
    ) {
        let legal_mask = masks.pin_rays[position] & masks.check_mask;
        let attack_mask = Bitmask::bishop_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask & legal_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_rook_moves(
        &self,
        position: Position,
        masks: &MoveGenMasks,
        moves: &mut ArrayVec<Move, 256>,
    ) {
        let legal_mask = masks.pin_rays[position] & masks.check_mask;
        let attack_mask = Bitmask::rook_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask & legal_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_queen_moves(
        &self,
        position: Position,
        masks: &MoveGenMasks,
        moves: &mut ArrayVec<Move, 256>,
    ) {
        let legal_mask = masks.pin_rays[position] & masks.check_mask;
        let attack_mask = Bitmask::queen_attack_mask(position, self.board.occupancy);
        let friendly_mask = self.board.colors[self.turn];
        (attack_mask & !friendly_mask & legal_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));
    }

    fn generate_king_moves(&self, position: Position, moves: &mut ArrayVec<Move, 256>) {
        let attack_mask = Bitmask::KING_ATTACK_MASKS[position];
        let friendly_mask = self.board.colors[self.turn];
        let opponent_attack_mask = self
            .board
            .color_attack_mask(self.turn.flip(), self.board.pieces[Piece::king(self.turn)]);

        (attack_mask & !friendly_mask & !opponent_attack_mask)
            .for_each(|p| moves.push(Move::new(position, p, MoveType::Standard)));

        if self
            .castling_rights
            .has(CastlingRights::QUEEN_SIDE[self.turn])
            && Bitmask::QS_CASTLE_PATH[self.turn] & opponent_attack_mask == Bitmask::EMPTY
            && Bitmask::QS_CASTLE_GAP[self.turn] & self.board.occupancy == Bitmask::EMPTY
        {
            moves.push(Move::QUEEN_SIDE_CASTLING[self.turn]);
        }

        if self
            .castling_rights
            .has(CastlingRights::KING_SIDE[self.turn])
            && Bitmask::KS_CASTLE_PATH[self.turn] & opponent_attack_mask == Bitmask::EMPTY
            && Bitmask::KS_CASTLE_GAP[self.turn] & self.board.occupancy == Bitmask::EMPTY
        {
            moves.push(Move::KING_SIDE_CASTLING[self.turn]);
        }
    }

    fn is_en_passant_legal(
        &self,
        attacker: Position,
        en_passant: Position,
        masks: &MoveGenMasks,
    ) -> bool {
        if !masks.pin_rays[attacker].contains(en_passant) {
            return false;
        }

        let captured = Position::en_passant_captured(attacker, en_passant);

        if masks.check_mask & (en_passant.mask() | captured.mask()) == Bitmask::EMPTY {
            return false;
        }

        let king_pos = self.board.pieces[Piece::king(self.turn)].lsb();
        if king_pos.rank() != attacker.rank() {
            return true;
        }

        let opponent_sliders = self.board.pieces[Piece::rook(self.turn.flip())]
            | self.board.pieces[Piece::queen(self.turn.flip())];

        let occupancy_after = self.board.occupancy & !(attacker.mask() | captured.mask());

        let west_attackers = Bitmask::RAYS[Direction::West][king_pos] & opponent_sliders;
        if west_attackers != Bitmask::EMPTY {
            let between = Bitmask::BETWEEN_MASKS[west_attackers.msb()][king_pos];
            if between & occupancy_after == Bitmask::EMPTY {
                return false;
            }
        }

        let east_attackers = Bitmask::RAYS[Direction::East][king_pos] & opponent_sliders;
        if east_attackers != Bitmask::EMPTY {
            let between = Bitmask::BETWEEN_MASKS[east_attackers.lsb()][king_pos];
            if between & occupancy_after == Bitmask::EMPTY {
                return false;
            }
        }

        true
    }

    // ------------------------------------------------------------------------
    // Zobrist Hashing
    // ------------------------------------------------------------------------

    fn generate_hash(&mut self) {
        self.hash = 0;

        self.board.mailbox.iter().enumerate().for_each(|(i, sqr)| {
            if let Some(piece) = sqr {
                self.hash ^= RAND_PLACEMENT[*piece][i];
            };
        });

        if let Some(en_passant) = self.en_passant {
            self.hash ^= RAND_EN_PASSANT[en_passant.file() as usize];
        }

        self.hash ^= RAND_CASTLING[self.castling_rights];
        self.hash ^= RAND_COLOR[self.turn];
    }

    // ------------------------------------------------------------------------
    // Performance Testing
    // ------------------------------------------------------------------------

    pub fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let moves = self.generate_moves();

        for m in moves {
            self.make_move(m);
            nodes += self.perft(depth - 1);
            self.unmake_move();
        }

        nodes
    }

    pub fn divide(&mut self, depth: u8) {
        if depth == 0 {
            return;
        }

        println!("--- Divide Depth {} ---", depth);
        let mut total_nodes = 0;
        let moves = self.generate_moves();

        for m in moves {
            self.make_move(m);
            let nodes = self.perft(depth - 1);
            self.unmake_move();

            println!("{}: {}", m, nodes);
            total_nodes += nodes;
        }

        println!("\nTotal nodes: {}", total_nodes);
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "Turn: {}", self.turn)?;
        writeln!(f, "Castling: {}", self.castling_rights)?;
        match self.en_passant {
            Some(p) => writeln!(f, "En Passant: {}", p)?,
            None => writeln!(f, "En Passant: -")?,
        };
        writeln!(f, "Halfmove Clock: {}", self.halfmove_clock)?;
        writeln!(f, "Fullmove Number: {}", self.fullmove_number)
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_depth_1() {
        let mut state = State::new();
        assert_eq!(state.perft(1), 20);
    }

    #[test]
    fn perft_depth_2() {
        let mut state = State::new();
        assert_eq!(state.perft(2), 400);
    }

    #[test]
    fn perft_depth_3() {
        let mut state = State::new();
        assert_eq!(state.perft(3), 8_902);
    }

    #[test]
    fn perft_depth_4() {
        let mut state = State::new();
        assert_eq!(state.perft(4), 197_281);
    }

    #[test]
    fn perft_depth_5() {
        let mut state = State::new();
        assert_eq!(state.perft(5), 4_865_609);
    }
}
