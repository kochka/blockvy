//! Piece locking: stamp the active piece into the board, detect completed
//! lines, and draw the next piece from the bag.
//!
//! The cycle is split so the visual layer can hold the completed rows on
//! screen briefly (see `board/clear_delay.rs`):
//!
//! 1. [`stamp_lock`] writes the piece into the board and reports which rows
//!    just became full — without clearing them.
//! 2. [`draw_next_piece`] pulls the next tetromino from the bag and reports
//!    whether it tops out on spawn.
//!
//! [`finalize_lock`] wires both together (plus [`clear_full_lines`]) for
//! callers that don't want a clear-delay pause — currently the tests, and
//! the immediate path when no line was completed.

use bevy::prelude::*;

use crate::pieces::SevenBag;

use super::active_piece::ActivePiece;
use super::collision::can_place;
use super::grid::Board;
use super::lines::{ClearedRows, clear_full_lines, detect_full_rows};

#[derive(Debug, Clone, Copy)]
pub struct LockOutcome {
    pub lines_cleared: u32,
    /// Row indices of the cleared lines in the pre-clear frame. `None`
    /// marks unused slots (up to 4 rows can be cleared at once).
    pub cleared_rows: ClearedRows,
    pub topped_out: bool,
}

impl Default for LockOutcome {
    fn default() -> Self {
        Self {
            lines_cleared: 0,
            cleared_rows: [None; 4],
            topped_out: false,
        }
    }
}

/// Fired after a piece has been locked into the board **and** any line
/// clearing has resolved. Higher-level systems (score, sound, particles,
/// game-over transition) react to this — the delay introduced by the
/// clear animation is invisible to them.
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

/// Writes the piece into the board and returns the rows that just became
/// full — *without* clearing them. The caller decides when to actually
/// collapse the stack (immediately, or after a short delay for the
/// clear-line animation).
pub fn stamp_lock(board: &mut Board, piece: &ActivePiece) -> ClearedRows {
    lock_piece(board, piece);
    detect_full_rows(board)
}

/// Pulls the next tetromino from the bag. Returns the spawned piece and
/// whether it tops out: block-out is checked by attempting to descend one
/// row into the visible playfield, since bare spawn always succeeds in
/// the buffer.
pub fn draw_next_piece(bag: &mut SevenBag, board: &Board) -> (ActivePiece, bool) {
    let kind = bag.next();
    let next_piece = ActivePiece::new(kind, kind.spawn_origin());
    let topped_out =
        !can_place(&next_piece, board) || !can_place(&next_piece.moved_down(), board);
    (next_piece, topped_out)
}

/// Atomic lock cycle for callers that don't want to defer the clear.
/// Currently used by tests and by the "no lines to clear" fast path.
pub fn finalize_lock(
    piece: &mut ActivePiece,
    board: &mut Board,
    bag: &mut SevenBag,
) -> LockOutcome {
    let cleared_rows = stamp_lock(board, piece);
    let lines_cleared = clear_full_lines(board);
    let (next_piece, topped_out) = draw_next_piece(bag, board);
    *piece = next_piece;

    LockOutcome {
        lines_cleared,
        cleared_rows,
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
    fn stamp_lock_reports_cleared_rows_without_clearing() {
        let mut board = Board::default();
        for x in 0..BOARD_WIDTH as i32 {
            if x != 5 {
                board.set(x, 0, TetrominoKind::I);
            }
        }
        // Vertical I plugs (5,0..3) — completes row 0.
        let mut piece = ActivePiece::new(TetrominoKind::I, IVec2::new(3, 0));
        piece.rotation = crate::pieces::Rotation::East;

        let rows = stamp_lock(&mut board, &piece);
        assert_eq!(rows[0], Some(0));
        assert_eq!(rows[1], None);
        // Row 0 must STILL be full — stamp_lock only detects.
        for x in 0..BOARD_WIDTH as i32 {
            assert!(board.get(x, 0).is_some());
        }
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
        assert_eq!(outcome.cleared_rows[0], Some(0));
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
