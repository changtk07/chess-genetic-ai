use super::piece::{Color, Piece, Type};
use super::r#move::{
    CastleMove, DoubleAdvanceMove, EnPassantMove, Move, NormalMove, PromotionMove,
};

#[derive(Clone, PartialEq)]
pub struct Position(pub i8, pub i8);

impl Position {
    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.0) && (0..8).contains(&self.1)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.0 as u8 + b'a') as char, self.1 + 1)
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

    pub fn set_piece(&mut self, &Position(x, y): &Position, piece: Option<Piece>) {
        if let Some(cell) = self
            .0
            .get_mut(x as usize)
            .and_then(|row| row.get_mut(y as usize))
        {
            *cell = piece;
        }
    }

    pub fn is_position_empty(&self, position: &Position) -> bool {
        position.is_valid() && self.get_piece(position).is_none()
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

    pub fn is_position_in_check(&self, position: &Position, opponent: &Color) -> bool {
        position.is_valid()
            && (self.is_position_in_check_by_pawn(position, opponent)
                || self.is_position_in_check_by_rook_or_queen(position, opponent)
                || self.is_position_in_check_by_knight(position, opponent)
                || self.is_position_in_check_by_bishop_or_queen(position, opponent)
                || self.is_position_in_check_by_king(position, opponent))
    }

    fn is_position_in_check_by_pawn(&self, &Position(x, y): &Position, opponent: &Color) -> bool {
        let rank = match opponent {
            Color::White => x - 1,
            Color::Black => x + 1,
        };

        let positions = [Position(rank, y - 1), Position(rank, y + 1)];

        positions.iter().any(|pos| {
            matches!(
                self.get_piece(pos),
                Some(Piece(color, Type::Pawn)) if *color == *opponent,
            )
        })
    }

    fn is_position_in_check_by_rook_or_queen(
        &self,
        &Position(x, y): &Position,
        opponent: &Color,
    ) -> bool {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for i in 1..8 {
                let pos = Position(x + i * dx, y + i * dy);
                if !pos.is_valid() {
                    break;
                }

                match self.get_piece(&pos) {
                    None => continue,
                    Some(Piece(color, Type::Rook | Type::Queen)) if color == opponent => {
                        return true
                    }
                    _ => break,
                }
            }
        }

        false
    }

    fn is_position_in_check_by_knight(&self, &Position(x, y): &Position, opponent: &Color) -> bool {
        let positions = [
            Position(x + 1, y - 2),
            Position(x + 2, y - 1),
            Position(x + 2, y + 1),
            Position(x + 1, y + 2),
            Position(x - 1, y + 2),
            Position(x - 2, y + 1),
            Position(x - 2, y - 1),
            Position(x - 1, y - 2),
        ];

        positions.iter().any(|pos| {
            matches!(
                self.get_piece(pos),
                Some(Piece(color, Type::Knight)) if *color == *opponent,
            )
        })
    }

    fn is_position_in_check_by_bishop_or_queen(
        &self,
        &Position(x, y): &Position,
        opponent: &Color,
    ) -> bool {
        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            for i in 1..8 {
                let pos = Position(x + i * dx, y + i * dy);
                if !pos.is_valid() {
                    break;
                }

                match self.get_piece(&pos) {
                    None => continue,
                    Some(Piece(color, Type::Bishop | Type::Queen)) if color == opponent => {
                        return true
                    }
                    _ => break,
                }
            }
        }

        false
    }

    fn is_position_in_check_by_king(&self, &Position(x, y): &Position, opponent: &Color) -> bool {
        let positions = [
            Position(x - 1, y - 1),
            Position(x - 1, y),
            Position(x - 1, y + 1),
            Position(x + 1, y - 1),
            Position(x + 1, y),
            Position(x + 1, y + 1),
            Position(x, y - 1),
            Position(x, y + 1),
        ];

        positions.iter().any(|pos| {
            matches!(
                self.get_piece(pos),
                Some(Piece(color, Type::King)) if *color == *opponent,
            )
        })
    }
}
