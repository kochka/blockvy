use bevy::prelude::*;

use super::board_panel::spawn_board_panel;
use super::next_panel::spawn_next_panel;
use super::score_panel::spawn_score_panel;

const ROOT_BG: Color = Color::srgb(0.08, 0.08, 0.10);
const SIDE_WIDTH: f32 = 180.0;
const SIDE_HEIGHT_PCT: f32 = 95.0;
const SIDE_GAP: f32 = 16.0;

pub fn spawn_layout(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(ROOT_BG),
        ))
        .with_children(|root| {
            spawn_left_column(root);
            spawn_board_panel(root);
            spawn_right_column(root);
        });
}

fn spawn_left_column(parent: &mut ChildSpawnerCommands) {
    parent.spawn(side_column_node());
}

fn spawn_right_column(parent: &mut ChildSpawnerCommands) {
    parent.spawn(side_column_node()).with_children(|col| {
        spawn_score_panel(col);
        spawn_next_panel(col);
    });
}

fn side_column_node() -> Node {
    Node {
        width: Val::Px(SIDE_WIDTH),
        height: Val::Percent(SIDE_HEIGHT_PCT),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(SIDE_GAP),
        ..default()
    }
}
