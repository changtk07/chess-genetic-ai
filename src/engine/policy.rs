use crate::chess::{
    moves::{Move, MoveType},
    types::Direction,
};

pub(crate) const N_MOVE_PLANES: usize = 73;

trait PolicyEncoding {
    fn index(&self) -> usize;
}

impl PolicyEncoding for Move {
    fn index(&self) -> usize {
        let (from, to, mv_type) = self.unpack();
        let base = from.0 as usize * N_MOVE_PLANES;
        let (dr, df) = (
            to.rank() as i8 - from.rank() as i8,
            to.file() as i8 - from.file() as i8,
        );

        match mv_type {
            MoveType::PromotionKnight => return base + (65 + df) as usize,
            MoveType::PromotionBishop => return base + (68 + df) as usize,
            MoveType::PromotionRook => return base + (71 + df) as usize,
            _ => (),
        };

        let plane = match (dr, df) {
            (2, 1) => 56,
            (1, 2) => 57,
            (-1, 2) => 58,
            (-2, 1) => 59,
            (-2, -1) => 60,
            (-1, -2) => 61,
            (1, -2) => 62,
            (2, -1) => 63,
            _ => {
                let (dir, dist) = match (dr.signum(), df.signum()) {
                    (1, 0) => (Direction::North, dr),
                    (-1, 0) => (Direction::South, -dr),
                    (0, 1) => (Direction::East, df),
                    (0, -1) => (Direction::West, -df),
                    (1, 1) => (Direction::NorthEast, dr),
                    (1, -1) => (Direction::NorthWest, dr),
                    (-1, 1) => (Direction::SouthEast, -dr),
                    (-1, -1) => (Direction::SouthWest, -dr),
                    _ => unreachable!(),
                };
                dir as usize * 7 + dist as usize - 1
            }
        };

        base + plane
    }
}
