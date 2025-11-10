use sqlx::SqlitePool;
use shared::{AuthMessage, AuthResponse, CharacterSummary};
use crate::db;
use crate::auth::{hash_password, verify_password, create_token, SessionManager, SessionData};

const TOKEN_DURATION_HOURS: i64 = 24;

/// Handle user registration
pub async fn handle_register(
    pool: &SqlitePool,
    username: String,
    password: String,
    email: Option<String>,
) -> AuthResponse {
    // Validate username
    if username.len() < 3 || username.len() > 20 {
        return AuthResponse::RegisterFailed {
            reason: "Username must be between 3 and 20 characters".to_string(),
        };
    }

    // Validate password
    if password.len() < 8 {
        return AuthResponse::RegisterFailed {
            reason: "Password must be at least 8 characters".to_string(),
        };
    }

    // Check if username already exists
    match db::users::username_exists(pool, &username).await {
        Ok(true) => {
            return AuthResponse::RegisterFailed {
                reason: "Username already exists".to_string(),
            };
        }
        Ok(false) => {}
        Err(e) => {
            log::error!("Database error checking username: {}", e);
            return AuthResponse::RegisterFailed {
                reason: "Internal server error".to_string(),
            };
        }
    }

    // Hash password
    let password_hash = match hash_password(&password) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Error hashing password: {}", e);
            return AuthResponse::RegisterFailed {
                reason: "Internal server error".to_string(),
            };
        }
    };

    // Create user
    match db::users::create_user(pool, &username, &password_hash, email.as_deref()).await {
        Ok(_user_id) => {
            log::info!("User '{}' registered successfully", username);
            AuthResponse::RegisterSuccess
        }
        Err(e) => {
            log::error!("Error creating user: {}", e);
            AuthResponse::RegisterFailed {
                reason: "Failed to create user".to_string(),
            }
        }
    }
}

/// Handle user login
pub async fn handle_login(
    pool: &SqlitePool,
    session_manager: &mut SessionManager,
    username: String,
    password: String,
) -> AuthResponse {
    // Find user by username
    let user = match db::users::find_by_username(pool, &username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return AuthResponse::LoginFailed {
                reason: "Invalid username or password".to_string(),
            };
        }
        Err(e) => {
            log::error!("Database error finding user: {}", e);
            return AuthResponse::LoginFailed {
                reason: "Internal server error".to_string(),
            };
        }
    };

    // Verify password
    match verify_password(&password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => {
            return AuthResponse::LoginFailed {
                reason: "Invalid username or password".to_string(),
            };
        }
        Err(e) => {
            log::error!("Error verifying password: {}", e);
            return AuthResponse::LoginFailed {
                reason: "Internal server error".to_string(),
            };
        }
    }

    // Update last login
    if let Err(e) = db::users::update_last_login(pool, user.id).await {
        log::error!("Error updating last login: {}", e);
    }

    // Get user's characters
    let characters = match db::characters::get_user_characters(pool, user.id).await {
        Ok(chars) => chars.into_iter().map(|c| CharacterSummary {
            id: c.id,
            name: c.name,
            class: match c.class.as_str() {
                "Krieger" => shared::CharacterClass::Krieger,
                "Ninja" => shared::CharacterClass::Ninja,
                "Sura" => shared::CharacterClass::Sura,
                "Schamane" => shared::CharacterClass::Schamane,
                _ => shared::CharacterClass::Krieger,
            },
            level: c.level,
            last_played: c.last_played.map(|dt| dt.to_rfc3339()),
            specialization: None,  // TODO: Load from DB after migration
        }).collect(),
        Err(e) => {
            log::error!("Error loading characters: {}", e);
            vec![]
        }
    };

    // Create JWT token
    let token = match create_token(user.id, &user.username, TOKEN_DURATION_HOURS) {
        Ok(t) => t,
        Err(e) => {
            log::error!("Error creating token: {}", e);
            return AuthResponse::LoginFailed {
                reason: "Internal server error".to_string(),
            };
        }
    };

    // Add session
    let session = SessionData::new(user.id, user.username.clone(), token.clone(), TOKEN_DURATION_HOURS);
    session_manager.add_session(token.clone(), session);

    log::info!("User '{}' logged in successfully", username);

    AuthResponse::LoginSuccess {
        token,
        characters,
    }
}
