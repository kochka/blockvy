//! SRS rotation states.
//!
//! Each tetromino's blocks are given as offsets within its bounding box,
//! with the box origin at the bottom-left (y = 0 at the bottom).
//!
//! - I-piece: 4x4 bounding box.
//! - All others: 3x3 bounding box.
//!
//! `grid_position` on `ActivePiece` is the world position of the bounding
//! box's bottom-left corner. `blocks_for` is pure data — no Bevy dependencies
//! beyond `IVec2` for arithmetic convenience.

use bevy::math::IVec2;

use super::rotation::Rotation;
use super::tetromino::TetrominoKind;

pub fn blocks_for(kind: TetrominoKind, rotation: Rotation) -> [IVec2; 4] {
    match kind {
        TetrominoKind::I => i_blocks(rotation),
        TetrominoKind::O => o_blocks(rotation),
        TetrominoKind::T => t_blocks(rotation),
        TetrominoKind::S => s_blocks(rotation),
        TetrominoKind::Z => z_blocks(rotation),
        TetrominoKind::J => j_blocks(rotation),
        TetrominoKind::L => l_blocks(rotation),
    }
}

const fn v(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}

fn i_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(0, 2), v(1, 2), v(2, 2), v(3, 2)],
        Rotation::East => [v(2, 0), v(2, 1), v(2, 2), v(2, 3)],
        Rotation::South => [v(0, 1), v(1, 1), v(2, 1), v(3, 1)],
        Rotation::West => [v(1, 0), v(1, 1), v(1, 2), v(1, 3)],
    }
}

fn o_blocks(_rotation: Rotation) -> [IVec2; 4] {
    [v(1, 1), v(2, 1), v(1, 2), v(2, 2)]
}

fn t_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(1, 2), v(0, 1), v(1, 1), v(2, 1)],
        Rotation::East => [v(1, 2), v(1, 1), v(2, 1), v(1, 0)],
        Rotation::South => [v(0, 1), v(1, 1), v(2, 1), v(1, 0)],
        Rotation::West => [v(1, 2), v(0, 1), v(1, 1), v(1, 0)],
    }
}

fn s_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(1, 2), v(2, 2), v(0, 1), v(1, 1)],
        Rotation::East => [v(1, 2), v(1, 1), v(2, 1), v(2, 0)],
        Rotation::South => [v(1, 1), v(2, 1), v(0, 0), v(1, 0)],
        Rotation::West => [v(0, 2), v(0, 1), v(1, 1), v(1, 0)],
    }
}

fn z_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(0, 2), v(1, 2), v(1, 1), v(2, 1)],
        Rotation::East => [v(2, 2), v(1, 1), v(2, 1), v(1, 0)],
        Rotation::South => [v(0, 1), v(1, 1), v(1, 0), v(2, 0)],
        Rotation::West => [v(1, 2), v(0, 1), v(1, 1), v(0, 0)],
    }
}

fn j_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(0, 2), v(0, 1), v(1, 1), v(2, 1)],
        Rotation::East => [v(1, 2), v(2, 2), v(1, 1), v(1, 0)],
        Rotation::South => [v(0, 1), v(1, 1), v(2, 1), v(2, 0)],
        Rotation::West => [v(1, 2), v(1, 1), v(0, 0), v(1, 0)],
    }
}

fn l_blocks(rotation: Rotation) -> [IVec2; 4] {
    match rotation {
        Rotation::North => [v(2, 2), v(0, 1), v(1, 1), v(2, 1)],
        Rotation::East => [v(1, 2), v(1, 1), v(1, 0), v(2, 0)],
        Rotation::South => [v(0, 1), v(1, 1), v(2, 1), v(0, 0)],
        Rotation::West => [v(0, 2), v(1, 2), v(1, 1), v(1, 0)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROTATIONS: [Rotation; 4] = [
        Rotation::North,
        Rotation::East,
        Rotation::South,
        Rotation::West,
    ];

    #[test]
    fn every_piece_has_four_distinct_blocks_in_every_rotation() {
        for kind in TetrominoKind::ALL {
            for rotation in ROTATIONS {
                let blocks = blocks_for(kind, rotation);
                let unique: std::collections::HashSet<_> = blocks.iter().collect();
                assert_eq!(
                    unique.len(),
                    4,
                    "duplicate cell in {:?} {:?}: {:?}",
                    kind,
                    rotation,
                    blocks
                );
            }
        }
    }

    #[test]
    fn blocks_stay_within_bounding_box() {
        for kind in TetrominoKind::ALL {
            let box_size = match kind {
                TetrominoKind::I => 4,
                _ => 3,
            };
            for rotation in ROTATIONS {
                for block in blocks_for(kind, rotation) {
                    assert!(
                        block.x >= 0 && block.x < box_size,
                        "x out of box for {:?} {:?}: {:?}",
                        kind,
                        rotation,
                        block
                    );
                    assert!(
                        block.y >= 0 && block.y < box_size,
                        "y out of box for {:?} {:?}: {:?}",
                        kind,
                        rotation,
                        block
                    );
                }
            }
        }
    }

    #[test]
    fn o_piece_is_rotation_invariant() {
        let reference = blocks_for(TetrominoKind::O, Rotation::North);
        for rotation in ROTATIONS {
            let blocks = blocks_for(TetrominoKind::O, rotation);
            assert_eq!(sorted(blocks), sorted(reference));
        }
    }

    #[test]
    fn t_north_matches_canonical_shape() {
        // T-piece in North orientation should be:
        //   . X .
        //   X X X
        //   . . .
        let blocks = sorted(blocks_for(TetrominoKind::T, Rotation::North));
        let expected = sorted([v(0, 1), v(1, 1), v(2, 1), v(1, 2)]);
        assert_eq!(blocks, expected);
    }

    #[test]
    fn i_north_is_horizontal_row() {
        let blocks = sorted(blocks_for(TetrominoKind::I, Rotation::North));
        let expected = sorted([v(0, 2), v(1, 2), v(2, 2), v(3, 2)]);
        assert_eq!(blocks, expected);
    }

    fn sorted(mut blocks: [IVec2; 4]) -> [IVec2; 4] {
        blocks.sort_by_key(|v| (v.y, v.x));
        blocks
    }
}
