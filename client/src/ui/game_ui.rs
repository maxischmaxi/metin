use bevy::prelude::*;
use crate::GameState;
use crate::GameFont;
use super::{button_system, UILayerStack, UILayerType};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>()
            .init_resource::<DevModeState>()
            .add_systems(OnEnter(GameState::InGame), (setup_game_ui, setup_dev_panel))
            .add_systems(OnExit(GameState::InGame), cleanup_game_ui)
            .add_systems(Update, (
                update_instructions,
                update_stat_bars,
                update_xp_bar,
                handle_bottom_bar_buttons,
                handle_dev_xp_key,
                handle_dev_toggle_key,
                handle_dev_panel_buttons,
                update_dev_panel_visibility,
                update_dev_panel_text,
                button_system,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource)]
pub struct PlayerStats {
    pub character_name: String,
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub level: i32,
    pub experience: i64,
    pub xp_needed: i64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            character_name: "Unknown".to_string(),
            health: 100.0,
            max_health: 100.0,
            mana: 100.0,
            max_mana: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            level: 1,
            experience: 0,
            xp_needed: shared::calculate_xp_for_level(2),
        }
    }
}

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct ManaBar;

#[derive(Component)]
struct StaminaBar;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ManaText;

#[derive(Component)]
struct StaminaText;

#[derive(Component)]
struct XpBar;

#[derive(Component)]
struct LevelText;

#[derive(Component)]
enum BottomBarButton {
    Map,
    Inventory,
    Menu,
}

#[derive(Component)]
struct AbilitySlot(u8); // 1-9

/// Resource to track dev mode state
#[derive(Resource)]
pub struct DevModeState {
    pub enabled: bool,
}

impl Default for DevModeState {
    fn default() -> Self {
        Self { enabled: true } // Dev mode ON by default for testing
    }
}

#[derive(Component)]
struct DevPanel;

#[derive(Component)]
struct DevLevelText;

#[derive(Component)]
enum DevButton {
    AddLevel,
    RemoveLevel,
    Add1000XP,
    ResetLevel,
}

fn setup_game_ui(mut commands: Commands, font: Res<GameFont>, mut ui_stack: ResMut<UILayerStack>) {
    // Register base game UI layer
    ui_stack.push_layer(UILayerType::GameUI);
    
    let font_handle = font.0.clone();
    
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            // Make UI transparent to mouse clicks (allows clicking through to 3D world)
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..default()
        },
        GameUI,
    ))
    .with_children(|parent| {
        // Bottom Bar Container - ABSOLUTE POSITIONING (KOMPAKT)
        parent.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            // Bottom bar should block clicks (it has buttons)
            focus_policy: bevy::ui::FocusPolicy::Block,
            ..default()
        })
        .with_children(|parent| {
            // Container for XP Circle + Stats
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                focus_policy: bevy::ui::FocusPolicy::Pass,
                ..default()
            })
            .with_children(|parent| {
                // XP Circle (LEFT)
                create_xp_circle(parent, font_handle.clone());
                
                // Stats Panel (RIGHT of circle)
                create_stats_panel(parent, font_handle.clone());
            });

            // MIDDLE - Ability Slots (1-9)
            create_ability_slots(parent, font_handle.clone());

            // RIGHT SIDE - Menu Buttons
            create_menu_buttons(parent, font_handle.clone());
        });
    });
}

fn create_xp_circle(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        // XP Circle Container
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..default()
        })
        .with_children(|parent| {
            // Background circle (dark)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                border_color: Color::srgb(0.4, 0.4, 0.5).into(),
                ..default()
            });
            
            // XP Fill circle (grows with percentage)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(0.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::srgb(0.9, 0.7, 0.2).into(),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                XpBar,
            ));
            
            // Level text in center
            parent.spawn((
                TextBundle::from_section(
                    "1",
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: Color::srgb(1.0, 0.9, 0.3),
                        ..default()
                    },
                ).with_style(Style {
                    position_type: PositionType::Absolute,
                    ..default()
                }),
                LevelText,
            ));
        });
        
        // "XP" label below circle
        parent.spawn(TextBundle::from_section(
            "XP",
            TextStyle {
                font: font.clone(),
                font_size: 11.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(2.0)),
            ..default()
        }));
    });
}

fn create_stats_panel(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(240.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(2.0),
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        // Health Bar
        create_stat_bar(
            parent,
            "HP",
            Color::srgb(0.8, 0.2, 0.2),
            HealthBar,
            HealthText,
            font.clone(),
        );

        // Mana Bar
        create_stat_bar(
            parent,
            "MP",
            Color::srgb(0.2, 0.4, 0.9),
            ManaBar,
            ManaText,
            font.clone(),
        );

        // Stamina Bar
        create_stat_bar(
            parent,
            "ST",
            Color::srgb(0.3, 0.8, 0.3),
            StaminaBar,
            StaminaText,
            font.clone(),
        );
    });
}

fn create_stat_bar(
    parent: &mut ChildBuilder,
    label: &str,
    color: Color,
    bar_marker: impl Component,
    text_marker: impl Component,
    font: Handle<Font>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        // Label (compact)
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 12.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ).with_style(Style {
            width: Val::Px(20.0),
            ..default()
        }));

        // Bar background
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(14.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
            border_color: Color::srgb(0.4, 0.4, 0.4).into(),
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..default()
        })
        .with_children(|parent| {
            // Bar fill
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                },
                bar_marker,
            ));
        });

        // Value text (compact)
        parent.spawn((
            TextBundle::from_section(
                "100/100",
                TextStyle {
                    font: font.clone(),
                    font_size: 11.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            text_marker,
        ));
    });
}

fn create_ability_slots(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            align_items: AlignItems::Center,
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        for i in 1..=9 {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(42.0),
                        height: Val::Px(42.0),
                        border: UiRect::all(Val::Px(1.5)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.25, 0.9).into(),
                    border_color: Color::srgb(0.5, 0.5, 0.6).into(),
                    focus_policy: bevy::ui::FocusPolicy::Pass,
                    ..default()
                },
                AbilitySlot(i),
            ))
            .with_children(|parent| {
                // Slot number
                parent.spawn(TextBundle::from_section(
                    i.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                        ..default()
                    },
                ));
            });
        }
    });
}

fn create_menu_buttons(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        // Map Button
        create_menu_button(parent, "Map", BottomBarButton::Map, font.clone());

        // Inventory Button
        create_menu_button(parent, "Inv", BottomBarButton::Inventory, font.clone());

        // Menu Button
        create_menu_button(parent, "Menu", BottomBarButton::Menu, font.clone());
    });
}

fn create_menu_button(
    parent: &mut ChildBuilder,
    label: &str,
    button_type: BottomBarButton,
    font: Handle<Font>,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(42.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.5)),
                ..default()
            },
            background_color: Color::srgb(0.25, 0.25, 0.3).into(),
            border_color: Color::srgb(0.5, 0.5, 0.6).into(),
            ..default()
        },
        button_type,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn update_stat_bars(
    stats: Res<PlayerStats>,
    mut health_bar_query: Query<&mut Style, (With<HealthBar>, Without<ManaBar>, Without<StaminaBar>)>,
    mut mana_bar_query: Query<&mut Style, (With<ManaBar>, Without<HealthBar>, Without<StaminaBar>)>,
    mut stamina_bar_query: Query<&mut Style, (With<StaminaBar>, Without<HealthBar>, Without<ManaBar>)>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<ManaText>, Without<StaminaText>)>,
    mut mana_text_query: Query<&mut Text, (With<ManaText>, Without<HealthText>, Without<StaminaText>)>,
    mut stamina_text_query: Query<&mut Text, (With<StaminaText>, Without<HealthText>, Without<ManaText>)>,
) {
    // Update health bar
    if let Ok(mut style) = health_bar_query.get_single_mut() {
        let percentage = (stats.health / stats.max_health * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(percentage);
    }
    if let Ok(mut text) = health_text_query.get_single_mut() {
        text.sections[0].value = format!("{:.0}/{:.0}", stats.health, stats.max_health);
    }

    // Update mana bar
    if let Ok(mut style) = mana_bar_query.get_single_mut() {
        let percentage = (stats.mana / stats.max_mana * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(percentage);
    }
    if let Ok(mut text) = mana_text_query.get_single_mut() {
        text.sections[0].value = format!("{:.0}/{:.0}", stats.mana, stats.max_mana);
    }

    // Update stamina bar
    if let Ok(mut style) = stamina_bar_query.get_single_mut() {
        let percentage = (stats.stamina / stats.max_stamina * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(percentage);
    }
    if let Ok(mut text) = stamina_text_query.get_single_mut() {
        text.sections[0].value = format!("{:.0}/{:.0}", stats.stamina, stats.max_stamina);
    }
}

fn handle_bottom_bar_buttons(
    interaction_query: Query<(&Interaction, &BottomBarButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                BottomBarButton::Map => {
                    info!("Map button clicked (not yet implemented)");
                    // TODO: Open map
                }
                BottomBarButton::Inventory => {
                    info!("Inventory button clicked (not yet implemented)");
                    // TODO: Open inventory
                }
                BottomBarButton::Menu => {
                    info!("Menu button clicked - opening pause menu");
                    next_state.set(GameState::Paused);
                }
            }
        }
    }
}

fn update_instructions(
    _keyboard: Res<ButtonInput<KeyCode>>,
) {
    // ESC handling is now centralized in ui_stack.rs
}

fn update_xp_bar(
    stats: Res<PlayerStats>,
    mut xp_bar_query: Query<&mut Style, With<XpBar>>,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    // Update XP circle (scales from 0% to 100% size)
    if let Ok(mut style) = xp_bar_query.get_single_mut() {
        let percentage = if stats.xp_needed > 0 {
            (stats.experience as f32 / stats.xp_needed as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            100.0 // Max level
        };
        
        // Scale circle size based on XP percentage
        style.width = Val::Percent(percentage);
        style.height = Val::Percent(percentage);
    }

    // Update level text in center of circle
    if let Ok(mut text) = level_text_query.get_single_mut() {
        text.sections[0].value = format!("{}", stats.level);
    }
}

fn handle_dev_xp_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    network: Option<Res<crate::networking::NetworkClient>>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        if let Some(network) = network {
            use shared::ClientMessage;
            if let Err(e) = network.send_message(&ClientMessage::GainExperience { amount: 1000 }) {
                error!("Failed to send GainExperience: {}", e);
            } else {
                info!("Sent +1000 XP request (Dev Key 'K')");
            }
        }
    }
}

// ============================================================================
// DEV MODE PANEL
// ============================================================================

/// Setup dev mode panel (top-right corner) - Compact version
fn setup_dev_panel(
    mut commands: Commands,
    font: Res<GameFont>,
    player_stats: Res<PlayerStats>,
    dev_mode: Res<DevModeState>,
) {
    let font_handle = font.0.clone();
    
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                display: if dev_mode.enabled { Display::Flex } else { Display::None },
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.85).into(),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        DevPanel,
        GameUI,
    ))
    .with_children(|parent| {
        // Title with Level Display (compact)
        parent.spawn((
            TextBundle::from_section(
                format!("🔧 DEV | Lvl {}", player_stats.level),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 14.0,
                    color: Color::srgb(1.0, 0.8, 0.2),
                    ..default()
                },
            ),
            DevLevelText,
        ));

        // Row container for level buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        }).with_children(|row| {
            create_dev_button_compact(row, "+Lvl", DevButton::AddLevel, Color::srgb(0.2, 0.6, 0.2), font_handle.clone());
            create_dev_button_compact(row, "-Lvl", DevButton::RemoveLevel, Color::srgb(0.7, 0.2, 0.2), font_handle.clone());
        });

        // Row container for XP buttons
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        }).with_children(|row| {
            create_dev_button_compact(row, "+1K", DevButton::Add1000XP, Color::srgb(0.2, 0.4, 0.7), font_handle.clone());
            create_dev_button_compact(row, "→1", DevButton::ResetLevel, Color::srgb(0.5, 0.3, 0.1), font_handle.clone());
        });

        // Compact instructions
        parent.spawn(TextBundle::from_section(
            "F3: Toggle",
            TextStyle {
                font: font_handle.clone(),
                font_size: 10.0,
                color: Color::srgb(0.4, 0.4, 0.4),
                ..default()
            },
        ));
    });
}

fn create_dev_button_compact(
    parent: &mut ChildBuilder,
    label: &str,
    button_type: DevButton,
    color: Color,
    font: Handle<Font>,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(55.0),
                height: Val::Px(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: color.into(),
            ..default()
        },
        button_type,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font,
                font_size: 12.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

/// Toggle dev mode with F3 key
fn handle_dev_toggle_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dev_mode: ResMut<DevModeState>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        dev_mode.enabled = !dev_mode.enabled;
        info!("Dev Mode: {}", if dev_mode.enabled { "ON" } else { "OFF" });
    }
}

/// Handle dev panel button clicks
fn handle_dev_panel_buttons(
    mut interaction_query: Query<(&Interaction, &DevButton), Changed<Interaction>>,
    network: Option<Res<crate::networking::NetworkClient>>,
    player_stats: Res<PlayerStats>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let Some(network) = network.as_ref() else { continue };

            match button {
                DevButton::AddLevel => {
                    // Calculate XP needed to reach next level
                    let current_level = player_stats.level;
                    let next_level = current_level + 1;
                    let xp_for_next = shared::calculate_xp_for_level(next_level);
                    let xp_needed = xp_for_next - player_stats.experience;
                    
                    if let Err(e) = network.send_message(&shared::ClientMessage::GainExperience { 
                        amount: xp_needed 
                    }) {
                        error!("Failed to send AddLevel XP: {}", e);
                    } else {
                        info!("Dev: Adding level (sending {} XP)", xp_needed);
                    }
                }
                DevButton::RemoveLevel => {
                    if player_stats.level > 1 {
                        // Send negative XP to trigger level-down on server
                        // At level 100 with 0 XP, we still need to send negative
                        // to trigger the level-down
                        let xp_to_remove = if player_stats.experience > 0 {
                            -(player_stats.experience + 1)
                        } else {
                            -1  // At 0 XP, send -1 to trigger level-down
                        };
                        
                        if let Err(e) = network.send_message(&shared::ClientMessage::GainExperience { 
                            amount: xp_to_remove 
                        }) {
                            error!("Failed to send RemoveLevel: {}", e);
                        } else {
                            info!("Dev: -1 Level from {} (sending {} XP)", player_stats.level, xp_to_remove);
                        }
                    } else {
                        warn!("Already at level 1, cannot remove level");
                    }
                }
                DevButton::Add1000XP => {
                    if let Err(e) = network.send_message(&shared::ClientMessage::GainExperience { 
                        amount: 1000 
                    }) {
                        error!("Failed to send +1000 XP: {}", e);
                    } else {
                        info!("Dev: Adding 1000 XP");
                    }
                }
                DevButton::ResetLevel => {
                    // Reset to level 1 (XP = 0)
                    let xp_to_remove = -(player_stats.experience as i64);
                    
                    if let Err(e) = network.send_message(&shared::ClientMessage::GainExperience { 
                        amount: xp_to_remove 
                    }) {
                        error!("Failed to reset level: {}", e);
                    } else {
                        info!("Dev: Resetting to level 1");
                    }
                }
            }
        }
    }
}

/// Update dev panel visibility based on dev mode state
fn update_dev_panel_visibility(
    dev_mode: Res<DevModeState>,
    mut panel_query: Query<&mut Style, With<DevPanel>>,
) {
    if dev_mode.is_changed() {
        for mut style in panel_query.iter_mut() {
            style.display = if dev_mode.enabled { Display::Flex } else { Display::None };
        }
    }
}

/// Update dev panel level text
fn update_dev_panel_text(
    player_stats: Res<PlayerStats>,
    mut text_query: Query<&mut Text, With<DevLevelText>>,
) {
    if player_stats.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("🔧 DEV | Lvl {}", player_stats.level);
        }
    }
}

fn cleanup_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameUI>>,
    mut ui_stack: ResMut<UILayerStack>,
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::GameUI);
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
