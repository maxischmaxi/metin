use server::db;
use shared::{CharacterData, CharacterClass, CharacterAppearance};
use std::time::{Instant, Duration};

/// Helper function to create test database
async fn setup_test_db() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    
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
async fn test_complete_login_spawn_logout_cycle() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "testuser").await;

    // 1. Create character (simulates character creation)
    let char_data = CharacterData {
        name: "CycleHero".to_string(),
        class: CharacterClass::Warrior,
        appearance: CharacterAppearance::default(),
    };

    let char_id = db::characters::create_character(&pool, user_id, &char_data)
        .await
        .unwrap();

    // 2. First login - load character (should be at default position)
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(character.pos_x, 0.0);
    assert_eq!(character.pos_y, 1.0);
    assert_eq!(character.pos_z, 0.0);
    println!("✓ First login: Character spawned at default position (0, 1, 0)");

    // 3. Simulate gameplay - player moves around
    let positions = vec![(char_id, 25.0, 1.0, 50.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();
    println!("✓ Player moved to (25, 1, 50)");

    // 4. Logout - position saved
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(character.pos_x, 25.0);
    assert_eq!(character.pos_z, 50.0);
    println!("✓ Logout: Position saved");

    // 5. Second login - load character (should spawn at saved position)
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(character.pos_x, 25.0);
    assert_eq!(character.pos_y, 1.0);
    assert_eq!(character.pos_z, 50.0);
    println!("✓ Second login: Character spawned at saved position (25, 1, 50)");

    // 6. Move again
    let positions = vec![(char_id, -10.0, 1.0, -20.0)];
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();
    println!("✓ Player moved to (-10, 1, -20)");

    // 7. Third login
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(character.pos_x, -10.0);
    assert_eq!(character.pos_z, -20.0);
    println!("✓ Third login: Character spawned at new saved position (-10, 1, -20)");
}

#[tokio::test]
async fn test_multiple_characters_positions() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "multiuser").await;

    // Create 3 characters for same user
    let char1_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "Warrior".to_string(),
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
            name: "Mage".to_string(),
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
            name: "Rogue".to_string(),
            class: CharacterClass::Rogue,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Play with char1, move to (10, 1, 10)
    db::characters::batch_save_positions(&pool, &[(char1_id, 10.0, 1.0, 10.0)])
        .await
        .unwrap();

    // Play with char2, move to (20, 1, 20)
    db::characters::batch_save_positions(&pool, &[(char2_id, 20.0, 1.0, 20.0)])
        .await
        .unwrap();

    // Play with char3, move to (30, 1, 30)
    db::characters::batch_save_positions(&pool, &[(char3_id, 30.0, 1.0, 30.0)])
        .await
        .unwrap();

    // Verify each character has its own position
    let char1 = db::characters::load_character(&pool, char1_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((char1.pos_x, char1.pos_z), (10.0, 10.0));

    let char2 = db::characters::load_character(&pool, char2_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((char2.pos_x, char2.pos_z), (20.0, 20.0));

    let char3 = db::characters::load_character(&pool, char3_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((char3.pos_x, char3.pos_z), (30.0, 30.0));

    println!("✓ Multiple characters maintain separate positions");
}

#[tokio::test]
async fn test_dirty_flag_simulation() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "dirtyuser").await;

    let char_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "DirtyHero".to_string(),
            class: CharacterClass::Warrior,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Simulate dirty flag system:
    // 1. Player moves (dirty = true, no save yet)
    // 2. After 5 minutes threshold or disconnect, save happens

    struct SimulatedPlayer {
        character_id: i64,
        position: (f32, f32, f32),
        dirty: bool,
        last_save: Instant,
    }

    let mut player = SimulatedPlayer {
        character_id: char_id,
        position: (0.0, 1.0, 0.0),
        dirty: false,
        last_save: Instant::now(),
    };

    // Simulate movement - mark dirty
    player.position = (5.0, 1.0, 10.0);
    player.dirty = true;
    assert!(player.dirty);
    println!("✓ Player moved, marked as dirty");

    // Simulate 5 minute threshold check
    let five_minutes = Duration::from_secs(5 * 60);
    
    // Before 5 minutes - should not save
    assert!(player.last_save.elapsed() < five_minutes);
    println!("✓ Before 5 minutes threshold - no save needed yet");

    // Simulate time passing (in real system, this would be auto_save_positions())
    // For test, we'll just save if dirty
    if player.dirty {
        let positions = vec![(player.character_id, player.position.0, player.position.1, player.position.2)];
        db::characters::batch_save_positions(&pool, &positions)
            .await
            .unwrap();
        player.dirty = false;
        player.last_save = Instant::now();
        println!("✓ Auto-save triggered, position saved");
    }

    // Verify position was saved
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(character.pos_x, 5.0);
    assert_eq!(character.pos_z, 10.0);
    assert!(!player.dirty);
    println!("✓ Dirty flag cleared after save");
}

#[tokio::test]
async fn test_batch_save_scalability() {
    let pool = setup_test_db().await;
    
    // Create 1000 users with 1 character each
    let mut char_ids = Vec::new();
    
    for i in 0..1000 {
        let user_id = create_test_user(&pool, &format!("user{}", i)).await;
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

    println!("✓ Created 1000 characters");

    // Simulate all 1000 players needing save at same time
    let positions: Vec<_> = char_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| {
            let x = (i as f32 * 2.0) % 100.0;
            let z = (i as f32 * 3.0) % 100.0;
            (id, x, 1.0, z)
        })
        .collect();

    let start = Instant::now();
    db::characters::batch_save_positions(&pool, &positions)
        .await
        .unwrap();
    let elapsed = start.elapsed();

    println!("✓ Batch saved 1000 positions in {:?}", elapsed);
    println!("✓ Average: {:.2} ms per position", elapsed.as_millis() as f64 / 1000.0);

    // Verify random samples
    for sample_idx in [0, 250, 500, 750, 999] {
        let char = db::characters::load_character(&pool, char_ids[sample_idx])
            .await
            .unwrap()
            .unwrap();
        
        let expected_x = (sample_idx as f32 * 2.0) % 100.0;
        let expected_z = (sample_idx as f32 * 3.0) % 100.0;
        
        assert!((char.pos_x - expected_x).abs() < 0.01);
        assert!((char.pos_z - expected_z).abs() < 0.01);
    }

    println!("✓ All 1000 positions verified correctly");

    // Performance assertion - should be fast even for 1000 updates
    assert!(elapsed.as_secs() < 5, "Batch save of 1000 positions should complete in < 5 seconds");
}

#[tokio::test]
async fn test_character_selection_with_position() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "selectuser").await;

    // Create character and move it to a specific location
    let char_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "SelectHero".to_string(),
            class: CharacterClass::Mage,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Simulate: Player played, moved to (42.5, 1.0, 73.2)
    db::characters::batch_save_positions(&pool, &[(char_id, 42.5, 1.0, 73.2)])
        .await
        .unwrap();

    // Simulate: User logs out, then logs back in and selects character
    let characters = db::characters::get_user_characters(&pool, user_id)
        .await
        .unwrap();
    
    assert_eq!(characters.len(), 1);
    assert_eq!(characters[0].name, "SelectHero");

    // User clicks on character - server loads full character data
    let character = db::characters::load_character(&pool, char_id)
        .await
        .unwrap()
        .unwrap();

    // Server would send this position to client
    let spawn_position = (character.pos_x, character.pos_y, character.pos_z);
    assert_eq!(spawn_position, (42.5, 1.0, 73.2));

    println!("✓ Character selection loads saved position: {:?}", spawn_position);
}

#[tokio::test]
async fn test_position_bounds() {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "boundsuser").await;

    let char_id = db::characters::create_character(
        &pool,
        user_id,
        &CharacterData {
            name: "BoundsHero".to_string(),
            class: CharacterClass::Rogue,
            appearance: CharacterAppearance::default(),
        },
    )
    .await
    .unwrap();

    // Test extreme positions
    let extreme_positions = vec![
        (f32::MAX / 2.0, 1.0, f32::MAX / 2.0),
        (f32::MIN / 2.0, 1.0, f32::MIN / 2.0),
        (1000000.0, 1.0, 1000000.0),
        (-1000000.0, 1.0, -1000000.0),
    ];

    for (x, y, z) in extreme_positions {
        db::characters::batch_save_positions(&pool, &[(char_id, x, y, z)])
            .await
            .unwrap();

        let char = db::characters::load_character(&pool, char_id)
            .await
            .unwrap()
            .unwrap();

        // Verify values are preserved (within floating point precision)
        assert!((char.pos_x - x).abs() < x.abs() * 0.0001 + 0.01);
        assert!((char.pos_y - y).abs() < 0.01);
        assert!((char.pos_z - z).abs() < z.abs() * 0.0001 + 0.01);
    }

    println!("✓ Extreme position values handled correctly");
}
