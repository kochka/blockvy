//! Single source of truth for tetromino colors. Used by the board renderer
//! (solid + ghost) and by the next-piece preview, so all surfaces agree on
//! the palette.

use bevy::prelude::Color;

use crate::pieces::TetrominoKind;

pub fn piece_color(kind: TetrominoKind) -> Color {
    let (r, g, b) = piece_rgb(kind);
    Color::srgb(r, g, b)
}

pub fn piece_color_alpha(kind: TetrominoKind, alpha: f32) -> Color {
    let (r, g, b) = piece_rgb(kind);
    Color::srgba(r, g, b, alpha)
}

fn piece_rgb(kind: TetrominoKind) -> (f32, f32, f32) {
    match kind {
        TetrominoKind::I => (0.0, 0.85, 0.95),
        TetrominoKind::O => (0.95, 0.85, 0.0),
        TetrominoKind::T => (0.65, 0.20, 0.85),
        TetrominoKind::S => (0.20, 0.85, 0.20),
        TetrominoKind::Z => (0.90, 0.20, 0.20),
        TetrominoKind::J => (0.20, 0.30, 0.90),
        TetrominoKind::L => (0.95, 0.55, 0.10),
    }
}
