use super::active_piece::ActivePiece;
use super::grid::{BOARD_HEIGHT, BOARD_WIDTH, Board};

/// Returns `true` if every block of `piece` fits inside the playfield without
/// overlapping a locked cell. Blocks above the visible playfield (within the
/// spawn buffer) are tolerated so SRS rotations near the top can succeed.
pub fn can_place(piece: &ActivePiece, board: &Board) -> bool {
    for block in piece.blocks() {
        if block.x < 0 || block.x >= BOARD_WIDTH as i32 {
            return false;
        }

        if block.y < 0 {
            return false;
        }

        if (block.y as usize) >= BOARD_HEIGHT {
            continue;
        }

        if board.cells[block.y as usize][block.x as usize].is_some() {
            return false;
        }
    }

    true
}

/// Returns the piece moved straight down to its resting position. Does NOT
/// lock — the caller triggers locking as a separate step.
pub fn hard_drop(piece: &ActivePiece, board: &Board) -> ActivePiece {
    let mut current = *piece;
    while can_place(&current.moved_down(), board) {
        current = current.moved_down();
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Rotation, TetrominoKind};
    use bevy::math::IVec2;

    #[test]
    fn fits_on_empty_board() {
        let board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        assert!(can_place(&piece, &board));
    }

    #[test]
    fn rejects_left_overflow() {
        let board = Board::default();
        // T-North has its leftmost block at offset (0, 1); placing at x = -1 puts it out of bounds.
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(-1, 10));
        assert!(!can_place(&piece, &board));
    }

    #[test]
    fn rejects_right_overflow() {
        let board = Board::default();
        // T-North has its rightmost block at offset (2, 1); board width is 10 → x must be <= 7.
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(8, 10));
        assert!(!can_place(&piece, &board));
    }

    #[test]
    fn rejects_bottom_overflow() {
        let board = Board::default();
        // T-North has its lowest blocks at offset y = 1; placing at y = -1 puts them at y = 0 (ok)
        // but at y = -2 the bottom row drops to y = -1.
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, -2));
        assert!(!can_place(&piece, &board));
    }

    #[test]
    fn allows_blocks_above_visible_playfield() {
        let board = Board::default();
        // I-piece East rotation sits vertically and can extend above the visible board.
        let mut piece = ActivePiece::new(TetrominoKind::I, IVec2::new(3, 18));
        piece.rotation = Rotation::East;
        assert!(can_place(&piece, &board));
    }

    #[test]
    fn rejects_overlap_with_locked_cell() {
        let mut board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 0));
        // T-North bottom-row blocks land at y = 1; occupy one of them.
        board.set(5, 1, TetrominoKind::S);
        assert!(!can_place(&piece, &board));
    }

    #[test]
    fn allows_piece_resting_directly_on_locked_cell() {
        let mut board = Board::default();
        // Lock a row at y = 0; T at y = 1 has its bottom blocks at y = 2 (no overlap).
        for x in 0..BOARD_WIDTH as i32 {
            board.set(x, 0, TetrominoKind::I);
        }
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 1));
        assert!(can_place(&piece, &board));
    }

    #[test]
    fn hard_drop_lands_piece_on_floor_on_empty_board() {
        let board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        let landed = hard_drop(&piece, &board);
        // T-North bottom blocks are at offset y=1, so origin y=-1 places them on the floor.
        assert_eq!(landed.grid_position, IVec2::new(4, -1));
        assert!(can_place(&landed, &board));
    }

    #[test]
    fn hard_drop_stacks_on_locked_row() {
        let mut board = Board::default();
        for x in 0..BOARD_WIDTH as i32 {
            board.set(x, 0, TetrominoKind::I);
        }
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 18));
        let landed = hard_drop(&piece, &board);
        // Lowest free row is y=1 for T-North blocks; origin y = 0.
        assert_eq!(landed.grid_position, IVec2::new(4, 0));
    }
}
