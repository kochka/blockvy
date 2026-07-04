use bevy::prelude::*;

use crate::board::GravityTimer;
use crate::pieces::SevenBag;

use super::score::{Score, apply_score_on_lock, sync_gravity_to_level};
use super::state::{
    GameState, ResumeFromPause, check_game_over, game_over_keyboard, on_game_over_enter,
    pause_on_escape, start_run,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // `GameRules` is inserted by `SettingsPlugin` (see `crate::persistence`),
        // so we don't init it here.
        app.init_resource::<SevenBag>()
            .init_resource::<Score>()
            .init_resource::<ResumeFromPause>()
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), start_run)
            .add_systems(OnEnter(GameState::Paused), pause_gravity_timer)
            .add_systems(OnExit(GameState::Paused), resume_gravity_timer)
            .add_systems(OnEnter(GameState::GameOver), on_game_over_enter)
            .add_systems(
                Update,
                (
                    apply_score_on_lock,
                    sync_gravity_to_level.after(apply_score_on_lock),
                    check_game_over,
                ),
            )
            .add_systems(Update, pause_on_escape.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                game_over_keyboard.run_if(in_state(GameState::GameOver)),
            );
    }
}

/// Freezes the gravity timer so the partial tick accumulated while playing
/// survives the pause and the piece does not silently drop on resume.
fn pause_gravity_timer(mut timer: ResMut<GravityTimer>) {
    timer.0.pause();
}

fn resume_gravity_timer(mut timer: ResMut<GravityTimer>) {
    timer.0.unpause();
}
