use bevy::math::IVec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoKind {
    pub const ALL: [TetrominoKind; 7] = [
        TetrominoKind::I,
        TetrominoKind::O,
        TetrominoKind::T,
        TetrominoKind::S,
        TetrominoKind::Z,
        TetrominoKind::J,
        TetrominoKind::L,
    ];

    /// Canonical SRS spawn origin (bounding-box bottom-left, in board coords).
    ///
    /// All pieces are centered horizontally at the standard SRS columns and
    /// drop their bottom row into the first buffer row (y = 20). The y origin
    /// differs because the I-piece uses a 4×4 box while the others use a 3×3.
    pub fn spawn_origin(self) -> IVec2 {
        match self {
            TetrominoKind::I => IVec2::new(3, 18),
            _ => IVec2::new(3, 19),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Rotation, blocks_for};

    #[test]
    fn spawn_origin_places_lowest_block_in_buffer() {
        // Every piece's lowest spawn block should land on y = 20 (the first
        // buffer row above the visible playfield).
        for kind in TetrominoKind::ALL {
            let origin = kind.spawn_origin();
            let lowest_y = blocks_for(kind, Rotation::North)
                .iter()
                .map(|b| origin.y + b.y)
                .min()
                .unwrap();
            assert_eq!(lowest_y, 20, "{:?} lowest spawn block y", kind);
        }
    }

    #[test]
    fn spawn_columns_match_srs_canonical_layout() {
        // I and O span specific column ranges; JLSTZ span columns 3-5.
        let columns = |kind: TetrominoKind| {
            let origin = kind.spawn_origin();
            let mut xs: Vec<i32> = blocks_for(kind, Rotation::North)
                .iter()
                .map(|b| origin.x + b.x)
                .collect();
            xs.sort();
            xs.dedup();
            xs
        };
        assert_eq!(columns(TetrominoKind::I), vec![3, 4, 5, 6]);
        assert_eq!(columns(TetrominoKind::O), vec![4, 5]);
        assert_eq!(columns(TetrominoKind::T), vec![3, 4, 5]);
    }
}
