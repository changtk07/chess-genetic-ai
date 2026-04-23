use std::ops::*;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub(crate) const fn flip(self) -> Self {
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
pub(crate) enum Piece {
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
    pub(crate) const fn pawn(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6) }
    }

    pub(crate) const fn rook(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 1) }
    }

    pub(crate) const fn knight(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 2) }
    }

    pub(crate) const fn bishop(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 3) }
    }

    pub(crate) const fn queen(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 4) }
    }

    pub(crate) const fn king(color: Color) -> Self {
        unsafe { std::mem::transmute((color as u8) * 6 + 5) }
    }

    pub(crate) const fn color(self) -> Color {
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
pub(crate) enum MoveType {
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
pub(crate) struct Position(pub(crate) u8);

impl Position {
    pub(crate) const A1: Self = Self(0);
    pub(crate) const C1: Self = Self(2);
    pub(crate) const D1: Self = Self(3);
    pub(crate) const E1: Self = Self(4);
    pub(crate) const F1: Self = Self(5);
    pub(crate) const G1: Self = Self(6);
    pub(crate) const H1: Self = Self(7);
    pub(crate) const A8: Self = Self(56);
    pub(crate) const C8: Self = Self(58);
    pub(crate) const D8: Self = Self(59);
    pub(crate) const E8: Self = Self(60);
    pub(crate) const F8: Self = Self(61);
    pub(crate) const G8: Self = Self(62);
    pub(crate) const H8: Self = Self(63);
    pub(crate) const QS_CASTLE_ROOK: [(Self, Self); 2] =
        [(Self::A1, Self::D1), (Self::A8, Self::D8)];
    pub(crate) const KS_CASTLE_ROOK: [(Self, Self); 2] =
        [(Self::H1, Self::F1), (Self::H8, Self::F8)];

    pub(crate) const fn middle_of(a: Self, b: Self) -> Self {
        Self((a.0 + b.0) / 2)
    }

    pub(crate) const fn en_passant_captured(from: Self, to: Self) -> Self {
        Self(((from.0 >> 3) << 3) + (to.0 & 0b111)) // = (from/8)*8 + (to%8)
    }

    pub(crate) const fn mask(self) -> BitBoard {
        BitBoard(1u64 << self.0)
    }

    pub(crate) const fn offset_unchecked(self, delta: i8) -> Self {
        Self((self.0 as i8 + delta) as u8)
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
pub(crate) struct CastlingRights(u8);

impl CastlingRights {
    // Bit layout (4 bits total, LSB first):
    //   Bit 0 (0b0001): Black queenside  — cleared when A8 rook or E8 king moves
    //   Bit 1 (0b0010): Black kingside   — cleared when H8 rook or E8 king moves
    //   Bit 2 (0b0100): White queenside  — cleared when A1 rook or E1 king moves
    //   Bit 3 (0b1000): White kingside   — cleared when H1 rook or E1 king moves
    pub(crate) const QUEEN_SIDE: [Self; 2] = [Self(0b0100), Self(0b0001)];
    pub(crate) const KING_SIDE: [Self; 2] = [Self(0b1000), Self(0b0010)];

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

    pub(crate) const fn new() -> Self {
        Self(0b1111)
    }

    pub(crate) const fn has(self, right: Self) -> bool {
        (self.0 & right.0) != 0
    }

    pub(crate) fn update(self, from: Position, to: Position) -> Self {
        Self(self.0 & Self::MASKS[from] & Self::MASKS[to])
    }
}

impl MoveType {
    const fn new(val: u16) -> Self {
        match val {
            1 => Self::DoublePush,
            2 => Self::EnPassant,
            3 => Self::PromotionRook,
            4 => Self::PromotionKnight,
            5 => Self::PromotionBishop,
            6 => Self::PromotionQueen,
            7 => Self::KingSideCastling,
            8 => Self::QueenSideCastling,
            _ => Self::Standard,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Move(u16);

impl Move {
    pub(crate) const QUEEN_SIDE_CASTLING: [Self; 2] = [
        Self::new(Position::E1, Position::C1, MoveType::QueenSideCastling),
        Self::new(Position::E8, Position::C8, MoveType::QueenSideCastling),
    ];

    pub(crate) const KING_SIDE_CASTLING: [Self; 2] = [
        Self::new(Position::E1, Position::G1, MoveType::KingSideCastling),
        Self::new(Position::E8, Position::G8, MoveType::KingSideCastling),
    ];

    pub(crate) const fn new(from: Position, to: Position, move_type: MoveType) -> Self {
        let mut mv = 0u16;
        mv |= (from.0 as u16 & 0x3F) << 10;
        mv |= (to.0 as u16 & 0x3F) << 4;
        mv |= (move_type as u16) & 0x0F;
        Self(mv)
    }

    pub(crate) const fn unwrap(self) -> (Position, Position, MoveType) {
        (self.from(), self.to(), self.move_type())
    }

    pub(crate) const fn from(self) -> Position {
        Position(((self.0 >> 10) & 0x3F) as u8)
    }

    pub(crate) const fn to(self) -> Position {
        Position(((self.0 >> 4) & 0x3F) as u8)
    }

    pub(crate) const fn move_type(self) -> MoveType {
        MoveType::new(self.0 & 0x0F)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
    NorthEast = 4,
    NorthWest = 5,
    SouthEast = 6,
    SouthWest = 7,
}

impl<T> Index<Direction> for [T; 8] {
    type Output = T;
    #[inline]
    fn index(&self, index: Direction) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Direction> for [T; 8] {
    #[inline]
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct BitBoard(u64);

impl BitBoard {
    pub(crate) const EMPTY: Self = Self(0);
    pub(crate) const FULL: Self = Self(!0);

    pub(crate) const QS_CASTLE_PATH: [Self; 2] =
        [Self(0x000000000000001C), Self(0x1C00000000000000)];

    pub(crate) const KS_CASTLE_PATH: [Self; 2] =
        [Self(0x0000000000000070), Self(0x7000000000000000)];

    pub(crate) const QS_CASTLE_GAP: [Self; 2] =
        [Self(0x000000000000000E), Self(0x0E00000000000000)];

    pub(crate) const KS_CASTLE_GAP: [Self; 2] =
        [Self(0x0000000000000060), Self(0x6000000000000000)];

    pub(crate) const RANK_MASKS: [Self; 8] = [
        Self(0x00000000000000FF),
        Self(0x000000000000FF00),
        Self(0x0000000000FF0000),
        Self(0x00000000FF000000),
        Self(0x000000FF00000000),
        Self(0x0000FF0000000000),
        Self(0x00FF000000000000),
        Self(0xFF00000000000000),
    ];

    pub(crate) const KING_ATTACK_MASKS: [Self; 64] = {
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

    pub(crate) const KNIGHT_ATTACK_MASKS: [Self; 64] = {
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
            masks[i] = Self(mask);
            i += 1;
        }
        masks
    };

    pub(crate) const PAWN_ATTACK_MASKS: [[Self; 64]; 2] = {
        let mut white_masks = [Self::EMPTY; 64];
        let mut i = 0;
        while i < 56 {
            let mut mask = 0u64;
            let file = i % 8;
            mask |= if file < 7 { 1u64 << (i + 9) } else { 0 };
            mask |= if file > 0 { 1u64 << (i + 7) } else { 0 };
            white_masks[i] = Self(mask);
            i += 1;
        }
        let mut black_masks = [Self::EMPTY; 64];
        let mut i = 63;
        while i > 7 {
            let mut mask = 0u64;
            let file = i % 8;
            mask |= if file < 7 { 1u64 << (i - 7) } else { 0 };
            mask |= if file > 0 { 1u64 << (i - 9) } else { 0 };
            black_masks[i] = Self(mask);
            i -= 1;
        }
        [white_masks, black_masks]
    };

    const RAYS: [[Self; 64]; 8] = {
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
            generate_ray(-1, 1),  // North West
            generate_ray(1, -1),  // South East
            generate_ray(-1, -1), // South West
        ]
    };

    const BETWEEN_MASKS: [[Self; 64]; 64] = {
        const fn set_bits_between(start: usize, end: usize, step: usize) -> BitBoard {
            let mut mask = 0u64;
            let mut i = start + step;
            while i < end {
                mask |= 1u64 << i;
                i += step;
            }
            BitBoard(mask)
        }

        let mut masks = [[BitBoard::EMPTY; 64]; 64];

        let mut i = 0;
        while i < 64 {
            let (rank_i, file_i) = (i as i8 / 8, i as i8 % 8);
            let mut j = i + 1;
            while j < 64 {
                let (rank_j, file_j) = (j as i8 / 8, j as i8 % 8);
                if rank_i == rank_j {
                    let mask = set_bits_between(i, j, 1);
                    masks[i][j] = mask;
                    masks[j][i] = mask;
                } else if file_i == file_j {
                    let mask = set_bits_between(i, j, 8);
                    masks[i][j] = mask;
                    masks[j][i] = mask;
                } else if rank_j - rank_i == file_j - file_i {
                    let mask = set_bits_between(i, j, 9);
                    masks[i][j] = mask;
                    masks[j][i] = mask;
                } else if rank_j - rank_i == file_i - file_j {
                    let mask = set_bits_between(i, j, 7);
                    masks[i][j] = mask;
                    masks[j][i] = mask;
                }
                j += 1;
            }
            i += 1;
        }

        masks
    };

    pub(crate) const fn is_empty(self, position: Position) -> bool {
        self.0 & position.mask().0 == 0
    }

    pub(crate) const fn is_not_empty(self, position: Position) -> bool {
        self.0 & position.mask().0 != 0
    }

    const fn has_only_one_set(self) -> bool {
        self.0.is_power_of_two()
    }

    const fn set(self, position: Position) -> Self {
        Self(self.0 | position.mask().0)
    }

    const fn unset(self, position: Position) -> Self {
        Self(self.0 & !position.mask().0)
    }

    fn lsb(self) -> Position {
        debug_assert!(self.0 != 0, "Called lsb() on an empty BitBoard");
        Position(self.0.trailing_zeros() as u8)
    }

    fn msb(self) -> Position {
        debug_assert!(self.0 != 0, "Called msb() on an empty BitBoard");
        Position(63 - self.0.leading_zeros() as u8)
    }

    pub(crate) fn rook_attack_mask(position: Position, occupancy: Self) -> Self {
        let mut mask = Self::EMPTY;

        // North (Index 0) - Positive direction, blocker is LSB
        let north = Self::RAYS[Direction::North][position];
        let north_blockers = north & occupancy;
        mask |= if north_blockers.0 != 0 {
            north ^ Self::RAYS[Direction::North][north_blockers.lsb()]
        } else {
            north
        };

        // South (Index 1) - Negative direction, blocker is MSB
        let south = Self::RAYS[Direction::South][position];
        let south_blockers = south & occupancy;
        mask |= if south_blockers.0 != 0 {
            south ^ Self::RAYS[Direction::South][south_blockers.msb()]
        } else {
            south
        };

        // East (Index 2) - Positive direction, blocker is LSB
        let east = Self::RAYS[Direction::East][position];
        let east_blockers = east & occupancy;
        mask |= if east_blockers.0 != 0 {
            east ^ Self::RAYS[Direction::East][east_blockers.lsb()]
        } else {
            east
        };

        // West (Index 3) - Negative direction, blocker is MSB
        let west = Self::RAYS[Direction::West][position];
        let west_blockers = west & occupancy;
        mask |= if west_blockers.0 != 0 {
            west ^ Self::RAYS[Direction::West][west_blockers.msb()]
        } else {
            west
        };

        mask
    }

    pub(crate) fn bishop_attack_mask(position: Position, occupancy: Self) -> Self {
        let mut mask = Self::EMPTY;

        // North East (Index 4) - Positive direction, blocker is LSB
        let north_east = Self::RAYS[Direction::NorthEast][position];
        let north_east_blockers = north_east & occupancy;
        mask |= if north_east_blockers.0 != 0 {
            north_east ^ Self::RAYS[Direction::NorthEast][north_east_blockers.lsb()]
        } else {
            north_east
        };

        // North West (Index 5) - Positive direction, blocker is LSB
        let north_west = Self::RAYS[Direction::NorthWest][position];
        let north_west_blockers = north_west & occupancy;
        mask |= if north_west_blockers.0 != 0 {
            north_west ^ Self::RAYS[Direction::NorthWest][north_west_blockers.lsb()]
        } else {
            north_west
        };

        // South East (Index 6) - Negative direction, blocker is MSB
        let south_east = Self::RAYS[Direction::SouthEast][position];
        let south_east_blockers = south_east & occupancy;
        mask |= if south_east_blockers.0 != 0 {
            south_east ^ Self::RAYS[Direction::SouthEast][south_east_blockers.msb()]
        } else {
            south_east
        };

        // South West (Index 7) - Negative direction, blocker is MSB
        let south_west = Self::RAYS[Direction::SouthWest][position];
        let south_west_blockers = south_west & occupancy;
        mask |= if south_west_blockers.0 != 0 {
            south_west ^ Self::RAYS[Direction::SouthWest][south_west_blockers.msb()]
        } else {
            south_west
        };

        mask
    }

    pub(crate) fn queen_attack_mask(position: Position, occupancy: Self) -> Self {
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

pub(crate) struct MoveGenMasks {
    pub(crate) pin_rays: [BitBoard; 64],
    pub(crate) check_mask: BitBoard,
    pub(crate) checker_count: u8,
}

impl MoveGenMasks {
    const fn new() -> Self {
        Self {
            pin_rays: [BitBoard::FULL; 64],
            check_mask: BitBoard::FULL,
            checker_count: 0,
        }
    }

    pub(crate) const fn is_double_check(&self) -> bool {
        self.checker_count >= 2
    }
}

pub(crate) struct Board {
    pub(crate) pieces: [BitBoard; 12],
    pub(crate) colors: [BitBoard; 2],
    pub(crate) occupancy: BitBoard,
    pub(crate) mailbox: [Option<Piece>; 64],
}

impl Board {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn is_occupied(&self, position: Position) -> bool {
        self.occupancy.is_not_empty(position)
    }

    pub(crate) fn is_not_occupied(&self, position: Position) -> bool {
        self.occupancy.is_empty(position)
    }

    pub(crate) fn color_attack_mask(&self, color: Color, ignroe: BitBoard) -> BitBoard {
        let mut mask = BitBoard::EMPTY;
        let occupancy_with_ignore = self.occupancy & !ignroe;

        mask = self.pieces[Piece::pawn(color)]
            .map(|p| BitBoard::PAWN_ATTACK_MASKS[color][p])
            .fold(mask, |acc, attack| acc | attack);

        mask = self.pieces[Piece::knight(color)]
            .map(|p| BitBoard::KNIGHT_ATTACK_MASKS[p])
            .fold(mask, |acc, attack| acc | attack);

        mask = self.pieces[Piece::king(color)]
            .map(|p| BitBoard::KING_ATTACK_MASKS[p])
            .fold(mask, |acc, attack| acc | attack);

        mask = self.pieces[Piece::bishop(color)]
            .map(|p| BitBoard::bishop_attack_mask(p, occupancy_with_ignore))
            .fold(mask, |acc, attack| acc | attack);

        mask = self.pieces[Piece::rook(color)]
            .map(|p| BitBoard::rook_attack_mask(p, occupancy_with_ignore))
            .fold(mask, |acc, attack| acc | attack);

        mask = self.pieces[Piece::queen(color)]
            .map(|p| BitBoard::queen_attack_mask(p, occupancy_with_ignore))
            .fold(mask, |acc, attack| acc | attack);

        mask
    }

    pub(crate) fn move_gen_masks(&self, color: Color) -> MoveGenMasks {
        let mut masks = MoveGenMasks::new();
        let opponent = color.flip();
        let king_position = self.pieces[Piece::king(color)].lsb();

        let opponent_straight_slider =
            self.pieces[Piece::rook(opponent)] | self.pieces[Piece::queen(opponent)];
        let opponent_diagonal_slider =
            self.pieces[Piece::bishop(opponent)] | self.pieces[Piece::queen(opponent)];

        for (direction, sliders, is_positive_ray) in [
            (Direction::North, opponent_straight_slider, true),
            (Direction::South, opponent_straight_slider, false),
            (Direction::East, opponent_straight_slider, true),
            (Direction::West, opponent_straight_slider, false),
            (Direction::NorthEast, opponent_diagonal_slider, true),
            (Direction::NorthWest, opponent_diagonal_slider, true),
            (Direction::SouthEast, opponent_diagonal_slider, false),
            (Direction::SouthWest, opponent_diagonal_slider, false),
        ] {
            let attack_mask = BitBoard::RAYS[direction][king_position] & sliders;
            if attack_mask == BitBoard::EMPTY {
                continue;
            }

            let attacker_position = if is_positive_ray {
                attack_mask.lsb()
            } else {
                attack_mask.msb()
            };

            let between_mask = BitBoard::BETWEEN_MASKS[attacker_position][king_position];
            let blocker_mask = between_mask & self.occupancy;

            if blocker_mask == BitBoard::EMPTY {
                masks.check_mask &= between_mask | attacker_position.mask();
                masks.checker_count += 1;
                if masks.is_double_check() {
                    return masks;
                }
                continue;
            }

            let friendly_blocker_mask = between_mask & self.colors[color];
            if blocker_mask.has_only_one_set() && blocker_mask == friendly_blocker_mask {
                masks.pin_rays[blocker_mask.lsb()] = between_mask | attacker_position.mask();
            }
        }

        for attacker_position in
            BitBoard::KNIGHT_ATTACK_MASKS[king_position] & self.pieces[Piece::knight(opponent)]
        {
            masks.check_mask &= attacker_position.mask();
            masks.checker_count += 1;
            if masks.is_double_check() {
                return masks;
            }
        }

        for attacker_position in
            BitBoard::PAWN_ATTACK_MASKS[color][king_position] & self.pieces[Piece::pawn(opponent)]
        {
            masks.check_mask &= attacker_position.mask();
            masks.checker_count += 1;
            if masks.is_double_check() {
                return masks;
            }
        }

        masks
    }

    pub(crate) fn set_piece(&mut self, position: Position, piece: Piece) {
        if let Some(idx) = self.mailbox[position] {
            self.pieces[idx] = self.pieces[idx].unset(position);
            self.colors[idx.color()] = self.colors[idx.color()].unset(position);
        }
        self.pieces[piece] = self.pieces[piece].set(position);
        self.mailbox[position] = Some(piece);
        self.occupancy = self.occupancy.set(position);
        self.colors[piece.color()] = self.colors[piece.color()].set(position);
    }

    pub(crate) fn unset_piece(&mut self, position: Position) {
        let Some(idx) = self.mailbox[position] else {
            return;
        };
        self.pieces[idx] = self.pieces[idx].unset(position);
        self.mailbox[position] = None;
        self.occupancy = self.occupancy.unset(position);
        self.colors[idx.color()] = self.colors[idx.color()].unset(position);
    }

    pub(crate) fn move_piece(
        &mut self,
        from: Position,
        to: Position,
    ) -> (Option<Piece>, Option<Piece>) {
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
