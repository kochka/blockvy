//! Shared style + visual feedback for the menu screens' buttons.
//! Home, Options and GameOver all reuse the same node bundle and color
//! transitions so the menus feel coherent.

use bevy::prelude::*;

pub const BUTTON_WIDTH: f32 = 220.0;
pub const BUTTON_HEIGHT: f32 = 48.0;

const BUTTON_BG: Color = Color::srgb(0.14, 0.14, 0.18);
const BUTTON_BG_HOVER: Color = Color::srgb(0.22, 0.22, 0.28);
const BUTTON_BG_PRESS: Color = Color::srgb(0.30, 0.30, 0.40);
const BUTTON_BORDER: Color = Color::srgb(0.30, 0.30, 0.36);
const BUTTON_TEXT: Color = Color::srgb(0.92, 0.92, 0.95);

/// Spawns a labeled button as a child of `parent` and tags it with the
/// caller-provided marker so a single interaction system can dispatch on
/// the marker enum.
pub fn spawn_menu_button<M: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: M,
) {
    parent
        .spawn((
            marker,
            Button,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            BorderColor::all(BUTTON_BORDER),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(BUTTON_TEXT),
            ));
        });
}

/// One generic system that drives the hover / pressed background tint for
/// every menu button in the app.
pub fn update_button_visuals(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut buttons {
        *color = match interaction {
            Interaction::Pressed => BackgroundColor(BUTTON_BG_PRESS),
            Interaction::Hovered => BackgroundColor(BUTTON_BG_HOVER),
            Interaction::None => BackgroundColor(BUTTON_BG),
        };
    }
}
