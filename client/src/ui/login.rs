use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use crate::GameState;
use crate::GameFont;
use crate::networking::{NetworkClient, send_auth_request, AuthResponseEvent};
use shared::{AuthMessage, AuthResponse};

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoginState>()
            .add_systems(OnEnter(GameState::Login), setup_login)
            .add_systems(OnExit(GameState::Login), cleanup_login)
            .add_systems(Update, (
                login_buttons,
                handle_login_input,
                update_input_display,
                update_submit_button_text,
                update_status_display,
                handle_auth_response_ui,
                animate_background_particles,
                update_input_field_borders,
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
    QuitGame,
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
struct RegisterFields;

#[derive(Component)]
struct UsernameFieldBorder;

#[derive(Component)]
struct PasswordFieldBorder;

#[derive(Component)]
struct EmailFieldBorder;

#[derive(Component)]
struct BackgroundParticle {
    velocity: Vec2,
    spawn_time: f32,
}

// Medieval color palette
const MEDIEVAL_GOLD: Color = Color::srgb(0.85, 0.65, 0.13);
const MEDIEVAL_DARK_WOOD: Color = Color::srgb(0.15, 0.10, 0.08);
const MEDIEVAL_LIGHT_WOOD: Color = Color::srgb(0.25, 0.18, 0.12);
const MEDIEVAL_PARCHMENT: Color = Color::srgb(0.92, 0.87, 0.75);
const MEDIEVAL_INK: Color = Color::srgb(0.1, 0.08, 0.05);
const MEDIEVAL_RED: Color = Color::srgb(0.65, 0.15, 0.15);
const MEDIEVAL_BORDER_GOLD: Color = Color::srgb(0.75, 0.60, 0.15);
const MEDIEVAL_ACTIVE_GOLD: Color = Color::srgb(1.0, 0.80, 0.20);

fn setup_login(mut commands: Commands, mut login_state: ResMut<LoginState>, font: Res<GameFont>, time: Res<Time>) {
    login_state.username.clear();
    login_state.password.clear();
    login_state.email.clear();
    login_state.is_register_mode = false;
    login_state.active_field = InputField::Username;
    login_state.status_message.clear();

    let font_handle = font.0.clone();

    // Main container with animated background
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: Color::srgb(0.08, 0.06, 0.05).into(), // Dark medieval background
            ..default()
        },
        LoginUI,
    ))
    .with_children(|parent| {
        // Spawn background particles for atmosphere
        spawn_background_particles(parent, time.elapsed_seconds());
        
        // Main login panel (medieval wood panel)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(480.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: MEDIEVAL_DARK_WOOD.into(),
            border_color: MEDIEVAL_BORDER_GOLD.into(),
            ..default()
        })
        .with_children(|parent| {
            // Title with medieval shield style
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(35.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: MEDIEVAL_LIGHT_WOOD.into(),
                border_color: MEDIEVAL_GOLD.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "⚔ REALM OF LEGENDS ⚔",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 48.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
                
                parent.spawn(TextBundle::from_section(
                    "Enter the Medieval World",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        color: MEDIEVAL_PARCHMENT,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                }));
            });

            // Username field
            create_medieval_input_field(
                parent,
                "Username",
                UsernameDisplay,
                LoginButton::FocusUsername,
                UsernameFieldBorder,
                font_handle.clone(),
            );

            // Password field
            create_medieval_input_field(
                parent,
                "Password",
                PasswordDisplay,
                LoginButton::FocusPassword,
                PasswordFieldBorder,
                font_handle.clone(),
            );

            // Email field (for registration)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        display: Display::None, // Hidden by default
                        ..default()
                    },
                    ..default()
                },
                RegisterFields,
            ))
            .with_children(|parent| {
                create_medieval_input_field(
                    parent,
                    "Email",
                    EmailDisplay,
                    LoginButton::FocusEmail,
                    EmailFieldBorder,
                    font_handle.clone(),
                );
            });

            // Submit button (medieval style)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(25.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_RED.into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
                LoginButton::Submit,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "⚔ ENTER REALM ⚔",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 26.0,
                            color: MEDIEVAL_GOLD,
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
                        width: Val::Percent(100.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_LIGHT_WOOD.into(),
                    border_color: MEDIEVAL_BORDER_GOLD.into(),
                    ..default()
                },
                LoginButton::SwitchMode,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Create New Account",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 18.0,
                        color: MEDIEVAL_PARCHMENT,
                        ..default()
                    },
                ));
            });

            // Quit Game button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: MEDIEVAL_RED.into(),
                    border_color: MEDIEVAL_GOLD.into(),
                    ..default()
                },
                LoginButton::QuitGame,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Spiel beenden",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 18.0,
                        color: MEDIEVAL_GOLD,
                        ..default()
                    },
                ));
            });

            // Status message
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        color: MEDIEVAL_RED,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                }),
                StatusDisplay,
            ));
        });
    });
}

fn create_medieval_input_field(
    parent: &mut ChildBuilder,
    label: &str,
    display_component: impl Component,
    button_component: LoginButton,
    border_component: impl Component,
    font: Handle<Font>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Label
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: MEDIEVAL_GOLD,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        }));

        // Input field container (clickable)
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: MEDIEVAL_PARCHMENT.into(),
                border_color: MEDIEVAL_BORDER_GOLD.into(),
                ..default()
            },
            button_component,
            border_component,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: MEDIEVAL_INK,
                        ..default()
                    },
                ),
                display_component,
            ));
        });
    });
}

fn spawn_background_particles(parent: &mut ChildBuilder, current_time: f32) {
    // Spawn 30 floating particles for atmosphere (deterministic)
    for i in 0..30 {
        let x = (i as f32 * 73.5) % 100.0;
        let y = (i as f32 * 47.3) % 100.0;
        let speed_x = ((i as f32 * 12.7).sin() * 0.3 + 0.2) * if i % 2 == 0 { 1.0 } else { -1.0 };
        let speed_y = ((i as f32 * 8.3).cos() * 0.2 + 0.15);
        let size = 3.0 + ((i as f32 * 5.7).sin() * 1.5);
        let opacity = 0.2 + ((i as f32 * 7.1).cos() * 0.15);
        
        parent.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x),
                    top: Val::Percent(y),
                    width: Val::Px(size),
                    height: Val::Px(size),
                    ..default()
                },
                background_color: MEDIEVAL_GOLD.with_alpha(opacity).into(),
                ..default()
            },
            BackgroundParticle {
                velocity: Vec2::new(speed_x, speed_y),
                spawn_time: current_time,
            },
        ));
    }
}

fn animate_background_particles(
    time: Res<Time>,
    mut particle_query: Query<(&mut Style, &BackgroundParticle)>,
) {
    for (mut style, particle) in particle_query.iter_mut() {
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

fn update_input_field_borders(
    login_state: Res<LoginState>,
    mut username_query: Query<&mut BorderColor, (With<UsernameFieldBorder>, Without<PasswordFieldBorder>, Without<EmailFieldBorder>)>,
    mut password_query: Query<&mut BorderColor, (With<PasswordFieldBorder>, Without<UsernameFieldBorder>, Without<EmailFieldBorder>)>,
    mut email_query: Query<&mut BorderColor, (With<EmailFieldBorder>, Without<UsernameFieldBorder>, Without<PasswordFieldBorder>)>,
) {
    if !login_state.is_changed() {
        return;
    }
    
    // Update username border
    if let Ok(mut border) = username_query.get_single_mut() {
        *border = if login_state.active_field == InputField::Username {
            MEDIEVAL_ACTIVE_GOLD.into()
        } else {
            MEDIEVAL_BORDER_GOLD.into()
        };
    }
    
    // Update password border
    if let Ok(mut border) = password_query.get_single_mut() {
        *border = if login_state.active_field == InputField::Password {
            MEDIEVAL_ACTIVE_GOLD.into()
        } else {
            MEDIEVAL_BORDER_GOLD.into()
        };
    }
    
    // Update email border
    if let Ok(mut border) = email_query.get_single_mut() {
        *border = if login_state.active_field == InputField::Email {
            MEDIEVAL_ACTIVE_GOLD.into()
        } else {
            MEDIEVAL_BORDER_GOLD.into()
        };
    }
}

fn login_buttons(
    mut interaction_query: Query<(&Interaction, &LoginButton, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    mut login_state: ResMut<LoginState>,
    network: Option<Res<NetworkClient>>,
    mut register_fields: Query<&mut Style, With<RegisterFields>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, mut bg_color, mut border_color) in interaction_query.iter_mut() {
        // Handle hover effects for medieval buttons
        match *interaction {
            Interaction::Pressed => {
                match button {
                    LoginButton::Submit => {
                        *bg_color = Color::srgb(0.75, 0.20, 0.20).into(); // Brighter red
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                        
                        if let Some(network) = network.as_ref() {
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

                            if let Err(e) = send_auth_request(network, auth_msg) {
                                login_state.status_message = format!("Network error: {}", e);
                            } else {
                                login_state.status_message = "Connecting...".to_string();
                            }
                        }
                    }
                    LoginButton::SwitchMode => {
                        *bg_color = Color::srgb(0.30, 0.22, 0.16).into(); // Lighter wood
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                        
                        login_state.is_register_mode = !login_state.is_register_mode;
                        login_state.status_message.clear();
                        
                        // Toggle email field visibility
                        for mut style in register_fields.iter_mut() {
                            style.display = if login_state.is_register_mode {
                                Display::Flex
                            } else {
                                Display::None
                            };
                        }
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
                    LoginButton::QuitGame => {
                        *bg_color = Color::srgb(0.75, 0.15, 0.15).into(); // Brighter red
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                        exit.send(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                match button {
                    LoginButton::Submit => {
                        *bg_color = Color::srgb(0.70, 0.18, 0.18).into(); // Slightly brighter red
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                    }
                    LoginButton::SwitchMode => {
                        *bg_color = Color::srgb(0.28, 0.20, 0.14).into(); // Slightly lighter wood
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                    }
                    LoginButton::QuitGame => {
                        *bg_color = Color::srgb(0.70, 0.13, 0.13).into(); // Slightly brighter red
                        *border_color = MEDIEVAL_ACTIVE_GOLD.into();
                    }
                    _ => {}
                }
            }
            Interaction::None => {
                match button {
                    LoginButton::Submit => {
                        *bg_color = MEDIEVAL_RED.into();
                        *border_color = MEDIEVAL_GOLD.into();
                    }
                    LoginButton::SwitchMode => {
                        *bg_color = MEDIEVAL_LIGHT_WOOD.into();
                        *border_color = MEDIEVAL_BORDER_GOLD.into();
                    }
                    LoginButton::QuitGame => {
                        *bg_color = MEDIEVAL_RED.into();
                        *border_color = MEDIEVAL_GOLD.into();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_login_input(
    mut login_state: ResMut<LoginState>,
    mut char_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for event in char_events.read() {
        if event.state == ButtonState::Pressed {
            match event.key_code {
                KeyCode::Backspace => {
                    match login_state.active_field {
                        InputField::Username => { login_state.username.pop(); }
                        InputField::Password => { login_state.password.pop(); }
                        InputField::Email => { login_state.email.pop(); }
                    }
                }
                KeyCode::Tab => {
                    login_state.active_field = match login_state.active_field {
                        InputField::Username => InputField::Password,
                        InputField::Password => {
                            if login_state.is_register_mode {
                                InputField::Email
                            } else {
                                InputField::Username
                            }
                        }
                        InputField::Email => InputField::Username,
                    };
                }
                KeyCode::Enter => {
                    // Submit will be handled by button
                }
                _ => {
                    if let Some(text) = get_text_from_key(event.key_code, keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)) {
                        match login_state.active_field {
                            InputField::Username => {
                                if login_state.username.len() < 20 {
                                    login_state.username.push_str(&text);
                                }
                            }
                            InputField::Password => {
                                if login_state.password.len() < 30 {
                                    login_state.password.push_str(&text);
                                }
                            }
                            InputField::Email => {
                                if login_state.email.len() < 50 {
                                    login_state.email.push_str(&text);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_text_from_key(key: KeyCode, shift: bool) -> Option<String> {
    match key {
        KeyCode::KeyA => Some(if shift { "A" } else { "a" }.to_string()),
        KeyCode::KeyB => Some(if shift { "B" } else { "b" }.to_string()),
        KeyCode::KeyC => Some(if shift { "C" } else { "c" }.to_string()),
        KeyCode::KeyD => Some(if shift { "D" } else { "d" }.to_string()),
        KeyCode::KeyE => Some(if shift { "E" } else { "e" }.to_string()),
        KeyCode::KeyF => Some(if shift { "F" } else { "f" }.to_string()),
        KeyCode::KeyG => Some(if shift { "G" } else { "g" }.to_string()),
        KeyCode::KeyH => Some(if shift { "H" } else { "h" }.to_string()),
        KeyCode::KeyI => Some(if shift { "I" } else { "i" }.to_string()),
        KeyCode::KeyJ => Some(if shift { "J" } else { "j" }.to_string()),
        KeyCode::KeyK => Some(if shift { "K" } else { "k" }.to_string()),
        KeyCode::KeyL => Some(if shift { "L" } else { "l" }.to_string()),
        KeyCode::KeyM => Some(if shift { "M" } else { "m" }.to_string()),
        KeyCode::KeyN => Some(if shift { "N" } else { "n" }.to_string()),
        KeyCode::KeyO => Some(if shift { "O" } else { "o" }.to_string()),
        KeyCode::KeyP => Some(if shift { "P" } else { "p" }.to_string()),
        KeyCode::KeyQ => Some(if shift { "Q" } else { "q" }.to_string()),
        KeyCode::KeyR => Some(if shift { "R" } else { "r" }.to_string()),
        KeyCode::KeyS => Some(if shift { "S" } else { "s" }.to_string()),
        KeyCode::KeyT => Some(if shift { "T" } else { "t" }.to_string()),
        KeyCode::KeyU => Some(if shift { "U" } else { "u" }.to_string()),
        KeyCode::KeyV => Some(if shift { "V" } else { "v" }.to_string()),
        KeyCode::KeyW => Some(if shift { "W" } else { "w" }.to_string()),
        KeyCode::KeyX => Some(if shift { "X" } else { "x" }.to_string()),
        KeyCode::KeyY => Some(if shift { "Y" } else { "y" }.to_string()),
        KeyCode::KeyZ => Some(if shift { "Z" } else { "z" }.to_string()),
        KeyCode::Digit0 => Some(if shift { ")" } else { "0" }.to_string()),
        KeyCode::Digit1 => Some(if shift { "!" } else { "1" }.to_string()),
        KeyCode::Digit2 => Some(if shift { "@" } else { "2" }.to_string()),
        KeyCode::Digit3 => Some(if shift { "#" } else { "3" }.to_string()),
        KeyCode::Digit4 => Some(if shift { "$" } else { "4" }.to_string()),
        KeyCode::Digit5 => Some(if shift { "%" } else { "5" }.to_string()),
        KeyCode::Digit6 => Some(if shift { "^" } else { "6" }.to_string()),
        KeyCode::Digit7 => Some(if shift { "&" } else { "7" }.to_string()),
        KeyCode::Digit8 => Some(if shift { "*" } else { "8" }.to_string()),
        KeyCode::Digit9 => Some(if shift { "(" } else { "9" }.to_string()),
        KeyCode::Period => Some(if shift { ">" } else { "." }.to_string()),
        KeyCode::Minus => Some(if shift { "_" } else { "-" }.to_string()),
        KeyCode::Equal => Some(if shift { "+" } else { "=" }.to_string()),
        KeyCode::Space => Some(" ".to_string()),
        KeyCode::Semicolon => Some(if shift { ":" } else { ";" }.to_string()),
        KeyCode::Comma => Some(if shift { "<" } else { "," }.to_string()),
        KeyCode::Slash => Some(if shift { "?" } else { "/" }.to_string()),
        KeyCode::Backslash => Some(if shift { "|" } else { "\\" }.to_string()),
        KeyCode::BracketLeft => Some(if shift { "{" } else { "[" }.to_string()),
        KeyCode::BracketRight => Some(if shift { "}" } else { "]" }.to_string()),
        KeyCode::Quote => Some(if shift { "\"" } else { "'" }.to_string()),
        KeyCode::Backquote => Some(if shift { "~" } else { "`" }.to_string()),
        _ => None,
    }
}

fn update_input_display(
    login_state: Res<LoginState>,
    mut username_query: Query<&mut Text, (With<UsernameDisplay>, Without<PasswordDisplay>, Without<EmailDisplay>)>,
    mut password_query: Query<&mut Text, (With<PasswordDisplay>, Without<UsernameDisplay>, Without<EmailDisplay>)>,
    mut email_query: Query<&mut Text, (With<EmailDisplay>, Without<UsernameDisplay>, Without<PasswordDisplay>)>,
) {
    if !login_state.is_changed() {
        return;
    }

    // Username display
    if let Ok(mut text) = username_query.get_single_mut() {
        let display = if login_state.username.is_empty() {
            if login_state.active_field == InputField::Username {
                "|".to_string() // Just cursor when active and empty
            } else {
                "".to_string() // Nothing when inactive and empty
            }
        } else {
            if login_state.active_field == InputField::Username {
                format!("{}|", login_state.username)
            } else {
                login_state.username.clone()
            }
        };
        text.sections[0].value = display;
    }

    // Password display (masked)
    if let Ok(mut text) = password_query.get_single_mut() {
        let display = if login_state.password.is_empty() {
            if login_state.active_field == InputField::Password {
                "|".to_string()
            } else {
                "".to_string()
            }
        } else {
            let masked = "●".repeat(login_state.password.len());
            if login_state.active_field == InputField::Password {
                format!("{}|", masked)
            } else {
                masked
            }
        };
        text.sections[0].value = display;
    }

    // Email display
    if let Ok(mut text) = email_query.get_single_mut() {
        let display = if login_state.email.is_empty() {
            if login_state.active_field == InputField::Email {
                "|".to_string()
            } else {
                "".to_string()
            }
        } else {
            if login_state.active_field == InputField::Email {
                format!("{}|", login_state.email)
            } else {
                login_state.email.clone()
            }
        };
        text.sections[0].value = display;
    }
}

fn update_submit_button_text(
    login_state: Res<LoginState>,
    mut text_query: Query<&mut Text, With<SubmitButtonText>>,
) {
    if !login_state.is_changed() {
        return;
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = if login_state.is_register_mode {
            "⚔ CREATE ACCOUNT ⚔".to_string()
        } else {
            "⚔ ENTER REALM ⚔".to_string()
        };
    }
}

fn update_status_display(
    login_state: Res<LoginState>,
    mut text_query: Query<&mut Text, With<StatusDisplay>>,
) {
    if !login_state.is_changed() {
        return;
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = login_state.status_message.clone();
    }
}

fn handle_auth_response_ui(
    mut auth_events: EventReader<AuthResponseEvent>,
    mut login_state: ResMut<LoginState>,
) {
    for event in auth_events.read() {
        match &event.0 {
            AuthResponse::LoginSuccess { .. } => {
                login_state.status_message = "Success! Loading world...".to_string();
            }
            AuthResponse::LoginFailed { reason } => {
                login_state.status_message = format!("Login failed: {}", reason);
            }
            AuthResponse::RegisterSuccess => {
                login_state.status_message = "Account created! Please login.".to_string();
                login_state.is_register_mode = false;
            }
            AuthResponse::RegisterFailed { reason } => {
                login_state.status_message = format!("Registration failed: {}", reason);
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
