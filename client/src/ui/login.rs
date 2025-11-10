use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use crate::GameState;
use crate::GameFont;
use crate::networking::{NetworkClient, send_auth_request, AuthResponseEvent, ServerConnectionState};
use shared::{AuthMessage, AuthResponse};
use super::{button_system, NORMAL_BUTTON};

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoginState>()
            .add_systems(OnEnter(GameState::Login), setup_login)
            .add_systems(OnExit(GameState::Login), cleanup_login)
            .add_systems(Update, (
                button_system,
                login_buttons,
                handle_login_input,
                update_input_display,
                update_submit_button_text,
                update_status_display,
                update_server_status_display,
                handle_auth_response_ui,
            ).run_if(in_state(GameState::Login)));
    }
}

#[derive(Resource, Default)]
struct LoginState {
    username: String,
    password: String,
    email: String,
    is_register_mode: bool,
    active_field: InputField,
    status_message: String,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum InputField {
    #[default]
    Username,
    Password,
    Email,
}

#[derive(Component)]
struct LoginUI;

#[derive(Component)]
enum LoginButton {
    SwitchMode,
    Submit,
    FocusUsername,
    FocusPassword,
    FocusEmail,
}

#[derive(Component)]
struct SubmitButtonText;

#[derive(Component)]
struct UsernameDisplay;

#[derive(Component)]
struct PasswordDisplay;

#[derive(Component)]
struct EmailDisplay;

#[derive(Component)]
struct StatusDisplay;

#[derive(Component)]
struct ServerStatusDisplay;

#[derive(Component)]
struct RegisterFields;

fn setup_login(mut commands: Commands, mut login_state: ResMut<LoginState>, font: Res<GameFont>) {
    login_state.username.clear();
    login_state.password.clear();
    login_state.email.clear();
    login_state.is_register_mode = false;
    login_state.active_field = InputField::Username;
    login_state.status_message.clear();

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
        LoginUI,
    ))
    .with_children(|parent| {
        // Server Status Banner (at top)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.8, 0.2, 0.2, 0.9).into(), // Red by default
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "⚠ Server nicht erreichbar",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 22.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ServerStatusDisplay,
            ));
        });
        
        // Title
        parent.spawn(TextBundle::from_section(
            "Willkommen",
            TextStyle {
                font: font_handle.clone(),
                font_size: 70.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        }));

        // Username field
        create_input_field(parent, "Benutzername:", "Benutzername eingeben", UsernameDisplay, LoginButton::FocusUsername, font_handle.clone());

        // Password field
        create_input_field(parent, "Passwort:", "Passwort eingeben", PasswordDisplay, LoginButton::FocusPassword, font_handle.clone());

        // Email field (for registration)
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    display: Display::None, // Hidden by default
                    ..default()
                },
                ..default()
            },
            RegisterFields,
        ))
        .with_children(|parent| {
            create_input_field(parent, "E-Mail (optional):", "E-Mail eingeben", EmailDisplay, LoginButton::FocusEmail, font_handle.clone());
        });

        // Status message
        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 20.0,
                    color: Color::srgb(1.0, 0.3, 0.3),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
            StatusDisplay,
        ));

        // Buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Submit button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                    ..default()
                },
                LoginButton::Submit,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Einloggen",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    SubmitButtonText,
                ));
            });

            // Switch mode button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                LoginButton::SwitchMode,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Neuen Account erstellen",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // Instructions
        parent.spawn(TextBundle::from_section(
            "Tab = Feld wechseln | Enter = Bestätigen",
            TextStyle {
                font: font_handle.clone(),
                font_size: 18.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(30.0)),
            ..default()
        }));
    });
}

/// Performs the login/registration submission
fn perform_login_submit(login_state: &mut LoginState, network: Option<&NetworkClient>) {
    // Validate input
    if login_state.username.len() < 3 {
        login_state.status_message = "Benutzername muss mindestens 3 Zeichen lang sein".to_string();
        return;
    }
    if login_state.password.len() < 8 {
        login_state.status_message = "Passwort muss mindestens 8 Zeichen lang sein".to_string();
        return;
    }

    // Send auth request to server
    if let Some(network) = network {
        let auth_msg = if login_state.is_register_mode {
            AuthMessage::Register {
                username: login_state.username.clone(),
                password: login_state.password.clone(),
                email: if login_state.email.is_empty() {
                    None
                } else {
                    Some(login_state.email.clone())
                },
            }
        } else {
            AuthMessage::Login {
                username: login_state.username.clone(),
                password: login_state.password.clone(),
            }
        };

        match send_auth_request(network, auth_msg) {
            Ok(_) => {
                info!("{} request sent for user: {}", 
                    if login_state.is_register_mode { "Register" } else { "Login" },
                    login_state.username
                );
                login_state.status_message = "Verbinde mit Server...".to_string();
            }
            Err(e) => {
                error!("Failed to send auth request: {}", e);
                login_state.status_message = format!("Netzwerkfehler: {}", e);
            }
        }
    } else {
        error!("Network client not initialized");
        login_state.status_message = "Netzwerk nicht verfügbar".to_string();
    }
}

fn create_input_field(
    parent: &mut ChildBuilder,
    label: &str,
    placeholder: &str,
    display_marker: impl Component,
    button_type: LoginButton,
    font: Handle<Font>,
) {
    parent.spawn(TextBundle::from_section(
        label,
        TextStyle {
            font: font.clone(),
            font_size: 25.0,
            color: Color::WHITE,
            ..default()
        },
    ).with_style(Style {
        margin: UiRect::all(Val::Px(10.0)),
        ..default()
    }));

    parent.spawn((
        ButtonBundle {
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
        button_type,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                placeholder,
                TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::srgb(0.5, 0.5, 0.5),
                    ..default()
                },
            ),
            display_marker,
        ));
    });
}

fn handle_login_input(
    mut login_state: ResMut<LoginState>,
    mut evr_kbd: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    network: Option<Res<NetworkClient>>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                KeyCode::Enter => {
                    // Trigger submit when Enter is pressed
                    perform_login_submit(&mut login_state, network.as_deref());
                }
                KeyCode::Tab => {
                    login_state.active_field = match login_state.active_field {
                        InputField::Username => InputField::Password,
                        InputField::Password if login_state.is_register_mode => InputField::Email,
                        InputField::Password => InputField::Username,
                        InputField::Email => InputField::Username,
                    };
                }
                KeyCode::Backspace => {
                    match login_state.active_field {
                        InputField::Username => { login_state.username.pop(); }
                        InputField::Password => { login_state.password.pop(); }
                        InputField::Email => { login_state.email.pop(); }
                    }
                }
                KeyCode::Space => {
                    let text = match login_state.active_field {
                        InputField::Username => &mut login_state.username,
                        InputField::Password => &mut login_state.password,
                        InputField::Email => &mut login_state.email,
                    };
                    if text.len() < 50 {
                        text.push(' ');
                    }
                }
                _ => {}
            }
        }
    }

    // Handle character input
    for key in keys.get_just_pressed() {
        let text = match login_state.active_field {
            InputField::Username => &mut login_state.username,
            InputField::Password => &mut login_state.password,
            InputField::Email => &mut login_state.email,
        };

        if text.len() >= 50 {
            continue;
        }

        let character = match key {
            KeyCode::KeyA => Some('a'), KeyCode::KeyB => Some('b'),
            KeyCode::KeyC => Some('c'), KeyCode::KeyD => Some('d'),
            KeyCode::KeyE => Some('e'), KeyCode::KeyF => Some('f'),
            KeyCode::KeyG => Some('g'), KeyCode::KeyH => Some('h'),
            KeyCode::KeyI => Some('i'), KeyCode::KeyJ => Some('j'),
            KeyCode::KeyK => Some('k'), KeyCode::KeyL => Some('l'),
            KeyCode::KeyM => Some('m'), KeyCode::KeyN => Some('n'),
            KeyCode::KeyO => Some('o'), KeyCode::KeyP => Some('p'),
            KeyCode::KeyQ => Some('q'), KeyCode::KeyR => Some('r'),
            KeyCode::KeyS => Some('s'), KeyCode::KeyT => Some('t'),
            KeyCode::KeyU => Some('u'), KeyCode::KeyV => Some('v'),
            KeyCode::KeyW => Some('w'), KeyCode::KeyX => Some('x'),
            KeyCode::KeyY => Some('y'), KeyCode::KeyZ => Some('z'),
            KeyCode::Digit0 => Some('0'), KeyCode::Digit1 => Some('1'),
            KeyCode::Digit2 => Some('2'), KeyCode::Digit3 => Some('3'),
            KeyCode::Digit4 => Some('4'), KeyCode::Digit5 => Some('5'),
            KeyCode::Digit6 => Some('6'), KeyCode::Digit7 => Some('7'),
            KeyCode::Digit8 => Some('8'), KeyCode::Digit9 => Some('9'),
            KeyCode::Minus => Some('-'),
            KeyCode::Period => Some('.'),
            KeyCode::NumpadDecimal => Some('.'),
            _ => None,
        };

        if let Some(ch) = character {
            if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                text.push(ch.to_ascii_uppercase());
            } else {
                text.push(ch);
            }
        }
    }
}

fn update_input_display(
    login_state: Res<LoginState>,
    mut username_query: Query<&mut Text, (With<UsernameDisplay>, Without<PasswordDisplay>, Without<EmailDisplay>)>,
    mut password_query: Query<&mut Text, (With<PasswordDisplay>, Without<UsernameDisplay>, Without<EmailDisplay>)>,
    mut email_query: Query<&mut Text, (With<EmailDisplay>, Without<UsernameDisplay>, Without<PasswordDisplay>)>,
    time: Res<Time>,
) {
    let cursor = if (time.elapsed_seconds() * 2.0) as u32 % 2 == 0 { "_" } else { " " };

    for mut text in username_query.iter_mut() {
        let is_active = login_state.active_field == InputField::Username;
        let is_empty = login_state.username.is_empty();
        
        text.sections[0].value = if is_empty {
            if is_active {
                format!("Benutzername eingeben{}", cursor)
            } else {
                "Benutzername eingeben".to_string()
            }
        } else {
            if is_active {
                format!("{}{}", login_state.username, cursor)
            } else {
                login_state.username.clone()
            }
        };
        
        text.sections[0].style.color = if is_empty {
            Color::srgb(0.5, 0.5, 0.5)
        } else {
            Color::srgb(1.0, 1.0, 0.4)
        };
    }

    for mut text in password_query.iter_mut() {
        let is_active = login_state.active_field == InputField::Password;
        let is_empty = login_state.password.is_empty();
        let masked = "*".repeat(login_state.password.len());
        
        text.sections[0].value = if is_empty {
            if is_active {
                format!("Passwort eingeben{}", cursor)
            } else {
                "Passwort eingeben".to_string()
            }
        } else {
            if is_active {
                format!("{}{}", masked, cursor)
            } else {
                masked
            }
        };
        
        text.sections[0].style.color = if is_empty {
            Color::srgb(0.5, 0.5, 0.5)
        } else {
            Color::srgb(1.0, 1.0, 0.4)
        };
    }

    for mut text in email_query.iter_mut() {
        let is_active = login_state.active_field == InputField::Email;
        let is_empty = login_state.email.is_empty();
        
        text.sections[0].value = if is_empty {
            if is_active {
                format!("E-Mail eingeben{}", cursor)
            } else {
                "E-Mail eingeben".to_string()
            }
        } else {
            if is_active {
                format!("{}{}", login_state.email, cursor)
            } else {
                login_state.email.clone()
            }
        };
        
        text.sections[0].style.color = if is_empty {
            Color::srgb(0.5, 0.5, 0.5)
        } else {
            Color::srgb(1.0, 1.0, 0.4)
        };
    }
}

fn login_buttons(
    mut interaction_query: Query<(&Interaction, &LoginButton, &Children), Changed<Interaction>>,
    mut login_state: ResMut<LoginState>,
    mut register_fields_query: Query<&mut Style, With<RegisterFields>>,
    mut text_query: Query<&mut Text>,
    network: Option<Res<NetworkClient>>,
) {
    for (interaction, button, children) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                LoginButton::SwitchMode => {
                    login_state.is_register_mode = !login_state.is_register_mode;
                    login_state.status_message.clear();
                    
                    // Toggle register fields visibility
                    for mut style in register_fields_query.iter_mut() {
                        style.display = if login_state.is_register_mode {
                            Display::Flex
                        } else {
                            Display::None
                        };
                    }
                    
                    // Update switch mode button text
                    for &child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.sections[0].value = if login_state.is_register_mode {
                                "Zurück zum Login".to_string()
                            } else {
                                "Neuen Account erstellen".to_string()
                            };
                        }
                    }
                }
                LoginButton::Submit => {
                    perform_login_submit(&mut login_state, network.as_deref());
                }
                LoginButton::FocusUsername => {
                    login_state.active_field = InputField::Username;
                }
                LoginButton::FocusPassword => {
                    login_state.active_field = InputField::Password;
                }
                LoginButton::FocusEmail => {
                    login_state.active_field = InputField::Email;
                }
            }
        }
    }
}

fn update_submit_button_text(
    login_state: Res<LoginState>,
    mut query: Query<&mut Text, With<SubmitButtonText>>,
) {
    if login_state.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = if login_state.is_register_mode {
                "Registrieren".to_string()
            } else {
                "Einloggen".to_string()
            };
        }
    }
}

fn update_status_display(
    login_state: Res<LoginState>,
    mut query: Query<&mut Text, With<StatusDisplay>>,
) {
    if login_state.is_changed() && !login_state.status_message.is_empty() {
        for mut text in query.iter_mut() {
            text.sections[0].value = login_state.status_message.clone();
            // Set color based on message content
            if login_state.status_message.contains("erfolgreich") || login_state.status_message.contains("Verbinde") {
                text.sections[0].style.color = Color::srgb(0.3, 1.0, 0.3);
            } else if login_state.status_message.contains("fehlgeschlagen") || login_state.status_message.contains("fehler") || login_state.status_message.contains("muss") {
                text.sections[0].style.color = Color::srgb(1.0, 0.3, 0.3);
            } else {
                text.sections[0].style.color = Color::srgb(0.7, 0.7, 0.7);
            }
        }
    }
}

fn update_server_status_display(
    connection_state: Res<ServerConnectionState>,
    mut text_query: Query<(&mut Text, &Parent), With<ServerStatusDisplay>>,
    mut banner_query: Query<&mut BackgroundColor>,
) {
    if connection_state.is_changed() {
        for (mut text, parent) in text_query.iter_mut() {
            // Update text based on connection status
            if connection_state.is_connected {
                text.sections[0].value = "✓ Server verbunden".to_string();
            } else {
                text.sections[0].value = "⚠ Server nicht erreichbar".to_string();
            }
            
            // Update banner color
            if let Ok(mut bg_color) = banner_query.get_mut(parent.get()) {
                *bg_color = if connection_state.is_connected {
                    Color::srgba(0.2, 0.7, 0.2, 0.9).into() // Green
                } else {
                    Color::srgba(0.8, 0.2, 0.2, 0.9).into() // Red
                };
            }
        }
    }
}

fn handle_auth_response_ui(
    mut auth_events: EventReader<AuthResponseEvent>,
    mut login_state: ResMut<LoginState>,
    mut register_fields_query: Query<&mut Style, With<RegisterFields>>,
) {
    for event in auth_events.read() {
        match &event.0 {
            AuthResponse::LoginSuccess { .. } => {
                // Success handled by networking module
            }
            AuthResponse::LoginFailed { reason } => {
                login_state.status_message = format!("Login fehlgeschlagen: {}", reason);
            }
            AuthResponse::RegisterSuccess => {
                login_state.status_message = "✓ Registrierung erfolgreich! Du kannst dich jetzt einloggen.".to_string();
                login_state.is_register_mode = false;
                login_state.password.clear(); // Clear password for security
                login_state.email.clear();
                
                // Hide register fields
                for mut style in register_fields_query.iter_mut() {
                    style.display = Display::None;
                }
            }
            AuthResponse::RegisterFailed { reason } => {
                login_state.status_message = format!("Registrierung fehlgeschlagen: {}", reason);
            }
        }
    }
}

fn cleanup_login(
    mut commands: Commands,
    query: Query<Entity, With<LoginUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
