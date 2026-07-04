use bevy::prelude::*;

use crate::pieces::TetrominoKind;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_VISIBLE_HEIGHT: usize = 20;
/// Maximum vertical extent of any active piece above the visible playfield.
/// Set to 4 to cover I-piece SRS in-place rotation immediately after spawn,
/// which can place cells up to y = BOARD_VISIBLE_HEIGHT + 3.
pub const BOARD_BUFFER_HEIGHT: usize = 4;
pub const BOARD_HEIGHT: usize = BOARD_VISIBLE_HEIGHT;

#[derive(Resource)]
pub struct Board {
    /// Row-major, indexed as `cells[y][x]` with `y = 0` at the bottom.
    /// Only locked blocks inside the visible playfield are stored.
    /// Each cell remembers the kind of tetromino it came from for coloring.
    pub cells: [[Option<TetrominoKind>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

impl Board {
    pub fn in_bounds(x: i32, y: i32) -> bool {
        x >= 0
            && (x as usize) < BOARD_WIDTH
            && y >= 0
            && (y as usize) < BOARD_HEIGHT
    }

    pub fn get(&self, x: i32, y: i32) -> Option<TetrominoKind> {
        if !Self::in_bounds(x, y) {
            return None;
        }
        self.cells[y as usize][x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, kind: TetrominoKind) {
        if !Self::in_bounds(x, y) {
            return;
        }
        self.cells[y as usize][x as usize] = Some(kind);
    }

    pub fn clear_cell(&mut self, x: i32, y: i32) {
        if !Self::in_bounds(x, y) {
            return;
        }
        self.cells[y as usize][x as usize] = None;
    }

    pub fn is_empty_at(&self, x: i32, y: i32) -> bool {
        self.get(x, y).is_none()
    }

    pub fn reset(&mut self) {
        self.cells = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
    }
}

/// Converts a grid coordinate to the center of its cell in world space.
///
/// `origin` is the world position of the board's bottom-left corner.
/// `cell_px` is the side length of a single cell, derived at runtime from the
/// `BoardPanel` node size (the board is responsive).
pub fn cell_to_world(grid: IVec2, origin: Vec2, cell_px: f32) -> Vec2 {
    Vec2::new(
        origin.x + (grid.x as f32 + 0.5) * cell_px,
        origin.y + (grid.y as f32 + 0.5) * cell_px,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_bounds_rejects_negative_and_oversize() {
        assert!(Board::in_bounds(0, 0));
        assert!(Board::in_bounds(9, 19));
        assert!(!Board::in_bounds(-1, 0));
        assert!(!Board::in_bounds(0, -1));
        assert!(!Board::in_bounds(10, 0));
        assert!(!Board::in_bounds(0, 20));
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut board = Board::default();
        assert!(board.is_empty_at(3, 4));

        board.set(3, 4, TetrominoKind::T);
        assert_eq!(board.get(3, 4), Some(TetrominoKind::T));
        assert!(!board.is_empty_at(3, 4));

        board.clear_cell(3, 4);
        assert!(board.is_empty_at(3, 4));
    }

    #[test]
    fn out_of_bounds_set_is_ignored() {
        let mut board = Board::default();
        board.set(-1, 0, TetrominoKind::I);
        board.set(0, 20, TetrominoKind::I);
        assert!(board.cells.iter().flatten().all(|c| c.is_none()));
    }

    #[test]
    fn cell_to_world_places_origin_cell_at_half_cell_offset() {
        let origin = Vec2::ZERO;
        let cell = 30.0;
        assert_eq!(cell_to_world(IVec2::new(0, 0), origin, cell), Vec2::new(15.0, 15.0));
        assert_eq!(cell_to_world(IVec2::new(1, 0), origin, cell), Vec2::new(45.0, 15.0));
        assert_eq!(cell_to_world(IVec2::new(0, 1), origin, cell), Vec2::new(15.0, 45.0));
    }
}
