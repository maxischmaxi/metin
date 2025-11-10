use bevy::prelude::*;
use bevy::window::{WindowMode, PrimaryWindow};
use crate::GameState;
use crate::GameFont;
use shared::MMOSettings;
use super::{button_system, NORMAL_BUTTON, UILayerStack, UILayerType};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MMOSettings>()
            .add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(OnExit(GameState::Settings), cleanup_settings)
            .add_systems(Update, (
                button_system,
                settings_buttons,
                update_setting_displays,
            ).run_if(in_state(GameState::Settings)));
    }
}

#[derive(Component)]
struct SettingsUI;

#[derive(Component)]
enum SettingsButton {
    ToggleVsync,
    ToggleFullscreen,
    IncreaseMasterVolume,
    DecreaseMasterVolume,
    IncreaseMusicVolume,
    DecreaseMusicVolume,
    IncreaseSfxVolume,
    DecreaseSfxVolume,
    Back,
}

#[derive(Component)]
struct VsyncDisplay;

#[derive(Component)]
struct FullscreenDisplay;

#[derive(Component)]
struct MasterVolumeDisplay;

#[derive(Component)]
struct MusicVolumeDisplay;

#[derive(Component)]
struct SfxVolumeDisplay;

fn setup_settings(mut commands: Commands, settings: Res<MMOSettings>, font: Res<GameFont>, mut ui_stack: ResMut<UILayerStack>) {
    // Register layer
    ui_stack.push_layer(UILayerType::Settings);
    
    let _font_handle = font.0.clone();
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        SettingsUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Einstellungen",
            TextStyle {
                font_size: 55.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(25.0)),
            ..default()
        }));

        // Settings section
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                margin: UiRect::bottom(Val::Px(30.0)),
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgb(0.15, 0.15, 0.2).into(),
            border_color: Color::srgb(0.3, 0.3, 0.35).into(),
            ..default()
        })
        .with_children(|parent| {
            // Graphics header
            parent.spawn(TextBundle::from_section(
                "Grafik",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.8, 0.8, 0.2),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }));

            create_toggle_setting(parent, "VSync", settings.graphics.vsync,
                SettingsButton::ToggleVsync, VsyncDisplay);
            create_toggle_setting(parent, "Vollbild", settings.graphics.fullscreen,
                SettingsButton::ToggleFullscreen, FullscreenDisplay);

            // Audio header
            parent.spawn(TextBundle::from_section(
                "Audio",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.8, 0.8, 0.2),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(10.0)),
                ..default()
            }));

            create_volume_setting(parent, "Gesamtlautstärke", settings.audio.master_volume,
                SettingsButton::DecreaseMasterVolume, SettingsButton::IncreaseMasterVolume,
                MasterVolumeDisplay);
            create_volume_setting(parent, "Musik", settings.audio.music_volume,
                SettingsButton::DecreaseMusicVolume, SettingsButton::IncreaseMusicVolume,
                MusicVolumeDisplay);
            create_volume_setting(parent, "Soundeffekte", settings.audio.sfx_volume,
                SettingsButton::DecreaseSfxVolume, SettingsButton::IncreaseSfxVolume,
                SfxVolumeDisplay);
        });

        // Back button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(350.0),
                    height: Val::Px(65.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            SettingsButton::Back,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "← Zurück",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn create_toggle_setting(
    parent: &mut ChildBuilder,
    label: &str,
    value: bool,
    button_type: SettingsButton,
    display_marker: impl Component,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("{}:", label),
            TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    if value { "AN" } else { "AUS" },
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                display_marker,
            ));
        });
    });
}

fn create_volume_setting(
    parent: &mut ChildBuilder,
    label: &str,
    value: f32,
    decrease_button: SettingsButton,
    increase_button: SettingsButton,
    display_marker: impl Component,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("{}:", label),
            TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Decrease button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            decrease_button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "-",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });

        // Value display
        parent.spawn((
            TextBundle::from_section(
                format!("{:.0}%", value * 100.0),
                TextStyle {
                    font_size: 25.0,
                    color: Color::srgb(0.3, 0.8, 0.3),
                    ..default()
                },
            ),
            display_marker,
        ));

        // Increase button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            increase_button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn settings_buttons(
    mut interaction_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<MMOSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                SettingsButton::ToggleVsync => {
                    settings.graphics.vsync = !settings.graphics.vsync;
                    if let Ok(mut window) = window_query.get_single_mut() {
                        window.present_mode = if settings.graphics.vsync {
                            bevy::window::PresentMode::AutoVsync
                        } else {
                            bevy::window::PresentMode::AutoNoVsync
                        };
                    }
                }
                SettingsButton::ToggleFullscreen => {
                    settings.graphics.fullscreen = !settings.graphics.fullscreen;
                    if let Ok(mut window) = window_query.get_single_mut() {
                        window.mode = if settings.graphics.fullscreen {
                            WindowMode::BorderlessFullscreen
                        } else {
                            WindowMode::Windowed
                        };
                    }
                }
                SettingsButton::IncreaseMasterVolume => {
                    settings.audio.master_volume = (settings.audio.master_volume + 0.1).min(1.0);
                }
                SettingsButton::DecreaseMasterVolume => {
                    settings.audio.master_volume = (settings.audio.master_volume - 0.1).max(0.0);
                }
                SettingsButton::IncreaseMusicVolume => {
                    settings.audio.music_volume = (settings.audio.music_volume + 0.1).min(1.0);
                }
                SettingsButton::DecreaseMusicVolume => {
                    settings.audio.music_volume = (settings.audio.music_volume - 0.1).max(0.0);
                }
                SettingsButton::IncreaseSfxVolume => {
                    settings.audio.sfx_volume = (settings.audio.sfx_volume + 0.1).min(1.0);
                }
                SettingsButton::DecreaseSfxVolume => {
                    settings.audio.sfx_volume = (settings.audio.sfx_volume - 0.1).max(0.0);
                }
                SettingsButton::Back => {
                    info!("Going back to pause menu");
                    next_state.set(GameState::Paused);
                }
            }
        }
    }
}

fn update_setting_displays(
    settings: Res<MMOSettings>,
    mut vsync_query: Query<&mut Text, With<VsyncDisplay>>,
    mut fullscreen_query: Query<&mut Text, (With<FullscreenDisplay>, Without<VsyncDisplay>)>,
    mut master_volume_query: Query<&mut Text, (With<MasterVolumeDisplay>, Without<VsyncDisplay>, Without<FullscreenDisplay>)>,
    mut music_volume_query: Query<&mut Text, (With<MusicVolumeDisplay>, Without<MasterVolumeDisplay>, Without<VsyncDisplay>, Without<FullscreenDisplay>)>,
    mut sfx_volume_query: Query<&mut Text, (With<SfxVolumeDisplay>, Without<MusicVolumeDisplay>, Without<MasterVolumeDisplay>, Without<VsyncDisplay>, Without<FullscreenDisplay>)>,
) {
    if settings.is_changed() {
        for mut text in vsync_query.iter_mut() {
            text.sections[0].value = if settings.graphics.vsync { "AN".to_string() } else { "AUS".to_string() };
        }
        for mut text in fullscreen_query.iter_mut() {
            text.sections[0].value = if settings.graphics.fullscreen { "AN".to_string() } else { "AUS".to_string() };
        }
        for mut text in master_volume_query.iter_mut() {
            text.sections[0].value = format!("{:.0}%", settings.audio.master_volume * 100.0);
        }
        for mut text in music_volume_query.iter_mut() {
            text.sections[0].value = format!("{:.0}%", settings.audio.music_volume * 100.0);
        }
        for mut text in sfx_volume_query.iter_mut() {
            text.sections[0].value = format!("{:.0}%", settings.audio.sfx_volume * 100.0);
        }
    }
}

fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
    mut ui_stack: ResMut<UILayerStack>,
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::Settings);
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
