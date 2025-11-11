use bevy::prelude::*;
use bevy::app::AppExit;
use crate::GameState;
use crate::GameFont;
use crate::auth_state::AuthState;
use super::{button_system, NORMAL_BUTTON, UILayerStack, UILayerType};

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

fn setup_pause(mut commands: Commands, font: Res<GameFont>, mut ui_stack: ResMut<UILayerStack>) {
    // Register layer
    ui_stack.push_layer(UILayerType::PauseMenu);
    
    let font_handle = font.0.clone();
    
    // Outer fullscreen container (FULLY TRANSPARENT - game is visible!)
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::NONE.into(), // Completely transparent - no overlay!
            ..default()
        },
        PauseUI,
    ))
    .with_children(|parent| {
        // Inner floating window (pause menu box)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(520.0),
                padding: UiRect::all(Val::Px(40.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.90).into(), // Higher opacity for readability
            border_color: Color::srgba(0.5, 0.7, 1.0, 0.8).into(), // Brighter border for visibility
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Pause",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }));

            // Hint text
            parent.spawn(TextBundle::from_section(
                "Drücke ESC um fortzufahren",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            // Buttons in column layout
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Resume button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Settings button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Main Menu button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Logout button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Quit Game button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });
        });
    });
}

fn pause_buttons(
    mut interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<AuthState>,
    mut exit: EventWriter<AppExit>,
) {
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
    mut ui_stack: ResMut<UILayerStack>,
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::PauseMenu);
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
