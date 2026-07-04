//! "NEXT" preview panel.
//!
//! Renders the upcoming tetromino as scaled blocks inside a square preview
//! area. The shape is centered inside a virtual 4-cell grid so I (4 wide) and
//! the 3-wide pieces (J/L/S/T/Z) share the same visual scale.

use bevy::prelude::*;

use crate::pieces::{Rotation, SevenBag, TetrominoKind, blocks_for};

use super::block_sprite::{BlockSpriteStyle, spawn_block_sprite};

const PANEL_BG: Color = Color::srgb(0.12, 0.12, 0.16);
const PANEL_BORDER: Color = Color::srgb(0.20, 0.20, 0.25);
const LABEL_COLOR: Color = Color::srgb(0.65, 0.65, 0.70);
const PREVIEW_BG: Color = Color::srgb(0.04, 0.04, 0.06);

/// Number of cells along each side of the virtual preview grid. All pieces
/// are centered inside it so their relative scale matches the playfield.
const PREVIEW_GRID: f32 = 4.0;

#[derive(Component)]
pub struct NextPanel;

#[derive(Component)]
pub struct NextPreview;

#[derive(Component)]
pub struct NextPreviewBlock;

pub fn spawn_next_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            NextPanel,
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
            panel.spawn((
                Text::new("NEXT"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(LABEL_COLOR),
            ));
            panel.spawn((
                NextPreview,
                Node {
                    width: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                BackgroundColor(PREVIEW_BG),
            ));
        });
}

/// Mirrors [`SevenBag::peek_next`] into the preview panel. Diff-tracks the
/// currently shown piece so we only re-spawn UI nodes on actual changes.
pub fn update_next_preview(
    mut commands: Commands,
    mut bag: ResMut<SevenBag>,
    panel: Query<Entity, With<NextPreview>>,
    blocks: Query<Entity, With<NextPreviewBlock>>,
    mut current: Local<Option<TetrominoKind>>,
) {
    let upcoming = bag.peek_next();
    if *current == Some(upcoming) {
        return;
    }
    *current = Some(upcoming);

    for entity in &blocks {
        commands.entity(entity).despawn();
    }

    let Ok(panel_entity) = panel.single() else {
        return;
    };

    commands.entity(panel_entity).with_children(|parent| {
        draw_piece(parent, upcoming);
    });
}

fn draw_piece(parent: &mut ChildSpawnerCommands, kind: TetrominoKind) {
    let blocks = blocks_for(kind, Rotation::North);
    let (min_x, max_x, min_y, max_y) = bounding_box(&blocks);

    let width = (max_x - min_x + 1) as f32;
    let height = (max_y - min_y + 1) as f32;
    let cell_pct = 100.0 / PREVIEW_GRID;
    let offset_x_pct = (PREVIEW_GRID - width) * 0.5 * cell_pct;
    let offset_y_pct = (PREVIEW_GRID - height) * 0.5 * cell_pct;

    for block in blocks {
        let rel_x = (block.x - min_x) as f32;
        let rel_y = (block.y - min_y) as f32;
        spawn_block_sprite(
            parent,
            NextPreviewBlock,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(offset_x_pct + rel_x * cell_pct),
                bottom: Val::Percent(offset_y_pct + rel_y * cell_pct),
                width: Val::Percent(cell_pct),
                height: Val::Percent(cell_pct),
                ..default()
            },
            kind,
            BlockSpriteStyle::Active,
        );
    }
}

fn bounding_box(blocks: &[IVec2; 4]) -> (i32, i32, i32, i32) {
    let mut min_x = blocks[0].x;
    let mut max_x = blocks[0].x;
    let mut min_y = blocks[0].y;
    let mut max_y = blocks[0].y;
    for b in &blocks[1..] {
        min_x = min_x.min(b.x);
        max_x = max_x.max(b.x);
        min_y = min_y.min(b.y);
        max_y = max_y.max(b.y);
    }
    (min_x, max_x, min_y, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_for_t_north_is_3x2() {
        let blocks = blocks_for(TetrominoKind::T, Rotation::North);
        let (min_x, max_x, min_y, max_y) = bounding_box(&blocks);
        assert_eq!(max_x - min_x + 1, 3);
        assert_eq!(max_y - min_y + 1, 2);
    }

    #[test]
    fn bounding_box_for_i_north_is_4x1() {
        let blocks = blocks_for(TetrominoKind::I, Rotation::North);
        let (min_x, max_x, min_y, max_y) = bounding_box(&blocks);
        assert_eq!(max_x - min_x + 1, 4);
        assert_eq!(max_y - min_y + 1, 1);
    }

    #[test]
    fn bounding_box_for_o_is_2x2() {
        let blocks = blocks_for(TetrominoKind::O, Rotation::North);
        let (min_x, max_x, min_y, max_y) = bounding_box(&blocks);
        assert_eq!(max_x - min_x + 1, 2);
        assert_eq!(max_y - min_y + 1, 2);
    }
}
