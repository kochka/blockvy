//! GAME OVER overlay — spawned on [`GameState::GameOver`] entry and torn
//! down on exit. Sits as an absolutely-positioned root node above the
//! gameplay layout, with a high [`GlobalZIndex`] so the dim layer covers the
//! score and next panels too.

use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverOverlay;

pub fn spawn_game_over_overlay(mut commands: Commands) {
    commands
        .spawn((
            GameOverOverlay,
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
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press R to restart"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.70, 0.75)),
            ));
            parent.spawn((
                Text::new("Press Esc for menu"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.70, 0.75)),
            ));
        });
}

pub fn despawn_game_over_overlay(
    mut commands: Commands,
    overlay: Query<Entity, With<GameOverOverlay>>,
) {
    for entity in &overlay {
        commands.entity(entity).despawn();
    }
}
