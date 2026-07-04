//! Home screen — title, Start / Options / Quit. Opaque overlay so it
//! covers the gameplay layout that lives behind it.

use bevy::prelude::*;

use crate::game::GameState;

use super::menu_button::spawn_menu_button;

const SCREEN_BG: Color = Color::srgb(0.06, 0.06, 0.10);
const TITLE_COLOR: Color = Color::srgb(0.95, 0.95, 1.0);
const SUBTITLE_COLOR: Color = Color::srgb(0.55, 0.55, 0.65);

#[derive(Component)]
pub struct HomeOverlay;

/// Action a Home button triggers when pressed.
#[derive(Component, Clone, Copy, Debug)]
pub enum HomeButton {
    Start,
    Options,
    Quit,
}

pub fn spawn_home_overlay(mut commands: Commands) {
    commands
        .spawn((
            HomeOverlay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(SCREEN_BG),
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("BLOCKVY"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(TITLE_COLOR),
            ));
            root.spawn((
                Text::new("Yet Another Tetris"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(SUBTITLE_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(24.0)),
                    ..default()
                },
            ));
            spawn_menu_button(root, "Start", HomeButton::Start);
            spawn_menu_button(root, "Options", HomeButton::Options);
            spawn_menu_button(root, "Quit", HomeButton::Quit);
        });
}

pub fn despawn_home_overlay(mut commands: Commands, overlay: Query<Entity, With<HomeOverlay>>) {
    for entity in &overlay {
        commands.entity(entity).despawn();
    }
}

/// Routes Home button presses to state changes (or app exit). Only the
/// `Pressed` edge counts; hover/none are handled by the shared visual
/// system in [`super::menu_button`].
pub fn home_button_system(
    mut interactions: Query<(&Interaction, &HomeButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            HomeButton::Start => next_state.set(GameState::Playing),
            HomeButton::Options => next_state.set(GameState::Options),
            HomeButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

/// Enter starts the run, Escape quits — convenient shortcuts so the menu
/// is keyboard-only friendly.
pub fn home_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
