//! Line clearing: detect fully filled rows and remove them.
//!
//! The API is split in two phases so the animation layer can hold the
//! completed rows on screen for a beat before they actually disappear:
//!
//! * [`detect_full_rows`] returns the indices of the currently full rows,
//!   without touching the board. Callers use it to know which rows to
//!   highlight and to decide whether to enter the "line-clear delay".
//! * [`clear_rows`] removes the given rows and collapses the stack. It
//!   is called at the end of the delay, once the visual is done.
//!
//! [`clear_full_lines`] wires both together for tests and any caller that
//! wants the atomic "detect + clear" behaviour.

use super::grid::{BOARD_HEIGHT, BOARD_WIDTH, Board};

/// Maximum number of rows a single tetromino lock can clear (I-piece).
pub const MAX_CLEARED_ROWS: usize = 4;

/// Row indices, in the pre-clear frame, of every fully filled row.
/// `None` marks unused slots — populated left-to-right.
pub type ClearedRows = [Option<u8>; MAX_CLEARED_ROWS];

pub fn detect_full_rows(board: &Board) -> ClearedRows {
    let mut rows: ClearedRows = [None; MAX_CLEARED_ROWS];
    let mut next = 0;
    for y in 0..BOARD_HEIGHT {
        if board.cells[y].iter().all(|c| c.is_some()) {
            rows[next] = Some(y as u8);
            next += 1;
            if next == MAX_CLEARED_ROWS {
                break;
            }
        }
    }
    rows
}

/// Removes the given rows and collapses the stack downward. Returns the
/// number of rows that were actually removed. Rows are interpreted in the
/// pre-clear coordinate frame — the same frame [`detect_full_rows`] returns.
pub fn clear_rows(board: &mut Board, rows: &ClearedRows) -> u32 {
    let mut is_cleared = [false; BOARD_HEIGHT];
    let mut cleared = 0u32;
    for slot in rows.iter().flatten() {
        let y = *slot as usize;
        if y < BOARD_HEIGHT && !is_cleared[y] {
            is_cleared[y] = true;
            cleared += 1;
        }
    }
    if cleared == 0 {
        return 0;
    }

    let mut write = 0;
    for read in 0..BOARD_HEIGHT {
        if is_cleared[read] {
            continue;
        }
        if write != read {
            board.cells[write] = board.cells[read];
        }
        write += 1;
    }
    for y in write..BOARD_HEIGHT {
        board.cells[y] = [None; BOARD_WIDTH];
    }
    cleared
}

/// Convenience for tests and any caller that doesn't want the two-phase
/// dance: detect the full rows and clear them immediately.
pub fn clear_full_lines(board: &mut Board) -> u32 {
    let rows = detect_full_rows(board);
    clear_rows(board, &rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::TetrominoKind;

    fn fill_row(board: &mut Board, y: i32, kind: TetrominoKind) {
        for x in 0..BOARD_WIDTH as i32 {
            board.set(x, y, kind);
        }
    }

    #[test]
    fn no_full_rows_clears_nothing() {
        let mut board = Board::default();
        board.set(0, 0, TetrominoKind::I);
        board.set(1, 0, TetrominoKind::I);
        assert_eq!(clear_full_lines(&mut board), 0);
        assert_eq!(board.get(0, 0), Some(TetrominoKind::I));
        assert_eq!(board.get(1, 0), Some(TetrominoKind::I));
    }

    #[test]
    fn single_full_row_is_cleared_and_stack_drops() {
        let mut board = Board::default();
        fill_row(&mut board, 0, TetrominoKind::I);
        // Stray cell on row 1 should fall to row 0.
        board.set(3, 1, TetrominoKind::T);

        assert_eq!(clear_full_lines(&mut board), 1);

        for x in 0..BOARD_WIDTH as i32 {
            assert!(board.is_empty_at(x, 1));
        }
        assert_eq!(board.get(3, 0), Some(TetrominoKind::T));
    }

    #[test]
    fn multiple_full_rows_clear_in_one_pass() {
        let mut board = Board::default();
        fill_row(&mut board, 0, TetrominoKind::I);
        fill_row(&mut board, 1, TetrominoKind::I);
        fill_row(&mut board, 2, TetrominoKind::I);
        fill_row(&mut board, 3, TetrominoKind::I);
        // Marker on row 4 must drop to row 0.
        board.set(7, 4, TetrominoKind::S);

        assert_eq!(clear_full_lines(&mut board), 4);

        assert_eq!(board.get(7, 0), Some(TetrominoKind::S));
        for y in 1..BOARD_HEIGHT as i32 {
            for x in 0..BOARD_WIDTH as i32 {
                assert!(board.is_empty_at(x, y));
            }
        }
    }

    #[test]
    fn clears_non_contiguous_rows() {
        let mut board = Board::default();
        fill_row(&mut board, 0, TetrominoKind::I);
        // Marker on row 1 — must end up on the floor after row 0 is cleared.
        board.set(2, 1, TetrominoKind::Z);
        fill_row(&mut board, 3, TetrominoKind::I);
        // Marker on row 4 — must land on row 1 (row 3 cleared, plus row 0 cleared).
        board.set(8, 4, TetrominoKind::L);

        assert_eq!(clear_full_lines(&mut board), 2);
        assert_eq!(board.get(2, 0), Some(TetrominoKind::Z));
        assert_eq!(board.get(8, 2), Some(TetrominoKind::L));
    }

    #[test]
    fn detect_reports_rows_in_pre_clear_frame() {
        let mut board = Board::default();
        fill_row(&mut board, 0, TetrominoKind::I);
        fill_row(&mut board, 3, TetrominoKind::I);
        let rows = detect_full_rows(&board);
        assert_eq!(rows[0], Some(0));
        assert_eq!(rows[1], Some(3));
        assert_eq!(rows[2], None);
        assert_eq!(rows[3], None);
    }

    #[test]
    fn clear_rows_from_detected_indices_matches_atomic_clear() {
        let mut board_a = Board::default();
        let mut board_b = Board::default();
        for (x, kind) in [
            (0, TetrominoKind::T),
            (2, TetrominoKind::Z),
            (8, TetrominoKind::L),
        ] {
            board_a.set(x, 4, kind);
            board_b.set(x, 4, kind);
        }
        fill_row(&mut board_a, 0, TetrominoKind::I);
        fill_row(&mut board_a, 3, TetrominoKind::I);
        fill_row(&mut board_b, 0, TetrominoKind::I);
        fill_row(&mut board_b, 3, TetrominoKind::I);

        let rows = detect_full_rows(&board_a);
        let count_split = clear_rows(&mut board_a, &rows);
        let count_atomic = clear_full_lines(&mut board_b);

        assert_eq!(count_split, count_atomic);
        assert_eq!(board_a.cells, board_b.cells);
    }
}
