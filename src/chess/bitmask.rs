use super::types::*;
use std::ops::*;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Bitmask(pub(crate) u64);

impl Bitmask {
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

    pub(crate) const RANKS: [Self; 8] = [
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
            masks[i] = Bitmask(mask);
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

    pub(crate) const RAYS: [[Self; 64]; 8] = {
        const fn generate_ray(df: i8, dr: i8) -> [Bitmask; 64] {
            let mut masks = [Bitmask(0); 64];
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
                masks[i] = Bitmask(mask);
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

    pub(crate) const BETWEEN_MASKS: [[Self; 64]; 64] = {
        const fn set_bits_between(start: usize, end: usize, step: usize) -> Bitmask {
            let mut mask = 0u64;
            let mut i = start + step;
            while i < end {
                mask |= 1u64 << i;
                i += step;
            }
            Bitmask(mask)
        }

        let mut masks = [[Bitmask::EMPTY; 64]; 64];

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

    pub(crate) const fn contains(self, position: Position) -> bool {
        self.0 & position.mask().0 != 0
    }

    pub(crate) const fn has_only_one_set(self) -> bool {
        self.0.is_power_of_two()
    }

    pub(crate) const fn set_mut(&mut self, position: Position) {
        self.0 |= position.mask().0
    }

    pub(crate) const fn set(self, position: Position) -> Self {
        Self(self.0 | position.mask().0)
    }

    pub(crate) const fn unset(self, position: Position) -> Self {
        Self(self.0 & !position.mask().0)
    }

    pub(crate) const fn lsb(self) -> Position {
        debug_assert!(self.0 != 0, "Called lsb() on an empty Bitmask");
        Position(self.0.trailing_zeros() as u8)
    }

    pub(crate) const fn msb(self) -> Position {
        debug_assert!(self.0 != 0, "Called msb() on an empty Bitmask");
        Position(63 - self.0.leading_zeros() as u8)
    }

    pub(crate) fn rook_attack_mask(position: Position, occupancy: Self) -> Self {
        let mut mask = Self::EMPTY;

        // North (Index 0) - Positive direction, blocker is LSB
        let north = Self::RAYS[Direction::North as usize][position];
        let north_blockers = north & occupancy;
        mask |= if north_blockers.0 != 0 {
            north ^ Self::RAYS[Direction::North as usize][north_blockers.lsb()]
        } else {
            north
        };

        // South (Index 1) - Negative direction, blocker is MSB
        let south = Self::RAYS[Direction::South as usize][position];
        let south_blockers = south & occupancy;
        mask |= if south_blockers.0 != 0 {
            south ^ Self::RAYS[Direction::South as usize][south_blockers.msb()]
        } else {
            south
        };

        // East (Index 2) - Positive direction, blocker is LSB
        let east = Self::RAYS[Direction::East as usize][position];
        let east_blockers = east & occupancy;
        mask |= if east_blockers.0 != 0 {
            east ^ Self::RAYS[Direction::East as usize][east_blockers.lsb()]
        } else {
            east
        };

        // West (Index 3) - Negative direction, blocker is MSB
        let west = Self::RAYS[Direction::West as usize][position];
        let west_blockers = west & occupancy;
        mask |= if west_blockers.0 != 0 {
            west ^ Self::RAYS[Direction::West as usize][west_blockers.msb()]
        } else {
            west
        };

        mask
    }

    pub(crate) fn bishop_attack_mask(position: Position, occupancy: Self) -> Self {
        let mut mask = Self::EMPTY;

        // North East (Index 4) - Positive direction, blocker is LSB
        let north_east = Self::RAYS[Direction::NorthEast as usize][position];
        let north_east_blockers = north_east & occupancy;
        mask |= if north_east_blockers.0 != 0 {
            north_east ^ Self::RAYS[Direction::NorthEast as usize][north_east_blockers.lsb()]
        } else {
            north_east
        };

        // North West (Index 5) - Positive direction, blocker is LSB
        let north_west = Self::RAYS[Direction::NorthWest as usize][position];
        let north_west_blockers = north_west & occupancy;
        mask |= if north_west_blockers.0 != 0 {
            north_west ^ Self::RAYS[Direction::NorthWest as usize][north_west_blockers.lsb()]
        } else {
            north_west
        };

        // South East (Index 6) - Negative direction, blocker is MSB
        let south_east = Self::RAYS[Direction::SouthEast as usize][position];
        let south_east_blockers = south_east & occupancy;
        mask |= if south_east_blockers.0 != 0 {
            south_east ^ Self::RAYS[Direction::SouthEast as usize][south_east_blockers.msb()]
        } else {
            south_east
        };

        // South West (Index 7) - Negative direction, blocker is MSB
        let south_west = Self::RAYS[Direction::SouthWest as usize][position];
        let south_west_blockers = south_west & occupancy;
        mask |= if south_west_blockers.0 != 0 {
            south_west ^ Self::RAYS[Direction::SouthWest as usize][south_west_blockers.msb()]
        } else {
            south_west
        };

        mask
    }

    pub(crate) fn queen_attack_mask(position: Position, occupancy: Self) -> Self {
        Self::rook_attack_mask(position, occupancy) | Self::bishop_attack_mask(position, occupancy)
    }
}

impl Iterator for Bitmask {
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

impl BitOr for Bitmask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Bitmask(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitmask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitmask {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Bitmask(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitmask {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Bitmask {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Bitmask(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitmask {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitmask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Bitmask(!self.0)
    }
}

impl Shl<u32> for Bitmask {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self {
        Bitmask(self.0 << rhs)
    }
}

impl ShlAssign<u32> for Bitmask {
    #[inline]
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl Shr<u32> for Bitmask {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Bitmask(self.0 >> rhs)
    }
}

impl ShrAssign<u32> for Bitmask {
    #[inline]
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}
