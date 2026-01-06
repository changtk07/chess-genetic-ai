use std::ops::*;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    const fn idx(self) -> usize {
        self as usize
    }

    #[inline]
    pub(super) fn flip(self) -> Self {
        unsafe { std::mem::transmute(1 - (self as u8)) }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum Piece {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

impl Piece {
    #[inline]
    pub(super) const fn pawn(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6) }
    }

    #[inline]
    pub(super) const fn rook(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 1) }
    }

    #[inline]
    pub(super) const fn knight(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 2) }
    }

    #[inline]
    pub(super) const fn bishop(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 3) }
    }

    #[inline]
    pub(super) const fn queen(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 4) }
    }

    #[inline]
    pub(super) const fn king(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 5) }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum MoveType {
    Standard,
    DoublePush,
    EnPassant,
    PromotionRook,
    PromotionKnight,
    PromotionBishop,
    PromotionQueen,
    KingSideCastling,
    QueenSideCastling,
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct Position(pub(super) u8);

impl Position {
    pub(super) const H1: Self = Position(7);
    pub(super) const A1: Self = Position(0);
    pub(super) const H8: Self = Position(63);
    pub(super) const A8: Self = Position(56);
    pub(super) const F1: Self = Position(5);
    pub(super) const D1: Self = Position(3);
    pub(super) const F8: Self = Position(61);
    pub(super) const D8: Self = Position(59);

    pub(super) const fn middle_of(a: Self, b: Self) -> Self {
        Position((a.0 + b.0) / 2)
    }

    pub(super) const fn en_passant_captured(from: Self, to: Self) -> Self {
        Position(((from.0 >> 3) << 3) + (to.0 & 0b111)) // = (from/8)*8 + (to%8)
    }

    #[inline]
    const fn mask(self) -> BitBoard {
        BitBoard(1u64 << self.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct CastlingRights(u8);

impl CastlingRights {
    const MASKS: [u8; 64] = [
        11, 15, 15, 15, 3, 15, 15, 7, // 0-7
        15, 15, 15, 15, 15, 15, 15, 15, // 8-15
        15, 15, 15, 15, 15, 15, 15, 15, // 16-23
        15, 15, 15, 15, 15, 15, 15, 15, // 24-31
        15, 15, 15, 15, 15, 15, 15, 15, // 32-39
        15, 15, 15, 15, 15, 15, 15, 15, // 40-47
        15, 15, 15, 15, 15, 15, 15, 15, // 48-55
        14, 15, 15, 15, 12, 15, 15, 13, // 56-63
    ];

    pub(super) fn new() -> Self {
        Self(0b1111)
    }

    #[inline]
    pub(super) fn update(&mut self, from: Position, to: Position) {
        self.0 &= Self::MASKS[from.0 as usize] & Self::MASKS[to.0 as usize];
    }
}

impl MoveType {
    fn new(val: u16) -> Self {
        match val {
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
pub struct Move(u16);

impl Move {
    pub(super) fn from(self) -> Position {
        Position(((self.0 >> 10) & 0x3F) as u8)
    }

    pub(super) fn to(self) -> Position {
        Position(((self.0 >> 4) & 0x3F) as u8)
    }

    pub(super) fn move_type(self) -> MoveType {
        MoveType::new(self.0 & 0x000F)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
struct BitBoard(u64);

impl BitBoard {
    const EMPTY: BitBoard = BitBoard(0);

    #[inline]
    fn is_empty(self, position: Position) -> bool {
        self & position.mask() == Self::EMPTY
    }

    #[inline]
    fn set(self, position: Position) -> BitBoard {
        self | position.mask()
    }

    #[inline]
    fn unset(self, position: Position) -> BitBoard {
        self & !position.mask()
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl Shl<u32> for BitBoard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self {
        BitBoard(self.0 << rhs)
    }
}

impl ShlAssign<u32> for BitBoard {
    #[inline]
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl Shr<u32> for BitBoard {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self {
        BitBoard(self.0 >> rhs)
    }
}

impl ShrAssign<u32> for BitBoard {
    #[inline]
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}

struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    fn new() -> Self {
        Self([
            Some(Piece::WhiteRook),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteBishop),
            Some(Piece::WhiteQueen),
            Some(Piece::WhiteKing),
            Some(Piece::WhiteBishop),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteRook),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            Some(Piece::WhitePawn),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackRook),
            Some(Piece::BlackKnight),
            Some(Piece::BlackBishop),
            Some(Piece::BlackQueen),
            Some(Piece::BlackKing),
            Some(Piece::BlackBishop),
            Some(Piece::BlackKnight),
            Some(Piece::BlackRook),
        ])
    }
}

impl Index<Position> for Mailbox {
    type Output = Option<Piece>;
    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<Position> for Mailbox {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

pub(super) struct Board {
    pieces: [BitBoard; 12],
    mailbox: Mailbox,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            pieces: [
                BitBoard(0x000000000000FF00), // White Pawns
                BitBoard(0x0000000000000081), // White Rooks
                BitBoard(0x0000000000000042), // White Knights
                BitBoard(0x0000000000000024), // White Bishops
                BitBoard(0x0000000000000008), // White Queens
                BitBoard(0x0000000000000010), // White Kings
                BitBoard(0x00FF000000000000), // Black Pawns
                BitBoard(0x8100000000000000), // Black Rooks
                BitBoard(0x4200000000000000), // Black Knights
                BitBoard(0x2400000000000000), // Black Bishops
                BitBoard(0x0800000000000000), // Black Queens
                BitBoard(0x1000000000000000), // Black Kings
            ],
            mailbox: Mailbox::new(),
        }
    }

    pub(super) fn set_piece(&mut self, position: Position, piece: Piece) {
        if let Some(idx) = self.mailbox[position] {
            self.pieces[idx as usize] = self.pieces[idx as usize].unset(position);
        }
        self.pieces[piece as usize] = self.pieces[piece as usize].set(position);
        self.mailbox[position] = Some(piece);
    }

    pub(super) fn unset_piece(&mut self, position: Position) {
        let Some(idx) = self.mailbox[position] else {
            return;
        };

        self.pieces[idx as usize] = self.pieces[idx as usize].unset(position);
        self.mailbox[position] = None;
    }

    pub(super) fn move_piece(
        &mut self,
        from: Position,
        to: Position,
    ) -> (Option<Piece>, Option<Piece>) {
        let Some(from_idx) = self.mailbox[from] else {
            return (None, None);
        };

        let next_from_board = self.pieces[from_idx as usize].unset(from).set(to);

        let captured = self.mailbox[to];
        if let Some(to_idx) = captured {
            self.pieces[to_idx as usize] = self.pieces[to_idx as usize].unset(to);
        }

        self.pieces[from_idx as usize] = next_from_board;
        self.mailbox[to] = Some(from_idx);
        self.mailbox[from] = None;
        (Some(from_idx), captured)
    }
}
