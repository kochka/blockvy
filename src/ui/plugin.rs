use bevy::prelude::*;

use crate::game::GameState;

use super::board_render::render_board;
use super::game_over::{despawn_game_over_overlay, spawn_game_over_overlay};
use super::home::{despawn_home_overlay, home_button_system, home_keyboard, spawn_home_overlay};
use super::layout::spawn_layout;
use super::menu_button::update_button_visuals;
use super::next_panel::update_next_preview;
use super::options::{
    despawn_options_overlay, options_button_system, options_keyboard, refresh_option_labels,
    spawn_options_overlay,
};
use super::pause::{despawn_pause_overlay, pause_button_system, pause_keyboard, spawn_pause_overlay};
use super::score_panel::update_score_panel;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_layout))
            .add_systems(
                Update,
                (
                    render_board,
                    update_score_panel,
                    update_next_preview,
                    update_button_visuals,
                ),
            )
            .add_systems(OnEnter(GameState::Home), spawn_home_overlay)
            .add_systems(OnExit(GameState::Home), despawn_home_overlay)
            .add_systems(
                Update,
                (home_button_system, home_keyboard).run_if(in_state(GameState::Home)),
            )
            .add_systems(OnEnter(GameState::Options), spawn_options_overlay)
            .add_systems(OnExit(GameState::Options), despawn_options_overlay)
            .add_systems(
                Update,
                (options_button_system, refresh_option_labels, options_keyboard)
                    .run_if(in_state(GameState::Options)),
            )
            .add_systems(OnEnter(GameState::Paused), spawn_pause_overlay)
            .add_systems(OnExit(GameState::Paused), despawn_pause_overlay)
            .add_systems(
                Update,
                (pause_button_system, pause_keyboard).run_if(in_state(GameState::Paused)),
            )
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_overlay)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_overlay);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
