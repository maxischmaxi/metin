use bevy::prelude::*;
use shared::{ClientMessage, ServerMessage, AuthMessage, AuthResponse, SERVER_ADDR};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::auth_state::{AuthState, SpawnPosition};
use crate::GameState;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelingEvent>()
            .init_resource::<ServerConnectionState>()
            .add_systems(Startup, setup_network)
            .add_systems(Update, (
                process_incoming_messages,
                handle_auth_responses,
                handle_leveling_events,
                check_server_connection,
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
    
    /// Attempt to ping the server to check connectivity
    pub fn ping_server(&self) -> Result<(), String> {
        // Send a small test packet to check if server is reachable
        let test_msg = ClientMessage::Auth(AuthMessage::Login {
            username: "__ping__".to_string(),
            password: "__ping__".to_string(),
        });
        
        self.send_message(&test_msg)
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
            info!("Network client initialized");
            commands.insert_resource(client);
        }
        Err(e) => {
            error!("Failed to initialize network client: {}", e);
            connection_state.is_connected = false;
        }
    }
}

/// Periodically check server connection status
fn check_server_connection(
    time: Res<Time>,
    network: Option<Res<NetworkClient>>,
    mut connection_state: ResMut<ServerConnectionState>,
    game_state: Res<State<crate::GameState>>,
) {
    // Only check connection in Login state (not during gameplay)
    if *game_state.get() != crate::GameState::Login {
        return;
    }
    
    let elapsed = time.elapsed_seconds_f64();
    
    // Check if it's time to ping the server
    if elapsed - connection_state.last_check >= connection_state.check_interval {
        connection_state.last_check = elapsed;
        
        if let Some(network) = network {
            match network.ping_server() {
                Ok(_) => {
                    if !connection_state.is_connected {
                        info!("Server connection established");
                    }
                    connection_state.is_connected = true;
                }
                Err(e) => {
                    if connection_state.is_connected {
                        warn!("Lost connection to server: {}", e);
                    }
                    connection_state.is_connected = false;
                }
            }
        } else {
            connection_state.is_connected = false;
        }
    }
}

fn process_incoming_messages(
    network: Option<Res<NetworkClient>>,
    mut auth_events: EventWriter<AuthResponseEvent>,
    mut char_events: EventWriter<CharacterResponseEvent>,
    mut leveling_events: EventWriter<LevelingEvent>,
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
            ServerMessage::CharacterSelected { character_id, character_name, position, level, experience, max_health, max_mana, max_stamina, specialization } => {
                char_events.send(CharacterResponseEvent::Selected { 
                    character_id,
                    character_name,
                    position,
                    level,
                    experience,
                    max_health,
                    max_mana,
                    max_stamina,
                });
                // TODO: Store specialization in AuthState
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
        position: Vec3,
        level: i32,
        experience: i64,
        max_health: f32,
        max_mana: f32,
        max_stamina: f32,
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

#[allow(dead_code)]
#[derive(Component)]
pub struct OtherPlayer {
    pub id: u64,
}
