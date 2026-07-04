//! Settings persistence, delegated to Bevy's built-in [`bevy::settings`]
//! framework.
//!
//! Types opt in by deriving `SettingsGroup` (see [`GameRules`]); Bevy owns
//! the file format (TOML), the load-on-startup step, and the platform-
//! specific storage backend — a config file on native, browser storage on
//! wasm. All this plugin does is:
//! * Register [`SettingsPlugin`] with a reverse-DNS app id so the loader
//!   knows where to read from.
//! * Watch for changes to `GameRules` and queue a debounced save so a burst
//!   of toggles in the Options screen collapses to a single write.

use std::time::Duration;

use bevy::prelude::*;
use bevy::settings::{SaveSettingsDeferred, SettingsPlugin};

use crate::game::GameRules;

/// Coalesce a rapid series of toggles into a single write. Half a second is
/// well above input latency but small enough that a user closing the game
/// right after a change still sees the setting persisted.
const SAVE_DEBOUNCE: Duration = Duration::from_millis(500);

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin::new("dev.blockvy.blockvy"))
            .add_systems(Update, save_on_rules_change);
    }
}

/// The `Local<bool>` gate skips the unavoidable first-frame `is_changed`
/// pulse from `SettingsPlugin` inserting the resource — without it we would
/// queue a save on every launch even when nothing was touched.
fn save_on_rules_change(
    rules: Res<GameRules>,
    mut commands: Commands,
    mut initialized: Local<bool>,
) {
    if !*initialized {
        *initialized = true;
        return;
    }
    if !rules.is_changed() {
        return;
    }
    commands.queue(SaveSettingsDeferred(SAVE_DEBOUNCE));
}
