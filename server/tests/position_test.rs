use server::db;
use shared::{CharacterData, CharacterClass, CharacterAppearance};

/// Helper function to create test database
async fn setup_test_db() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Run migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_login TIMESTAMP
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT UNIQUE NOT NULL,
            class TEXT NOT NULL,
            level INTEGER DEFAULT 1,
            experience INTEGER DEFAULT 0,
            pos_x REAL DEFAULT 0.0,
            pos_y REAL DEFAULT 1.0,
            pos_z REAL DEFAULT 0.0,
            skin_color_r REAL DEFAULT 1.0,
            skin_color_g REAL DEFAULT 0.8,
            skin_color_b REAL DEFAULT 0.6,
            hair_color_r REAL DEFAULT 0.3,
            hair_color_g REAL DEFAULT 0.2,
            hair_color_b REAL DEFAULT 0.1,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_played TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

/// Helper to create a test user
async fn create_test_user(pool: &sqlx::SqlitePool, username: &str) -> i64 {
    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?1, ?2)")
        .bind(username)
        .bind("test_hash")
        .execute(pool)
        .await
        .unwrap();
    
    result.last_insert_rowid()
}

#[tokio::test]
async fn test_batch_save_positions_single() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    // Create a character
    let char_data = CharacterData {
        name: "TestHero".to_string(),
        class: CharacterClass::Warrior,
        appearance: CharacterAppearance::default(),
    };

    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();

    // Verify initial position is default
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(character.pos_x, 0.0);
    assert_eq!(character.pos_y, 1.0);
    assert_eq!(character.pos_z, 0.0);

    // Batch save new position
    let positions = vec![(char_id, 10.0, 2.0, 15.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    // Verify position was saved
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(character.pos_x, 10.0);
    assert_eq!(character.pos_y, 2.0);
    assert_eq!(character.pos_z, 15.0);
}

#[tokio::test]
async fn test_batch_save_positions_multiple() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    // Create multiple characters
    let char1_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "Hero1".to_string(),
            class: CharacterClass::Warrior,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    let char2_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "Hero2".to_string(),
            class: CharacterClass::Mage,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    let char3_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "Hero3".to_string(),
            class: CharacterClass::Rogue,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Batch save all positions in one transaction
    let positions = vec![
        (char1_id, 10.0, 1.0, 20.0),
        (char2_id, 30.0, 1.0, 40.0),
        (char3_id, 50.0, 1.0, 60.0),
    ];

    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    // Verify all positions were saved correctly
    let char1 = db::characters::load_character(&pool, char1_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char1.pos_x, 10.0);
    assert_eq!(char1.pos_z, 20.0);

    let char2 = db::characters::load_character(&pool, char2_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char2.pos_x, 30.0);
    assert_eq!(char2.pos_z, 40.0);

    let char3 = db::characters::load_character(&pool, char3_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char3.pos_x, 50.0);
    assert_eq!(char3.pos_z, 60.0);
}

#[tokio::test]
async fn test_batch_save_positions_empty() {
    let pool = setup_test_db().await;
    
    // Empty batch should not error
    let positions = vec![];
    let result = db::characters::batch_save_positions(&pool, &positions).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_position_persistence_across_loads() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    // Create character
    let char_data = CharacterData {
        name: "PersistHero".to_string(),
        class: CharacterClass::Warrior,
        appearance: CharacterAppearance::default(),
    };

    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();

    // Simulate movement: Update position multiple times
    let positions = vec![(char_id, 5.0, 1.0, 5.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    // Load and verify
    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char.pos_x, 5.0);
    assert_eq!(char.pos_z, 5.0);

    // Move again
    let positions = vec![(char_id, 25.0, 1.0, 35.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    // Reload and verify new position
    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char.pos_x, 25.0);
    assert_eq!(char.pos_z, 35.0);

    // Simulate logout/login by reloading
    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char.pos_x, 25.0);
    assert_eq!(char.pos_y, 1.0);
    assert_eq!(char.pos_z, 35.0);
}

#[tokio::test]
async fn test_update_position_single_call() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    let char_data = CharacterData {
        name: "UpdateHero".to_string(),
        class: CharacterClass::Mage,
        appearance: CharacterAppearance::default(),
    };

    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();

    // Use the single update_position function
    db::characters::update_position(&pool, char_id, 100.0, 2.0, 200.0)
        .await
        .unwrap();

    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(char.pos_x, 100.0);
    assert_eq!(char.pos_y, 2.0);
    assert_eq!(char.pos_z, 200.0);
}

#[tokio::test]
async fn test_position_updates_last_played() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    let char_data = CharacterData {
        name: "TimeHero".to_string(),
        class: CharacterClass::Rogue,
        appearance: CharacterAppearance::default(),
    };

    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();

    // Initial last_played should be None
    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert!(char.last_played.is_none());

    // Update position - should set last_played
    db::characters::update_position(&pool, char_id, 5.0, 1.0, 10.0)
        .await
        .unwrap();

    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert!(char.last_played.is_some());
}

#[tokio::test]
async fn test_batch_save_large_batch() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    // Create 100 characters
    let mut char_ids = Vec::new();
    for i in 0..100 {
        let char_id = db::characters::create_character(
            &pool,
            user_id,
            &CharacterData {
                name: format!("Hero{}", i),
                class: CharacterClass::Warrior,
                appearance: CharacterAppearance::default(),
            },
        )
        .await
        .unwrap();
        char_ids.push(char_id);
    }

    // Batch save all 100 positions
    let positions: Vec<_> = char_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i as f32 * 10.0, 1.0, i as f32 * 5.0))
        .collect();

    let start = std::time::Instant::now();
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();
    let elapsed = start.elapsed();

    println!("Batch saved 100 positions in {:?}", elapsed);

    // Verify random samples
    let char_0 = db::characters::load_character(&pool, char_ids[0])
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char_0.pos_x, 0.0);

    let char_50 = db::characters::load_character(&pool, char_ids[50])
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char_50.pos_x, 500.0);
    assert_eq!(char_50.pos_z, 250.0);

    let char_99 = db::characters::load_character(&pool, char_ids[99])
        .await
        .unwrap()
        .unwrap();
    assert_eq!(char_99.pos_x, 990.0);
    assert_eq!(char_99.pos_z, 495.0);

    // Batch save should be fast (< 1 second for 100 updates)
    assert!(elapsed.as_millis() < 1000);
}

#[tokio::test]
async fn test_negative_positions() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    let char_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "NegativeHero".to_string(),
            class: CharacterClass::Warrior,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Save negative positions (e.g., walking backwards from spawn)
    let positions = vec![(char_id, -50.0, 1.0, -100.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(char.pos_x, -50.0);
    assert_eq!(char.pos_z, -100.0);
}

#[tokio::test]
async fn test_float_precision() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    let char_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "PrecisionHero".to_string(),
            class: CharacterClass::Mage,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Save position with high precision
    let positions = vec![(char_id, 123.456789, 1.234567, 987.654321)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();

    let char = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    // SQLite REAL has limited precision, but should be close
    assert!((char.pos_x - 123.456789).abs() < 0.0001);
    assert!((char.pos_y - 1.234567).abs() < 0.0001);
    assert!((char.pos_z - 987.654321).abs() < 0.0001);
}
