#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

/// Controls how rotation handles collisions against walls or the stack.
///
/// `Off` is a stricter mode: a rotation that doesn't fit in place is rejected.
/// `Srs` applies the official SRS kick tables to find a valid offset.
#[derive(bevy::prelude::Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WallKickMode {
    Off,
    #[default]
    Srs,
}

impl Rotation {
    pub fn rotated(self, direction: RotationDirection) -> Rotation {
        use Rotation::*;
        use RotationDirection::*;
        match (self, direction) {
            (North, Clockwise) => East,
            (East, Clockwise) => South,
            (South, Clockwise) => West,
            (West, Clockwise) => North,
            (North, CounterClockwise) => West,
            (East, CounterClockwise) => North,
            (South, CounterClockwise) => East,
            (West, CounterClockwise) => South,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clockwise_cycle() {
        let mut r = Rotation::North;
        for expected in [Rotation::East, Rotation::South, Rotation::West, Rotation::North] {
            r = r.rotated(RotationDirection::Clockwise);
            assert_eq!(r, expected);
        }
    }

    #[test]
    fn counterclockwise_cycle() {
        let mut r = Rotation::North;
        for expected in [Rotation::West, Rotation::South, Rotation::East, Rotation::North] {
            r = r.rotated(RotationDirection::CounterClockwise);
            assert_eq!(r, expected);
        }
    }

    #[test]
    fn cw_then_ccw_is_identity() {
        for start in [Rotation::North, Rotation::East, Rotation::South, Rotation::West] {
            let round_trip = start
                .rotated(RotationDirection::Clockwise)
                .rotated(RotationDirection::CounterClockwise);
            assert_eq!(round_trip, start);
        }
    }
}
