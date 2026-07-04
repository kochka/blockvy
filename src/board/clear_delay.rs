//! Line-clear delay: the small window where the completed rows are held
//! on screen for the clear animation to play.
//!
//! When gravity or hard-drop locks a piece that completes at least one
//! row, the [`ActivePiece`] resource is removed and [`PendingLineClear`]
//! is inserted. Gravity and input already early-return on a missing
//! active piece, so no extra gating is needed for those.
//!
//! [`progress_pending_line_clear`] ticks the timer; when it finishes it
//! collapses the stack, spawns the next piece, and emits the deferred
//! [`PieceLocked`] message. Score, game-over detection and audio only
//! react at the end of this cycle, so they don't have to know about the
//! delay.

use std::time::Duration;

use bevy::prelude::*;

use crate::pieces::SevenBag;

use super::grid::Board;
use super::lines::{ClearedRows, clear_rows};
use super::lock::{LockOutcome, PieceLocked, draw_next_piece};

/// How long the completed rows are held on screen for the shader animation
pub const LINE_CLEAR_DELAY: Duration = Duration::from_millis(400);

#[derive(Resource, Debug, Clone)]
pub struct PendingLineClear {
    pub rows: ClearedRows,
    pub timer: Timer,
}

impl PendingLineClear {
    pub fn new(rows: ClearedRows) -> Self {
        Self {
            rows,
            timer: Timer::new(LINE_CLEAR_DELAY, TimerMode::Once),
        }
    }

    /// 0.0 at start, 1.0 at completion — drives the shader uniform.
    pub fn progress(&self) -> f32 {
        self.timer.fraction()
    }
}

pub fn progress_pending_line_clear(
    time: Res<Time>,
    pending: Option<ResMut<PendingLineClear>>,
    mut board: ResMut<Board>,
    mut bag: ResMut<SevenBag>,
    mut commands: Commands,
    mut lock_events: MessageWriter<PieceLocked>,
) {
    let Some(mut pending) = pending else {
        return;
    };
    pending.timer.tick(time.delta());
    if !pending.timer.is_finished() {
        return;
    }

    let rows = pending.rows;
    let lines_cleared = clear_rows(&mut board, &rows);
    let (next_piece, topped_out) = draw_next_piece(&mut bag, &board);
    commands.insert_resource(next_piece);
    commands.remove_resource::<PendingLineClear>();
    lock_events.write(PieceLocked {
        outcome: LockOutcome {
            lines_cleared,
            cleared_rows: rows,
            topped_out,
        },
    });
}
