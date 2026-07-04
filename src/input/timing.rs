use bevy::prelude::Resource;

/// Tunable input timings. Values are in milliseconds.
#[derive(Resource, Clone, Copy, Debug)]
pub struct InputTiming {
    /// Delayed Auto Shift: how long a horizontal direction must be held
    /// before auto-repeat begins.
    pub das_ms: u32,
    /// Auto Repeat Rate: interval between repeats once DAS has elapsed.
    pub arr_ms: u32,
    /// Interval between soft-drop steps while the soft-drop key is held.
    pub soft_drop_ms: u32,
}

impl Default for InputTiming {
    fn default() -> Self {
        Self {
            das_ms: 170,
            arr_ms: 50,
            soft_drop_ms: 50,
        }
    }
}
