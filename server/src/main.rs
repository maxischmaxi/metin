mod db;
mod auth;

use shared::{ClientMessage, ServerMessage, AuthMessage, SERVER_ADDR};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::time::{Instant, Duration};
use auth::SessionManager;
use shared::bevy::prelude::Vec3;

// Server-side player state with position tracking
#[derive(Debug, Clone)]
struct PlayerState {
    id: u64,
    character: shared::CharacterData,
    character_id: i64,      // DB ID for saving
    position: Vec3,
    dirty: bool,            // Position changed since last save?
    last_save: Instant,     // When was last DB save?
}

struct GameServer {
    socket: UdpSocket,
    db_pool: SqlitePool,
    session_manager: SessionManager,
    players: HashMap<String, PlayerState>,
    last_update: Instant,
    last_batch_save: Instant,
    save_interval: Duration,  // How often to auto-save (5 minutes)
}

impl GameServer {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database
        let database_url = "sqlite://game.db";
        let db_pool = db::init_database(database_url).await?;
        log::info!("Database initialized successfully");

        // Setup UDP socket
        let socket = UdpSocket::bind(SERVER_ADDR)?;
        socket.set_nonblocking(true)?;
        log::info!("Server started on {}", SERVER_ADDR);

        Ok(Self {
            socket,
            db_pool,
            session_manager: SessionManager::new(),
            players: HashMap::new(),
            last_update: Instant::now(),
            last_batch_save: Instant::now(),
            save_interval: Duration::from_secs(5 * 60), // 5 minutes
        })
    }

    async fn update(&mut self) {
        let mut buf = [0u8; 65536];
        
        // Receive messages
        while let Ok((size, src)) = self.socket.recv_from(&mut buf) {
            if let Ok(client_msg) = bincode::deserialize::<ClientMessage>(&buf[..size]) {
                self.handle_client_message(src, client_msg).await;
            }
        }

        // Cleanup expired sessions periodically
        if self.last_update.elapsed().as_secs() > 60 {
            let removed = self.session_manager.cleanup_expired();
            if removed > 0 {
                log::info!("Cleaned up {} expired sessions", removed);
            }
            self.last_update = Instant::now();
        }

        // Auto-save positions periodically (every 10 seconds check)
        if self.last_batch_save.elapsed().as_secs() >= 10 {
            self.auto_save_positions().await;
            self.last_batch_save = Instant::now();
        }
    }

    async fn handle_client_message(&mut self, client_addr: SocketAddr, message: ClientMessage) {
        match message {
            ClientMessage::Auth(auth_msg) => {
                self.handle_auth_message(client_addr, auth_msg).await;
            }
            ClientMessage::CreateCharacter { token, character } => {
                self.handle_create_character(client_addr, token, character).await;
            }
            ClientMessage::SelectCharacter { token, character_id } => {
                self.handle_select_character(client_addr, token, character_id).await;
            }
            ClientMessage::DeleteCharacter { token, character_id } => {
                self.handle_delete_character(client_addr, token, character_id).await;
            }
            ClientMessage::Join { character } => {
                log::info!("Player {} joined with character: {}", client_addr, character.name);
                
                let player_state = PlayerState {
                    id: client_addr.to_string().len() as u64,
                    character: character.clone(),
                    character_id: 0, // Will be set when we integrate with SelectCharacter
                    position: Vec3::new(0.0, 1.0, 0.0),
                    dirty: false,
                    last_save: Instant::now(),
                };

                self.players.insert(client_addr.to_string(), player_state);
            }
            ClientMessage::Move { direction } => {
                let addr_str = client_addr.to_string();
                if let Some(player) = self.players.get_mut(&addr_str) {
                    player.position += direction * 5.0 * 0.016;
                    player.dirty = true; // Mark as dirty for auto-save
                    log::debug!("Player {} moved to {:?}", addr_str, player.position);
                }
            }
            ClientMessage::UpdatePosition { position } => {
                let addr_str = client_addr.to_string();
                if let Some(player) = self.players.get_mut(&addr_str) {
                    player.position = position;
                    player.dirty = true;
                    log::debug!("Player {} position updated to {:?}", addr_str, player.position);
                }
            }
            ClientMessage::GainExperience { amount } => {
                self.handle_gain_experience(client_addr, amount).await;
            }
            ClientMessage::ChooseSpecialization { token, specialization } => {
                self.handle_choose_specialization(client_addr, token, specialization).await;
            }
            ClientMessage::Disconnect => {
                let addr_str = client_addr.to_string();
                log::info!("Player {} disconnecting", addr_str);
                
                // Save position before removing player
                if let Some(player) = self.players.get(&addr_str) {
                    self.save_player_position(player).await;
                }
                
                self.players.remove(&addr_str);
            }
        }
    }

    async fn handle_auth_message(&mut self, client_addr: SocketAddr, auth_msg: AuthMessage) {
        let response = match auth_msg {
            AuthMessage::Register { username, password, email } => {
                auth::handle_register(&self.db_pool, username, password, email).await
            }
            AuthMessage::Login { username, password } => {
                auth::handle_login(&self.db_pool, &mut self.session_manager, username, password).await
            }
        };

        self.send_response(client_addr, ServerMessage::AuthResponse(response));
    }

    async fn handle_create_character(&mut self, client_addr: SocketAddr, token: String, character: shared::CharacterData) {
        // Validate token
        let session = match self.session_manager.validate_token(&token) {
            Some(s) => s,
            None => {
                self.send_response(client_addr, ServerMessage::CharacterCreationFailed {
                    reason: "Invalid or expired token".to_string(),
                });
                return;
            }
        };

        // Check if character name exists
        match db::characters::character_name_exists(&self.db_pool, &character.name).await {
            Ok(true) => {
                self.send_response(client_addr, ServerMessage::CharacterCreationFailed {
                    reason: "Character name already exists".to_string(),
                });
                return;
            }
            Ok(false) => {}
            Err(e) => {
                log::error!("Error checking character name: {}", e);
                self.send_response(client_addr, ServerMessage::CharacterCreationFailed {
                    reason: "Internal server error".to_string(),
                });
                return;
            }
        }

        // Create character
        match db::characters::create_character(&self.db_pool, session.user_id, &character).await {
            Ok(char_id) => {
                log::info!("Character '{}' created for user {}", character.name, session.username);
                self.send_response(client_addr, ServerMessage::CharacterCreated { character_id: char_id });
            }
            Err(e) => {
                log::error!("Error creating character: {}", e);
                self.send_response(client_addr, ServerMessage::CharacterCreationFailed {
                    reason: "Failed to create character".to_string(),
                });
            }
        }
    }

    async fn handle_select_character(&mut self, client_addr: SocketAddr, token: String, character_id: i64) {
        // Validate token
        let session = match self.session_manager.validate_token(&token) {
            Some(s) => s,
            None => {
                self.send_response(client_addr, ServerMessage::CharacterSelectionFailed {
                    reason: "Invalid or expired token".to_string(),
                });
                return;
            }
        };

        let user_id = session.user_id;
        let username = session.username.clone();

        // Verify character belongs to user and load position
        match db::characters::load_character(&self.db_pool, character_id).await {
            Ok(Some(character)) => {
                if character.user_id != user_id {
                    self.send_response(client_addr, ServerMessage::CharacterSelectionFailed {
                        reason: "Character does not belong to you".to_string(),
                    });
                    return;
                }

                // Extract saved position from database
                let position = Vec3::new(character.pos_x, character.pos_y, character.pos_z);

                // Set character in session
                if let Some(session) = self.session_manager.get_session_mut(&token) {
                    session.set_character(character_id);
                    log::info!(
                        "User {} selected character {} ({}) at position {:?}", 
                        username, character_id, character.name, position
                    );
                    
                    // Create PlayerState for this character (entering world)
                    let character_data = character.to_character_data();
                    let player_state = PlayerState {
                        id: client_addr.to_string().len() as u64, // Simple ID generation
                        character: character_data,
                        character_id,
                        position,
                        dirty: false,
                        last_save: Instant::now(),
                    };
                    
                    self.players.insert(client_addr.to_string(), player_state);
                    log::info!("Player state created for character {} at {:?}", character_id, position);
                    
                    // Convert string class to CharacterClass
                    let char_class = match character.class.as_str() {
                        "Krieger" => shared::CharacterClass::Krieger,
                        "Ninja" => shared::CharacterClass::Ninja,
                        "Sura" => shared::CharacterClass::Sura,
                        "Schamane" => shared::CharacterClass::Schamane,
                        _ => shared::CharacterClass::Krieger,
                    };
                    
                    // Calculate stats for level
                    let (max_health, max_mana, max_stamina) = 
                        shared::calculate_stats_for_level(character.level, &char_class);
                    
                    // Parse specialization from DB
                    let specialization = character.specialization.as_ref().and_then(|s| {
                        shared::Specialization::from_string(s)
                    });
                    
                    // Send character_id, name, class, position, level, XP, and stats to client
                    self.send_response(client_addr, ServerMessage::CharacterSelected { 
                        character_id,
                        character_name: character.name.clone(),
                        character_class: char_class,
                        position,
                        level: character.level,
                        experience: character.experience,
                        max_health,
                        max_mana,
                        max_stamina,
                        specialization,
                    });
                }
            }
            Ok(None) => {
                self.send_response(client_addr, ServerMessage::CharacterSelectionFailed {
                    reason: "Character not found".to_string(),
                });
            }
            Err(e) => {
                log::error!("Error getting character: {}", e);
                self.send_response(client_addr, ServerMessage::CharacterSelectionFailed {
                    reason: "Internal server error".to_string(),
                });
            }
        }
    }

    async fn handle_delete_character(&mut self, client_addr: SocketAddr, token: String, character_id: i64) {
        let session = match self.session_manager.validate_token(&token) {
            Some(s) => s,
            None => {
                self.send_response(client_addr, ServerMessage::CharacterDeletionFailed {
                    reason: "Invalid or expired token".to_string(),
                });
                return;
            }
        };

        match db::characters::delete_character(&self.db_pool, character_id, session.user_id).await {
            Ok(true) => {
                self.send_response(client_addr, ServerMessage::CharacterDeleted {
                    character_id,
                });
            }
            Ok(false) => {
                self.send_response(client_addr, ServerMessage::CharacterDeletionFailed {
                    reason: "Character not found or not owned by you".to_string(),
                });
            }
            Err(e) => {
                log::error!("Error deleting character: {}", e);
                self.send_response(client_addr, ServerMessage::CharacterDeletionFailed {
                    reason: "Internal server error".to_string(),
                });
            }
        }
    }

    async fn handle_choose_specialization(
        &mut self,
        client_addr: SocketAddr,
        token: String,
        specialization: shared::Specialization,
    ) {
        // 1. Validate token
        let session = match self.session_manager.validate_token(&token) {
            Some(s) => s,
            None => {
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationFailed {
                        reason: "Invalid or expired token".to_string(),
                    },
                );
                return;
            }
        };

        let user_id = session.user_id;

        // 2. Get current character ID from session
        let character_id = match session.character_id {
            Some(id) => id,
            None => {
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationFailed {
                        reason: "No character selected".to_string(),
                    },
                );
                return;
            }
        };

        // 3. Load character from database
        let character = match db::characters::load_character(&self.db_pool, character_id).await {
            Ok(Some(c)) => c,
            Ok(None) => {
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationFailed {
                        reason: "Character not found".to_string(),
                    },
                );
                return;
            }
            Err(e) => {
                log::error!("Error loading character: {}", e);
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationFailed {
                        reason: "Internal server error".to_string(),
                    },
                );
                return;
            }
        };

        // 4. Verify ownership
        if character.user_id != user_id {
            self.send_response(
                client_addr,
                ServerMessage::SpecializationFailed {
                    reason: "Character does not belong to you".to_string(),
                },
            );
            return;
        }

        // 5. Check level requirement (must be at least level 5)
        if character.level < 5 {
            self.send_response(
                client_addr,
                ServerMessage::SpecializationFailed {
                    reason: format!("You must reach level 5 first (current: {})", character.level),
                },
            );
            return;
        }

        // 6. Check if specialization already chosen
        if character.specialization.is_some() {
            self.send_response(
                client_addr,
                ServerMessage::SpecializationFailed {
                    reason: "You have already chosen a specialization".to_string(),
                },
            );
            return;
        }

        // 7. Verify specialization matches character class
        let char_class = match character.class.as_str() {
            "Krieger" => shared::CharacterClass::Krieger,
            "Ninja" => shared::CharacterClass::Ninja,
            "Sura" => shared::CharacterClass::Sura,
            "Schamane" => shared::CharacterClass::Schamane,
            _ => shared::CharacterClass::Krieger,
        };

        if !specialization.is_valid_for_class(char_class) {
            self.send_response(
                client_addr,
                ServerMessage::SpecializationFailed {
                    reason: format!(
                        "Specialization {} is not valid for class {}",
                        specialization.name(),
                        char_class.as_str()
                    ),
                },
            );
            return;
        }

        // 8. Save specialization to database
        let spec_str = specialization.as_str();
        match db::characters::update_specialization(&self.db_pool, character_id, spec_str).await {
            Ok(_) => {
                log::info!(
                    "Character {} (user {}) chose specialization: {}",
                    character.name,
                    user_id,
                    specialization.name()
                );

                // Update player state if they're in the world
                let addr_str = client_addr.to_string();
                if let Some(player) = self.players.get_mut(&addr_str) {
                    player.character.specialization = Some(specialization);
                }

                // Send success response
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationChosen { specialization },
                );
            }
            Err(e) => {
                log::error!("Error saving specialization: {}", e);
                self.send_response(
                    client_addr,
                    ServerMessage::SpecializationFailed {
                        reason: "Failed to save specialization".to_string(),
                    },
                );
            }
        }
    }

    fn send_response(&self, addr: SocketAddr, message: ServerMessage) {
        if let Ok(data) = bincode::serialize(&message) {
            if let Err(e) = self.socket.send_to(&data, addr) {
                log::error!("Error sending response to {}: {}", addr, e);
            }
        }
    }

    /// Auto-save positions for players that need it (dirty flag + time-based)
    async fn auto_save_positions(&mut self) {
        let mut positions_to_save = Vec::new();
        let now = Instant::now();

        // Collect all players that need saving
        for player in self.players.values_mut() {
            if player.dirty && player.last_save.elapsed() >= self.save_interval {
                positions_to_save.push((
                    player.character_id,
                    player.position.x,
                    player.position.y,
                    player.position.z,
                ));
                player.dirty = false;
                player.last_save = now;
            }
        }

        // Batch save all positions in one transaction
        if !positions_to_save.is_empty() {
            match db::characters::batch_save_positions(&self.db_pool, &positions_to_save).await {
                Ok(_) => {
                    log::info!("Auto-saved positions for {} players", positions_to_save.len());
                }
                Err(e) => {
                    log::error!("Error batch saving positions: {}", e);
                    // Re-mark as dirty on error so we retry
                    for (char_id, _, _, _) in positions_to_save {
                        if let Some(player) = self.players.values_mut().find(|p| p.character_id == char_id) {
                            player.dirty = true;
                        }
                    }
                }
            }
        }
    }

    /// Save a single player's position immediately (for disconnects)
    async fn save_player_position(&self, player: &PlayerState) {
        if player.dirty {
            match db::characters::update_position(
                &self.db_pool,
                player.character_id,
                player.position.x,
                player.position.y,
                player.position.z,
            ).await {
                Ok(_) => {
                    log::info!("Saved position for character {} on disconnect", player.character_id);
                }
                Err(e) => {
                    log::error!("Error saving position on disconnect: {}", e);
                }
            }
        }
    }

    /// Handle experience gain and level-ups
    async fn handle_gain_experience(&mut self, client_addr: SocketAddr, amount: i64) {
        let addr_str = client_addr.to_string();
        
        // Extract data we need before borrowing
        let (character_id, current_level, current_xp, character_class) = {
            let player = match self.players.get(&addr_str) {
                Some(p) => p,
                None => {
                    log::warn!("No player state for {}", addr_str);
                    return;
                }
            };
            (player.character_id, player.character.level, player.character.experience, player.character.class.clone())
        };
        
        // Add experience (can be negative for level down)
        let mut new_xp = current_xp + amount;
        
        log::info!("Character {} gained {} XP (total: {})", character_id, amount, new_xp);
        
        let mut new_level = current_level;
        let mut level_changed = false;
        
        // Handle level-downs FIRST (negative XP) - for DEV commands
        if new_xp < 0 {
            if new_level > 1 {
                // Go down one level and set XP to 0
                new_level -= 1;
                new_xp = 0;
                level_changed = true;
                log::info!("DEV: Character {} leveled DOWN to {} (XP reset to 0)", character_id, new_level);
            } else {
                // At level 1, just set XP to 0
                new_xp = 0;
                log::info!("DEV: Character {} already at level 1, XP reset to 0", character_id);
            }
        } else {
            // Handle level-ups (positive XP) - only if XP is positive!
            while new_level < 100 {
                let xp_needed = shared::calculate_xp_for_level(new_level + 1);
                
                if new_xp >= xp_needed {
                    // Level up!
                    new_xp -= xp_needed;
                    new_level += 1;
                    level_changed = true;
                    log::info!("Character {} leveled UP to {}!", character_id, new_level);
                } else {
                    break;
                }
            }
        }
        
        // Update character data in player state
        if let Some(player) = self.players.get_mut(&addr_str) {
            player.character.level = new_level;
            player.character.experience = new_xp;
        }
        
        // Calculate XP needed for next level
        let xp_needed = if new_level < 100 {
            shared::calculate_xp_for_level(new_level + 1)
        } else {
            0 // Max level
        };
        
        // Send experience gained message
        self.send_response(client_addr, ServerMessage::ExperienceGained {
            amount,
            new_total: new_xp,
            xp_needed,
        });
        
        // If level changed (up or down), send level-up message with new stats
        if level_changed {
            let char_class = match character_class.as_str() {
                "Krieger" => shared::CharacterClass::Krieger,
                "Ninja" => shared::CharacterClass::Ninja,
                "Sura" => shared::CharacterClass::Sura,
                "Schamane" => shared::CharacterClass::Schamane,
                _ => shared::CharacterClass::Krieger,
            };
            
            let (max_health, max_mana, max_stamina) = 
                shared::calculate_stats_for_level(new_level, &char_class);
            
            self.send_response(client_addr, ServerMessage::LevelUp {
                new_level,
                new_max_health: max_health,
                new_max_mana: max_mana,
                new_max_stamina: max_stamina,
            });
            
            log::info!(
                "Character {} is now level {} (HP: {}, Mana: {}, Stamina: {})",
                character_id, new_level, max_health, max_mana, max_stamina
            );
        }
        
        // Save to database
        match db::characters::update_level_and_xp(&self.db_pool, character_id, new_level, new_xp).await {
            Ok(_) => {
                log::debug!("Updated level/XP in database for character {}", character_id);
            }
            Err(e) => {
                log::error!("Error updating level/XP in database: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let mut server = GameServer::new().await?;
    
    log::info!("Game server running. Press Ctrl+C to stop.");
    
    loop {
        server.update().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await; // ~60 FPS
    }
}
