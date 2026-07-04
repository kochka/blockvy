use bevy::prelude::*;

use crate::game::GameState;

use super::clear_delay::progress_pending_line_clear;
use super::gravity::{GravityTimer, apply_gravity};
use super::grid::Board;
use super::lock::PieceLocked;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<GravityTimer>()
            .add_message::<PieceLocked>()
            .add_systems(
                Update,
                (
                    apply_gravity,
                    // Runs after gravity so a lock initiated this frame gets
                    // the animation window; when the delay expires the same
                    // system emits the PieceLocked event score/game-over
                    // logic reacts to.
                    progress_pending_line_clear.after(apply_gravity),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
