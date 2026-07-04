//! Rotation against a board, with optional SRS wall kicks.

use crate::pieces::{RotationDirection, WallKickMode, srs_wall_kicks};

use super::active_piece::ActivePiece;
use super::collision::can_place;
use super::grid::Board;

/// Returns the piece after rotation. If no valid position is found, returns
/// the original piece unchanged.
///
/// Order of attempts:
/// 1. In-place rotation.
/// 2. If kicks are enabled, walk the SRS kick table for the transition.
pub fn try_rotate(
    piece: ActivePiece,
    direction: RotationDirection,
    board: &Board,
    mode: WallKickMode,
) -> ActivePiece {
    let rotated = piece.rotated(direction);

    if can_place(&rotated, board) {
        return rotated;
    }

    if mode == WallKickMode::Off {
        return piece;
    }

    for &offset in srs_wall_kicks(piece.kind, piece.rotation, rotated.rotation) {
        let candidate = rotated.moved_by(offset);
        if can_place(&candidate, board) {
            return candidate;
        }
    }

    piece
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BOARD_WIDTH;
    use crate::pieces::{Rotation, TetrominoKind};
    use bevy::math::IVec2;

    #[test]
    fn in_place_rotation_on_empty_board() {
        let board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 10));
        let rotated = try_rotate(piece, RotationDirection::Clockwise, &board, WallKickMode::Srs);
        assert_eq!(rotated.rotation, Rotation::East);
        assert_eq!(rotated.grid_position, piece.grid_position);
    }

    #[test]
    fn srs_kick_recovers_rotation_against_left_wall() {
        let board = Board::default();
        // T-East has its leftmost block at offset x=1. Place piece against the
        // left wall (origin x=-1) so the rotation away from East→South fails
        // in place and must be rescued by a kick. A bare in-place rotation
        // would land an S-block at x=-1, which collides with the wall.
        let mut piece = ActivePiece::new(TetrominoKind::T, IVec2::new(-1, 10));
        piece.rotation = Rotation::East;
        let rotated = try_rotate(
            piece,
            RotationDirection::Clockwise,
            &board,
            WallKickMode::Srs,
        );
        assert_eq!(rotated.rotation, Rotation::South);
        assert_ne!(rotated.grid_position, piece.grid_position);
    }

    #[test]
    fn off_mode_rejects_rotation_against_wall() {
        let board = Board::default();
        let mut piece = ActivePiece::new(TetrominoKind::T, IVec2::new(-1, 10));
        piece.rotation = Rotation::East;
        let rotated = try_rotate(
            piece,
            RotationDirection::Clockwise,
            &board,
            WallKickMode::Off,
        );
        assert_eq!(rotated.rotation, Rotation::East);
        assert_eq!(rotated.grid_position, piece.grid_position);
    }

    #[test]
    fn no_valid_kick_keeps_piece_unchanged() {
        // Fill the board so absolutely no rotation can fit anywhere.
        let mut board = Board::default();
        for y in 0..20 {
            for x in 0..BOARD_WIDTH as i32 {
                board.set(x, y, TetrominoKind::I);
            }
        }
        // Make room for the current T-North footprint only, nothing rotated.
        for block in ActivePiece::new(TetrominoKind::T, IVec2::new(4, 10)).blocks() {
            board.clear_cell(block.x, block.y);
        }
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 10));
        let rotated = try_rotate(
            piece,
            RotationDirection::Clockwise,
            &board,
            WallKickMode::Srs,
        );
        assert_eq!(rotated.rotation, piece.rotation);
        assert_eq!(rotated.grid_position, piece.grid_position);
    }
}
