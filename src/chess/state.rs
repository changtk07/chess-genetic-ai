use super::board::{Board, Position};
use super::piece::{Color, Piece, PieceType};
use super::r#move::{
    CastlingMove, CastlingRights, Move, PawnDoubleAdvanceMove, PawnEnPassantMove,
    PawnPromotionMove, StandardMove,
};

#[derive(Clone)]
pub struct State {
    board: Board,
    turn: Color,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    full_moves: usize,
    half_moves: usize,
    white_king_pos: Position,
    black_king_pos: Position,
    move_history: Vec<Move>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "turn: {}", self.turn)?;
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
            turn: Color::White,
            en_passant: None,
            castling_rights: CastlingRights::new(),
            half_moves: 0,
            full_moves: 0,
            white_king_pos: Position(0, 4),
            black_king_pos: Position(7, 4),
            move_history: Vec::new(),
        }
    }

    fn set_king_position(&mut self, pos: &Position) {
        match self.turn {
            Color::White => self.white_king_pos = pos.clone(),
            Color::Black => self.black_king_pos = pos.clone(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // MAKE MOVE
    ///////////////////////////////////////////////////////////////////////////

    pub fn make_move(&mut self, mv: &Move) {
        self.en_passant = None;
        if self.turn == Color::Black {
            self.full_moves += 1;
        }
        self.half_moves += 1;
        self.turn = self.turn.opposite();
        self.move_history.push(mv.clone());

        match mv {
            Move::Standard(mv) => self.make_normal_move(mv),
            Move::PawnDoubleAdvance(mv) => self.make_double_advance_move(mv),
            Move::PawnEnPassant(mv) => self.make_en_passant_move(mv),
            Move::PawnPromotion(mv) => self.make_promotion_move(mv),
            Move::Castling(mv) => self.make_castle_move(mv),
        }
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

    fn make_castle_move(&mut self, mv: &CastlingMove) {
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
                Color::White,
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

    ///////////////////////////////////////////////////////////////////////////
    // VALIDATE MOVE
    ///////////////////////////////////////////////////////////////////////////

    pub fn validate_move(&self, mv: &Move) -> bool {
        match mv {
            Move::Standard(normal) => self.validate_normal_move(normal),
            Move::PawnDoubleAdvance(double_advance) => {
                self.validate_double_advance_move(double_advance)
            }
            Move::PawnEnPassant(en_passant) => self.validate_en_passant_move(en_passant),
            Move::PawnPromotion(promotion) => self.validate_promotion_move(promotion),
            Move::Castling(castle) => self.validate_castle_move(castle),
        }
    }

    fn validate_normal_move(&self, mv: &StandardMove) -> bool {
        if !mv.from.is_valid() || !mv.to.is_valid() || mv.from == mv.to {
            return false;
        }

        let piece = match self.board.get_piece(&mv.from) {
            Some(piece) if *piece.color() == self.turn => piece,
            _ => return false,
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

        let (forward_one, capture_left, capture_right) = match self.turn {
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
                && self.board.is_position_color(&mv.to, &self.turn.opposite()))
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
            .is_position_empty_or_color(&mv.to, &self.turn.opposite())
    }

    fn validate_knight_normal_move(&self, mv: &StandardMove) -> bool {
        let rank_diff = mv.from.0.abs_diff(mv.to.0);
        let file_diff = mv.from.1.abs_diff(mv.to.1);

        ((file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1))
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.turn.opposite())
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
            .is_position_empty_or_color(&mv.to, &self.turn.opposite())
    }

    fn validate_queen_normal_move(&self, mv: &StandardMove) -> bool {
        self.validate_rook_normal_move(mv) || self.validate_bishop_normal_move(mv)
    }

    fn validate_king_normal_move(&self, mv: &StandardMove) -> bool {
        mv.from.0.abs_diff(mv.to.0) <= 1
            && mv.from.1.abs_diff(mv.to.1) <= 1
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.turn.opposite())
    }

    fn validate_double_advance_move(&self, mv: &PawnDoubleAdvanceMove) -> bool {
        if !mv.from.is_valid()
            || !mv.to.is_valid()
            || !self
                .board
                .is_position_piece(&mv.from, &Piece(self.turn.clone(), PieceType::Pawn))
        {
            return false;
        }

        let (start_rank, forward_one, forward_two) = match self.turn {
            Color::White => (
                1,
                Position(mv.from.0 + 1, mv.from.1),
                Position(mv.from.0 + 2, mv.from.1),
            ),
            Color::Black => (
                6,
                Position(mv.from.0 - 1, mv.from.1),
                Position(mv.from.0 - 2, mv.from.1),
            ),
        };

        mv.from.0 == start_rank
            && mv.to == forward_two
            && self.board.is_position_empty(&forward_one)
            && self.board.is_position_empty(&mv.to)
    }

    fn validate_en_passant_move(&self, mv: &PawnEnPassantMove) -> bool {
        if !mv.from.is_valid() || !mv.to.is_valid() || mv.from == mv.to {
            return false;
        }

        // Check mv.from on board is pawn of current turn
        match self.board.get_piece(&mv.from) {
            Some(piece) if *piece == Piece(self.turn.clone(), PieceType::Pawn) => (),
            _ => return false,
        }

        // Check mv.to is en passant position
        let en_passant = match &self.en_passant {
            Some(position) if mv.to == *position => position,
            _ => return false,
        };

        // Check mv.from file is valid
        if mv.from.1 != en_passant.0 + 1 && mv.from.1 != en_passant.0 - 1 {
            return false;
        }

        // Check mv.from rank is valid
        match self.turn {
            Color::White => mv.from.0 == en_passant.0 - 1,
            Color::Black => mv.from.0 == en_passant.0 + 1,
        }
    }

    fn validate_promotion_move(&self, mv: &PawnPromotionMove) -> bool {
        mv.pawn.from.is_valid()
            && mv.pawn.to.is_valid()
            && mv.pawn.from != mv.pawn.to
            && *mv.promotion.color() == self.turn
            && matches!(
                mv.promotion.kind(),
                PieceType::Rook | PieceType::Knight | PieceType::Bishop | PieceType::Queen
            )
            && self
                .board
                .is_position_piece(&mv.pawn.from, &Piece(self.turn.clone(), PieceType::Pawn))
            && self.validate_pawn_normal_move(&mv.pawn)
    }

    fn validate_castle_move(&self, mv: &CastlingMove) -> bool {
        let (me, opponent, king_from, pass_thru, king_to, rook_from) = match mv {
            CastlingMove::WhiteKing => (
                Color::White,
                Color::Black,
                Position(0, 4),
                Position(0, 5),
                Position(0, 6),
                Position(0, 7),
            ),
            CastlingMove::WhiteQueen => (
                Color::White,
                Color::Black,
                Position(0, 4),
                Position(0, 3),
                Position(0, 2),
                Position(0, 0),
            ),
            CastlingMove::BlackKing => (
                Color::Black,
                Color::White,
                Position(7, 4),
                Position(7, 5),
                Position(7, 6),
                Position(7, 7),
            ),
            &CastlingMove::BlackQueen => (
                Color::Black,
                Color::White,
                Position(7, 4),
                Position(7, 3),
                Position(7, 2),
                Position(7, 0),
            ),
        };

        self.castling_rights.white_king
            && self.turn == me
            && self
                .board
                .is_position_piece(&king_from, &Piece(me.clone(), PieceType::King))
            && self
                .board
                .is_position_piece(&rook_from, &Piece(me.clone(), PieceType::Rook))
            && self.board.is_position_empty(&pass_thru)
            && self.board.is_position_empty(&king_to)
            && !self.board.is_position_in_check(&king_from, &Color::Black)
            && !self.board.is_position_in_check(&pass_thru, &Color::Black)
            && !self.board.is_position_in_check(&king_to, &Color::Black)
    }

    ///////////////////////////////////////////////////////////////////////////
    // AVAILABLE MOVES
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_next_states(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        self.board.for_each(|pos, piece| {
            moves.extend(match piece {
                Some(Piece(color, PieceType::Pawn)) if *color == self.turn => {
                    self.get_pawn_moves(pos)
                }
                Some(Piece(color, PieceType::Rook)) if *color == self.turn => {
                    self.get_rook_moves(pos)
                }
                Some(Piece(color, PieceType::Knight)) if *color == self.turn => {
                    self.get_knight_moves(pos)
                }
                Some(Piece(color, PieceType::Bishop)) if *color == self.turn => {
                    self.get_bishop_moves(pos)
                }
                Some(Piece(color, PieceType::Queen)) if *color == self.turn => {
                    self.get_queen_moves(pos)
                }
                Some(Piece(color, PieceType::King)) if *color == self.turn => {
                    self.get_king_moves(pos)
                }
                _ => vec![],
            });
        });

        moves
    }

    fn get_pawn_moves(&self, pos: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        let (starting_rank, forward_one, forward_two, capture_left, capture_right) = match self.turn
        {
            Color::White => (
                1,
                Position(pos.0 + 1, pos.1),
                Position(pos.0 + 2, pos.1),
                Position(pos.0 + 1, pos.1 - 1),
                Position(pos.0 + 1, pos.1 + 1),
            ),
            Color::Black => (
                6,
                Position(pos.0 - 1, pos.1),
                Position(pos.0 - 2, pos.1),
                Position(pos.0 - 1, pos.1 - 1),
                Position(pos.0 - 1, pos.1 + 1),
            ),
        };

        if self.board.is_position_empty(&forward_one) {
            moves.push(Move::Standard(StandardMove {
                from: pos.clone(),
                to: forward_one,
            }));

            if pos.0 == starting_rank && self.board.is_position_empty(&forward_two) {
                moves.push(Move::PawnDoubleAdvance(PawnDoubleAdvanceMove {
                    from: pos.clone(),
                    to: forward_two,
                }));
            }
        }

        let opponent = self.turn.opposite();

        if self.board.is_position_color(&capture_left, &opponent) {
            moves.push(Move::Standard(StandardMove {
                from: pos.clone(),
                to: capture_left.clone(),
            }));
        }

        if self.board.is_position_color(&capture_right, &opponent) {
            moves.push(Move::Standard(StandardMove {
                from: pos.clone(),
                to: capture_right.clone(),
            }));
        }

        if matches!(&self.en_passant, Some(en_passant) if *en_passant == capture_left) {
            moves.push(Move::PawnEnPassant(PawnEnPassantMove {
                from: pos.clone(),
                to: capture_left,
            }));
        }

        if matches!(&self.en_passant, Some(en_passant) if *en_passant == capture_right) {
            moves.push(Move::PawnEnPassant(PawnEnPassantMove {
                from: pos.clone(),
                to: capture_right,
            }));
        }

        moves
    }

    fn get_rook_moves(&self, pos: &Position) -> Vec<Move> {
        // TODO
        let mut moves = Vec::new();

        moves
    }

    fn get_knight_moves(&self, pos: &Position) -> Vec<Move> {
        // TODO
        let mut moves = Vec::new();

        moves
    }

    fn get_bishop_moves(&self, pos: &Position) -> Vec<Move> {
        // TODO
        let mut moves = Vec::new();

        moves
    }

    fn get_queen_moves(&self, pos: &Position) -> Vec<Move> {
        // TODO
        let mut moves = Vec::new();

        moves
    }

    fn get_king_moves(&self, pos: &Position) -> Vec<Move> {
        // TODO
        let mut moves = Vec::new();

        moves
    }
}
