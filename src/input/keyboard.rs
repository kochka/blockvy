use std::time::Duration;

use bevy::prelude::*;

use crate::audio::PlaySfx;
use crate::board::{
    ActivePiece, Board, PieceLocked, can_place, finalize_lock, hard_drop, try_rotate,
    try_step_down,
};
use crate::game::GameRules;
use crate::pieces::{RotationDirection, SevenBag};

use super::autoshift::{AutoshiftInput, AutoshiftState, update_autoshift};
use super::timing::InputTiming;

/// Independent timer driving soft-drop autorepeat while the soft-drop key is
/// held. Separate from gravity so soft-drop can fire much faster without
/// disturbing the base gravity cadence.
#[derive(Resource)]
pub struct SoftDropTimer(pub Timer);

impl Default for SoftDropTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(50), TimerMode::Repeating))
    }
}

pub fn handle_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    timing: Res<InputTiming>,
    rules: Res<GameRules>,
    mut autoshift: ResMut<AutoshiftState>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    active_piece: Option<ResMut<ActivePiece>>,
    mut board: ResMut<Board>,
    mut bag: ResMut<SevenBag>,
    mut lock_events: MessageWriter<PieceLocked>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Some(mut piece) = active_piece else {
        return;
    };

    // Rotation — fires on press only, never on hold. Compare rotation
    // before/after `try_rotate`: a rejected rotation returns the piece
    // unchanged, so no sound plays in that case.
    if keys.just_pressed(KeyCode::KeyZ) {
        let rotated = try_rotate(*piece, RotationDirection::CounterClockwise, &board, rules.wall_kicks);
        if rotated.rotation != piece.rotation {
            sfx.write(PlaySfx::Rotate);
        }
        *piece = rotated;
    }
    if keys.just_pressed(KeyCode::KeyX) || keys.just_pressed(KeyCode::ArrowUp) {
        let rotated = try_rotate(*piece, RotationDirection::Clockwise, &board, rules.wall_kicks);
        if rotated.rotation != piece.rotation {
            sfx.write(PlaySfx::Rotate);
        }
        *piece = rotated;
    }

    // Hard drop — snaps to floor, then locks and respawns immediately.
    if rules.hard_drop_enabled && keys.just_pressed(KeyCode::Space) {
        sfx.write(PlaySfx::HardDrop);
        *piece = hard_drop(&piece, &board);
        let outcome = finalize_lock(&mut piece, &mut board, &mut bag);
        lock_events.write(PieceLocked { outcome });
        // Skip the rest of this frame's input on the fresh piece — keys held
        // during a hard drop shouldn't carry over to the newly spawned piece.
        return;
    }

    // Horizontal auto-shift.
    apply_horizontal_shift(&mut piece, &board, &mut autoshift, &timing, &keys, &time);

    // Soft drop — independent fast timer, only ticks while the key is held.
    apply_soft_drop(&mut piece, &board, &mut soft_drop_timer, &timing, &keys, &time);
}

fn apply_horizontal_shift(
    piece: &mut ActivePiece,
    board: &Board,
    autoshift: &mut AutoshiftState,
    timing: &InputTiming,
    keys: &ButtonInput<KeyCode>,
    time: &Time,
) {
    let input = AutoshiftInput {
        left_just: keys.just_pressed(KeyCode::ArrowLeft),
        right_just: keys.just_pressed(KeyCode::ArrowRight),
        left_held: keys.pressed(KeyCode::ArrowLeft),
        right_held: keys.pressed(KeyCode::ArrowRight),
    };

    let delta_ms = time.delta().as_secs_f32() * 1000.0;
    let shifts = update_autoshift(autoshift, input, timing, delta_ms);
    if shifts == 0 {
        return;
    }

    let direction = shifts.signum();
    for _ in 0..shifts.abs() {
        let candidate = piece.moved_by(IVec2::new(direction, 0));
        if !can_place(&candidate, board) {
            break;
        }
        *piece = candidate;
    }
}

fn apply_soft_drop(
    piece: &mut ActivePiece,
    board: &Board,
    timer: &mut SoftDropTimer,
    timing: &InputTiming,
    keys: &ButtonInput<KeyCode>,
    time: &Time,
) {
    let held = keys.pressed(KeyCode::ArrowDown);
    if !held {
        timer.0.reset();
        return;
    }

    // Keep the timer's period in sync with the configurable soft drop rate
    // (so live tuning in dev tools still works).
    let desired = Duration::from_millis(timing.soft_drop_ms as u64);
    if timer.0.duration() != desired {
        timer.0.set_duration(desired);
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        timer.0.reset();
        try_step_down(piece, board);
        return;
    }

    timer.0.tick(time.delta());
    for _ in 0..timer.0.times_finished_this_tick() {
        if !try_step_down(piece, board) {
            break;
        }
    }
}
