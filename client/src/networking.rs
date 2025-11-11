use bevy::prelude::*;
use shared::{ClientMessage, ServerMessage, AuthMessage, AuthResponse, SERVER_ADDR};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::auth_state::AuthState;
use crate::GameState;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelingEvent>()
            .add_event::<CharacterResponseEvent>()
            .init_resource::<ServerConnectionState>()
            .add_systems(Startup, setup_network)
            .add_systems(Update, (
                process_incoming_messages,
                handle_auth_responses,
                handle_leveling_events,
                handle_character_responses,
            ));
    }
}

/// Resource to track server connection status
#[derive(Resource)]
pub struct ServerConnectionState {
    pub is_connected: bool,
    pub last_check: f64,
    pub check_interval: f64, // seconds between checks
}

impl Default for ServerConnectionState {
    fn default() -> Self {
        Self {
            is_connected: false,
            last_check: 0.0,
            check_interval: 2.0, // Check every 2 seconds
        }
    }
}

#[derive(Resource)]
pub struct NetworkClient {
    socket: Arc<Mutex<UdpSocket>>,
    incoming_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
    server_addr: String,
}

impl NetworkClient {
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(true)?;
        
        let socket = Arc::new(Mutex::new(socket));
        let incoming_messages = Arc::new(Mutex::new(VecDeque::new()));
        
        // Start listener thread
        let socket_clone = socket.clone();
        let messages_clone = incoming_messages.clone();
        
        std::thread::spawn(move || {
            listen_for_messages(socket_clone, messages_clone);
        });
        
        Ok(Self {
            socket,
            incoming_messages,
            server_addr: SERVER_ADDR.to_string(),
        })
    }
    
    pub fn send_message(&self, message: &ClientMessage) -> Result<(), String> {
        let data = bincode::serialize(message)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        let socket = self.socket.lock().unwrap();
        socket.send_to(&data, &self.server_addr)
            .map_err(|e| format!("Send error: {}", e))?;
        
        Ok(())
    }
    
    pub fn get_message(&self) -> Option<ServerMessage> {
        let mut messages = self.incoming_messages.lock().unwrap();
        messages.pop_front()
    }
}

fn listen_for_messages(
    socket: Arc<Mutex<UdpSocket>>,
    messages: Arc<Mutex<VecDeque<ServerMessage>>>,
) {
    let mut buf = [0u8; 65536];
    
    loop {
        let socket = socket.lock().unwrap();
        match socket.recv_from(&mut buf) {
            Ok((size, _src)) => {
                if let Ok(msg) = bincode::deserialize::<ServerMessage>(&buf[..size]) {
                    let mut messages = messages.lock().unwrap();
                    messages.push_back(msg);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                drop(socket);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(e) => {
                error!("Network error: {}", e);
                drop(socket);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

fn setup_network(
    mut commands: Commands,
    mut connection_state: ResMut<ServerConnectionState>,
) {
    match NetworkClient::new() {
        Ok(client) => {
            info!("Network client initialized - ready to connect to {}", SERVER_ADDR);
            connection_state.is_connected = true; // Assume connected when client initializes
            commands.insert_resource(client);
        }
        Err(e) => {
            error!("Failed to initialize network client: {}", e);
            connection_state.is_connected = false;
        }
    }
}

// Connection state is now managed by successful message transmission
// No need for periodic ping checks that spam fake login requests

fn process_incoming_messages(
    network: Option<Res<NetworkClient>>,
    mut auth_events: EventWriter<AuthResponseEvent>,
    mut char_events: EventWriter<CharacterResponseEvent>,
    mut leveling_events: EventWriter<LevelingEvent>,
    mut game_time: ResMut<crate::skybox::GameTime>,
) {
    let Some(network) = network else { return };
    
    while let Some(message) = network.get_message() {
        match message {
            ServerMessage::AuthResponse(response) => {
                auth_events.send(AuthResponseEvent(response));
            }
            ServerMessage::CharacterCreated { character_id } => {
                char_events.send(CharacterResponseEvent::Created { character_id });
            }
            ServerMessage::CharacterCreationFailed { reason } => {
                char_events.send(CharacterResponseEvent::CreationFailed { reason });
            }
            ServerMessage::CharacterSelected { character_id, character_name, position, character_class, level, experience, max_health, max_mana, max_stamina, specialization } => {
                char_events.send(CharacterResponseEvent::Selected { 
                    character_id,
                    character_name,
                    position,
                    character_class,
                    level,
                    experience,
                    max_health,
                    max_mana,
                    max_stamina,
                    specialization,
                });
            }
            ServerMessage::CharacterSelectionFailed { reason } => {
                char_events.send(CharacterResponseEvent::SelectionFailed { reason });
            }
            ServerMessage::CharacterDeleted { character_id } => {
                char_events.send(CharacterResponseEvent::Deleted { character_id });
            }
            ServerMessage::CharacterDeletionFailed { reason } => {
                char_events.send(CharacterResponseEvent::DeletionFailed { reason });
            }
            ServerMessage::ExperienceGained { amount, new_total, xp_needed } => {
                leveling_events.send(LevelingEvent::ExperienceGained { amount, new_total, xp_needed });
            }
            ServerMessage::LevelUp { new_level, new_max_health, new_max_mana, new_max_stamina } => {
                leveling_events.send(LevelingEvent::LevelUp { 
                    new_level, 
                    new_max_health, 
                    new_max_mana, 
                    new_max_stamina 
                });
            }
            ServerMessage::TimeUpdate { hour } => {
                // Receive initial time from server (only once after login)
                if !game_time.time_synced {
                    game_time.sync_from_server(hour);
                    info!("✅ Time synchronized from server: {:02.1}:00", hour);
                } else {
                    // Ignore subsequent time updates (shouldn't happen but just in case)
                    debug!("Ignoring time update - already synced");
                }
            }
            _ => {
                // Handle other messages (gameplay, etc.)
            }
        }
    }
}

// Events for auth responses
#[derive(Event)]
pub struct AuthResponseEvent(pub AuthResponse);

#[derive(Event)]
pub enum CharacterResponseEvent {
    Created { character_id: i64 },
    CreationFailed { reason: String },
    Selected { 
        character_id: i64,
        character_name: String,
        character_class: shared::CharacterClass,
        position: Vec3,
        level: i32,
        experience: i64,
        max_health: f32,
        max_mana: f32,
        max_stamina: f32,
        specialization: Option<shared::Specialization>,
    },
    SelectionFailed { reason: String },
    Deleted { character_id: i64 },
    DeletionFailed { reason: String },
}

#[derive(Event)]
pub enum LevelingEvent {
    ExperienceGained { amount: i64, new_total: i64, xp_needed: i64 },
    LevelUp { new_level: i32, new_max_health: f32, new_max_mana: f32, new_max_stamina: f32 },
}

fn handle_auth_responses(
    mut auth_events: EventReader<AuthResponseEvent>,
    mut auth_state: ResMut<AuthState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in auth_events.read() {
        match &event.0 {
            AuthResponse::LoginSuccess { token, characters } => {
                info!("Login successful! Received {} characters", characters.len());
                
                // Store auth data
                let username = "user".to_string(); // TODO: Store username from login
                auth_state.login(token.clone(), username, characters.clone());
                
                // Go to character selection
                next_state.set(GameState::CharacterSelection);
            }
            AuthResponse::LoginFailed { reason } => {
                error!("Login failed: {}", reason);
                // TODO: Show error in UI
            }
            AuthResponse::RegisterSuccess => {
                info!("Registration successful! Please login.");
                // TODO: Show success message and switch to login
            }
            AuthResponse::RegisterFailed { reason } => {
                error!("Registration failed: {}", reason);
                // TODO: Show error in UI
            }
        }
    }
}

fn handle_leveling_events(
    mut leveling_events: EventReader<LevelingEvent>,
    mut player_stats: ResMut<crate::ui::PlayerStats>,
) {
    for event in leveling_events.read() {
        match event {
            LevelingEvent::ExperienceGained { amount, new_total, xp_needed } => {
                info!("+{} XP! ({}/{})", amount, new_total, xp_needed);
                player_stats.experience = *new_total;
                player_stats.xp_needed = *xp_needed;
            }
            LevelingEvent::LevelUp { new_level, new_max_health, new_max_mana, new_max_stamina } => {
                info!("🎉 LEVEL UP! Now level {}", new_level);
                info!("  HP: {} → {}", player_stats.max_health, new_max_health);
                info!("  Mana: {} → {}", player_stats.max_mana, new_max_mana);
                info!("  Stamina: {} → {}", player_stats.max_stamina, new_max_stamina);
                
                player_stats.level = *new_level;
                player_stats.max_health = *new_max_health;
                player_stats.max_mana = *new_max_mana;
                player_stats.max_stamina = *new_max_stamina;
                
                // Restore health/mana/stamina to full on level up
                player_stats.health = *new_max_health;
                player_stats.mana = *new_max_mana;
                player_stats.stamina = *new_max_stamina;
                
                // Calculate XP needed for next level
                if *new_level < 100 {
                    player_stats.xp_needed = shared::calculate_xp_for_level(new_level + 1);
                } else {
                    player_stats.xp_needed = 0; // Max level
                }
            }
        }
    }
}

// Helper function to send auth request
pub fn send_auth_request(
    network: &NetworkClient,
    auth_msg: AuthMessage,
) -> Result<(), String> {
    network.send_message(&ClientMessage::Auth(auth_msg))
}

// Helper function to send character creation request
pub fn send_create_character(
    network: &NetworkClient,
    token: &str,
    character: shared::CharacterData,
) -> Result<(), String> {
    network.send_message(&ClientMessage::CreateCharacter {
        token: token.to_string(),
        character,
    })
}

/// Global handler for character response events
fn handle_character_responses(
    mut char_events: EventReader<CharacterResponseEvent>,
    mut spawn_position: ResMut<crate::auth_state::SpawnPosition>,
    mut player_stats: ResMut<crate::ui::PlayerStats>,
    mut auth_state: ResMut<crate::auth_state::AuthState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in char_events.read() {
        match event {
            CharacterResponseEvent::Selected { 
                character_id,
                character_name,
                character_class,
                position,
                level,
                experience,
                max_health,
                max_mana,
                max_stamina,
                specialization,
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
                
                // Store class and specialization directly from server message
                auth_state.class = Some(*character_class);
                auth_state.specialization = *specialization;
                
                info!("  Class: {}", character_class.as_str());
                if let Some(spec) = specialization {
                    info!("  Specialization: {}", spec.name());
                } else {
                    info!("  No specialization chosen yet (unlocks at Level 5)");
                }
                
                // Transition to InGame
                next_state.set(GameState::InGame);
            }
            CharacterResponseEvent::SelectionFailed { reason } => {
                error!("Character selection failed: {}", reason);
                // TODO: Show error in UI
            }
            CharacterResponseEvent::Created { character_id } => {
                info!("Character created with ID: {}", character_id);
                // NOTE: The character_creation module will handle auto-selecting this character
            }
            CharacterResponseEvent::CreationFailed { reason } => {
                error!("Character creation failed: {}", reason);
                // TODO: Show error in UI
            }
            _ => {}
        }
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct OtherPlayer {
    pub id: u64,
}
