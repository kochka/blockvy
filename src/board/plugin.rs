use bevy::prelude::*;

use crate::game::GameState;

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
                apply_gravity.run_if(in_state(GameState::Playing)),
            );
    }
}
