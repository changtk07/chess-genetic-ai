use super::piece::{Color, Piece, Type};
use super::r#move::{
    CastleMove, DoubleAdvanceMove, EnPassantMove, Move, NormalMove, PromotionMove,
};

#[derive(Clone, PartialEq)]
pub struct Position(pub u8, pub u8);

impl Position {
    pub fn is_valid(&self) -> bool {
        self.0 < 8 && self.1 < 8
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.0 + b'a') as char, self.1 + 1)
    }
}

#[derive(Clone)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().rev().enumerate() {
            write!(f, "{} | ", 8 - i)?;
            for piece in row {
                match piece {
                    Some(p) => write!(f, "{} ", p)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   ----------------")?;
        writeln!(f, "    a b c d e f g h")?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        const INITIAL_BOARD: [[Option<Piece>; 8]; 8] = [
            [
                Some(Piece(Color::White, Type::Rook)),
                Some(Piece(Color::White, Type::Knight)),
                Some(Piece(Color::White, Type::Bishop)),
                Some(Piece(Color::White, Type::Queen)),
                Some(Piece(Color::White, Type::King)),
                Some(Piece(Color::White, Type::Bishop)),
                Some(Piece(Color::White, Type::Knight)),
                Some(Piece(Color::White, Type::Rook)),
            ],
            [
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
                Some(Piece(Color::White, Type::Pawn)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
                Some(Piece(Color::Black, Type::Pawn)),
            ],
            [
                Some(Piece(Color::Black, Type::Rook)),
                Some(Piece(Color::Black, Type::Knight)),
                Some(Piece(Color::Black, Type::Bishop)),
                Some(Piece(Color::Black, Type::Queen)),
                Some(Piece(Color::Black, Type::King)),
                Some(Piece(Color::Black, Type::Bishop)),
                Some(Piece(Color::Black, Type::Knight)),
                Some(Piece(Color::Black, Type::Rook)),
            ],
        ];
        Board(INITIAL_BOARD)
    }

    pub fn get_piece(&self, &Position(x, y): &Position) -> &Option<Piece> {
        self.0
            .get(x as usize)
            .and_then(|row| row.get(y as usize))
            .unwrap_or(&None)
    }

    fn set_piece(&mut self, &Position(x, y): &Position, piece: Option<Piece>) {
        if let Some(cell) = self
            .0
            .get_mut(x as usize)
            .and_then(|row| row.get_mut(y as usize))
        {
            *cell = piece;
        }
    }

    pub fn is_position_empty(&self, position: &Position) -> bool {
        self.get_piece(position).is_none()
    }

    pub fn is_position_piece(&self, position: &Position, piece: &Piece) -> bool {
        self.get_piece(position)
            .as_ref()
            .map_or(false, |got| got == piece)
    }

    pub fn is_position_color(&self, position: &Position, color: &Color) -> bool {
        self.get_piece(position)
            .as_ref()
            .map_or(false, |piece| piece.0 == *color)
    }

    pub fn is_position_empty_or_color(&self, position: &Position, color: &Color) -> bool {
        self.is_position_empty(position) || self.is_position_color(position, color)
    }

    pub fn apply_move(&mut self, mv: &Move) {
        match mv {
            Move::Normal(mv) => self.apply_normal_move(mv),
            Move::DoubleAdvance(mv) => self.apply_double_advance_move(mv),
            Move::EnPassant(mv) => self.apply_en_passant_move(mv),
            Move::Promotion(mv) => self.apply_promotion_move(mv),
            Move::Castle(mv) => self.apply_castle_move(mv),
        }
    }

    pub fn apply_move_copy(&self, mv: &Move) -> Board {
        let mut new_board = self.clone();
        new_board.apply_move(mv);
        new_board
    }

    fn apply_normal_move(&mut self, mv: &NormalMove) {
        self.set_piece(&mv.to, self.get_piece(&mv.from).clone());
        self.set_piece(&mv.from, None);
    }

    fn apply_double_advance_move(&mut self, mv: &DoubleAdvanceMove) {
        self.set_piece(&mv.to, self.get_piece(&mv.from).clone());
        self.set_piece(&mv.from, None);
    }

    fn apply_en_passant_move(&mut self, mv: &EnPassantMove) {
        self.set_piece(&mv.to, self.get_piece(&mv.from).clone());
        self.set_piece(&mv.from, None);
        self.set_piece(&Position(mv.from.0, mv.to.1), None);
    }

    fn apply_promotion_move(&mut self, mv: &PromotionMove) {
        self.set_piece(&mv.pawn.from, Some(mv.promotion.clone()));
        self.apply_normal_move(&mv.pawn);
    }

    fn apply_castle_move(&mut self, mv: &CastleMove) {
        self.apply_normal_move(&mv.king);
        self.apply_normal_move(&mv.rook);
    }
}
