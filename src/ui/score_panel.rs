use bevy::prelude::*;

use crate::game::Score;

const PANEL_BG: Color = Color::srgb(0.12, 0.12, 0.16);
const PANEL_BORDER: Color = Color::srgb(0.20, 0.20, 0.25);
const LABEL_COLOR: Color = Color::srgb(0.65, 0.65, 0.70);
const VALUE_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct ScorePanel;

#[derive(Component)]
pub struct ScoreValueText;

#[derive(Component)]
pub struct LevelValueText;

#[derive(Component)]
pub struct LinesValueText;

pub fn spawn_score_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            ScorePanel,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|panel| {
            spawn_field(panel, "SCORE", "0", ScoreValueText);
            spawn_field(panel, "LEVEL", "1", LevelValueText);
            spawn_field(panel, "LINES", "0", LinesValueText);
        });
}

fn spawn_field<M: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value: &str,
    marker: M,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|field| {
            field.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(LABEL_COLOR),
            ));
            field.spawn((
                marker,
                Text::new(value),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(VALUE_COLOR),
            ));
        });
}

/// Mirrors the [`Score`] resource into the score panel's three value texts.
/// Runs every frame but only writes when the resource has changed, so it's
/// effectively free between line clears.
pub fn update_score_panel(
    score: Res<Score>,
    mut score_q: Query<
        &mut Text,
        (
            With<ScoreValueText>,
            Without<LevelValueText>,
            Without<LinesValueText>,
        ),
    >,
    mut level_q: Query<
        &mut Text,
        (
            With<LevelValueText>,
            Without<ScoreValueText>,
            Without<LinesValueText>,
        ),
    >,
    mut lines_q: Query<
        &mut Text,
        (
            With<LinesValueText>,
            Without<ScoreValueText>,
            Without<LevelValueText>,
        ),
    >,
) {
    if !score.is_changed() {
        return;
    }
    if let Ok(mut text) = score_q.single_mut() {
        *text = Text::new(score.value.to_string());
    }
    if let Ok(mut text) = level_q.single_mut() {
        *text = Text::new(score.level.to_string());
    }
    if let Ok(mut text) = lines_q.single_mut() {
        *text = Text::new(score.lines_cleared.to_string());
    }
}
