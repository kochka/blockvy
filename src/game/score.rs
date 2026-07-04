//! Score, line count and level progression.
//!
//! Points use the classic line-clear table multiplied by the current level.
//! Hard-drop and soft-drop bonuses are intentionally left out for now — they
//! can be layered on later with extra bookkeeping in the input system.

use bevy::prelude::*;

use crate::board::{GravityTimer, PieceLocked, gravity_interval_for};

#[derive(Resource, Debug, Clone, Copy)]
pub struct Score {
    pub value: u32,
    pub lines_cleared: u32,
    pub level: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            value: 0,
            lines_cleared: 0,
            level: 1,
        }
    }
}

impl Score {
    /// Awards points for the given number of lines and bumps the level once
    /// per 10 cleared lines. Points use the level at the moment of the clear,
    /// so the milestone clear that triggers a level-up still scores at the
    /// previous level multiplier.
    pub fn award(&mut self, lines_cleared: u32) {
        if lines_cleared == 0 {
            return;
        }
        let base = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 800,
        };
        self.value += base * self.level;
        self.lines_cleared += lines_cleared;
        self.level = 1 + self.lines_cleared / 10;
    }
}

/// System that consumes [`PieceLocked`] messages and updates the [`Score`].
pub fn apply_score_on_lock(mut events: MessageReader<PieceLocked>, mut score: ResMut<Score>) {
    for event in events.read() {
        score.award(event.outcome.lines_cleared);
    }
}

/// Re-tunes the gravity timer whenever the [`Score`] changes (run start,
/// level-up). Keeps the timer's already-elapsed time so a level-up
/// mid-tick doesn't reset progress toward the next fall step.
pub fn sync_gravity_to_level(score: Res<Score>, mut gravity: ResMut<GravityTimer>) {
    if !score.is_changed() {
        return;
    }
    let new_interval = gravity_interval_for(score.level);
    if gravity.0.duration() != new_interval {
        gravity.0.set_duration(new_interval);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_zero_level_one() {
        let s = Score::default();
        assert_eq!(s.value, 0);
        assert_eq!(s.lines_cleared, 0);
        assert_eq!(s.level, 1);
    }

    #[test]
    fn zero_lines_changes_nothing() {
        let mut s = Score::default();
        s.award(0);
        assert_eq!(s.value, 0);
        assert_eq!(s.lines_cleared, 0);
    }

    #[test]
    fn single_clear_uses_base_table_at_level_one() {
        let mut s = Score::default();
        s.award(1);
        assert_eq!(s.value, 100);
        s.award(2);
        assert_eq!(s.value, 400);
        s.award(3);
        assert_eq!(s.value, 900);
    }

    #[test]
    fn tetris_at_level_one_awards_800() {
        let mut s = Score::default();
        s.award(4);
        assert_eq!(s.value, 800);
        assert_eq!(s.lines_cleared, 4);
    }

    #[test]
    fn level_increments_every_ten_lines() {
        let mut s = Score::default();
        for _ in 0..9 {
            s.award(1);
        }
        assert_eq!(s.lines_cleared, 9);
        assert_eq!(s.level, 1);
        // The 10th clear should bump the level — and still score at level 1.
        let before = s.value;
        s.award(1);
        assert_eq!(s.lines_cleared, 10);
        assert_eq!(s.level, 2);
        assert_eq!(s.value - before, 100);
    }

    #[test]
    fn level_multiplier_scales_subsequent_clears() {
        let mut s = Score::default();
        s.level = 3;
        s.award(2);
        assert_eq!(s.value, 900);
    }
}
