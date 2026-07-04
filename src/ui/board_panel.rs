use bevy::prelude::*;

use crate::board::{BOARD_VISIBLE_HEIGHT, BOARD_WIDTH};

const BOARD_BG: Color = Color::srgb(0.04, 0.04, 0.06);
const BOARD_BORDER: Color = Color::srgb(0.20, 0.20, 0.25);
const GRID_LINE: Color = Color::srgba(1.0, 1.0, 1.0, 0.01);

#[derive(Component)]
pub struct BoardPanel;

pub fn spawn_board_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            BoardPanel,
            Node {
                height: Val::Percent(95.0),
                aspect_ratio: Some(BOARD_WIDTH as f32 / BOARD_VISIBLE_HEIGHT as f32),
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(BOARD_BG),
            BorderColor::all(BOARD_BORDER),
        ))
        .with_children(spawn_board_grid);
}

fn spawn_board_grid(parent: &mut ChildSpawnerCommands) {
    let cell_w = 100.0 / BOARD_WIDTH as f32;
    let cell_h = 100.0 / BOARD_VISIBLE_HEIGHT as f32;

    for x in 1..BOARD_WIDTH {
        parent.spawn((
            grid_line_node(
                Val::Percent(x as f32 * cell_w),
                Val::Percent(0.0),
                Val::Px(1.0),
                Val::Percent(100.0),
            ),
            BackgroundColor(GRID_LINE),
        ));
    }

    for y in 1..BOARD_VISIBLE_HEIGHT {
        parent.spawn((
            grid_line_node(
                Val::Percent(0.0),
                Val::Percent(y as f32 * cell_h),
                Val::Percent(100.0),
                Val::Px(1.0),
            ),
            BackgroundColor(GRID_LINE),
        ));
    }
}

fn grid_line_node(left: Val, bottom: Val, width: Val, height: Val) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left,
        bottom,
        width,
        height,
        ..default()
    }
}
