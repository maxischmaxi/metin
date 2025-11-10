use bevy::prelude::*;
use bevy::app::AppExit;
use crate::GameState;
use crate::GameFont;
use crate::auth_state::{AuthState, SpawnPosition};
use crate::networking::{NetworkClient, CharacterResponseEvent};
use super::{button_system, NORMAL_BUTTON};
use shared::ClientMessage;

pub struct CharacterSelectionPlugin;

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app        .add_systems(OnEnter(GameState::CharacterSelection), setup_character_selection)
            .add_systems(OnExit(GameState::CharacterSelection), cleanup_character_selection)
            .add_systems(Update, (
                button_system,
                character_card_hover_system,
                character_selection_buttons,
                handle_character_selected,
            ).run_if(in_state(GameState::CharacterSelection)));
    }
}

#[derive(Component)]
struct CharacterSelectionUI;

#[derive(Component)]
enum SelectionButton {
    SelectCharacter(i64), // character_id
    NewCharacter,
    Logout,
    QuitGame,
}

fn setup_character_selection(mut commands: Commands, auth_state: Res<AuthState>, font: Res<GameFont>) {
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
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        CharacterSelectionUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Charakter auswählen",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }));

        // Show characters or placeholder
        if auth_state.characters.is_empty() {
            parent.spawn(TextBundle::from_section(
                "Du hast noch keine Charaktere erstellt",
                TextStyle {
                font: font_handle.clone(),
                    font_size: 28.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            }));
        } else {
            // Display characters from server
            for character in auth_state.characters.iter() {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            padding: UiRect::all(Val::Px(15.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.25).into(),
                        border_color: Color::srgb(0.4, 0.6, 0.8).into(),
                        ..default()
                    },
                    SelectionButton::SelectCharacter(character.id),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        &character.name,
                        TextStyle {
                font: font_handle.clone(),
                            font_size: 32.0,
                            color: Color::srgb(1.0, 1.0, 0.4),
                            ..default()
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        format!("Klasse: {} | Level: {}", character.class.as_str(), character.level),
                        TextStyle {
                font: font_handle.clone(),
                            font_size: 22.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    }));
                    if let Some(ref last_played) = character.last_played {
                        parent.spawn(TextBundle::from_section(
                            format!("Zuletzt gespielt: {}", last_played),
                            TextStyle {
                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::srgb(0.6, 0.6, 0.6),
                                ..default()
                            },
                        ).with_style(Style {
                            margin: UiRect::top(Val::Px(3.0)),
                            ..default()
                        }));
                    }
                    // Hint text for selection
                    parent.spawn(TextBundle::from_section(
                        "Klicken um diesen Charakter zu spielen",
                        TextStyle {
                font: font_handle.clone(),
                            font_size: 16.0,
                            color: Color::srgb(0.4, 0.7, 1.0),
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    }));
                });
            }
        }

        // Create New Character button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(350.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            SelectionButton::NewCharacter,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+ Neuen Charakter erstellen",
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
                    width: Val::Px(350.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            SelectionButton::Logout,
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
                    width: Val::Px(350.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.5, 0.1, 0.1).into(),
                ..default()
            },
            SelectionButton::QuitGame,
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
}

fn character_selection_buttons(
    mut interaction_query: Query<(&Interaction, &SelectionButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    network: Option<Res<NetworkClient>>,
    mut auth_state: ResMut<AuthState>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                SelectionButton::SelectCharacter(character_id) => {
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
        (&Interaction, &mut BorderColor, &SelectionButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color, button) in query.iter_mut() {
        // Only apply special hover to character cards
        if matches!(button, SelectionButton::SelectCharacter(_)) {
            match *interaction {
                Interaction::Pressed => {
                    *border_color = Color::srgb(0.2, 0.8, 0.2).into();
                }
                Interaction::Hovered => {
                    *border_color = Color::srgb(0.6, 0.8, 1.0).into();
                }
                Interaction::None => {
                    *border_color = Color::srgb(0.4, 0.6, 0.8).into();
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

/// Handle character selection response from server
fn handle_character_selected(
    mut char_events: EventReader<CharacterResponseEvent>,
    mut spawn_position: ResMut<SpawnPosition>,
    mut player_stats: ResMut<crate::ui::PlayerStats>,
    mut auth_state: ResMut<crate::auth_state::AuthState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in char_events.read() {
        match event {
            CharacterResponseEvent::Selected { 
                character_id,
                character_name,
                position,
                level,
                experience,
                max_health,
                max_mana,
                max_stamina,
            } => {
                info!("Character '{}' (ID: {}) selected (Level {})", character_name, character_id, level);
                info!("  Spawn position: {:?}", position);
                info!("  Stats: HP={}, Mana={}, Stamina={}", max_health, max_mana, max_stamina);
                info!("  XP: {}/{}", experience, shared::calculate_xp_for_level(level + 1));
                
                // Set spawn position
                spawn_position.0 = *position;
                
                // Initialize player stats from character data
                player_stats.character_name = character_name.clone();
                player_stats.level = *level;
                player_stats.experience = *experience;
                player_stats.max_health = *max_health;
                player_stats.max_mana = *max_mana;
                player_stats.max_stamina = *max_stamina;
                player_stats.health = *max_health;  // Start with full health
                player_stats.mana = *max_mana;      // Start with full mana
                player_stats.stamina = *max_stamina; // Start with full stamina
                
                // Calculate XP needed for next level
                if *level < 100 {
                    player_stats.xp_needed = shared::calculate_xp_for_level(level + 1);
                } else {
                    player_stats.xp_needed = 0; // Max level
                }
                
                // Store class and specialization from selected character
                let (class, specialization) = if let Some(selected_char) = auth_state.get_selected_character() {
                    (Some(selected_char.class), selected_char.specialization)
                } else {
                    (None, None)
                };
                
                auth_state.class = class;
                auth_state.specialization = specialization;
                
                if let Some(spec) = specialization {
                    info!("  Specialization: {}", spec.name());
                } else {
                    info!("  No specialization chosen yet (unlocks at Level 5)");
                }
                
                next_state.set(GameState::InGame);
            }
            CharacterResponseEvent::SelectionFailed { reason } => {
                error!("Character selection failed: {}", reason);
                // TODO: Show error in UI
            }
            _ => {}
        }
    }
}
