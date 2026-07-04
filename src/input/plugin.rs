use bevy::prelude::*;

use crate::board::apply_gravity;
use crate::game::GameState;

use super::autoshift::AutoshiftState;
use super::keyboard::{SoftDropTimer, handle_input};
use super::timing::InputTiming;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputTiming>()
            .init_resource::<AutoshiftState>()
            .init_resource::<SoftDropTimer>()
            // Input must run before gravity so player moves and gravity ticks
            // resolve in a deterministic order on the same frame.
            .add_systems(
                Update,
                handle_input
                    .before(apply_gravity)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
