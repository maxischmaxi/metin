use server::db;

#[tokio::test]
async fn test_database_initialization() {
    // Test DB initialization
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    
    // Verify pool is working
    let result = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_user_operations() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    
    // Create user
    let user_id = db::users::create_user(
        &pool,
        "testuser",
        "hashed_password",
        Some("test@example.com"),
    )
    .await
    .unwrap();
    
    assert!(user_id > 0);
    
    // Find user by username
    let user = db::users::find_by_username(&pool, "testuser")
        .await
        .unwrap();
    
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    
    // Check username exists
    let exists = db::users::username_exists(&pool, "testuser")
        .await
        .unwrap();
    assert!(exists);
    
    let not_exists = db::users::username_exists(&pool, "nonexistent")
        .await
        .unwrap();
    assert!(!not_exists);
}

#[tokio::test]
async fn test_character_operations() {
    let pool = db::init_database("sqlite::memory:").await.unwrap();
    
    // Create user first
    let user_id = db::users::create_user(&pool, "testuser", "hash", None)
        .await
        .unwrap();
    
    // Create character
    use shared::{CharacterData, CharacterClass, CharacterAppearance};
    
    let char_data = CharacterData {
        name: "TestHero".to_string(),
        class: CharacterClass::Warrior,
        appearance: CharacterAppearance::default(),
    };
    
    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();
    
    assert!(char_id > 0);
    
    // Get user characters
    let characters = db::characters::get_user_characters(&pool, user_id)
        .await
        .unwrap();
    
    assert_eq!(characters.len(), 1);
    assert_eq!(characters[0].name, "TestHero");
    assert_eq!(characters[0].class, "Warrior");
    
    // Load character
    let loaded = db::characters::load_character(&pool, char_id)
        .await
        .unwrap();
    
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.name, "TestHero");
    assert_eq!(loaded.class, "Warrior");
}
