use bevy::prelude::*;

use crate::pieces::{Rotation, RotationDirection, TetrominoKind, blocks_for};

#[derive(Resource, Clone, Copy, Debug)]
pub struct ActivePiece {
    pub kind: TetrominoKind,
    pub grid_position: IVec2,
    pub rotation: Rotation,
}

impl ActivePiece {
    pub fn new(kind: TetrominoKind, grid_position: IVec2) -> Self {
        Self {
            kind,
            grid_position,
            rotation: Rotation::North,
        }
    }

    /// Returns the absolute grid positions of the four blocks composing this piece.
    pub fn blocks(&self) -> [IVec2; 4] {
        blocks_for(self.kind, self.rotation).map(|offset| self.grid_position + offset)
    }

    pub fn moved_by(&self, offset: IVec2) -> ActivePiece {
        Self {
            grid_position: self.grid_position + offset,
            ..*self
        }
    }

    pub fn moved_down(&self) -> ActivePiece {
        self.moved_by(IVec2::new(0, -1))
    }

    pub fn rotated(&self, direction: RotationDirection) -> ActivePiece {
        Self {
            rotation: self.rotation.rotated(direction),
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_offset_by_grid_position() {
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        let blocks = piece.blocks();
        // T North block offsets: (1,2), (0,1), (1,1), (2,1) → translate by (4, 18).
        let mut sorted = blocks;
        sorted.sort_by_key(|v| (v.y, v.x));
        assert_eq!(
            sorted,
            [
                IVec2::new(4, 19),
                IVec2::new(5, 19),
                IVec2::new(6, 19),
                IVec2::new(5, 20),
            ]
        );
    }

    #[test]
    fn moved_down_decreases_y() {
        let piece = ActivePiece::new(TetrominoKind::I, IVec2::new(3, 20));
        let after = piece.moved_down();
        assert_eq!(after.grid_position, IVec2::new(3, 19));
    }

    #[test]
    fn rotated_changes_only_rotation() {
        let piece = ActivePiece::new(TetrominoKind::J, IVec2::new(3, 18));
        let cw = piece.rotated(RotationDirection::Clockwise);
        assert_eq!(cw.kind, piece.kind);
        assert_eq!(cw.grid_position, piece.grid_position);
        assert_eq!(cw.rotation, Rotation::East);
    }
}
