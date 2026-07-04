//! Options screen — toggles for the tunable [`GameRules`] fields. Each row
//! is a menu-styled button whose label reflects the current value; clicking
//! flips it and the persistence layer picks up the change automatically via
//! `Res::is_changed`, so no explicit save call lives here.

use bevy::prelude::*;

use crate::game::{GameRules, GameState};
use crate::pieces::WallKickMode;

use super::menu_button::{BUTTON_HEIGHT, spawn_menu_button};

const SCREEN_BG: Color = Color::srgb(0.06, 0.06, 0.10);
const TITLE_COLOR: Color = Color::srgb(0.95, 0.95, 1.0);
const HINT_COLOR: Color = Color::srgb(0.55, 0.55, 0.65);
const BUTTON_BG: Color = Color::srgb(0.14, 0.14, 0.18);
const BUTTON_BORDER: Color = Color::srgb(0.30, 0.30, 0.36);
const BUTTON_TEXT: Color = Color::srgb(0.92, 0.92, 0.95);
const OPTION_BUTTON_WIDTH: f32 = 320.0;

#[derive(Component)]
pub struct OptionsOverlay;

/// One value the Options screen can flip. Attached to a toggle button so a
/// single click handler can dispatch on it, and also to that button's text
/// child so [`refresh_option_labels`] can rewrite the label after a change.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionsToggle {
    HardDrop,
    Hold,
    Ghost,
    WallKicks,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum OptionsButton {
    Toggle(OptionsToggle),
    Back,
}

pub fn spawn_options_overlay(mut commands: Commands, rules: Res<GameRules>) {
    commands
        .spawn((
            OptionsOverlay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(SCREEN_BG),
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("OPTIONS"),
                TextFont {
                    font_size: FontSize::Px(56.0),
                    ..default()
                },
                TextColor(TITLE_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(24.0)),
                    ..default()
                },
            ));

            for toggle in [
                OptionsToggle::HardDrop,
                OptionsToggle::Hold,
                OptionsToggle::Ghost,
                OptionsToggle::WallKicks,
            ] {
                spawn_toggle_button(root, toggle, &rules);
            }

            root.spawn((
                Text::new("Changes are saved automatically."),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(HINT_COLOR),
                Node {
                    margin: UiRect::top(Val::Px(12.0)),
                    ..default()
                },
            ));

            spawn_menu_button(root, "Back", OptionsButton::Back);
        });
}

/// Wider variant of `spawn_menu_button` — the label carries both the option
/// name and its current value ("Hard Drop: ON"), so it needs more room than
/// the plain-verb buttons on Home / Pause.
fn spawn_toggle_button(
    parent: &mut ChildSpawnerCommands,
    toggle: OptionsToggle,
    rules: &GameRules,
) {
    parent
        .spawn((
            OptionsButton::Toggle(toggle),
            Button,
            Node {
                width: Val::Px(OPTION_BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            BorderColor::all(BUTTON_BORDER),
        ))
        .with_children(|button| {
            button.spawn((
                toggle,
                Text::new(toggle_label(toggle, rules)),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(BUTTON_TEXT),
            ));
        });
}

fn toggle_label(toggle: OptionsToggle, rules: &GameRules) -> String {
    let (name, value) = match toggle {
        OptionsToggle::HardDrop => ("Hard Drop", on_off(rules.hard_drop_enabled)),
        OptionsToggle::Hold => ("Hold", on_off(rules.hold_enabled)),
        OptionsToggle::Ghost => ("Ghost Piece", on_off(rules.ghost_enabled)),
        OptionsToggle::WallKicks => (
            "Wall Kicks",
            match rules.wall_kicks {
                WallKickMode::Off => "Off",
                WallKickMode::Srs => "SRS",
            },
        ),
    };
    format!("{name}: {value}")
}

fn on_off(v: bool) -> &'static str {
    if v { "On" } else { "Off" }
}

pub fn despawn_options_overlay(
    mut commands: Commands,
    overlay: Query<Entity, With<OptionsOverlay>>,
) {
    for entity in &overlay {
        commands.entity(entity).despawn();
    }
}

pub fn options_button_system(
    mut interactions: Query<
        (&Interaction, &OptionsButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut rules: ResMut<GameRules>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            OptionsButton::Toggle(OptionsToggle::HardDrop) => {
                rules.hard_drop_enabled = !rules.hard_drop_enabled;
            }
            OptionsButton::Toggle(OptionsToggle::Hold) => {
                rules.hold_enabled = !rules.hold_enabled;
            }
            OptionsButton::Toggle(OptionsToggle::Ghost) => {
                rules.ghost_enabled = !rules.ghost_enabled;
            }
            OptionsButton::Toggle(OptionsToggle::WallKicks) => {
                rules.wall_kicks = match rules.wall_kicks {
                    WallKickMode::Off => WallKickMode::Srs,
                    WallKickMode::Srs => WallKickMode::Off,
                };
            }
            OptionsButton::Back => next_state.set(GameState::Home),
        }
    }
}

/// Rewrites every visible toggle label whenever `GameRules` changes. Gated
/// on `is_changed` so it's a no-op between clicks.
pub fn refresh_option_labels(
    rules: Res<GameRules>,
    mut labels: Query<(&OptionsToggle, &mut Text)>,
) {
    if !rules.is_changed() {
        return;
    }
    for (toggle, mut text) in &mut labels {
        *text = Text::new(toggle_label(*toggle, &rules));
    }
}

pub fn options_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Backspace) {
        next_state.set(GameState::Home);
    }
}
