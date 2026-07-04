//! Renders locked cells, the ghost projection and the active piece inside the
//! [`BoardPanel`].
//!
//! Each block is a child UI node positioned absolutely with percent values,
//! so rendering tracks the panel's responsive size automatically and no
//! UI-to-world coordinate conversion is needed. The panel's `overflow: clip`
//! hides any active-piece blocks that sit in the spawn buffer above y = 20.
//!
//! Render order (back-to-front): locked cells → ghost → active piece. The
//! ghost is therefore visually overlaid by the active piece once they
//! coincide (i.e. the piece is grounded).

use bevy::prelude::*;

use crate::board::{ActivePiece, BOARD_VISIBLE_HEIGHT, BOARD_WIDTH, Board, projected_piece};
use crate::game::GameRules;
use crate::pieces::TetrominoKind;

use super::block_sprite::{BlockSpriteStyle, spawn_block_sprite};
use super::board_panel::BoardPanel;

#[derive(Component)]
pub struct RenderedBlock;

pub fn render_board(
    mut commands: Commands,
    board: Res<Board>,
    rules: Res<GameRules>,
    active_piece: Option<Res<ActivePiece>>,
    panel: Query<Entity, With<BoardPanel>>,
    rendered: Query<Entity, With<RenderedBlock>>,
) {
    for entity in &rendered {
        commands.entity(entity).despawn();
    }

    let Ok(panel_entity) = panel.single() else {
        return;
    };

    commands.entity(panel_entity).with_children(|parent| {
        for y in 0..board.cells.len() {
            for x in 0..board.cells[y].len() {
                if let Some(kind) = board.cells[y][x] {
                    spawn_block(
                        parent,
                        kind,
                        IVec2::new(x as i32, y as i32),
                        BlockSpriteStyle::Locked,
                    );
                }
            }
        }

        if let Some(piece) = active_piece {
            if rules.ghost_enabled {
                let ghost = projected_piece(&piece, &board);
                // Only render the ghost when it is not exactly under the
                // active piece — avoids a redundant translucent layer once
                // the piece is grounded.
                if ghost.grid_position != piece.grid_position {
                    for block in ghost.blocks() {
                        spawn_block(parent, piece.kind, block, BlockSpriteStyle::Ghost);
                    }
                }
            }

            for block in piece.blocks() {
                spawn_block(parent, piece.kind, block, BlockSpriteStyle::Active);
            }
        }
    });
}

fn spawn_block(
    parent: &mut ChildSpawnerCommands,
    kind: TetrominoKind,
    grid: IVec2,
    style: BlockSpriteStyle,
) {
    let cell_w = 100.0 / BOARD_WIDTH as f32;
    let cell_h = 100.0 / BOARD_VISIBLE_HEIGHT as f32;

    spawn_block_sprite(
        parent,
        RenderedBlock,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(grid.x as f32 * cell_w),
            bottom: Val::Percent(grid.y as f32 * cell_h),
            width: Val::Percent(cell_w),
            height: Val::Percent(cell_h),
            ..default()
        },
        kind,
        style,
    );
}
