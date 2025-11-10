use sqlx::{SqlitePool, Row};
use chrono::{DateTime, Utc};
use shared::{CharacterClass, CharacterData, CharacterAppearance};

#[derive(Debug, Clone)]
pub struct Character {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub experience: i64,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub skin_color_r: f32,
    pub skin_color_g: f32,
    pub skin_color_b: f32,
    pub hair_color_r: f32,
    pub hair_color_g: f32,
    pub hair_color_b: f32,
    pub created_at: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct CharacterSummary {
    pub id: i64,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub last_played: Option<DateTime<Utc>>,
}

/// Create a new character
pub async fn create_character(
    pool: &SqlitePool,
    user_id: i64,
    character_data: &CharacterData,
) -> Result<i64, sqlx::Error> {
    let class_str = character_data.class.as_str();
    let skin_r = character_data.appearance.skin_color[0];
    let skin_g = character_data.appearance.skin_color[1];
    let skin_b = character_data.appearance.skin_color[2];
    let hair_r = character_data.appearance.hair_color[0];
    let hair_g = character_data.appearance.hair_color[1];
    let hair_b = character_data.appearance.hair_color[2];

    let result = sqlx::query(
        r#"
        INSERT INTO characters (
            user_id, name, class,
            skin_color_r, skin_color_g, skin_color_b,
            hair_color_r, hair_color_g, hair_color_b
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#
    )
    .bind(user_id)
    .bind(&character_data.name)
    .bind(class_str)
    .bind(skin_r)
    .bind(skin_g)
    .bind(skin_b)
    .bind(hair_r)
    .bind(hair_g)
    .bind(hair_b)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Get all characters for a user
pub async fn get_user_characters(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<CharacterSummary>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, name, class, level, last_played FROM characters WHERE user_id = ?1 ORDER BY last_played DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| CharacterSummary {
        id: r.get(0),
        name: r.get(1),
        class: r.get(2),
        level: r.get(3),
        last_played: r.get(4),
    }).collect())
}

/// Load character by ID
pub async fn load_character(
    pool: &SqlitePool,
    character_id: i64,
) -> Result<Option<Character>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, name, class, level, experience,
               pos_x, pos_y, pos_z,
               skin_color_r, skin_color_g, skin_color_b,
               hair_color_r, hair_color_g, hair_color_b,
               created_at, last_played
        FROM characters
        WHERE id = ?1
        "#
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Character {
        id: r.get(0),
        user_id: r.get(1),
        name: r.get(2),
        class: r.get(3),
        level: r.get(4),
        experience: r.get(5),
        pos_x: r.get(6),
        pos_y: r.get(7),
        pos_z: r.get(8),
        skin_color_r: r.get(9),
        skin_color_g: r.get(10),
        skin_color_b: r.get(11),
        hair_color_r: r.get(12),
        hair_color_g: r.get(13),
        hair_color_b: r.get(14),
        created_at: r.get(15),
        last_played: r.get(16),
    }))
}

/// Update character position
pub async fn update_position(
    pool: &SqlitePool,
    character_id: i64,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        "UPDATE characters SET pos_x = ?1, pos_y = ?2, pos_z = ?3, last_played = ?4 WHERE id = ?5"
    )
    .bind(x)
    .bind(y)
    .bind(z)
    .bind(now)
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Batch update positions for multiple characters (optimized for scalability)
pub async fn batch_save_positions(
    pool: &SqlitePool,
    positions: &[(i64, f32, f32, f32)], // (character_id, x, y, z)
) -> Result<(), sqlx::Error> {
    if positions.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    let now = Utc::now();
    
    for (char_id, x, y, z) in positions {
        sqlx::query(
            "UPDATE characters SET pos_x = ?1, pos_y = ?2, pos_z = ?3, last_played = ?4 WHERE id = ?5"
        )
        .bind(x)
        .bind(y)
        .bind(z)
        .bind(now)
        .bind(char_id)
        .execute(&mut *tx)
        .await?;
    }
    
    tx.commit().await?;
    Ok(())
}

/// Update character last played timestamp
pub async fn update_last_played(
    pool: &SqlitePool,
    character_id: i64,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE characters SET last_played = ?1 WHERE id = ?2")
        .bind(now)
        .bind(character_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete character
pub async fn delete_character(
    pool: &SqlitePool,
    character_id: i64,
    user_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM characters WHERE id = ?1 AND user_id = ?2")
        .bind(character_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Check if character name exists
pub async fn character_name_exists(
    pool: &SqlitePool,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM characters WHERE name = ?1")
        .bind(name)
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get(0);
    Ok(count > 0)
}

/// Convert DB Character to CharacterData
impl Character {
    pub fn to_character_data(&self) -> CharacterData {
        let class = match self.class.as_str() {
            "Krieger" => CharacterClass::Krieger,
            "Ninja" => CharacterClass::Ninja,
            "Sura" => CharacterClass::Sura,
            "Schamane" => CharacterClass::Schamane,
            _ => CharacterClass::Krieger, // Default to Krieger
        };

        CharacterData {
            name: self.name.clone(),
            class,
            appearance: CharacterAppearance {
                skin_color: [self.skin_color_r, self.skin_color_g, self.skin_color_b],
                hair_color: [self.hair_color_r, self.hair_color_g, self.hair_color_b],
            },
            level: self.level,
            experience: self.experience,
            specialization: None,  // TODO: Load from DB after migration
        }
    }
}

/// Update character level and experience
pub async fn update_level_and_xp(
    pool: &SqlitePool,
    character_id: i64,
    level: i32,
    experience: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE characters SET level = ?1, experience = ?2 WHERE id = ?3"
    )
    .bind(level)
    .bind(experience)
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}
