use bevy::prelude::*;
use crate::GameState;
use crate::GameFont;
use crate::interaction::NpcDialogState;
use crate::npc::NpcType;
use crate::auth_state::AuthState;
use crate::ui::game_ui::PlayerStats;
use crate::networking::NetworkClient;
use shared::{Specialization, ClientMessage};
use super::{UILayerStack, UILayerType};

pub struct NpcDialogPlugin;

impl Plugin for NpcDialogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                spawn_npc_dialog,
                handle_dialog_buttons,
                cleanup_closed_dialog,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct NpcDialogUI;

#[derive(Component)]
enum DialogButton {
    Close,
    ChooseSpec(Specialization),
}

/// Spawn NPC dialog when state becomes active
fn spawn_npc_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    player_stats: Res<PlayerStats>,
    auth_state: Res<AuthState>,
    font: Res<GameFont>,
    existing_dialog: Query<Entity, With<NpcDialogUI>>,
) {
    // Only spawn if dialog is active and doesn't exist yet
    if !dialog_state.active || !existing_dialog.is_empty() {
        return;
    }
    
    // Note: Layer is registered in mouse_click_system for immediate ESC handling
    
    // Only handle SpecializationTrainer for now
    let Some(NpcType::SpecializationTrainer) = dialog_state.npc_type else { 
        return;
    };
    
    // Determine dialog content based on player level and specialization
    let (title, message, show_spec_buttons) = if player_stats.level < 5 {
        (
            "Meister der Künste".to_string(),
            "Du musst Level 5 erreichen, um eine Spezialisierung zu wählen.\n\nKehre zurück, wenn du stärker geworden bist.".to_string(),
            false,
        )
    } else if let Some(spec) = auth_state.specialization {
        (
            "Meister der Künste".to_string(),
            format!("Du hast bereits eine Spezialisierung gewählt:\n\n{}\n\nDieser Pfad ist nun dein Schicksal.", spec.name()),
            false,
        )
    } else {
        (
            "Wähle deine Spezialisierung".to_string(),
            "Du hast Level 5 erreicht! Es ist Zeit, deinen Pfad zu wählen.\n\nWähle weise, denn diese Entscheidung ist permanent!".to_string(),
            true,
        )
    };
    
    // Build dialog UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.6).into(),
            z_index: ZIndex::Global(200),
            ..default()
        },
        NpcDialogUI,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(700.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::srgb(0.15, 0.1, 0.05).into(),
            border_color: Color::srgb(0.6, 0.4, 0.1).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: font.0.clone(),
                    font_size: 36.0,
                    color: Color::srgb(1.0, 0.9, 0.3),
                },
            ).with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }));
            
            // Message
            parent.spawn(TextBundle::from_section(
                message,
                TextStyle {
                    font: font.0.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }));
            
            // Specialization buttons (only if level >= 5 and no spec chosen)
            if show_spec_buttons {
                // Try to get class from auth_state first, then from selected character
                let character_class = auth_state.class.or_else(|| {
                    auth_state.get_selected_character().map(|c| c.class)
                });
                
                if let Some(class) = character_class {
                    let spec1 = Specialization::from_class_and_index(class, 0).unwrap();
                    let spec2 = Specialization::from_class_and_index(class, 1).unwrap();
                    
                    info!("Showing specializations for class {}: {} and {}", 
                          class.as_str(), spec1.name(), spec2.name());
                    
                    // Spec buttons container
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        create_spec_button(parent, spec1, font.0.clone());
                        create_spec_button(parent, spec2, font.0.clone());
                    });
                } else {
                    // Fallback: show error if we can't determine class
                    parent.spawn(TextBundle::from_section(
                        "Fehler: Charakterklasse konnte nicht ermittelt werden.\nBitte logge dich erneut ein.",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 18.0,
                            color: Color::srgb(1.0, 0.3, 0.3),
                        },
                    ).with_style(Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..default()
                    }));
                    error!("Cannot determine character class for specialization selection!");
                }
            }
            
            // Close button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.2, 0.1).into(),
                    ..default()
                },
                DialogButton::Close,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Schließen",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
    });
}

fn create_spec_button(
    parent: &mut ChildBuilder,
    spec: Specialization,
    font: Handle<Font>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(10.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: Color::srgb(0.2, 0.15, 0.1).into(),
        border_color: Color::srgb(0.5, 0.4, 0.2).into(),
        border_radius: BorderRadius::all(Val::Px(8.0)),
        ..default()
    })
    .with_children(|parent| {
        // Spec name
        parent.spawn(TextBundle::from_section(
            spec.name(),
            TextStyle {
                font: font.clone(),
                font_size: 26.0,
                color: Color::srgb(1.0, 0.8, 0.2),
            },
        ).with_style(Style {
            align_self: AlignSelf::Center,
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        }));
        
        // Description
        parent.spawn(TextBundle::from_section(
            spec.description(),
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        }));
        
        // Choose button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(45.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            DialogButton::ChooseSpec(spec),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Wählen",
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

fn handle_dialog_buttons(
    interaction_query: Query<(&Interaction, &DialogButton), Changed<Interaction>>,
    mut dialog_state: ResMut<NpcDialogState>,
    mut auth_state: ResMut<AuthState>,
    network: Option<Res<NetworkClient>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                DialogButton::Close => {
                    dialog_state.close_dialog();
                }
                DialogButton::ChooseSpec(spec) => {
                    info!("Player chose specialization: {}", spec.name());
                    
                    // Send to server
                    if let Some(network) = &network {
                        if let Some(token) = auth_state.get_token() {
                            if let Err(e) = network.send_message(&ClientMessage::ChooseSpecialization {
                                token: token.to_string(),
                                specialization: *spec,
                            }) {
                                error!("Failed to send specialization choice: {}", e);
                            } else {
                                info!("Sent specialization choice to server");
                                // Optimistically set it locally
                                auth_state.specialization = Some(*spec);
                                dialog_state.close_dialog();
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Cleanup dialog UI when it's closed
fn cleanup_closed_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    dialog_query: Query<Entity, With<NpcDialogUI>>,
    mut ui_stack: ResMut<UILayerStack>,
) {
    if !dialog_state.active && !dialog_query.is_empty() {
        // Remove from stack
        ui_stack.remove_layer(UILayerType::NpcDialog);
        
        for entity in dialog_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
