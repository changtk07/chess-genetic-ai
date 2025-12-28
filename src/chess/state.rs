use super::board::*;
use super::piece::*;
use super::r#move::*;

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub player: Color,
    pub opponent: Color,
    pub en_passant: Option<Position>,
    pub castling_rights: CastlingRights,
    pub full_moves: usize,
    pub half_moves: usize,
    pub white_king_pos: Position,
    pub black_king_pos: Position,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "turn: {}", self.player)?;
        writeln!(f, "full moves: {}", self.full_moves)?;
        writeln!(f, "half moves: {}", self.half_moves)?;
        writeln!(f, "{}", self.board)?;
        Ok(())
    }
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            player: Color::White,
            opponent: Color::Black,
            en_passant: None,
            castling_rights: CastlingRights::new(),
            half_moves: 0,
            full_moves: 0,
            white_king_pos: Position(0, 4),
            black_king_pos: Position(7, 4),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // MAKE MOVE
    ///////////////////////////////////////////////////////////////////////////

    pub fn make_move(&mut self, mv: &Move) {
        self.en_passant = None;

        match mv {
            Move::Standard(mv) => self.make_normal_move(mv),
            Move::PawnDoubleAdvance(mv) => self.make_double_advance_move(mv),
            Move::PawnEnPassant(mv) => self.make_en_passant_move(mv),
            Move::PawnPromotion(mv) => self.make_promotion_move(mv),
            Move::Castling(mv) => self.make_castling_move(mv),
        }

        std::mem::swap(&mut self.player, &mut self.opponent);
        self.half_moves += 1;
        self.full_moves = self.half_moves >> 1;
    }

    pub fn make_move_copy(&self, mv: &Move) -> State {
        let mut new_game = self.clone();
        new_game.make_move(mv);
        new_game
    }

    fn make_normal_move(&mut self, mv: &StandardMove) {
        match self.board.get_piece(&mv.from) {
            Some(Piece(color, PieceType::King)) => {
                self.castling_rights.disable_both_sides(color);
                self.set_king_position(&mv.to);
            }
            Some(Piece(color, PieceType::Rook)) if mv.from.1 == 0 => {
                self.castling_rights.disable_queen_side(color)
            }
            Some(Piece(color, PieceType::Rook)) if mv.from.1 == 7 => {
                self.castling_rights.disable_king_side(color)
            }
            _ => (),
        }
        self.board
            .set_piece(&mv.to, self.board.get_piece(&mv.from).clone());
        self.board.set_piece(&mv.from, None);
    }

    fn make_double_advance_move(&mut self, mv: &PawnDoubleAdvanceMove) {
        self.en_passant = Some(Position((mv.from.0 + mv.to.0) / 2, mv.from.1));
        self.board
            .set_piece(&mv.to, self.board.get_piece(&mv.from).clone());
        self.board.set_piece(&mv.from, None);
    }

    fn make_en_passant_move(&mut self, mv: &PawnEnPassantMove) {
        self.board
            .set_piece(&mv.to, self.board.get_piece(&mv.from).clone());
        self.board.set_piece(&mv.from, None);
        self.board.set_piece(&Position(mv.from.0, mv.to.1), None);
    }

    fn make_promotion_move(&mut self, mv: &PawnPromotionMove) {
        self.board
            .set_piece(&mv.pawn.from, Some(mv.promotion.clone()));
        self.make_normal_move(&mv.pawn);
    }

    fn make_castling_move(&mut self, mv: &CastlingMove) {
        let (color, king_start, pass_thru, king_end, rook_start) = match mv {
            CastlingMove::WhiteKing => (
                Color::White,
                Position(0, 4),
                Position(0, 5),
                Position(0, 6),
                Position(0, 7),
            ),
            CastlingMove::WhiteQueen => (
                Color::White,
                Position(0, 4),
                Position(0, 3),
                Position(0, 2),
                Position(0, 0),
            ),
            CastlingMove::BlackKing => (
                Color::Black,
                Position(7, 4),
                Position(7, 5),
                Position(7, 6),
                Position(7, 7),
            ),
            CastlingMove::BlackQueen => (
                Color::Black,
                Position(7, 4),
                Position(7, 3),
                Position(7, 2),
                Position(7, 0),
            ),
        };

        self.set_king_position(&king_end);
        self.castling_rights.disable_both_sides(&color);
        self.board
            .set_piece(&king_end, Some(Piece(color.clone(), PieceType::King)));
        self.board.set_piece(&king_start, None);
        self.board
            .set_piece(&pass_thru, Some(Piece(color, PieceType::Rook)));
        self.board.set_piece(&rook_start, None);
    }

    fn set_king_position(&mut self, pos: &Position) {
        match self.player {
            Color::White => self.white_king_pos = pos.clone(),
            Color::Black => self.black_king_pos = pos.clone(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // VALIDATE MOVE
    ///////////////////////////////////////////////////////////////////////////

    pub fn validate_move(&self, mv: &Move) -> bool {
        let is_pseudo_legal = match mv {
            Move::Standard(normal) => self.validate_normal_move(normal),
            Move::PawnDoubleAdvance(double_advance) => {
                self.validate_double_advance_move(double_advance)
            }
            Move::PawnEnPassant(en_passant) => self.validate_en_passant_move(en_passant),
            Move::PawnPromotion(promotion) => self.validate_promotion_move(promotion),
            Move::Castling(castling) => self.validate_castling_move(castling),
        };

        if !is_pseudo_legal {
            return false;
        }

        let new_state = self.make_move_copy(mv);
        let king_pos = match self.player {
            Color::White => &new_state.white_king_pos,
            Color::Black => &new_state.black_king_pos,
        };

        !new_state
            .board
            .is_position_in_check(king_pos, &new_state.player)
    }

    fn validate_normal_move(&self, mv: &StandardMove) -> bool {
        let piece = match self.board.get_piece(&mv.from) {
            Some(piece) if *piece.color() == self.player => piece,
            _ => return false,
        };

        match self.board.get_piece(&mv.to) {
            Some(piece) if *piece.color() == self.player => return false,
            _ => (),
        };

        match piece.kind() {
            PieceType::Pawn => self.validate_pawn_normal_move(mv),
            PieceType::Rook => self.validate_rook_normal_move(mv),
            PieceType::Knight => self.validate_knight_normal_move(mv),
            PieceType::Bishop => self.validate_bishop_normal_move(mv),
            PieceType::Queen => self.validate_queen_normal_move(mv),
            PieceType::King => self.validate_king_normal_move(mv),
        }
    }

    fn validate_pawn_normal_move(&self, mv: &StandardMove) -> bool {
        if mv.from.0 == 0 || mv.from.0 == 7 {
            return false;
        }

        let (forward_one, capture_left, capture_right) = match self.player {
            Color::White => (
                Position(mv.from.0 + 1, mv.from.1),
                Position(mv.from.0 + 1, mv.from.1 - 1),
                Position(mv.from.0 + 1, mv.from.1 + 1),
            ),
            Color::Black => (
                Position(mv.from.0 - 1, mv.from.1),
                Position(mv.from.0 - 1, mv.from.1 - 1),
                Position(mv.from.0 - 1, mv.from.1 + 1),
            ),
        };

        (mv.to == forward_one && self.board.is_position_empty(&mv.to))
            || ((mv.to == capture_left || mv.to == capture_right)
                && self.board.is_position_color(&mv.to, &self.opponent))
    }

    fn validate_rook_normal_move(&self, mv: &StandardMove) -> bool {
        if mv.from.0 != mv.to.0 && mv.from.1 != mv.to.1 {
            return false;
        }

        let (start, end, is_rank) = if mv.from.0 == mv.to.0 {
            (mv.from.1.min(mv.to.1), mv.from.1.max(mv.to.1), false)
        } else {
            (mv.from.0.min(mv.to.0), mv.from.0.max(mv.to.0), true)
        };

        for i in (start + 1)..end {
            let position = if is_rank {
                Position(i, mv.from.1)
            } else {
                Position(mv.from.0, i)
            };

            if !self.board.is_position_empty(&position) {
                return false;
            }
        }

        self.board
            .is_position_empty_or_color(&mv.to, &self.opponent)
    }

    fn validate_knight_normal_move(&self, mv: &StandardMove) -> bool {
        let rank_diff = mv.from.0.abs_diff(mv.to.0);
        let file_diff = mv.from.1.abs_diff(mv.to.1);

        ((file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1))
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.opponent)
    }

    fn validate_bishop_normal_move(&self, mv: &StandardMove) -> bool {
        let diff_rank = (mv.from.0 - mv.to.0).abs();
        let diff_file = (mv.from.1 - mv.to.1).abs();

        if diff_rank != diff_file {
            return false;
        }

        let rank_step = if mv.from.0 < mv.to.0 { 1 } else { -1 };
        let file_step = if mv.from.1 < mv.to.1 { 1 } else { -1 };

        let mut rank = mv.from.0 + rank_step;
        let mut file = mv.from.1 + file_step;

        while rank != mv.to.0 && file != mv.to.1 {
            if !self.board.is_position_empty(&Position(rank, file)) {
                return false;
            }
            rank += rank_step;
            file += file_step;
        }

        self.board
            .is_position_empty_or_color(&mv.to, &self.opponent)
    }

    fn validate_queen_normal_move(&self, mv: &StandardMove) -> bool {
        self.validate_rook_normal_move(mv) || self.validate_bishop_normal_move(mv)
    }

    fn validate_king_normal_move(&self, mv: &StandardMove) -> bool {
        mv.from.0.abs_diff(mv.to.0) <= 1
            && mv.from.1.abs_diff(mv.to.1) <= 1
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.opponent)
    }

    fn validate_double_advance_move(&self, mv: &PawnDoubleAdvanceMove) -> bool {
        let (start_rank, pawn, forward_one, forward_two) = match self.player {
            Color::White => (
                1,
                Piece(Color::White, PieceType::Pawn),
                Position(mv.from.0 + 1, mv.from.1),
                Position(mv.from.0 + 2, mv.from.1),
            ),
            Color::Black => (
                6,
                Piece(Color::Black, PieceType::Pawn),
                Position(mv.from.0 - 1, mv.from.1),
                Position(mv.from.0 - 2, mv.from.1),
            ),
        };

        mv.from.0 == start_rank
            && mv.to == forward_two
            && self.board.is_position_piece(&mv.from, &pawn)
            && self.board.is_position_empty(&forward_one)
            && self.board.is_position_empty(&mv.to)
    }

    fn validate_en_passant_move(&self, mv: &PawnEnPassantMove) -> bool {
        if !self.board.is_position_empty(&mv.to)
            || !self.board.is_position_piece(
                &Position(mv.from.0, mv.to.1),
                &Piece(self.opponent.clone(), PieceType::Pawn),
            )
        {
            return false;
        }

        // Check mv.from on board is pawn of current turn
        match self.board.get_piece(&mv.from) {
            Some(piece) if *piece == Piece(self.player.clone(), PieceType::Pawn) => (),
            _ => return false,
        }

        // Check mv.to is en passant position
        let en_passant = match &self.en_passant {
            Some(position) if mv.to == *position => position,
            _ => return false,
        };

        // Check mv.from file is valid
        if mv.from.1 != en_passant.1 + 1 && mv.from.1 != en_passant.1 - 1 {
            return false;
        }

        // Check mv.from rank is valid
        match self.player {
            Color::White => mv.from.0 == en_passant.0 - 1,
            Color::Black => mv.from.0 == en_passant.0 + 1,
        }
    }

    fn validate_promotion_move(&self, mv: &PawnPromotionMove) -> bool {
        mv.pawn.from.is_valid()
            && mv.pawn.to.is_valid()
            && mv.pawn.from != mv.pawn.to
            && *mv.promotion.color() == self.player
            && matches!(
                mv.promotion.kind(),
                PieceType::Rook | PieceType::Knight | PieceType::Bishop | PieceType::Queen
            )
            && self
                .board
                .is_position_piece(&mv.pawn.from, &Piece(self.player.clone(), PieceType::Pawn))
            && self.validate_pawn_normal_move(&mv.pawn)
    }

    fn validate_castling_move(&self, mv: &CastlingMove) -> bool {
        match mv {
            CastlingMove::WhiteKing => {
                self.player == Color::White
                    && self.castling_rights.white_king
                    && self
                        .board
                        .is_position_piece(&Position(0, 4), &Piece(Color::White, PieceType::King))
                    && self
                        .board
                        .is_position_piece(&Position(0, 7), &Piece(Color::White, PieceType::Rook))
                    && self.board.is_position_empty(&Position(0, 5))
                    && self.board.is_position_empty(&Position(0, 6))
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 4), &Color::Black)
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 5), &Color::Black)
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 6), &Color::Black)
            }
            CastlingMove::WhiteQueen => {
                self.player == Color::White
                    && self.castling_rights.white_queen
                    && self
                        .board
                        .is_position_piece(&Position(0, 4), &Piece(Color::White, PieceType::King))
                    && self
                        .board
                        .is_position_piece(&Position(0, 0), &Piece(Color::White, PieceType::Rook))
                    && self.board.is_position_empty(&Position(0, 3))
                    && self.board.is_position_empty(&Position(0, 2))
                    && self.board.is_position_empty(&Position(0, 1))
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 4), &Color::Black)
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 3), &Color::Black)
                    && !self
                        .board
                        .is_position_in_check(&Position(0, 2), &Color::Black)
            }
            CastlingMove::BlackKing => {
                self.player == Color::Black
                    && self.castling_rights.black_king
                    && self
                        .board
                        .is_position_piece(&Position(7, 4), &Piece(Color::Black, PieceType::King))
                    && self
                        .board
                        .is_position_piece(&Position(7, 7), &Piece(Color::Black, PieceType::Rook))
                    && self.board.is_position_empty(&Position(7, 5))
                    && self.board.is_position_empty(&Position(7, 6))
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 4), &Color::White)
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 5), &Color::White)
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 6), &Color::White)
            }
            &CastlingMove::BlackQueen => {
                self.player == Color::Black
                    && self.castling_rights.black_queen
                    && self
                        .board
                        .is_position_piece(&Position(7, 4), &Piece(Color::Black, PieceType::King))
                    && self
                        .board
                        .is_position_piece(&Position(7, 0), &Piece(Color::Black, PieceType::Rook))
                    && self.board.is_position_empty(&Position(7, 3))
                    && self.board.is_position_empty(&Position(7, 2))
                    && self.board.is_position_empty(&Position(7, 1))
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 4), &Color::White)
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 3), &Color::White)
                    && !self
                        .board
                        .is_position_in_check(&Position(7, 2), &Color::White)
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // GENERATE LEGAL MOVES
    ///////////////////////////////////////////////////////////////////////////

    pub fn gen_legal_moves(&self) -> Vec<(Move, State)> {
        let moves = self.gen_potential_legal_moves();
        let mut moves_and_states = Vec::new();

        moves.into_iter().for_each(|mv| {
            let new_state = self.make_move_copy(&mv);
            let king_pos = match self.player {
                Color::White => &new_state.white_king_pos,
                Color::Black => &new_state.black_king_pos,
            };

            // Prevent moves that leave king in check
            if !new_state
                .board
                .is_position_in_check(king_pos, &new_state.player)
            {
                moves_and_states.push((mv, new_state));
            }
        });

        moves_and_states
    }

    fn gen_potential_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        self.board.for_each(|pos, piece| {
            moves.extend(match piece {
                Some(Piece(color, PieceType::Pawn)) if *color == self.player => {
                    self.gen_pawn_moves(pos)
                }
                Some(Piece(color, PieceType::Rook)) if *color == self.player => {
                    self.gen_rook_moves(pos)
                }
                Some(Piece(color, PieceType::Knight)) if *color == self.player => {
                    self.gen_knight_moves(pos)
                }
                Some(Piece(color, PieceType::Bishop)) if *color == self.player => {
                    self.gen_bishop_moves(pos)
                }
                Some(Piece(color, PieceType::Queen)) if *color == self.player => {
                    self.gen_queen_moves(pos)
                }
                Some(Piece(color, PieceType::King)) if *color == self.player => {
                    self.gen_king_moves(pos)
                }
                _ => vec![],
            });
        });

        moves
    }

    fn gen_pawn_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        let (starting_rank, promotion_rank, forward_one, forward_two, capture_left, capture_right) =
            match self.player {
                Color::White => (
                    1,
                    7,
                    Position(x + 1, y),
                    Position(x + 2, y),
                    Position(x + 1, y - 1),
                    Position(x + 1, y + 1),
                ),
                Color::Black => (
                    6,
                    0,
                    Position(x - 1, y),
                    Position(x - 2, y),
                    Position(x - 1, y - 1),
                    Position(x - 1, y + 1),
                ),
            };

        if self.board.is_position_empty(&forward_one) {
            let mv = StandardMove {
                from: Position(x, y),
                to: forward_one.clone(),
            };

            if forward_one.0 == promotion_rank {
                moves.extend(Move::all_pawn_promotions(&mv, &self.player));
            } else {
                moves.push(Move::Standard(mv));

                if x == starting_rank && self.board.is_position_empty(&forward_two) {
                    moves.push(Move::PawnDoubleAdvance(PawnDoubleAdvanceMove {
                        from: Position(x, y),
                        to: forward_two,
                    }));
                }
            }
        }

        if self.board.is_position_color(&capture_left, &self.opponent) {
            let mv = StandardMove {
                from: Position(x, y),
                to: capture_left.clone(),
            };

            if capture_left.0 == promotion_rank {
                moves.extend(Move::all_pawn_promotions(&mv, &self.player));
            } else {
                moves.push(Move::Standard(mv));
            }
        } else if matches!(&self.en_passant, Some(en_passant) if *en_passant == capture_left) {
            moves.push(Move::PawnEnPassant(PawnEnPassantMove {
                from: Position(x, y),
                to: capture_left,
            }));
        }

        if self.board.is_position_color(&capture_right, &self.opponent) {
            let mv = StandardMove {
                from: Position(x, y),
                to: capture_right.clone(),
            };

            if capture_right.0 == promotion_rank {
                moves.extend(Move::all_pawn_promotions(&mv, &self.player));
            } else {
                moves.push(Move::Standard(mv));
            }
        } else if matches!(&self.en_passant, Some(en_passant) if *en_passant == capture_right) {
            moves.push(Move::PawnEnPassant(PawnEnPassantMove {
                from: Position(x, y),
                to: capture_right,
            }));
        }

        moves
    }

    fn gen_rook_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for i in 1..8 {
                let pos = Position(x + i * dx, y + i * dy);
                if !pos.is_valid() {
                    break;
                }

                match self.board.get_piece(&pos) {
                    None => moves.push(Move::Standard(StandardMove {
                        from: Position(x, y),
                        to: pos,
                    })),
                    Some(Piece(color, _)) if *color == self.opponent => {
                        moves.push(Move::Standard(StandardMove {
                            from: Position(x, y),
                            to: pos,
                        }));
                        break;
                    }
                    _ => break,
                }
            }
        }

        moves
    }

    fn gen_knight_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        for pos in [
            Position(x + 1, y - 2),
            Position(x + 2, y - 1),
            Position(x + 2, y + 1),
            Position(x + 1, y + 2),
            Position(x - 1, y + 2),
            Position(x - 2, y + 1),
            Position(x - 2, y - 1),
            Position(x - 1, y - 2),
        ] {
            if !pos.is_valid() || self.board.is_position_color(&pos, &self.player) {
                continue;
            }
            moves.push(Move::Standard(StandardMove {
                from: Position(x, y),
                to: pos,
            }));
        }

        moves
    }

    fn gen_bishop_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            for i in 1..8 {
                let pos = Position(x + i * dx, y + i * dy);
                if !pos.is_valid() {
                    break;
                }

                match self.board.get_piece(&pos) {
                    None => moves.push(Move::Standard(StandardMove {
                        from: Position(x, y),
                        to: pos,
                    })),
                    Some(Piece(color, _)) if *color == self.opponent => {
                        moves.push(Move::Standard(StandardMove {
                            from: Position(x, y),
                            to: pos,
                        }));
                        break;
                    }
                    _ => break,
                }
            }
        }

        moves
    }

    fn gen_queen_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.extend(self.gen_rook_moves(&Position(x, y)));
        moves.extend(self.gen_bishop_moves(&Position(x, y)));

        moves
    }

    fn gen_king_moves(&self, &Position(x, y): &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        for pos in [
            Position(x + 1, y - 1),
            Position(x + 1, y),
            Position(x + 1, y + 1),
            Position(x, y - 1),
            Position(x, y + 1),
            Position(x - 1, y - 1),
            Position(x - 1, y),
            Position(x - 1, y + 1),
        ] {
            if !pos.is_valid() || self.board.is_position_color(&pos, &self.player) {
                continue;
            }
            moves.push(Move::Standard(StandardMove {
                from: Position(x, y),
                to: pos,
            }));
        }

        if self.validate_castling_move(&CastlingMove::WhiteKing) {
            moves.push(Move::Castling(CastlingMove::WhiteKing));
        }
        if self.validate_castling_move(&CastlingMove::WhiteQueen) {
            moves.push(Move::Castling(CastlingMove::WhiteQueen));
        }
        if self.validate_castling_move(&CastlingMove::BlackKing) {
            moves.push(Move::Castling(CastlingMove::BlackKing));
        }
        if self.validate_castling_move(&CastlingMove::BlackQueen) {
            moves.push(Move::Castling(CastlingMove::BlackQueen));
        }

        moves
    }
}
