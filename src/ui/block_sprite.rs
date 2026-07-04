//! Reusable UI block visual.
//!
//! The block itself is still pure Bevy UI: a colored root node plus a few
//! absolutely-positioned child layers for bevel, shine and shadow.

use bevy::prelude::*;

use crate::pieces::TetrominoKind;

use super::block_color::{piece_color, piece_color_alpha};

#[derive(Clone, Copy)]
pub enum BlockSpriteStyle {
    Locked,
    Active,
    Ghost,
}

pub fn spawn_block_sprite<M: Component>(
    parent: &mut ChildSpawnerCommands,
    marker: M,
    mut node: Node,
    kind: TetrominoKind,
    style: BlockSpriteStyle,
) {
    let palette = sprite_palette(kind, style);
    node.border_radius = BorderRadius::all(Val::Percent(10.0));
    node.overflow = Overflow::clip();

    parent
        .spawn((marker, node, BackgroundColor(palette.base)))
        .with_children(|block| {
            spawn_layer(block, layer_node(5.0, 5.0, 68.0, 13.0), palette.top);
            spawn_layer(block, layer_node(5.0, 5.0, 13.0, 68.0), palette.left);
            spawn_layer(block, layer_node(21.0, 18.0, 42.0, 13.0), palette.shine);
            spawn_layer(block, layer_node(12.0, 80.0, 83.0, 15.0), palette.bottom);
            spawn_layer(block, layer_node(80.0, 12.0, 15.0, 83.0), palette.right);
        });
}

fn spawn_layer(parent: &mut ChildSpawnerCommands, node: Node, color: Color) {
    parent.spawn((node, BackgroundColor(color)));
}

fn layer_node(left: f32, top: f32, width: f32, height: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(left),
        top: Val::Percent(top),
        width: Val::Percent(width),
        height: Val::Percent(height),
        border_radius: BorderRadius::all(Val::Percent(20.0)),
        ..default()
    }
}

struct SpritePalette {
    base: Color,
    top: Color,
    left: Color,
    shine: Color,
    bottom: Color,
    right: Color,
}

fn sprite_palette(kind: TetrominoKind, style: BlockSpriteStyle) -> SpritePalette {
    match style {
        BlockSpriteStyle::Locked => SpritePalette {
            base: piece_color(kind),
            top: Color::srgba(1.0, 1.0, 1.0, 0.22),
            left: Color::srgba(1.0, 1.0, 1.0, 0.14),
            shine: Color::srgba(1.0, 1.0, 1.0, 0.08),
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.26),
            right: Color::srgba(0.0, 0.0, 0.0, 0.18),
        },
        BlockSpriteStyle::Active => SpritePalette {
            base: piece_color(kind),
            top: Color::srgba(1.0, 1.0, 1.0, 0.36),
            left: Color::srgba(1.0, 1.0, 1.0, 0.24),
            shine: Color::srgba(1.0, 1.0, 1.0, 0.16),
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.20),
            right: Color::srgba(0.0, 0.0, 0.0, 0.12),
        },
        BlockSpriteStyle::Ghost => SpritePalette {
            base: piece_color_alpha(kind, 0.12),
            top: Color::srgba(1.0, 1.0, 1.0, 0.10),
            left: Color::srgba(1.0, 1.0, 1.0, 0.07),
            shine: Color::srgba(1.0, 1.0, 1.0, 0.04),
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.06),
            right: Color::srgba(0.0, 0.0, 0.0, 0.04),
        },
    }
}
