-- Create characters table
CREATE TABLE IF NOT EXISTS characters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT UNIQUE NOT NULL,
    class TEXT NOT NULL,
    level INTEGER DEFAULT 1,
    experience INTEGER DEFAULT 0,
    
    -- Position
    pos_x REAL DEFAULT 0.0,
    pos_y REAL DEFAULT 1.0,
    pos_z REAL DEFAULT 0.0,
    
    -- Appearance
    skin_color_r REAL DEFAULT 1.0,
    skin_color_g REAL DEFAULT 0.8,
    skin_color_b REAL DEFAULT 0.6,
    hair_color_r REAL DEFAULT 0.3,
    hair_color_g REAL DEFAULT 0.2,
    hair_color_b REAL DEFAULT 0.1,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_played TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_characters_user ON characters(user_id);
CREATE INDEX IF NOT EXISTS idx_characters_name ON characters(name);
