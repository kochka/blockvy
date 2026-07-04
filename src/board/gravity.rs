//! Discrete gravity: on each tick, attempt to move the active piece down by
//! one row. When the move would collide, the piece is locked into the board
//! and the next piece is drawn in place.

use std::time::Duration;

use bevy::prelude::*;

use crate::pieces::SevenBag;

use super::active_piece::ActivePiece;
use super::clear_delay::PendingLineClear;
use super::collision::can_place;
use super::grid::Board;
use super::lock::{LockOutcome, PieceLocked, draw_next_piece, stamp_lock};

/// Level-indexed gravity interval (milliseconds). Roughly NES-style: level
/// 1 ≈ 800 ms, halving every ~3 levels until 20+. The score level is
/// 1-based (level 1 = first run); levels beyond the table clamp to the
/// fastest entry.
const GRAVITY_TABLE_MS: [u64; 20] = [
    800, 720, 630, 550, 470, 380, 300, 220, 130, 100, //
    80, 80, 80, 70, 70, 70, 50, 50, 50, 30,
];

/// Returns the gravity interval for a 1-based level. Level 0 is treated
/// as level 1; levels past 20 clamp to the table's fastest entry.
pub fn gravity_interval_for(level: u32) -> Duration {
    let idx = (level.saturating_sub(1) as usize).min(GRAVITY_TABLE_MS.len() - 1);
    Duration::from_millis(GRAVITY_TABLE_MS[idx])
}

#[derive(Resource)]
pub struct GravityTimer(pub Timer);

impl Default for GravityTimer {
    fn default() -> Self {
        Self(Timer::new(gravity_interval_for(1), TimerMode::Repeating))
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut timer: ResMut<GravityTimer>,
    active_piece: Option<ResMut<ActivePiece>>,
    mut board: ResMut<Board>,
    mut bag: ResMut<SevenBag>,
    mut lock_events: MessageWriter<PieceLocked>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());

    let Some(mut piece) = active_piece else {
        return;
    };

    for _ in 0..timer.0.times_finished_this_tick() {
        if !try_step_down(&mut piece, &board) {
            resolve_lock(
                &mut piece,
                &mut board,
                &mut bag,
                &mut lock_events,
                &mut commands,
            );
            return;
        }
    }
}

/// Locks the piece and either finishes immediately (no completed rows) or
/// hands control off to the line-clear delay (rows detected). Kept public
/// so the input layer's hard-drop path can share the same resolution.
pub fn resolve_lock(
    piece: &mut ActivePiece,
    board: &mut Board,
    bag: &mut SevenBag,
    lock_events: &mut MessageWriter<PieceLocked>,
    commands: &mut Commands,
) {
    let cleared_rows = stamp_lock(board, piece);
    if cleared_rows.iter().any(Option::is_some) {
        commands.remove_resource::<ActivePiece>();
        commands.insert_resource(PendingLineClear::new(cleared_rows));
        return;
    }

    let (next_piece, topped_out) = draw_next_piece(bag, board);
    *piece = next_piece;
    lock_events.write(PieceLocked {
        outcome: LockOutcome {
            lines_cleared: 0,
            cleared_rows,
            topped_out,
        },
    });
}

/// Attempts to move the piece down by one row. Returns `true` and mutates
/// the piece if the new position is valid; otherwise leaves it untouched.
pub fn try_step_down(piece: &mut ActivePiece, board: &Board) -> bool {
    let candidate = piece.moved_down();
    if !can_place(&candidate, board) {
        return false;
    }
    *piece = candidate;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::TetrominoKind;
    use bevy::math::IVec2;

    #[test]
    fn steps_down_on_empty_board() {
        let board = Board::default();
        let mut piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 10));
        let advanced = try_step_down(&mut piece, &board);
        assert!(advanced);
        assert_eq!(piece.grid_position.y, 9);
    }

    #[test]
    fn holds_on_floor() {
        let board = Board::default();
        // T-North's bottom blocks sit at offset y=1, so y_origin=-1 places them on the floor.
        let mut piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, -1));
        let advanced = try_step_down(&mut piece, &board);
        assert!(!advanced);
        assert_eq!(piece.grid_position.y, -1);
    }

    #[test]
    fn holds_on_locked_stack() {
        let mut board = Board::default();
        // Fill row 5 completely; a T resting just above (origin y = 5) cannot descend.
        for x in 0..BOARD_WIDTH_FOR_TEST {
            board.set(x, 5, TetrominoKind::I);
        }
        let mut piece = ActivePiece::new(TetrominoKind::T, IVec2::new(4, 5));
        let advanced = try_step_down(&mut piece, &board);
        assert!(!advanced);
        assert_eq!(piece.grid_position.y, 5);
    }

    const BOARD_WIDTH_FOR_TEST: i32 = crate::board::BOARD_WIDTH as i32;

    #[test]
    fn gravity_interval_level_one_is_starting_speed() {
        assert_eq!(gravity_interval_for(1), Duration::from_millis(800));
    }

    #[test]
    fn gravity_interval_level_ten_matches_table() {
        assert_eq!(gravity_interval_for(10), Duration::from_millis(100));
    }

    #[test]
    fn gravity_interval_past_table_clamps_to_fastest() {
        assert_eq!(gravity_interval_for(20), Duration::from_millis(30));
        assert_eq!(gravity_interval_for(99), Duration::from_millis(30));
    }

    #[test]
    fn gravity_interval_level_zero_does_not_panic_and_uses_level_one() {
        assert_eq!(gravity_interval_for(0), Duration::from_millis(800));
    }
}
