use server::{db, auth};

#[tokio::test]
async fn test_user_registration_and_login() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    let mut session_manager = auth::SessionManager::new();
    
    // Test registration
    let response = auth::handle_register(
        &pool,
        "testuser".to_string(),
        "password123".to_string(),
        Some("test@example.com".to_string()),
    )
    .await;
    
    match response {
        shared::AuthResponse::RegisterSuccess => {
            // Success
        }
        shared::AuthResponse::RegisterFailed { reason } => {
            panic!("Registration failed: {}", reason);
        }
        _ => panic!("Unexpected response"),
    }
    
    // Test login
    let response = auth::handle_login(
        &pool,
        &mut session_manager,
        "testuser".to_string(),
        "password123".to_string(),
    )
    .await;
    
    match response {
        shared::AuthResponse::LoginSuccess { token, characters } => {
            assert!(!token.is_empty());
            assert_eq!(characters.len(), 0); // No characters yet
        }
        shared::AuthResponse::LoginFailed { reason } => {
            panic!("Login failed: {}", reason);
        }
        _ => panic!("Unexpected response"),
    }
}

#[tokio::test]
async fn test_registration_validation() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    
    // Test short username
    let response = auth::handle_register(
        &pool,
        "ab".to_string(),
        "password123".to_string(),
        None,
    )
    .await;
    
    match response {
        shared::AuthResponse::RegisterFailed { reason } => {
            assert!(reason.contains("3 and 20 characters"));
        }
        _ => panic!("Expected registration to fail"),
    }
    
    // Test short password
    let response = auth::handle_register(
        &pool,
        "testuser".to_string(),
        "short".to_string(),
        None,
    )
    .await;
    
    match response {
        shared::AuthResponse::RegisterFailed { reason } => {
            assert!(reason.contains("at least 8 characters"));
        }
        _ => panic!("Expected registration to fail"),
    }
}

#[tokio::test]
async fn test_duplicate_username() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    
    // Register first user
    auth::handle_register(
        &pool,
        "testuser".to_string(),
        "password123".to_string(),
        None,
    )
    .await;
    
    // Try to register same username again
    let response = auth::handle_register(
        &pool,
        "testuser".to_string(),
        "differentpass".to_string(),
        None,
    )
    .await;
    
    match response {
        shared::AuthResponse::RegisterFailed { reason } => {
            assert!(reason.contains("already exists"));
        }
        _ => panic!("Expected registration to fail"),
    }
}

#[tokio::test]
async fn test_invalid_login() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    let mut session_manager = auth::SessionManager::new();
    
    // Try to login with non-existent user
    let response = auth::handle_login(
        &pool,
        &mut session_manager,
        "nonexistent".to_string(),
        "password123".to_string(),
    )
    .await;
    
    match response {
        shared::AuthResponse::LoginFailed { reason } => {
            assert!(reason.contains("Invalid username or password"));
        }
        _ => panic!("Expected login to fail"),
    }
}

#[tokio::test]
async fn test_wrong_password() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    let mut session_manager = auth::SessionManager::new();
    
    // Register user
    auth::handle_register(
        &pool,
        "testuser".to_string(),
        "correctpass".to_string(),
        None,
    )
    .await;
    
    // Try to login with wrong password
    let response = auth::handle_login(
        &pool,
        &mut session_manager,
        "testuser".to_string(),
        "wrongpass".to_string(),
    )
    .await;
    
    match response {
        shared::AuthResponse::LoginFailed { reason } => {
            assert!(reason.contains("Invalid username or password"));
        }
        _ => panic!("Expected login to fail"),
    }
}

#[test]
fn test_password_hashing() {
    let password = "mypassword123";
    let hash = auth::hash_password(password).unwrap();
    
    // Verify correct password
    assert!(auth::verify_password(password, &hash).unwrap());
    
    // Verify wrong password
    assert!(!auth::verify_password("wrongpassword", &hash).unwrap());
}

#[test]
fn test_jwt_token() {
    let token = auth::create_token(123, "testuser", 24).unwrap();
    
    let claims = auth::verify_token(&token).unwrap();
    assert_eq!(claims.user_id, 123);
    assert_eq!(claims.username, "testuser");
}
