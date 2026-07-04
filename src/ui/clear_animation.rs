//! Shader-driven clear-line overlay.
//!
//! When [`PendingLineClear`] is inserted (see `board/clear_delay.rs`),
//! one strip is spawned per completed row as a child of the
//! [`BoardPanel`]. Each strip carries a [`ClearLineMaterial`] whose
//! `params.x` is driven by the pending timer's progress. When the
//! resource is removed at the end of the delay, the strips are
//! despawned.
//!
//! The overlay sits on top of the existing block sprites (which
//! `render_board` keeps drawing while the pending clear is active,
//! because the board cells haven't been mutated yet), giving the shader
//! something to flash over.

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

use crate::board::{BOARD_VISIBLE_HEIGHT, PendingLineClear};

use super::board_panel::BoardPanel;

/// Kept small so we can pack more channels later without touching WGSL.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ClearLineMaterial {
    /// x = progress (0..1). y/z/w reserved for future use (row color...).
    #[uniform(0)]
    pub params: Vec4,
}

impl UiMaterial for ClearLineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_clear.wgsl".into()
    }
}

#[derive(Component)]
pub struct ClearLineOverlay;

pub fn spawn_clear_line_overlays(
    mut commands: Commands,
    pending: Option<Res<PendingLineClear>>,
    panel: Query<Entity, With<BoardPanel>>,
    existing: Query<Entity, With<ClearLineOverlay>>,
    mut materials: ResMut<Assets<ClearLineMaterial>>,
) {
    let Some(pending) = pending else {
        return;
    };
    // Only spawn on the frame the resource was added. Once the overlays
    // exist, `update_clear_line_overlays` drives their material progress.
    if !pending.is_added() {
        return;
    }
    let Ok(panel_entity) = panel.single() else {
        return;
    };
    // Belt-and-suspenders: if the previous cycle left an overlay
    // (shouldn't happen => `despawn_clear_line_overlays` clears it) get
    // rid of it before spawning a new one.
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let cell_h = 100.0 / BOARD_VISIBLE_HEIGHT as f32;
    commands.entity(panel_entity).with_children(|parent| {
        for row_slot in pending.rows {
            let Some(row) = row_slot else { continue };
            parent.spawn((
                ClearLineOverlay,
                MaterialNode(materials.add(ClearLineMaterial { params: Vec4::ZERO })),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.0),
                    bottom: Val::Percent(row as f32 * cell_h),
                    width: Val::Percent(100.0),
                    height: Val::Percent(cell_h),
                    ..default()
                },
                // Sit above the locked-cell siblings so the flash reads
                // clearly. Local ZIndex is enough here,the panel's
                // `overflow: clip()` still contains us because the
                // overlay is a plain child, not a new stacking context.
                ZIndex(10),
            ));
        }
    });
}

pub fn update_clear_line_overlays(
    pending: Option<Res<PendingLineClear>>,
    overlays: Query<&MaterialNode<ClearLineMaterial>, With<ClearLineOverlay>>,
    mut materials: ResMut<Assets<ClearLineMaterial>>,
) {
    let Some(pending) = pending else {
        return;
    };
    let progress = pending.progress();
    for handle in &overlays {
        if let Some(mut material) = materials.get_mut(&handle.0) {
            material.params.x = progress;
        }
    }
}

pub fn despawn_clear_line_overlays(
    mut commands: Commands,
    pending: Option<Res<PendingLineClear>>,
    overlays: Query<Entity, With<ClearLineOverlay>>,
) {
    if pending.is_some() {
        return;
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
}
