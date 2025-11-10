use bevy::prelude::*;
use bevy::app::AppExit;
use crate::GameState;
use crate::GameFont;
use crate::auth_state::AuthState;
use super::{button_system, NORMAL_BUTTON};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_pause)
            .add_systems(OnExit(GameState::Paused), cleanup_pause)
            .add_systems(Update, (
                button_system,
                pause_buttons,
            ).run_if(in_state(GameState::Paused)));
    }
}

#[derive(Component)]
struct PauseUI;

#[derive(Component)]
enum PauseButton {
    Resume,
    Settings,
    MainMenu,
    Logout,
    QuitGame,
}

fn setup_pause(mut commands: Commands, font: Res<GameFont>) {
    let font_handle = font.0.clone();
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
            background_color: Color::srgba(0.1, 0.1, 0.15, 0.95).into(),
            ..default()
        },
        PauseUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Pause",
            TextStyle {
                font: font_handle.clone(),
                font_size: 70.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }));

        // Hint text
        parent.spawn(TextBundle::from_section(
            "Drücke ESC um fortzufahren",
            TextStyle {
                font: font_handle.clone(),
                font_size: 22.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(50.0)),
            ..default()
        }));

        // Buttons in column layout
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Resume button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                    ..default()
                },
                PauseButton::Resume,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Weiterspielen",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Settings button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                PauseButton::Settings,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Einstellungen",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Main Menu button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                PauseButton::MainMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Zum Hauptmenü",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Logout button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                PauseButton::Logout,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Ausloggen",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Quit Game button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.5, 0.1, 0.1).into(),
                    ..default()
                },
                PauseButton::QuitGame,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Spiel beenden",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    });
}

fn pause_buttons(
    mut interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<AuthState>,
    mut exit: EventWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // ESC key to resume
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
        return;
    }

    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                PauseButton::Resume => {
                    info!("Resuming game");
                    next_state.set(GameState::InGame);
                }
                PauseButton::Settings => {
                    info!("Opening settings");
                    next_state.set(GameState::Settings);
                }
                PauseButton::MainMenu => {
                    info!("Returning to character selection");
                    next_state.set(GameState::CharacterSelection);
                }
                PauseButton::Logout => {
                    info!("Logging out");
                    auth_state.logout();
                    next_state.set(GameState::Login);
                }
                PauseButton::QuitGame => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn cleanup_pause(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
