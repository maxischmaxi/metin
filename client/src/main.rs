mod auth_state;
mod building;
mod camera;
mod collision;
mod interaction;
mod networking;
mod npc;
mod player;
mod ui;

use auth_state::{AuthState, SpawnPosition};

use bevy::prelude::*;
use ui::{UIStackPlugin, LoginPlugin, CharacterCreationPlugin, CharacterSelectionPlugin, GameUIPlugin, SettingsPlugin, PausePlugin, NpcDialogPlugin};
use networking::NetworkingPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;
use npc::NpcPlugin;
use interaction::InteractionPlugin;
use collision::CollisionPlugin;
use building::BuildingPlugin;

/// Global font resource for UI
#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Login,
    CharacterSelection,
    CharacterCreation,
    InGame,
    Paused,
    Settings,
}

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MMORPG".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }));
    
    // Load font BEFORE initializing UI plugins
    let asset_server = app.world().resource::<AssetServer>();
    let font_handle = asset_server.load("fonts/momo/momo.ttf");
    app.insert_resource(GameFont(font_handle));
    
    app.init_state::<GameState>()
        .init_resource::<AuthState>()
        .init_resource::<SpawnPosition>()
        .add_event::<networking::AuthResponseEvent>()
        .add_event::<networking::CharacterResponseEvent>()
        .add_plugins((
            NetworkingPlugin,
            BuildingPlugin,
            CollisionPlugin,
            PlayerPlugin,
            CameraPlugin,
            NpcPlugin,
            InteractionPlugin,
            UIStackPlugin,  // Must be before other UI plugins
            LoginPlugin,
            CharacterSelectionPlugin,
            CharacterCreationPlugin,
            GameUIPlugin,
            PausePlugin,
            SettingsPlugin,
            NpcDialogPlugin,
        ))
        .run();
}
