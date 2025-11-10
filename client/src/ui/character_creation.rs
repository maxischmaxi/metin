use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use crate::GameState;
use crate::GameFont;
use crate::networking::{NetworkClient, send_create_character, CharacterResponseEvent};
use crate::auth_state::AuthState;
use shared::{CharacterClass, CharacterData, CharacterAppearance, ClientMessage};
use super::{button_system, NORMAL_BUTTON};

pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterBuilder>()
            .add_systems(OnEnter(GameState::CharacterCreation), setup_character_creation)
            .add_systems(OnExit(GameState::CharacterCreation), cleanup_character_creation)
            .add_systems(Update, (
                button_system,
                character_creation_buttons,
                update_character_preview,
                handle_text_input,
                update_name_display,
                handle_character_created,
            ).run_if(in_state(GameState::CharacterCreation)));
    }
}

#[derive(Resource, Default)]
struct CharacterBuilder {
    name: String,
    class: CharacterClass,
}

#[derive(Component)]
struct CharacterCreationUI;

#[derive(Component)]
enum CreationButton {
    ClassKrieger,
    ClassNinja,
    ClassSura,
    ClassSchamane,
    Create,
    Back,
}

#[derive(Component)]
struct NameInputDisplay;

#[derive(Component)]
struct NameInputBox;

#[derive(Component)]
struct ClassDisplay;

fn setup_character_creation(mut commands: Commands, mut builder: ResMut<CharacterBuilder>, font: Res<GameFont>) {
    let font_handle = font.0.clone();
    builder.name = String::from("Hero");
    builder.class = CharacterClass::Krieger;

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
        CharacterCreationUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Charakter erstellen",
            TextStyle {
                font: font_handle.clone(),
                font_size: 55.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(25.0)),
            ..default()
        }));

        // Name section
        parent.spawn(TextBundle::from_section(
            "Charaktername: (Tippen zum Bearbeiten)",
            TextStyle {
                font: font_handle.clone(),
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        }));

        // Name input box with background
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                border_color: Color::srgb(0.4, 0.6, 0.8).into(),
                ..default()
            },
            NameInputBox,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Hero_",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 30.0,
                        color: Color::srgb(1.0, 1.0, 0.4),
                        ..default()
                    },
                ),
                NameInputDisplay,
            ));
        });

        // Input hint
        parent.spawn(TextBundle::from_section(
            "(Rücktaste zum Löschen, max. 20 Zeichen)",
            TextStyle {
                font: font_handle.clone(),
                font_size: 18.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }));

        // Class section
        parent.spawn(TextBundle::from_section(
            "Klasse wählen:",
            TextStyle {
                font: font_handle.clone(),
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        }));

        // Class buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            create_class_button(parent, "Krieger", CreationButton::ClassKrieger, font_handle.clone());
            create_class_button(parent, "Ninja", CreationButton::ClassNinja, font_handle.clone());
            create_class_button(parent, "Sura", CreationButton::ClassSura, font_handle.clone());
            create_class_button(parent, "Schamane", CreationButton::ClassSchamane, font_handle.clone());
        });

        // Current class display
        parent.spawn((
            TextBundle::from_section(
                "Gewählte Klasse: Krieger",
                TextStyle {
                font: font_handle.clone(),
                    font_size: 30.0,
                    color: Color::srgb(0.3, 0.8, 0.3),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            }),
            ClassDisplay,
        ));

        // Bottom buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Back button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(230.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                CreationButton::Back,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "← Zurück",
                    TextStyle {
                font: font_handle.clone(),
                        font_size: 28.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Create button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(230.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                    ..default()
                },
                CreationButton::Create,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Erstellen ✓",
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
}

fn create_class_button(parent: &mut ChildBuilder, label: &str, button_type: CreationButton, font: Handle<Font>) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(60.0),
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
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn character_creation_buttons(
    mut interaction_query: Query<(&Interaction, &CreationButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut builder: ResMut<CharacterBuilder>,
    auth_state: Res<AuthState>,
    network: Option<Res<NetworkClient>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                CreationButton::ClassKrieger => builder.class = CharacterClass::Krieger,
                CreationButton::ClassNinja => builder.class = CharacterClass::Ninja,
                CreationButton::ClassSura => builder.class = CharacterClass::Sura,
                CreationButton::ClassSchamane => builder.class = CharacterClass::Schamane,
                CreationButton::Create => {
                    // Create character (starts at level 1 with 0 XP, no specialization yet)
                    let character = CharacterData {
                        name: if builder.name.is_empty() { "Hero".to_string() } else { builder.name.clone() },
                        class: builder.class,
                        appearance: CharacterAppearance::default(),
                        level: 1,
                        experience: 0,
                        specialization: None,  // Unlocked at level 5
                    };

                    info!("Creating character: {:?}", character);

                    // Send to server if authenticated
                    if let Some(token) = auth_state.get_token() {
                        if let Some(network) = network.as_ref() {
                            match send_create_character(network, token, character) {
                                Ok(_) => {
                                    info!("Character creation request sent - waiting for server response");
                                    // NOTE: We wait for CharacterCreated event before transitioning
                                    // The handle_character_created system will automatically select 
                                    // the new character and transition to InGame
                                }
                                Err(e) => {
                                    error!("Failed to send character creation: {}", e);
                                }
                            }
                        }
                    } else {
                        warn!("No auth token, going to game anyway (offline mode)");
                        next_state.set(GameState::InGame);
                    }
                }
                CreationButton::Back => {
                    next_state.set(GameState::CharacterSelection);
                }
            }
        }
    }
}

fn handle_text_input(
    mut builder: ResMut<CharacterBuilder>,
    mut evr_kbd: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                KeyCode::Backspace => {
                    builder.name.pop();
                }
                KeyCode::Space => {
                    if builder.name.len() < 20 {
                        builder.name.push(' ');
                    }
                }
                _ => {}
            }
        }
    }

    // Handle character input
    for key in keys.get_just_pressed() {
        if builder.name.len() >= 20 {
            continue;
        }

        let character = match key {
            KeyCode::KeyA => Some('A'),
            KeyCode::KeyB => Some('B'),
            KeyCode::KeyC => Some('C'),
            KeyCode::KeyD => Some('D'),
            KeyCode::KeyE => Some('E'),
            KeyCode::KeyF => Some('F'),
            KeyCode::KeyG => Some('G'),
            KeyCode::KeyH => Some('H'),
            KeyCode::KeyI => Some('I'),
            KeyCode::KeyJ => Some('J'),
            KeyCode::KeyK => Some('K'),
            KeyCode::KeyL => Some('L'),
            KeyCode::KeyM => Some('M'),
            KeyCode::KeyN => Some('N'),
            KeyCode::KeyO => Some('O'),
            KeyCode::KeyP => Some('P'),
            KeyCode::KeyQ => Some('Q'),
            KeyCode::KeyR => Some('R'),
            KeyCode::KeyS => Some('S'),
            KeyCode::KeyT => Some('T'),
            KeyCode::KeyU => Some('U'),
            KeyCode::KeyV => Some('V'),
            KeyCode::KeyW => Some('W'),
            KeyCode::KeyX => Some('X'),
            KeyCode::KeyY => Some('Y'),
            KeyCode::KeyZ => Some('Z'),
            KeyCode::Digit0 => Some('0'),
            KeyCode::Digit1 => Some('1'),
            KeyCode::Digit2 => Some('2'),
            KeyCode::Digit3 => Some('3'),
            KeyCode::Digit4 => Some('4'),
            KeyCode::Digit5 => Some('5'),
            KeyCode::Digit6 => Some('6'),
            KeyCode::Digit7 => Some('7'),
            KeyCode::Digit8 => Some('8'),
            KeyCode::Digit9 => Some('9'),
            _ => None,
        };

        if let Some(ch) = character {
            // Check if shift is pressed for lowercase
            if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                builder.name.push(ch);
            } else {
                builder.name.push(ch.to_ascii_lowercase());
            }
        }
    }
}

fn update_name_display(
    builder: Res<CharacterBuilder>,
    mut query: Query<&mut Text, With<NameInputDisplay>>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        // Show cursor blinking effect
        let cursor = if (time.elapsed_seconds() * 2.0) as u32 % 2 == 0 {
            "_"
        } else {
            " "
        };

        // Always show cursor since this is the active input field
        text.sections[0].value = if builder.name.is_empty() {
            format!("Hero{}", cursor)
        } else {
            format!("{}{}", builder.name, cursor)
        };
    }
}

fn update_character_preview(
    builder: Res<CharacterBuilder>,
    mut query: Query<&mut Text, With<ClassDisplay>>,
) {
    if builder.is_changed() {
        for mut text in query.iter_mut() {
            let class_name = match builder.class {
                CharacterClass::Krieger => "Krieger",
                CharacterClass::Ninja => "Ninja",
                CharacterClass::Sura => "Sura",
                CharacterClass::Schamane => "Schamane",
            };
            text.sections[0].value = format!("Gewählte Klasse: {}", class_name);
        }
    }
}

/// Handle CharacterCreated event - automatically select the new character
fn handle_character_created(
    mut char_events: EventReader<CharacterResponseEvent>,
    auth_state: Res<AuthState>,
    network: Option<Res<NetworkClient>>,
) {
    for event in char_events.read() {
        match event {
            CharacterResponseEvent::Created { character_id } => {
                info!("Character created with ID: {} - auto-selecting", character_id);
                
                // Automatically select the newly created character
                if let Some(token) = auth_state.get_token() {
                    if let Some(network) = network.as_ref() {
                        let select_msg = ClientMessage::SelectCharacter {
                            token: token.to_string(),
                            character_id: *character_id,
                        };
                        
                        match network.send_message(&select_msg) {
                            Ok(_) => {
                                info!("Auto-select character request sent");
                                // The CharacterSelected event will be handled by character_selection.rs
                                // which will initialize PlayerStats and transition to InGame
                            }
                            Err(e) => {
                                error!("Failed to auto-select character: {}", e);
                            }
                        }
                    }
                }
            }
            CharacterResponseEvent::CreationFailed { reason } => {
                error!("Character creation failed: {}", reason);
                // TODO: Show error message in UI
            }
            _ => {}
        }
    }
}

fn cleanup_character_creation(
    mut commands: Commands,
    query: Query<Entity, With<CharacterCreationUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
