use bevy::prelude::*;

mod audio;
mod board;
mod game;
mod input;
mod persistence;
mod pieces;
mod ui;

use audio::AudioPlugin;
use board::BoardPlugin;
use game::GamePlugin;
use input::InputPlugin;
use persistence::PersistencePlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Blockvy".to_string(),
                // On wasm, the canvas tracks its parent's CSS size so the
                // viewport fills the page. No-op on native.
                fit_canvas_to_parent: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 720.0,
                    min_height: 640.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        // PersistencePlugin first: it registers `SettingsPlugin`, which
        // must run before any plugin that reads `GameRules`.
        .add_plugins((
            PersistencePlugin,
            GamePlugin,
            BoardPlugin,
            UiPlugin,
            InputPlugin,
            AudioPlugin,
        ))
        .run();
}
