//! Horizontal auto-shift state machine (DAS / ARR).
//!
//! Decoupled from Bevy input types so the logic is trivially unit-testable:
//! callers translate keyboard state into [`AutoshiftInput`] and feed it in.

use bevy::prelude::Resource;

use super::timing::InputTiming;

#[derive(Resource, Default, Debug)]
pub struct AutoshiftState {
    /// Currently active direction: -1 (left), 0 (idle), +1 (right).
    direction: i32,
    /// Time accumulated since the active direction was first pressed, in ms.
    elapsed_ms: f32,
    /// True once DAS has elapsed and the autorepeat phase has begun.
    charged: bool,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct AutoshiftInput {
    pub left_just: bool,
    pub right_just: bool,
    pub left_held: bool,
    pub right_held: bool,
}

/// Returns a signed shift count for this frame.
/// Positive = shifts to the right, negative = shifts to the left.
pub fn update_autoshift(
    state: &mut AutoshiftState,
    input: AutoshiftInput,
    timing: &InputTiming,
    delta_ms: f32,
) -> i32 {
    if input.right_just || input.left_just {
        // Newest just-pressed wins.
        state.direction = if input.right_just { 1 } else { -1 };
        state.elapsed_ms = 0.0;
        state.charged = false;
        return state.direction;
    }

    let held = match (input.left_held, input.right_held) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    };

    if held == 0 || held != state.direction {
        *state = AutoshiftState::default();
        return 0;
    }

    state.elapsed_ms += delta_ms;
    let das = timing.das_ms as f32;
    let arr = (timing.arr_ms as f32).max(1.0);

    let mut shifts = 0;
    if !state.charged && state.elapsed_ms >= das {
        state.charged = true;
        state.elapsed_ms -= das;
        shifts += 1;
    }
    if state.charged {
        while state.elapsed_ms >= arr {
            state.elapsed_ms -= arr;
            shifts += 1;
        }
    }

    shifts * state.direction
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timing() -> InputTiming {
        InputTiming {
            das_ms: 170,
            arr_ms: 50,
            soft_drop_ms: 50,
        }
    }

    fn just_right() -> AutoshiftInput {
        AutoshiftInput {
            right_just: true,
            right_held: true,
            ..Default::default()
        }
    }

    fn hold_right() -> AutoshiftInput {
        AutoshiftInput {
            right_held: true,
            ..Default::default()
        }
    }

    #[test]
    fn just_pressed_emits_immediate_shift() {
        let mut state = AutoshiftState::default();
        let shifts = update_autoshift(&mut state, just_right(), &timing(), 0.0);
        assert_eq!(shifts, 1);
    }

    #[test]
    fn no_repeat_before_das_elapses() {
        let mut state = AutoshiftState::default();
        update_autoshift(&mut state, just_right(), &timing(), 0.0);
        // 160 ms of holding still < 170 ms DAS threshold.
        let shifts = update_autoshift(&mut state, hold_right(), &timing(), 160.0);
        assert_eq!(shifts, 0);
    }

    #[test]
    fn first_repeat_fires_after_das() {
        let mut state = AutoshiftState::default();
        update_autoshift(&mut state, just_right(), &timing(), 0.0);
        let shifts = update_autoshift(&mut state, hold_right(), &timing(), 180.0);
        assert_eq!(shifts, 1);
    }

    #[test]
    fn arr_emits_multiple_shifts_per_long_frame() {
        let mut state = AutoshiftState::default();
        update_autoshift(&mut state, just_right(), &timing(), 0.0);
        // Charge DAS first.
        update_autoshift(&mut state, hold_right(), &timing(), 170.0);
        // 150 ms after charge → 3 ARR ticks of 50 ms each.
        let shifts = update_autoshift(&mut state, hold_right(), &timing(), 150.0);
        assert_eq!(shifts, 3);
    }

    #[test]
    fn release_resets_state() {
        let mut state = AutoshiftState::default();
        update_autoshift(&mut state, just_right(), &timing(), 0.0);
        update_autoshift(&mut state, hold_right(), &timing(), 200.0);
        // Now release.
        let shifts = update_autoshift(&mut state, AutoshiftInput::default(), &timing(), 16.0);
        assert_eq!(shifts, 0);
        // And re-pressing should fire immediately again.
        let shifts = update_autoshift(&mut state, just_right(), &timing(), 0.0);
        assert_eq!(shifts, 1);
    }

    #[test]
    fn switching_direction_resets_and_shifts_once() {
        let mut state = AutoshiftState::default();
        update_autoshift(&mut state, just_right(), &timing(), 0.0);
        update_autoshift(&mut state, hold_right(), &timing(), 300.0); // charged

        let left_just = AutoshiftInput {
            left_just: true,
            left_held: true,
            ..Default::default()
        };
        let shifts = update_autoshift(&mut state, left_just, &timing(), 0.0);
        assert_eq!(shifts, -1);
    }
}
