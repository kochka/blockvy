//! PAUSE overlay — spawned on [`GameState::Paused`] entry and torn down on
//! exit. Mirrors the GameOver overlay structure: dim layer above the
//! gameplay panels with title and three actions.

use bevy::prelude::*;

use crate::game::{GameState, ResumeFromPause};

use super::menu_button::spawn_menu_button;

#[derive(Component)]
pub struct PauseOverlay;

/// Action a Paused button triggers when pressed.
#[derive(Component, Clone, Copy, Debug)]
pub enum PauseButton {
    Resume,
    Restart,
    Home,
}

pub fn spawn_pause_overlay(mut commands: Commands) {
    commands
        .spawn((
            PauseOverlay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.70)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));
            spawn_menu_button(parent, "Resume", PauseButton::Resume);
            spawn_menu_button(parent, "Restart", PauseButton::Restart);
            spawn_menu_button(parent, "Home", PauseButton::Home);
        });
}

pub fn despawn_pause_overlay(mut commands: Commands, overlay: Query<Entity, With<PauseOverlay>>) {
    for entity in &overlay {
        commands.entity(entity).despawn();
    }
}

/// Routes Pause button presses. Resume signals [`ResumeFromPause`] so the
/// `OnEnter(Playing)` reset is skipped; Restart leaves the flag untouched so
/// the same enter handler does its full fresh-run setup.
pub fn pause_button_system(
    mut interactions: Query<(&Interaction, &PauseButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut resume: ResMut<ResumeFromPause>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            PauseButton::Resume => {
                resume.0 = true;
                next_state.set(GameState::Playing);
            }
            PauseButton::Restart => {
                next_state.set(GameState::Playing);
            }
            PauseButton::Home => {
                next_state.set(GameState::Home);
            }
        }
    }
}

/// **Esc** resumes (paired with the Playing-side Esc that triggered the
/// pause), **R** restarts the run, **H** returns to the home screen.
/// R/H sit on the same physical key on AZERTY and QWERTY, so the bindings
/// stay portable.
pub fn pause_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut resume: ResMut<ResumeFromPause>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        resume.0 = true;
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::KeyH) {
        next_state.set(GameState::Home);
    }
}
