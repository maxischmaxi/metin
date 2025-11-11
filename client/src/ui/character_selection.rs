use bevy::prelude::*;
use bevy::app::AppExit;
use crate::GameState;
use crate::GameFont;
use crate::auth_state::AuthState;
use crate::networking::NetworkClient;
use crate::skybox::GameTime;
use super::{button_system, NORMAL_BUTTON};
use shared::ClientMessage;

pub struct CharacterSelectionPlugin;

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CharacterSelection), setup_character_selection)
            .add_systems(OnExit(GameState::CharacterSelection), cleanup_character_selection)
            .add_systems(Update, (
                button_system,
                character_card_hover_system,
                character_selection_buttons,
                animate_particles,
            ).run_if(in_state(GameState::CharacterSelection)));
    }
}

// Medieval color palette
const MEDIEVAL_GOLD: Color = Color::srgb(0.85, 0.65, 0.13);
const MEDIEVAL_DARK_WOOD: Color = Color::srgb(0.2, 0.13, 0.08);
const MEDIEVAL_PARCHMENT: Color = Color::srgb(0.95, 0.87, 0.70);
const MEDIEVAL_BLOOD_RED: Color = Color::srgb(0.6, 0.1, 0.1);
const MEDIEVAL_DARK_STONE: Color = Color::srgb(0.15, 0.15, 0.18);
const MEDIEVAL_SILVER: Color = Color::srgb(0.75, 0.75, 0.80);
const MEDIEVAL_EMERALD: Color = Color::srgb(0.13, 0.55, 0.13);

#[derive(Component)]
struct CharacterSelectionUI;

#[derive(Component)]
enum SelectionButton {
    SelectCharacter(i64), // character_id
    NewCharacter,
    Logout,
    QuitGame,
}

#[derive(Component)]
struct FloatingParticle {
    velocity: Vec2,
    spawn_time: f32,
}

fn setup_character_selection(
    mut commands: Commands, 
    auth_state: Res<AuthState>, 
    font: Res<GameFont>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let font_handle = font.0.clone();
    let time_synced = game_time.time_synced;
    let current_time = time.elapsed_seconds();
    
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
            background_color: MEDIEVAL_DARK_STONE.into(),
            ..default()
        },
        CharacterSelectionUI,
    ))
    .with_children(|parent| {
        // Animated background particles
        for i in 0..25 {
            let x = (i as f32 * 73.5) % 100.0;
            let y = (i as f32 * 47.3) % 100.0;
            let speed_x = ((i as f32 * 12.7).sin() * 0.3 + 0.2) * if i % 2 == 0 { 1.0 } else { -1.0 };
            let speed_y = ((i as f32 * 8.3).cos() * 0.2 + 0.15);
            
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        width: Val::Px(4.0),
                        height: Val::Px(4.0),
                        ..default()
                    },
                    background_color: MEDIEVAL_GOLD.with_alpha(0.3).into(),
                    ..default()
                },
                FloatingParticle {
                    velocity: Vec2::new(speed_x, speed_y),
                    spawn_time: current_time,
                },
            ));
        }
        
        // Title
        parent.spawn(TextBundle::from_section(
            "⚔ WÄHLE DEINEN HELDEN ⚔",
            TextStyle {
                font: font_handle.clone(),
                font_size: 65.0,
                color: MEDIEVAL_GOLD,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }));
        
        // Subtitle
        parent.spawn(TextBundle::from_section(
            "Betrete das Königreich mit deinem tapferen Krieger",
            TextStyle {
                font: font_handle.clone(),
                font_size: 22.0,
                color: MEDIEVAL_PARCHMENT.with_alpha(0.8),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }));
        
        // Time sync status warning
        if !time_synced {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_DARK_WOOD.with_alpha(0.8).into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
            )
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "⏳ Die Zeit wird synchronisiert...",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 24.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
            });
        }

        // Character list container with medieval frame
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(25.0)),
                    margin: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    max_height: Val::Px(500.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: MEDIEVAL_DARK_WOOD.with_alpha(0.7).into(),
                border_color: MEDIEVAL_GOLD.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            // Show characters or placeholder
            if auth_state.characters.is_empty() {
                parent.spawn(TextBundle::from_section(
                    "📜 Keine Helden gefunden 📜",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 32.0,
                        color: MEDIEVAL_PARCHMENT.with_alpha(0.7),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }));
                
                parent.spawn(TextBundle::from_section(
                    "Erschaffe deinen ersten Charakter, um dein Abenteuer zu beginnen!",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 20.0,
                        color: MEDIEVAL_SILVER.with_alpha(0.8),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }));
            } else {
                // Display characters from server
                for character in auth_state.characters.iter() {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(550.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                margin: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Column,
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Start,
                                ..default()
                            },
                            background_color: MEDIEVAL_DARK_STONE.with_alpha(0.9).into(),
                            border_color: MEDIEVAL_SILVER.into(),
                            ..default()
                        },
                        SelectionButton::SelectCharacter(character.id),
                    ))
                    .with_children(|parent| {
                        // Character name with level badge
                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(15.0),
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            }
                        )
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                &character.name,
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 36.0,
                                    color: MEDIEVAL_GOLD,
                                    ..default()
                                },
                            ));
                            
                            // Level badge
                            parent.spawn(
                                NodeBundle {
                                    style: Style {
                                        padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    background_color: MEDIEVAL_BLOOD_RED.into(),
                                    border_color: MEDIEVAL_GOLD.into(),
                                    ..default()
                                }
                            )
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("Lvl {}", character.level),
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: 22.0,
                                        color: MEDIEVAL_GOLD,
                                        ..default()
                                    },
                                ));
                            });
                        });
                        
                        // Class info
                        parent.spawn(TextBundle::from_section(
                            format!("⚔ Klasse: {}", character.class.as_str()),
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 24.0,
                                color: MEDIEVAL_PARCHMENT,
                                ..default()
                            },
                        ).with_style(Style {
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        }));
                        
                        // Last played
                        if let Some(ref last_played) = character.last_played {
                            parent.spawn(TextBundle::from_section(
                                format!("📅 Zuletzt gespielt: {}", last_played),
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 18.0,
                                    color: MEDIEVAL_SILVER.with_alpha(0.7),
                                    ..default()
                                },
                            ).with_style(Style {
                                margin: UiRect::top(Val::Px(8.0)),
                                ..default()
                            }));
                        }
                        
                        // Divider
                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(1.0),
                                    margin: UiRect::vertical(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: MEDIEVAL_GOLD.with_alpha(0.3).into(),
                                ..default()
                            }
                        );
                        
                        // Action hint
                        parent.spawn(TextBundle::from_section(
                            "⚔ Klicken, um das Abenteuer fortzusetzen ⚔",
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: MEDIEVAL_EMERALD,
                                ..default()
                            },
                        ));
                    });
                }
            }
        });

        // Bottom buttons container
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|parent| {
            // Create New Character button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_EMERALD.into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
                SelectionButton::NewCharacter,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "⚔ NEUER HELD ⚔",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 26.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
            });

            // Logout button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_DARK_WOOD.into(),
                    border_color: MEDIEVAL_SILVER.into(),
                    ..default()
                },
                SelectionButton::Logout,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Ausloggen",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 24.0,
                        color: MEDIEVAL_PARCHMENT,
                        ..default()
                    },
                ));
            });

            // Quit Game button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_BLOOD_RED.into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
                SelectionButton::QuitGame,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Beenden",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 24.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
            });
        });
    });
}

fn animate_particles(
    mut query: Query<(&mut Style, &FloatingParticle)>,
    time: Res<Time>,
) {
    for (mut style, particle) in query.iter_mut() {
        // Update position
        if let Val::Percent(x) = style.left {
            let new_x = (x + particle.velocity.x * time.delta_seconds() * 10.0) % 100.0;
            style.left = Val::Percent(if new_x < 0.0 { new_x + 100.0 } else { new_x });
        }
        
        if let Val::Percent(y) = style.top {
            let new_y = (y + particle.velocity.y * time.delta_seconds() * 10.0) % 100.0;
            style.top = Val::Percent(if new_y < 0.0 { new_y + 100.0 } else { new_y });
        }
    }
}

fn character_selection_buttons(
    mut interaction_query: Query<(&Interaction, &SelectionButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    network: Option<Res<NetworkClient>>,
    mut auth_state: ResMut<AuthState>,
    game_time: Res<GameTime>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                SelectionButton::SelectCharacter(character_id) => {
                    // Block character selection until time is synced
                    if !game_time.time_synced {
                        warn!("⏰ Cannot select character - waiting for time sync!");
                        continue;
                    }
                    
                    if let Some(network) = network.as_ref() {
                        // Clone token before mutable borrow
                        let token = auth_state.get_token().map(|t| t.to_string());
                        
                        if let Some(token) = token {
                            info!("Requesting character selection for ID: {}", character_id);
                            
                            // Store selected character ID in auth state
                            auth_state.select_character(*character_id);
                            
                            if let Err(e) = network.send_message(&ClientMessage::SelectCharacter {
                                token,
                                character_id: *character_id,
                            }) {
                                error!("Failed to send character selection: {}", e);
                            }
                            // Don't transition yet - wait for server response with position
                        }
                    }
                }
                SelectionButton::NewCharacter => {
                    // Block character creation until time is synced
                    if !game_time.time_synced {
                        warn!("⏰ Cannot create character - waiting for time sync!");
                        continue;
                    }
                    next_state.set(GameState::CharacterCreation);
                }
                SelectionButton::Logout => {
                    info!("User logging out");
                    auth_state.logout();
                    next_state.set(GameState::Login);
                }
                SelectionButton::QuitGame => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn character_card_hover_system(
    mut query: Query<
        (&Interaction, &mut BorderColor, &mut BackgroundColor, &SelectionButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color, mut bg_color, button) in query.iter_mut() {
        match button {
            SelectionButton::SelectCharacter(_) => {
                match *interaction {
                    Interaction::Pressed => {
                        *border_color = MEDIEVAL_GOLD.into();
                        *bg_color = MEDIEVAL_DARK_STONE.with_alpha(1.0).into();
                    }
                    Interaction::Hovered => {
                        *border_color = MEDIEVAL_GOLD.into();
                        *bg_color = MEDIEVAL_DARK_STONE.with_alpha(0.95).into();
                    }
                    Interaction::None => {
                        *border_color = MEDIEVAL_SILVER.into();
                        *bg_color = MEDIEVAL_DARK_STONE.with_alpha(0.9).into();
                    }
                }
            }
            SelectionButton::NewCharacter => {
                match *interaction {
                    Interaction::Pressed => {
                        *bg_color = MEDIEVAL_EMERALD.with_alpha(0.7).into();
                    }
                    Interaction::Hovered => {
                        *bg_color = MEDIEVAL_EMERALD.with_alpha(0.9).into();
                    }
                    Interaction::None => {
                        *bg_color = MEDIEVAL_EMERALD.into();
                    }
                }
            }
            SelectionButton::Logout => {
                match *interaction {
                    Interaction::Pressed => {
                        *bg_color = MEDIEVAL_DARK_WOOD.with_alpha(0.7).into();
                    }
                    Interaction::Hovered => {
                        *bg_color = MEDIEVAL_DARK_WOOD.with_alpha(0.9).into();
                    }
                    Interaction::None => {
                        *bg_color = MEDIEVAL_DARK_WOOD.into();
                    }
                }
            }
            SelectionButton::QuitGame => {
                match *interaction {
                    Interaction::Pressed => {
                        *bg_color = MEDIEVAL_BLOOD_RED.with_alpha(0.7).into();
                    }
                    Interaction::Hovered => {
                        *bg_color = MEDIEVAL_BLOOD_RED.with_alpha(0.9).into();
                    }
                    Interaction::None => {
                        *bg_color = MEDIEVAL_BLOOD_RED.into();
                    }
                }
            }
        }
    }
}

fn cleanup_character_selection(
    mut commands: Commands,
    query: Query<Entity, With<CharacterSelectionUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// NOTE: Character selection handling (handle_character_selected) has been moved to networking.rs
// as a global handler that works in all game states, not just CharacterSelection state
