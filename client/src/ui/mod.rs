mod login;
mod character_creation;
mod character_selection;
mod game_ui;
mod npc_dialog;
mod pause;
mod settings;

pub use login::LoginPlugin;
pub use character_creation::CharacterCreationPlugin;
pub use character_selection::CharacterSelectionPlugin;
pub use game_ui::{GameUIPlugin, PlayerStats};
pub use npc_dialog::NpcDialogPlugin;
pub use pause::PausePlugin;
pub use settings::SettingsPlugin;

use bevy::prelude::*;

// Common UI styles
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}
