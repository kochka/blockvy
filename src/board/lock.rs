//! Piece locking: stamp the active piece into the board, clear filled lines,
//! and draw the next piece in place.

use bevy::prelude::*;

use crate::pieces::SevenBag;

use super::active_piece::ActivePiece;
use super::collision::can_place;
use super::grid::Board;
use super::lines::clear_full_lines;

#[derive(Debug, Clone, Copy, Default)]
pub struct LockOutcome {
    pub lines_cleared: u32,
    pub topped_out: bool,
}

/// Fired after a piece has been locked into the board. Lets higher-level
/// systems (score, sound, particles) react without coupling them to the
/// gravity/input systems that trigger the lock.
#[derive(Message, Debug, Clone, Copy)]
pub struct PieceLocked {
    pub outcome: LockOutcome,
}

/// Stamps every block of `piece` into `board`. Blocks above the visible
/// playfield (spawn buffer) are silently dropped — they are not cells.
pub fn lock_piece(board: &mut Board, piece: &ActivePiece) {
    for block in piece.blocks() {
        if Board::in_bounds(block.x, block.y) {
            board.set(block.x, block.y, piece.kind);
        }
    }
}

/// Full lock cycle: writes the piece into the board, clears completed lines,
/// draws the next piece from the bag, and assigns it to `piece` in place.
///
/// `topped_out` is true when the freshly drawn piece either overlaps the
/// stack at its spawn origin or cannot descend one row into the playfield —
/// the canonical block-out condition. Recovery (board wipe, state change) is
/// the caller's responsibility.
pub fn finalize_lock(
    piece: &mut ActivePiece,
    board: &mut Board,
    bag: &mut SevenBag,
) -> LockOutcome {
    lock_piece(board, piece);
    let lines_cleared = clear_full_lines(board);

    let next_kind = bag.next();
    let next_piece = ActivePiece::new(next_kind, next_kind.spawn_origin());
    // Spawn pieces sit in the buffer (y >= 20) so `can_place(next_piece)`
    // alone never fails — it's the first attempted descent that exposes a
    // stack reaching into the spawn columns.
    let topped_out = !can_place(&next_piece, board)
        || !can_place(&next_piece.moved_down(), board);

    *piece = next_piece;

    LockOutcome {
        lines_cleared,
        topped_out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BOARD_WIDTH;
    use crate::pieces::TetrominoKind;
    use bevy::math::IVec2;

    #[test]
    fn lock_writes_each_block_into_the_board() {
        let mut board = Board::default();
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 0));
        lock_piece(&mut board, &piece);

        // T-North footprint relative to origin (4, 0):
        //   bottom row at y = 1: (4,1), (5,1), (6,1)
        //   tip at y = 2:       (5,2)
        assert_eq!(board.get(4, 1), Some(TetrominoKind::T));
        assert_eq!(board.get(5, 1), Some(TetrominoKind::T));
        assert_eq!(board.get(6, 1), Some(TetrominoKind::T));
        assert_eq!(board.get(5, 2), Some(TetrominoKind::T));
    }

    #[test]
    fn lock_ignores_blocks_above_the_visible_playfield() {
        let mut board = Board::default();
        // Spawn-row T sits with its tip at y = 20 (first buffer row).
        let piece = ActivePiece::new(TetrominoKind::T, IVec2::new(3, 19));
        lock_piece(&mut board, &piece);
        // Bottom row of the piece lands at y = 20 — none should be stored.
        assert!(board.cells.iter().flatten().all(|c| c.is_none()));
    }

    #[test]
    fn finalize_lock_flags_top_out_when_stack_blocks_spawn_columns() {
        // Fill row 19 across the spawn columns (3..=6). The next piece —
        // whichever kind — has at least one block in those columns when
        // attempting to descend from y = 20 to y = 19, so block-out triggers.
        let mut board = Board::default();
        for x in 3..=6 {
            board.set(x, 19, TetrominoKind::I);
        }
        let mut piece = ActivePiece::new(TetrominoKind::O, IVec2::new(0, 0));
        let mut bag = SevenBag::from_seed(0);

        let outcome = finalize_lock(&mut piece, &mut board, &mut bag);

        assert!(outcome.topped_out);
    }

    #[test]
    fn finalize_lock_clears_a_full_row_and_spawns_next() {
        let mut board = Board::default();
        // Pre-fill the bottom row except for column 5.
        for x in 0..BOARD_WIDTH as i32 {
            if x != 5 {
                board.set(x, 0, TetrominoKind::I);
            }
        }
        // Vertical I-piece (East): offsets are (2,0..3) → with origin (3,0)
        // the blocks land at (5,0), (5,1), (5,2), (5,3) — plugging the hole.
        let mut piece = ActivePiece::new(TetrominoKind::I, IVec2::new(3, 0));
        piece.rotation = crate::pieces::Rotation::East;
        let mut bag = SevenBag::from_seed(1);

        let outcome = finalize_lock(&mut piece, &mut board, &mut bag);

        assert_eq!(outcome.lines_cleared, 1);
        assert!(!outcome.topped_out);
        // Row 0 has been cleared, three vertical I blocks dropped one row to y = 0,1,2.
        assert_eq!(board.get(5, 0), Some(TetrominoKind::I));
        assert_eq!(board.get(5, 1), Some(TetrominoKind::I));
        assert_eq!(board.get(5, 2), Some(TetrominoKind::I));
        assert!(board.is_empty_at(5, 3));
        // Old row 0 must be gone — column 0 used to be filled, now empty.
        assert!(board.is_empty_at(0, 0));
        // And `piece` was replaced with a freshly spawned next piece.
        assert_eq!(piece.rotation, crate::pieces::Rotation::North);
    }

}
