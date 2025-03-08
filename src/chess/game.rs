use super::board::{Board, Position};
use super::piece::{Color, Piece, Type};
use super::r#move::{
    CastleMove, CastlingRights, DoubleAdvanceMove, EnPassantMove, Move, NormalMove, PromotionMove,
};

#[derive(Clone)]
pub struct Game {
    board: Board,
    turn: Color,
    en_passant: Option<Position>,
    castling_rights: CastlingRights,
    full_moves: usize,
    half_moves: usize,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "turn: {}", self.turn)?;
        writeln!(f, "full moves: {}", self.full_moves)?;
        writeln!(f, "half moves: {}", self.half_moves)?;
        writeln!(f, "{}", self.board)?;
        Ok(())
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            turn: Color::White,
            en_passant: None,
            castling_rights: CastlingRights::new(),
            half_moves: 0,
            full_moves: 0,
        }
    }

    pub fn make_move(&mut self, mv: &Move) {
        if self.turn == Color::Black {
            self.full_moves += 1;
        }
        self.half_moves += 1;
        self.turn = self.turn.opposite();
        self.board.apply_move(mv);
        self.update_en_passant(mv);
        self.update_castling_rights(mv);
    }

    pub fn make_move_copy(&self, mv: &Move) -> Game {
        let mut new_game = self.clone();
        new_game.make_move(mv);
        new_game
    }

    fn update_en_passant(&mut self, mv: &Move) {
        self.en_passant = match mv {
            Move::DoubleAdvance(mv) => Some(Position((mv.from.0 + mv.to.0) / 2, mv.from.1)),
            _ => None,
        }
    }

    fn update_castling_rights(&mut self, mv: &Move) {
        if let Move::Normal(normal) = mv {
            match self.board.get_piece(&normal.from) {
                Some(Piece(color, Type::King)) => self.castling_rights.disable_both_sides(color),
                Some(Piece(color, Type::Rook)) if normal.from.1 == 0 => {
                    self.castling_rights.disable_queen_side(color)
                }
                Some(Piece(color, Type::Rook)) if normal.from.1 == 7 => {
                    self.castling_rights.disable_king_side(color)
                }
                _ => (),
            }
        }
    }

    pub fn validate_move(&self, mv: &Move) -> bool {
        match mv {
            Move::Normal(normal) => self.validate_normal_move(normal),
            Move::DoubleAdvance(double_advance) => {
                self.validate_double_advance_move(double_advance)
            }
            Move::EnPassant(en_passant) => self.validate_en_passant_move(en_passant),
            Move::Promotion(promotion) => self.validate_promotion_move(promotion),
            Move::Castle(castle) => self.validate_castle_move(castle),
        }
    }

    fn validate_normal_move(&self, mv: &NormalMove) -> bool {
        if !mv.from.is_valid() || !mv.to.is_valid() || mv.from == mv.to {
            return false;
        }

        let piece = match self.board.get_piece(&mv.from) {
            Some(piece) if *piece.color() == self.turn => piece,
            _ => return false,
        };

        match piece.kind() {
            Type::Pawn => self.validate_pawn_normal_move(mv),
            Type::Rook => self.validate_rook_normal_move(mv),
            Type::Knight => self.validate_knight_normal_move(mv),
            Type::Bishop => self.validate_bishop_normal_move(mv),
            Type::Queen => self.validate_queen_normal_move(mv),
            Type::King => self.validate_king_normal_move(mv),
        }
    }

    fn validate_pawn_normal_move(&self, mv: &NormalMove) -> bool {
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

    fn validate_rook_normal_move(&self, mv: &NormalMove) -> bool {
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

    fn validate_knight_normal_move(&self, mv: &NormalMove) -> bool {
        let rank_diff = mv.from.0.abs_diff(mv.to.0);
        let file_diff = mv.from.1.abs_diff(mv.to.1);

        ((file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1))
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.turn.opposite())
    }

    fn validate_bishop_normal_move(&self, mv: &NormalMove) -> bool {
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

    fn validate_queen_normal_move(&self, mv: &NormalMove) -> bool {
        self.validate_rook_normal_move(mv) || self.validate_bishop_normal_move(mv)
    }

    fn validate_king_normal_move(&self, mv: &NormalMove) -> bool {
        mv.from.0.abs_diff(mv.to.0) <= 1
            && mv.from.1.abs_diff(mv.to.1) <= 1
            && self
                .board
                .is_position_empty_or_color(&mv.to, &self.turn.opposite())
    }

    fn validate_double_advance_move(&self, mv: &DoubleAdvanceMove) -> bool {
        if !mv.from.is_valid()
            || !mv.to.is_valid()
            || !self
                .board
                .is_position_piece(&mv.from, &Piece(self.turn.clone(), Type::Pawn))
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

    fn validate_en_passant_move(&self, mv: &EnPassantMove) -> bool {
        if !mv.from.is_valid() || !mv.to.is_valid() || mv.from == mv.to {
            return false;
        }

        // Check mv.from on board is pawn of current turn
        match self.board.get_piece(&mv.from) {
            Some(piece) if *piece == Piece(self.turn.clone(), Type::Pawn) => (),
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

    fn validate_promotion_move(&self, mv: &PromotionMove) -> bool {
        mv.pawn.from.is_valid()
            && mv.pawn.to.is_valid()
            && mv.pawn.from != mv.pawn.to
            && *mv.promotion.color() == self.turn
            && matches!(
                mv.promotion.kind(),
                Type::Rook | Type::Knight | Type::Bishop | Type::Queen
            )
            && self
                .board
                .is_position_piece(&mv.pawn.from, &Piece(self.turn.clone(), Type::Pawn))
            && self.validate_pawn_normal_move(&mv.pawn)
    }

    fn validate_castle_move(&self, mv: &CastleMove) -> bool {
        let (me, opponent, king_from, pass_thru, king_to, rook_from) = match mv {
            CastleMove::WhiteKingSide => (
                Color::White,
                Color::Black,
                Position(0, 4),
                Position(0, 5),
                Position(0, 6),
                Position(0, 7),
            ),
            CastleMove::WhiteQueenSide => (
                Color::White,
                Color::Black,
                Position(0, 4),
                Position(0, 3),
                Position(0, 2),
                Position(0, 0),
            ),
            CastleMove::BlackKingSide => (
                Color::Black,
                Color::White,
                Position(7, 4),
                Position(7, 5),
                Position(7, 6),
                Position(7, 7),
            ),
            &CastleMove::BlackQueenSide => (
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
                .is_position_piece(&king_from, &Piece(me.clone(), Type::King))
            && self
                .board
                .is_position_piece(&rook_from, &Piece(me.clone(), Type::Rook))
            && self.board.is_position_empty(&pass_thru)
            && self.board.is_position_empty(&king_to)
            && !self.board.is_position_in_check(&king_from, &Color::Black)
            && !self.board.is_position_in_check(&pass_thru, &Color::Black)
            && !self.board.is_position_in_check(&king_to, &Color::Black)
    }
}
