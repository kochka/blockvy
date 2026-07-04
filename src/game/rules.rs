use bevy::prelude::{Reflect, ReflectDefault, ReflectResource, Resource};
use bevy::settings::{ReflectSettingsGroup, SettingsGroup};

use crate::pieces::WallKickMode;

/// Game-wide rule toggles. Defaults match the base Blockvy ruleset:
/// hard drop on, hold off, ghost piece on, SRS wall kicks.
///
/// Persisted through Bevy's built-in [`bevy::settings`] framework:
/// * `SettingsGroup` + `#[reflect(SettingsGroup)]` register this as one
///   section of the settings file.
/// * `Reflect` + `Default` are what the framework needs to load / reset.
#[derive(Resource, SettingsGroup, Reflect, Clone, Copy, Debug)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct GameRules {
    pub hard_drop_enabled: bool,
    pub hold_enabled: bool,
    pub ghost_enabled: bool,
    pub wall_kicks: WallKickMode,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            hard_drop_enabled: true,
            hold_enabled: false,
            ghost_enabled: true,
            wall_kicks: WallKickMode::Srs,
        }
    }
}
