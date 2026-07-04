//! Line clearing: remove fully filled rows and let the rows above fall down.

use super::grid::{BOARD_HEIGHT, BOARD_WIDTH, Board};

/// Detects every fully filled row, removes them, shifts the rows above down,
/// and refills the top with empty rows. Returns the number of cleared lines.
pub fn clear_full_lines(board: &mut Board) -> u32 {
    let mut cleared = 0;
    let mut y = 0;
    while y < BOARD_HEIGHT {
        if board.cells[y].iter().all(|c| c.is_some()) {
            for row in y..BOARD_HEIGHT - 1 {
                board.cells[row] = board.cells[row + 1];
            }
            board.cells[BOARD_HEIGHT - 1] = [None; BOARD_WIDTH];
            cleared += 1;
            // Don't advance y: the row that just fell into this slot must be
            // re-checked (it could itself be full).
        } else {
            y += 1;
        }
    }
    cleared
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
}
