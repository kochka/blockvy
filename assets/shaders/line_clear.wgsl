// Clear-line animation shader.
//
// Rendered as a UI overlay stretched across a completed row while the
// row is held on screen (see `board/clear_delay.rs`). Below this overlay
// the row's block cells are still drawn by `ui/board_render.rs`, so the
// shader just adds a bright flash + sweeping band on top; when the timer
// finishes the whole overlay is despawned and the row collapses.
//
// `params.x` is the animation progress (0.0 → 1.0). The other components
// are reserved so we can pass through per-row data later (piece color,
// row index, ...) without churning the material layout.

#import bevy_ui::ui_vertex_output::UiVertexOutput

struct ClearLineMaterial {
    params: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: ClearLineMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let t = clamp(material.params.x, 0.0, 1.0);
    let uv = in.uv;

    // Full-row flash that fades over the first ~60% of the animation.
    let base_flash = 1.0 - smoothstep(0.0, 0.6, t);

    // Bright band swept left → right, peaks in the first half.
    let band_x = t * 2.0;
    let band_dist = uv.x - band_x;
    let band = exp(-band_dist * band_dist * 25.0)
        * (1.0 - smoothstep(0.4, 0.65, t));

    // Slight vertical shading, the beam is brighter at the row's center.
    let vertical = clamp(1.0 - abs(uv.y - 0.5) * 1.2, 0.3, 1.0);

    let intensity = clamp(base_flash * 0.75 + band * 1.6, 0.0, 1.6) * vertical;

    // Tinted slightly toward warm white for a friendlier flash.
    let color = vec3<f32>(1.0, 0.98, 0.9) * intensity;
    let alpha = clamp(intensity, 0.0, 0.95);

    return vec4<f32>(color, alpha);
}
