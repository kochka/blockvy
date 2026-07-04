//! Official SRS wall kick tables.
//!
//! Canonical SRS data is documented in y-down (screen coordinates with row 0
//! at the top). Blockvy uses y-up (y = 0 at the bottom of the board), so the
//! y component of every offset has been negated when transcribed here.
//!
//! - J / L / S / T / Z share a single table.
//! - I has its own table.
//! - O has no meaningful kicks.
//!
//! SRS only defines kicks for 90° transitions (N<->E, E<->S, S<->W, W<->N).
//! Any other transition returns an empty slice.

use bevy::math::IVec2;

use super::rotation::Rotation;
use super::tetromino::TetrominoKind;

const fn k(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}

// JLSTZ kick tables — one entry per 90° transition.
const JLSTZ_N_E: &[IVec2] = &[k(0, 0), k(-1, 0), k(-1, -1), k(0, 2), k(-1, 2)];
const JLSTZ_E_N: &[IVec2] = &[k(0, 0), k(1, 0), k(1, 1), k(0, -2), k(1, -2)];
const JLSTZ_E_S: &[IVec2] = &[k(0, 0), k(1, 0), k(1, 1), k(0, -2), k(1, -2)];
const JLSTZ_S_E: &[IVec2] = &[k(0, 0), k(-1, 0), k(-1, -1), k(0, 2), k(-1, 2)];
const JLSTZ_S_W: &[IVec2] = &[k(0, 0), k(1, 0), k(1, -1), k(0, 2), k(1, 2)];
const JLSTZ_W_S: &[IVec2] = &[k(0, 0), k(-1, 0), k(-1, 1), k(0, -2), k(-1, -2)];
const JLSTZ_W_N: &[IVec2] = &[k(0, 0), k(-1, 0), k(-1, 1), k(0, -2), k(-1, -2)];
const JLSTZ_N_W: &[IVec2] = &[k(0, 0), k(1, 0), k(1, -1), k(0, 2), k(1, 2)];

// I kick tables.
const I_N_E: &[IVec2] = &[k(0, 0), k(-2, 0), k(1, 0), k(-2, 1), k(1, -2)];
const I_E_N: &[IVec2] = &[k(0, 0), k(2, 0), k(-1, 0), k(2, -1), k(-1, 2)];
const I_E_S: &[IVec2] = &[k(0, 0), k(-1, 0), k(2, 0), k(-1, -2), k(2, 1)];
const I_S_E: &[IVec2] = &[k(0, 0), k(1, 0), k(-2, 0), k(1, 2), k(-2, -1)];
const I_S_W: &[IVec2] = &[k(0, 0), k(2, 0), k(-1, 0), k(2, -1), k(-1, 2)];
const I_W_S: &[IVec2] = &[k(0, 0), k(-2, 0), k(1, 0), k(-2, 1), k(1, -2)];
const I_W_N: &[IVec2] = &[k(0, 0), k(1, 0), k(-2, 0), k(1, 2), k(-2, -1)];
const I_N_W: &[IVec2] = &[k(0, 0), k(-1, 0), k(2, 0), k(-1, -2), k(2, 1)];

pub fn srs_wall_kicks(
    kind: TetrominoKind,
    from: Rotation,
    to: Rotation,
) -> &'static [IVec2] {
    match kind {
        TetrominoKind::O => &[],
        TetrominoKind::I => i_kicks(from, to),
        _ => jlstz_kicks(from, to),
    }
}

fn jlstz_kicks(from: Rotation, to: Rotation) -> &'static [IVec2] {
    use Rotation::*;
    match (from, to) {
        (North, East) => JLSTZ_N_E,
        (East, North) => JLSTZ_E_N,
        (East, South) => JLSTZ_E_S,
        (South, East) => JLSTZ_S_E,
        (South, West) => JLSTZ_S_W,
        (West, South) => JLSTZ_W_S,
        (West, North) => JLSTZ_W_N,
        (North, West) => JLSTZ_N_W,
        _ => &[],
    }
}

fn i_kicks(from: Rotation, to: Rotation) -> &'static [IVec2] {
    use Rotation::*;
    match (from, to) {
        (North, East) => I_N_E,
        (East, North) => I_E_N,
        (East, South) => I_E_S,
        (South, East) => I_S_E,
        (South, West) => I_S_W,
        (West, South) => I_W_S,
        (West, North) => I_W_N,
        (North, West) => I_N_W,
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TRANSITIONS: [(Rotation, Rotation); 8] = [
        (Rotation::North, Rotation::East),
        (Rotation::East, Rotation::North),
        (Rotation::East, Rotation::South),
        (Rotation::South, Rotation::East),
        (Rotation::South, Rotation::West),
        (Rotation::West, Rotation::South),
        (Rotation::West, Rotation::North),
        (Rotation::North, Rotation::West),
    ];

    #[test]
    fn jlstz_has_five_offsets_per_valid_transition() {
        for kind in [
            TetrominoKind::J,
            TetrominoKind::L,
            TetrominoKind::S,
            TetrominoKind::T,
            TetrominoKind::Z,
        ] {
            for (from, to) in VALID_TRANSITIONS {
                let kicks = srs_wall_kicks(kind, from, to);
                assert_eq!(
                    kicks.len(),
                    5,
                    "{:?} {:?} -> {:?} should have 5 kicks",
                    kind,
                    from,
                    to
                );
                assert_eq!(kicks[0], IVec2::ZERO, "first kick must be (0, 0)");
            }
        }
    }

    #[test]
    fn i_has_five_offsets_per_valid_transition() {
        for (from, to) in VALID_TRANSITIONS {
            let kicks = srs_wall_kicks(TetrominoKind::I, from, to);
            assert_eq!(kicks.len(), 5);
            assert_eq!(kicks[0], IVec2::ZERO);
        }
    }

    #[test]
    fn o_has_no_kicks() {
        for (from, to) in VALID_TRANSITIONS {
            assert!(srs_wall_kicks(TetrominoKind::O, from, to).is_empty());
        }
    }

    #[test]
    fn unsupported_transitions_return_empty() {
        // 180° and identity transitions are not defined by SRS.
        assert!(srs_wall_kicks(TetrominoKind::T, Rotation::North, Rotation::South).is_empty());
        assert!(srs_wall_kicks(TetrominoKind::I, Rotation::East, Rotation::West).is_empty());
        assert!(srs_wall_kicks(TetrominoKind::J, Rotation::North, Rotation::North).is_empty());
    }

    #[test]
    fn jlstz_n_to_e_matches_canonical() {
        // Canonical SRS N->E for JLSTZ (y-up): (0,0), (-1,0), (-1,-1), (0,+2), (-1,+2)
        let kicks = srs_wall_kicks(TetrominoKind::T, Rotation::North, Rotation::East);
        assert_eq!(
            kicks,
            &[
                IVec2::new(0, 0),
                IVec2::new(-1, 0),
                IVec2::new(-1, -1),
                IVec2::new(0, 2),
                IVec2::new(-1, 2),
            ]
        );
    }

    #[test]
    fn i_n_to_e_matches_canonical() {
        // Canonical SRS N->E for I (y-up): (0,0), (-2,0), (+1,0), (-2,+1), (+1,-2)
        let kicks = srs_wall_kicks(TetrominoKind::I, Rotation::North, Rotation::East);
        assert_eq!(
            kicks,
            &[
                IVec2::new(0, 0),
                IVec2::new(-2, 0),
                IVec2::new(1, 0),
                IVec2::new(-2, 1),
                IVec2::new(1, -2),
            ]
        );
    }
}
