# Code Style & Conventions

## Language & Comments
- **UI Text:** German (Deutsche Texte in allen UI-Komponenten)
- **Code Comments:** English
- **Variable/Function Names:** English, snake_case

## Rust Conventions

### Naming
- **Functions/Methods:** `snake_case`
- **Types/Structs/Enums:** `PascalCase`
- **Constants:** `SCREAMING_SNAKE_CASE`
- **Components:** `PascalCase` (Bevy convention)
- **Systems:** `snake_case` (Bevy convention)
- **Resources:** `PascalCase` (Bevy convention)

### Examples
```rust
// ✅ Good
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub struct AuthState { ... }
pub enum GameState { ... }
fn setup_login(...) { ... }
fn handle_auth_response(...) { ... }

// ❌ Bad
pub const normalButton: Color = ...;
pub struct auth_state { ... }
fn SetupLogin(...) { ... }
```

## Bevy-Specific Patterns

### State-Based UI
Each screen is a separate plugin with OnEnter/OnExit systems:
```rust
pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Login), setup_login)
            .add_systems(OnExit(GameState::Login), cleanup_login)
            .add_systems(Update, login_systems.run_if(in_state(GameState::Login)));
    }
}
```

### Component Markers
Use marker components for entity cleanup:
```rust
#[derive(Component)]
struct LoginUI;  // Marks all UI entities for this screen

fn cleanup(mut commands: Commands, query: Query<Entity, With<LoginUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

### Query Conflict Avoidance
Separate systems when multiple systems need mutable access:
```rust
// ✅ Good: Separate systems
fn update_health(mut query: Query<&mut Text, With<HealthDisplay>>) { ... }
fn update_mana(mut query: Query<&mut Text, With<ManaDisplay>>) { ... }

// Or use Without filter:
fn update_displays(
    mut health: Query<&mut Text, (With<HealthDisplay>, Without<ManaDisplay>)>,
    mut mana: Query<&mut Text, (With<ManaDisplay>, Without<HealthDisplay>)>,
) { ... }
```

### Resource for State Management
```rust
#[derive(Resource, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
    // ...
}
```

## Database Conventions

### Migrations
- Numbered sequentially: `001_create_users.sql`, `002_create_characters.sql`
- Always include indices for foreign keys
- Use `AUTOINCREMENT` for primary keys

### SQL Style
```sql
-- ✅ Good: Readable, explicit
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_users_username ON users(username);
```

## Error Handling

### Logging
```rust
// Use log macros
info!("Server started on {}", addr);
warn!("Token validation failed");
error!("Database connection failed: {}", err);
```

### Result Types
```rust
// Use ? operator, propagate errors
pub async fn create_user(pool: &SqlitePool, username: &str) -> Result<i64, sqlx::Error> {
    let user = sqlx::query!(/* ... */).fetch_one(pool).await?;
    Ok(user.id)
}
```

## UI Styling Constants

All UI colors defined in `client/src/ui/mod.rs`:
```rust
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
```

## German UI Text Examples
```rust
// Buttons
"Einloggen", "Registrieren", "Erstellen", "Zurück", "Spiel beenden"

// Messages
"✓ Registrierung erfolgreich!"
"Benutzername muss mindestens 3 Zeichen haben"
"Wähle deine Spezialisierung"

// Labels
"Benutzername:", "Passwort:", "Einstellungen", "Grafik", "Audio"
```

## Documentation
- Complex systems: Detailed comments in English
- Important behaviors: Comment with ⭐ WICHTIG or ⭐ KRITISCH
- Lessons learned: Document in separate .md files
