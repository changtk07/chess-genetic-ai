use std::ops::*;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline]
    pub fn flip(self) -> Self {
        unsafe { std::mem::transmute(1 - (self as u8)) }
    }
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;
    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Color> for [T; 2] {
    #[inline]
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Piece {
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
    pub const fn pawn(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6) }
    }

    #[inline]
    pub const fn rook(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 1) }
    }

    #[inline]
    pub const fn knight(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 2) }
    }

    #[inline]
    pub const fn bishop(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 3) }
    }

    #[inline]
    pub const fn queen(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 4) }
    }

    #[inline]
    pub const fn king(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 5) }
    }

    #[inline]
    pub const fn color(self) -> Color {
        if (self as u8) < 6 {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl<T> Index<Piece> for [T; 12] {
    type Output = T;
    #[inline]
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Piece> for [T; 12] {
    #[inline]
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MoveType {
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
pub struct Position(pub u8);

impl Position {
    pub const H1: Self = Position(7);
    pub const A1: Self = Position(0);
    pub const H8: Self = Position(63);
    pub const A8: Self = Position(56);
    pub const F1: Self = Position(5);
    pub const D1: Self = Position(3);
    pub const F8: Self = Position(61);
    pub const D8: Self = Position(59);

    pub const fn middle_of(a: Self, b: Self) -> Self {
        Position((a.0 + b.0) / 2)
    }

    pub const fn en_passant_captured(from: Self, to: Self) -> Self {
        Position(((from.0 >> 3) << 3) + (to.0 & 0b111)) // = (from/8)*8 + (to%8)
    }

    #[inline]
    pub const fn mask(self) -> BitBoard {
        BitBoard(1u64 << self.0)
    }

    #[inline]
    pub const fn offset_unchecked(self, delta: i8) -> Position {
        Position((self.0 as i8 + delta) as u8)
    }
}

impl<T> Index<Position> for [T; 64] {
    type Output = T;
    #[inline]
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<Position> for [T; 64] {
    #[inline]
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CastlingRights(u8);

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

    pub fn new() -> Self {
        Self(0b1111)
    }

    #[inline]
    pub fn update(&mut self, from: Position, to: Position) {
        self.0 &= Self::MASKS[from] & Self::MASKS[to];
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
    pub fn new(from: Position, to: Position, move_type: MoveType) -> Self {
        let mut mv = 0u16;
        mv |= (from.0 as u16 & 0x3F) << 10;
        mv |= (to.0 as u16 & 0x3F) << 4;
        mv |= (move_type as u16) & 0x0F;
        Move(mv)
    }

    pub fn from(self) -> Position {
        Position(((self.0 >> 10) & 0x3F) as u8)
    }

    pub fn to(self) -> Position {
        Position(((self.0 >> 4) & 0x3F) as u8)
    }

    pub fn move_type(self) -> MoveType {
        MoveType::new(self.0 & 0x0F)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub const RANK_MASKS: [BitBoard; 8] = [
        BitBoard(0x00000000000000FF),
        BitBoard(0x000000000000FF00),
        BitBoard(0x0000000000FF0000),
        BitBoard(0x00000000FF000000),
        BitBoard(0x000000FF00000000),
        BitBoard(0x0000FF0000000000),
        BitBoard(0x00FF000000000000),
        BitBoard(0xFF00000000000000),
    ];

    pub const KING_ATTACK_MASKS: [BitBoard; 64] = {
        let mut masks = [Self::EMPTY; 64];
        let mut i = 0;
        while i < 64 {
            let mut mask = 0u64;
            let (rank, file) = (i / 8, i % 8);
            if rank > 0 {
                mask |= 1u64 << (i - 8);
                mask |= if file > 0 { 1u64 << (i - 9) } else { 0 };
                mask |= if file < 7 { 1u64 << (i - 7) } else { 0 };
            }
            if rank < 7 {
                mask |= 1u64 << (i + 8);
                mask |= if file > 0 { 1u64 << (i + 7) } else { 0 };
                mask |= if file < 7 { 1u64 << (i + 9) } else { 0 };
            }
            mask |= if file > 0 { 1u64 << (i - 1) } else { 0 };
            mask |= if file < 7 { 1u64 << (i + 1) } else { 0 };
            masks[i] = BitBoard(mask);
            i += 1;
        }
        masks
    };

    pub const KNIGHT_ATTACK_MASKS: [BitBoard; 64] = {
        let mut masks = [Self::EMPTY; 64];
        let mut i = 0;
        while i < 64 {
            let mut mask = 0u64;
            let (rank, file) = (i / 8, i % 8);
            if rank < 6 {
                mask |= if file < 7 { 1u64 << (i + 17) } else { 0 };
                mask |= if file > 0 { 1u64 << (i + 15) } else { 0 };
            }
            if rank < 7 {
                mask |= if file < 6 { 1u64 << (i + 10) } else { 0 };
                mask |= if file > 1 { 1u64 << (i + 6) } else { 0 };
            }
            if rank > 0 {
                mask |= if file < 6 { 1u64 << (i - 6) } else { 0 };
                mask |= if file > 1 { 1u64 << (i - 10) } else { 0 };
            }
            if rank > 1 {
                mask |= if file < 7 { 1u64 << (i - 15) } else { 0 };
                mask |= if file > 0 { 1u64 << (i - 17) } else { 0 };
            }
            masks[i] = BitBoard(mask);
            i += 1;
        }
        masks
    };

    pub const PAWN_ATTACK_MASKS: [[BitBoard; 64]; 2] = {
        let mut white_masks = [Self::EMPTY; 64];
        let mut i = 0;
        while i < 56 {
            let mut mask = 0u64;
            let (rank, file) = (i / 8, i % 8);
            mask |= if file < 7 { 1u64 << (i + 9) } else { 0 };
            mask |= if file > 0 { 1u64 << (i + 7) } else { 0 };
            white_masks[i] = BitBoard(mask);
            i += 1;
        }
        let mut black_masks = [Self::EMPTY; 64];
        let mut i = 63;
        while i > 7 {
            let mut mask = 0u64;
            let (rank, file) = (i / 8, i % 8);
            mask |= if file < 7 { 1u64 << (i - 7) } else { 0 };
            mask |= if file > 0 { 1u64 << (i - 9) } else { 0 };
            black_masks[i] = BitBoard(mask);
            i -= 1;
        }
        [white_masks, black_masks]
    };

    const RAYS: [[BitBoard; 64]; 8] = {
        const fn generate_ray(df: i8, dr: i8) -> [BitBoard; 64] {
            let mut masks = [BitBoard(0); 64];
            let mut i = 0;
            while i < 64 {
                let mut mask = 0u64;
                let (mut rank, mut file) = ((i / 8) as i8, (i % 8) as i8);
                loop {
                    file += df;
                    rank += dr;
                    if file < 0 || file > 7 || rank < 0 || rank > 7 {
                        break;
                    }
                    mask |= 1u64 << (rank * 8 + file);
                }
                masks[i] = BitBoard(mask);
                i += 1;
            }
            masks
        }
        [
            generate_ray(0, 1),   // North
            generate_ray(0, -1),  // South
            generate_ray(1, 0),   // East
            generate_ray(-1, 0),  // West
            generate_ray(1, 1),   // North East
            generate_ray(-1, 1),  // South East
            generate_ray(1, -1),  // North West
            generate_ray(-1, -1), // South West
        ]
    };

    #[inline]
    pub fn is_empty(self, position: Position) -> bool {
        self & position.mask() == Self::EMPTY
    }

    #[inline]
    pub fn is_not_empty(self, position: Position) -> bool {
        self & position.mask() != Self::EMPTY
    }

    #[inline]
    fn set(self, position: Position) -> BitBoard {
        self | position.mask()
    }

    #[inline]
    fn unset(self, position: Position) -> BitBoard {
        self & !position.mask()
    }

    #[inline]
    fn lsb(self) -> Position {
        debug_assert!(self.0 != 0, "Called lsb() on an empty BitBoard");
        Position(self.0.trailing_zeros() as u8)
    }

    #[inline]
    fn msb(self) -> Position {
        debug_assert!(self.0 != 0, "Called msb() on an empty BitBoard");
        Position(63 - self.0.leading_zeros() as u8)
    }

    pub fn rook_attack_mask(position: Position, occupancy: BitBoard) -> BitBoard {
        let mut mask = BitBoard::EMPTY;

        // North (Index 0) - Positive direction, blocker is LSB
        let north = Self::RAYS[0][position];
        let north_blockers = north & occupancy;
        mask |= if north_blockers.0 != 0 {
            north ^ Self::RAYS[0][north_blockers.lsb()]
        } else {
            north
        };

        // South (Index 1) - Negative direction, blocker is MSB
        let south = Self::RAYS[1][position];
        let south_blockers = south & occupancy;
        mask |= if south_blockers.0 != 0 {
            south ^ Self::RAYS[1][south_blockers.msb()]
        } else {
            south
        };

        // East (Index 2) - Positive direction, blocker is LSB
        let east = Self::RAYS[2][position];
        let east_blockers = east & occupancy;
        mask |= if east_blockers.0 != 0 {
            east ^ Self::RAYS[2][east_blockers.lsb()]
        } else {
            east
        };

        // West (Index 3) - Negative direction, blocker is MSB
        let west = Self::RAYS[3][position];
        let west_blockers = west & occupancy;
        mask |= if west_blockers.0 != 0 {
            west ^ Self::RAYS[3][west_blockers.msb()]
        } else {
            west
        };

        mask
    }

    pub fn bishop_attack_mask(position: Position, occupancy: BitBoard) -> BitBoard {
        let mut mask = BitBoard::EMPTY;

        // North East (Index 4) - Positive direction, blocker is LSB
        let north_east = Self::RAYS[4][position];
        let north_east_blockers = north_east & occupancy;
        mask |= if north_east_blockers.0 != 0 {
            north_east ^ Self::RAYS[4][north_east_blockers.lsb()]
        } else {
            north_east
        };

        // South East (Index 5) - Negative direction, blocker is MSB
        let south_east = Self::RAYS[5][position];
        let south_east_blockers = south_east & occupancy;
        mask |= if south_east_blockers.0 != 0 {
            south_east ^ Self::RAYS[5][south_east_blockers.msb()]
        } else {
            south_east
        };

        // North West (Index 6) - Positive direction, blocker is LSB
        let north_west = Self::RAYS[6][position];
        let north_west_blockers = north_west & occupancy;
        mask |= if north_west_blockers.0 != 0 {
            north_west ^ Self::RAYS[6][north_west_blockers.lsb()]
        } else {
            north_west
        };

        // South West (Index 7) - Negative direction, blocker is MSB
        let south_west = Self::RAYS[7][position];
        let south_west_blockers = south_west & occupancy;
        mask |= if south_west_blockers.0 != 0 {
            south_west ^ Self::RAYS[7][south_west_blockers.msb()]
        } else {
            south_west
        };

        mask
    }

    pub fn queen_attack_mask(position: Position, occupancy: BitBoard) -> BitBoard {
        Self::rook_attack_mask(position, occupancy) | Self::bishop_attack_mask(position, occupancy)
    }
}

impl Iterator for BitBoard {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let p = Position(self.0.trailing_zeros() as u8);
            self.0 &= self.0 - 1;
            Some(p)
        }
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

pub struct Board {
    pub pieces: [BitBoard; 12],
    pub colors: [BitBoard; 2],
    pub occupancy: BitBoard,
    pub mailbox: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
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
            mailbox: [
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
            ],
            colors: [BitBoard(0x000000000000FFFF), BitBoard(0xFFFF000000000000)],
            occupancy: BitBoard(0xFFFF00000000FFFF),
        }
    }

    pub fn is_occupied(&self, position: Position) -> bool {
        self.occupancy.is_not_empty(position)
    }

    pub fn is_not_occupied(&self, position: Position) -> bool {
        self.occupancy.is_empty(position)
    }

    pub fn set_piece(&mut self, position: Position, piece: Piece) {
        if let Some(idx) = self.mailbox[position] {
            self.pieces[idx] = self.pieces[idx].unset(position);
        }
        self.pieces[piece] = self.pieces[piece].set(position);
        self.mailbox[position] = Some(piece);
        self.occupancy = self.occupancy.set(position);
        self.colors[piece.color()] = self.colors[piece.color()].set(position);
    }

    pub fn unset_piece(&mut self, position: Position) {
        let Some(idx) = self.mailbox[position] else {
            return;
        };
        self.pieces[idx] = self.pieces[idx].unset(position);
        self.mailbox[position] = None;
        self.occupancy = self.occupancy.unset(position);
        self.colors[idx.color()] = self.colors[idx.color()].unset(position);
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> (Option<Piece>, Option<Piece>) {
        let Some(from_idx) = self.mailbox[from] else {
            return (None, None);
        };

        let next_from_board = self.pieces[from_idx].unset(from).set(to);

        let captured = self.mailbox[to];
        if let Some(to_idx) = captured {
            self.pieces[to_idx] = self.pieces[to_idx].unset(to);
            self.colors[to_idx.color()] = self.colors[to_idx.color()].unset(to);
        }

        self.pieces[from_idx] = next_from_board;
        self.mailbox[to] = Some(from_idx);
        self.mailbox[from] = None;
        self.colors[from_idx.color()] = self.colors[from_idx.color()].unset(from).set(to);
        self.occupancy = self.occupancy.unset(from).set(to);
        (Some(from_idx), captured)
    }
}
