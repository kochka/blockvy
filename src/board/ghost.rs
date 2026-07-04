//! Ghost piece projection.
//!
//! The ghost is a visual-only hint of where the active piece would land if
//! hard-dropped right now. It must never affect collision, locking or any
//! other gameplay state.

use super::active_piece::ActivePiece;
use super::collision::hard_drop;
use super::grid::Board;

/// Returns the piece projected straight down to its resting position. The
/// returned piece keeps the same kind and rotation as the input.
pub fn projected_piece(piece: &ActivePiece, board: &Board) -> ActivePiece {
    hard_drop(piece, board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BOARD_WIDTH;
    use crate::pieces::TetrominoKind;
    use bevy::math::IVec2;

    #[test]
    fn projection_lands_on_floor_on_empty_board() {
        let board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        let ghost = projected_piece(&piece, &board);
        // T-North bottom blocks at offset y=1, so floor origin is y=-1.
        assert_eq!(ghost.grid_position, IVec2::new(4, -1));
    }

    #[test]
    fn projection_does_not_mutate_the_input() {
        let board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::S, IVec2::new(3, 15));
        let original = piece;
        let _ = projected_piece(&piece, &board);
        assert_eq!(piece.grid_position, original.grid_position);
        assert_eq!(piece.rotation, original.rotation);
    }

    #[test]
    fn projection_stacks_on_locked_cells() {
        let mut board = Board::default();
        for x in 0..BOARD_WIDTH as i32 {
            board.set(x, 0, TetrominoKind::I);
        }
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        let ghost = projected_piece(&piece, &board);
        // T-North rests on top of the locked row: bottom blocks at y=1 → origin y=0.
        assert_eq!(ghost.grid_position, IVec2::new(4, 0));
    }
}
