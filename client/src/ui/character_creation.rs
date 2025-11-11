use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use crate::GameState;
use crate::GameFont;
use crate::networking::{NetworkClient, send_create_character, CharacterResponseEvent};
use crate::auth_state::AuthState;
use shared::{CharacterClass, CharacterData, CharacterAppearance, ClientMessage};
use super::{button_system, NORMAL_BUTTON, CustomColorButton};

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
                animate_particles,
            ).run_if(in_state(GameState::CharacterCreation)));
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
const MEDIEVAL_ROYAL_BLUE: Color = Color::srgb(0.15, 0.25, 0.55);
const MEDIEVAL_PURPLE: Color = Color::srgb(0.45, 0.15, 0.55);

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

#[derive(Component)]
struct FloatingParticle {
    velocity: Vec2,
    spawn_time: f32,
}

fn setup_character_creation(
    mut commands: Commands, 
    mut builder: ResMut<CharacterBuilder>, 
    font: Res<GameFont>,
    time: Res<Time>,
) {
    let font_handle = font.0.clone();
    let current_time = time.elapsed_seconds();
    
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
            background_color: MEDIEVAL_DARK_STONE.into(),
            ..default()
        },
        CharacterCreationUI,
    ))
    .with_children(|parent| {
        // Animated background particles (fewer than login)
        for i in 0..20 {
            let x = (i as f32 * 67.3) % 100.0;
            let y = (i as f32 * 43.7) % 100.0;
            let speed_x = ((i as f32 * 11.3).sin() * 0.25 + 0.15) * if i % 2 == 0 { 1.0 } else { -1.0 };
            let speed_y = ((i as f32 * 7.9).cos() * 0.18 + 0.12);
            
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
                    background_color: MEDIEVAL_GOLD.with_alpha(0.25).into(),
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
            "⚔ ERSCHAFFE DEINEN HELDEN ⚔",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: MEDIEVAL_GOLD,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(25.0)),
            ..default()
        }));

        // Subtitle
        parent.spawn(TextBundle::from_section(
            "Wähle Namen und Klasse für deine Legende",
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

        // Main content frame
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(30.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: MEDIEVAL_DARK_WOOD.with_alpha(0.7).into(),
                border_color: MEDIEVAL_GOLD.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            // Name section
            parent.spawn(TextBundle::from_section(
                "📜 CHARAKTERNAME 📜",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 28.0,
                    color: MEDIEVAL_GOLD,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }));

            // Name input box with medieval styling
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(450.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_PARCHMENT.with_alpha(0.9).into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
                NameInputBox,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Hero|",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 32.0,
                            color: MEDIEVAL_DARK_WOOD,
                            ..default()
                        },
                    ),
                    NameInputDisplay,
                ));
            });

            // Input hint
            parent.spawn(TextBundle::from_section(
                "(Tippe deinen Namen • Rücktaste löscht • Max. 20 Zeichen)",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 18.0,
                    color: MEDIEVAL_SILVER.with_alpha(0.8),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(25.0)),
                ..default()
            }));

            // Divider
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Px(2.0),
                        margin: UiRect::vertical(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_GOLD.with_alpha(0.5).into(),
                    ..default()
                }
            );

            // Class section
            parent.spawn(TextBundle::from_section(
                "⚔ WÄHLE DEINE KLASSE ⚔",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 28.0,
                    color: MEDIEVAL_GOLD,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            }));

            // Class buttons
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(15.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                create_class_button(parent, "⚔\nKrieger", CreationButton::ClassKrieger, font_handle.clone(), MEDIEVAL_BLOOD_RED);
                create_class_button(parent, "🗡\nNinja", CreationButton::ClassNinja, font_handle.clone(), MEDIEVAL_ROYAL_BLUE);
                create_class_button(parent, "🔥\nSura", CreationButton::ClassSura, font_handle.clone(), MEDIEVAL_PURPLE);
                create_class_button(parent, "✨\nSchamane", CreationButton::ClassSchamane, font_handle.clone(), MEDIEVAL_EMERALD);
            });

            // Current class display
            parent.spawn((
                TextBundle::from_section(
                    "Gewählte Klasse: ⚔ Krieger",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 30.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
                ClassDisplay,
            ));
        });

        // Bottom buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(25.0),
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
                        width: Val::Px(250.0),
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
                CreationButton::Back,
                CustomColorButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "← Zurück",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 26.0,
                        color: MEDIEVAL_PARCHMENT,
                        ..default()
                    },
                ));
            });

            // Create button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(350.0),
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
                CreationButton::Create,
                CustomColorButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "⚔ HELD ERSCHAFFEN ⚔",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 26.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
            });
        });
    });
}

fn create_class_button(
    parent: &mut ChildBuilder, 
    label: &str, 
    button_type: CreationButton, 
    font: Handle<Font>,
    color: Color,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(90.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: color.into(),
            border_color: MEDIEVAL_GOLD.into(),
            ..default()
        },
        button_type,
        CustomColorButton,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: MEDIEVAL_GOLD,
                ..default()
            },
        ).with_style(Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }));
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

fn character_creation_buttons(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &CreationButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut builder: ResMut<CharacterBuilder>,
    auth_state: Res<AuthState>,
    network: Option<Res<NetworkClient>>,
) {
    for (interaction, mut bg_color, button) in interaction_query.iter_mut() {
        // Handle hover effects for class buttons
        match button {
            CreationButton::ClassKrieger => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_BLOOD_RED.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_BLOOD_RED.into(),
                    _ => {}
                }
            }
            CreationButton::ClassNinja => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_ROYAL_BLUE.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_ROYAL_BLUE.into(),
                    _ => {}
                }
            }
            CreationButton::ClassSura => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_PURPLE.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_PURPLE.into(),
                    _ => {}
                }
            }
            CreationButton::ClassSchamane => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_EMERALD.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_EMERALD.into(),
                    _ => {}
                }
            }
            CreationButton::Back => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_DARK_WOOD.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_DARK_WOOD.into(),
                    _ => {}
                }
            }
            CreationButton::Create => {
                match *interaction {
                    Interaction::Hovered => *bg_color = MEDIEVAL_BLOOD_RED.with_alpha(0.9).into(),
                    Interaction::None => *bg_color = MEDIEVAL_BLOOD_RED.into(),
                    _ => {}
                }
            }
        }
        
        // Handle clicks
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
        // Show cursor blinking effect (custom cursor only)
        let cursor = if (time.elapsed_seconds() * 2.0) as u32 % 2 == 0 {
            "|"
        } else {
            ""
        };

        // Display name with custom cursor
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
            let (class_name, icon) = match builder.class {
                CharacterClass::Krieger => ("Krieger", "⚔"),
                CharacterClass::Ninja => ("Ninja", "🗡"),
                CharacterClass::Sura => ("Sura", "🔥"),
                CharacterClass::Schamane => ("Schamane", "✨"),
            };
            text.sections[0].value = format!("Gewählte Klasse: {} {}", icon, class_name);
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
