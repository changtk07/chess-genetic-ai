use std::ops::*;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub const fn idx(self) -> usize {
        self as usize
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MoveType {
    Standard = 0,
    DoublePush = 1,
    EnPassant = 2,
    PromotionRook = 3,
    PromotionKnight = 4,
    PromotionBishop = 5,
    PromotionQueen = 6,
    KingSideCastling = 7,
    QueenSideCastling = 8,
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(u8);

impl Position {
    pub(super) const fn mask(self) -> BitBoard {
        BitBoard(1u64 << self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Move(u16);

impl Move {
    pub(super) fn from(self) -> Position {
        Position(((self.0 >> 10) & 0x3F) as u8)
    }

    pub(super) fn to(self) -> Position {
        Position(((self.0 >> 4) & 0x3F) as u8)
    }

    pub(super) fn move_type(self) -> MoveType {
        match self.0 & 0x000F {
            1 => MoveType::DoublePush,
            2 => MoveType::EnPassant,
            3 => MoveType::PromotionRook,
            4 => MoveType::PromotionKnight,
            5 => MoveType::PromotionBishop,
            6 => MoveType::PromotionQueen,
            7 => MoveType::KingSideCastling,
            8 => MoveType::QueenSideCastling,
            _ => MoveType::Standard,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitBoard(u64);

const EMPTY_BIT_BOARD: BitBoard = BitBoard(0);

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl Shl<u32> for BitBoard {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self {
        BitBoard(self.0 << rhs)
    }
}

impl ShlAssign<u32> for BitBoard {
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl Shr<u32> for BitBoard {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self {
        BitBoard(self.0 >> rhs)
    }
}

impl ShrAssign<u32> for BitBoard {
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}

pub struct PieceBoards([BitBoard; 2]);

impl Index<Color> for PieceBoards {
    type Output = BitBoard;

    fn index(&self, index: Color) -> &Self::Output {
        &self.0[index.idx()]
    }
}

impl IndexMut<Color> for PieceBoards {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.0[index.idx()]
    }
}

pub struct Board {
    pawns: PieceBoards,
    rooks: PieceBoards,
    knights: PieceBoards,
    bishops: PieceBoards,
    queens: PieceBoards,
    kings: PieceBoards,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            pawns: PieceBoards([BitBoard(0x000000000000FF00), BitBoard(0x00FF000000000000)]),
            rooks: PieceBoards([BitBoard(0x0000000000000081), BitBoard(0x8100000000000000)]),
            knights: PieceBoards([BitBoard(0x0000000000000042), BitBoard(0x4200000000000000)]),
            bishops: PieceBoards([BitBoard(0x0000000000000024), BitBoard(0x2400000000000000)]),
            queens: PieceBoards([BitBoard(0x0000000000000008), BitBoard(0x0800000000000000)]),
            kings: PieceBoards([BitBoard(0x0000000000000010), BitBoard(0x1000000000000000)]),
        }
    }

    fn get_occupied(&self, color: Color) -> BitBoard {
        self.pawns[color]
            | self.rooks[color]
            | self.knights[color]
            | self.bishops[color]
            | self.queens[color]
            | self.kings[color]
    }

    fn find_pieces(&self, mask: BitBoard) -> BitBoard {
        if self.pawns[Color::White] & mask != EMPTY_BIT_BOARD {
            self.pawns[Color::White]
        } else if self.pawns[Color::Black] & mask != EMPTY_BIT_BOARD {
            self.pawns[Color::Black]
        } else if self.rooks[Color::White] & mask != EMPTY_BIT_BOARD {
            self.rooks[Color::White]
        } else if self.rooks[Color::Black] & mask != EMPTY_BIT_BOARD {
            self.rooks[Color::Black]
        } else if self.knights[Color::White] & mask != EMPTY_BIT_BOARD {
            self.knights[Color::White]
        } else if self.knights[Color::Black] & mask != EMPTY_BIT_BOARD {
            self.knights[Color::Black]
        } else if self.bishops[Color::White] & mask != EMPTY_BIT_BOARD {
            self.bishops[Color::White]
        } else if self.bishops[Color::Black] & mask != EMPTY_BIT_BOARD {
            self.bishops[Color::Black]
        } else if self.queens[Color::White] & mask != EMPTY_BIT_BOARD {
            self.queens[Color::White]
        } else if self.queens[Color::Black] & mask != EMPTY_BIT_BOARD {
            self.queens[Color::Black]
        } else if self.kings[Color::White] & mask != EMPTY_BIT_BOARD {
            self.kings[Color::White]
        } else {
            self.kings[Color::Black]
        }
    }

    fn move_piece(&mut self, from: Position, to: Position) {
        let pieces = self.find_pieces(from.mask());
    }
}
