use super::piece::*;

#[derive(Clone, PartialEq)]
pub struct Position(pub i8, pub i8);

impl Position {
    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.0) && (0..8).contains(&self.1)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.1 as u8 + b'a') as char, self.0 + 1)
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
                Some(Piece(Color::White, PieceType::Rook)),
                Some(Piece(Color::White, PieceType::Knight)),
                Some(Piece(Color::White, PieceType::Bishop)),
                Some(Piece(Color::White, PieceType::Queen)),
                Some(Piece(Color::White, PieceType::King)),
                Some(Piece(Color::White, PieceType::Bishop)),
                Some(Piece(Color::White, PieceType::Knight)),
                Some(Piece(Color::White, PieceType::Rook)),
            ],
            [
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
                Some(Piece(Color::White, PieceType::Pawn)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
                Some(Piece(Color::Black, PieceType::Pawn)),
            ],
            [
                Some(Piece(Color::Black, PieceType::Rook)),
                Some(Piece(Color::Black, PieceType::Knight)),
                Some(Piece(Color::Black, PieceType::Bishop)),
                Some(Piece(Color::Black, PieceType::Queen)),
                Some(Piece(Color::Black, PieceType::King)),
                Some(Piece(Color::Black, PieceType::Bishop)),
                Some(Piece(Color::Black, PieceType::Knight)),
                Some(Piece(Color::Black, PieceType::Rook)),
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

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Position, &Option<Piece>),
    {
        for (x, row) in self.0.iter().enumerate() {
            for (y, piece) in row.iter().enumerate() {
                f(&Position(x as i8, y as i8), piece)
            }
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

    pub fn is_position_piece_type(&self, position: &Position, piece_type: &PieceType) -> bool {
        self.get_piece(position)
            .as_ref()
            .map_or(false, |piece| piece.1 == *piece_type)
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
                Some(Piece(color, PieceType::Pawn)) if *color == *opponent,
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
                    Some(Piece(color, PieceType::Rook | PieceType::Queen)) if color == opponent => {
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
                Some(Piece(color, PieceType::Knight)) if *color == *opponent,
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
                    Some(Piece(color, PieceType::Bishop | PieceType::Queen))
                        if color == opponent =>
                    {
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
                Some(Piece(color, PieceType::King)) if *color == *opponent,
            )
        })
    }
}
