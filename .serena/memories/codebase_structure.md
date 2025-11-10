# Codebase Structure

## Project Layout (Monorepo)
```
game/
├── server/          # Game Server (UDP, Auth, Database)
├── client/          # Bevy Game Client
├── shared/          # Shared Data Structures
├── target/          # Build artifacts (ignored)
├── game.db          # SQLite database (auto-generated)
└── Cargo.toml       # Workspace configuration
```

## Server Structure (server/)
```
server/
├── src/
│   ├── main.rs              # UDP server loop, game logic
│   ├── lib.rs               # Library exports
│   ├── auth/                # Authentication module
│   │   ├── mod.rs           # Auth exports
│   │   ├── handlers.rs      # Register/Login handlers (~160 lines)
│   │   ├── jwt.rs           # JWT token creation/validation (~80 lines)
│   │   ├── password.rs      # bcrypt hashing (~60 lines)
│   │   └── session.rs       # Session management (~130 lines)
│   └── db/                  # Database module
│       ├── mod.rs           # DB initialization, migrations
│       ├── users.rs         # User CRUD operations
│       └── characters.rs    # Character CRUD operations
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_characters.sql
│   └── 003_add_specialization.sql
├── tests/
│   ├── auth_test.rs         # 19 auth tests
│   └── db_test.rs           # 3 database tests
└── Cargo.toml
```

## Client Structure (client/)
```
client/
├── src/
│   ├── main.rs              # Entry point, GameStates, Plugin registration
│   ├── auth_state.rs        # AuthState resource (~35 lines)
│   ├── networking.rs        # UDP client, message handling
│   ├── camera.rs            # Orbit camera system
│   ├── player.rs            # Player movement, spawning, world setup
│   ├── npc.rs               # NPC spawning, nameplate system (~200 lines)
│   ├── interaction.rs       # NPC interaction, raycast (~120 lines)
│   ├── collision.rs         # Auto-collider system (~1850 lines)
│   └── ui/                  # UI modules
│       ├── mod.rs           # UI exports, button system, color constants
│       ├── login.rs         # Login/Register screen (~380 lines)
│       ├── character_selection.rs  # Character selection UI
│       ├── character_creation.rs   # Character creation UI
│       ├── game_ui.rs       # In-game UI (health bars, XP, dev tools)
│       ├── pause.rs         # Pause menu
│       ├── settings.rs      # Settings menu (graphics, audio)
│       ├── npc_dialog.rs    # NPC dialog system (~250 lines)
│       └── ui_stack.rs      # UI layer priority management (~150 lines)
├── assets/
│   └── fonts/               # Game fonts (momo, coolvetica, lemon_milk, you_idiot)
└── Cargo.toml
```

## Shared Structure (shared/)
```
shared/
├── src/
│   └── lib.rs               # All shared types
│       ├── Messages (ClientMessage, ServerMessage)
│       ├── CharacterClass enum (4 classes)
│       ├── Specialization enum (8 specs)
│       ├── SkillId enum (40 skills)
│       ├── CharacterData struct
│       ├── CharacterSummary struct
│       ├── MMOSettings struct
│       └── Helper functions (XP calculation, stats calculation)
└── Cargo.toml
```

## Key File Descriptions

### server/src/main.rs
- GameServer struct with UDP socket
- Main game loop (60 FPS)
- Message routing (handle_client_message)
- Handlers: Auth, CreateCharacter, SelectCharacter, ChooseSpecialization, GainExperience
- Session cleanup (every 60s)

### client/src/main.rs
- GameState enum definition
- Plugin registration order (important: UIStackPlugin before other UI plugins)
- Resource initialization (AuthState, NetworkClient, MMOSettings, etc.)

### client/src/collision.rs (1850 lines)
- AutoCollider system with 3 detail levels (Low, Medium, High)
- CollisionLOD for distance-based optimization
- CollisionCache for 35% performance improvement
- Visual debugging (F1-F4 wireframe rendering)
- parry3d integration for High detail Quickhull

### client/src/ui/ui_stack.rs (150 lines)
- UILayerStack resource for priority-based UI management
- UILayerType enum (GameUI, PauseMenu, Settings, NpcDialog)
- Centralized ESC key handler
- Prevents UI conflicts (e.g., ESC closes dialog before opening pause)

### shared/src/lib.rs
- Network protocol definitions (all ClientMessage and ServerMessage variants)
- Character system (4 classes, 8 specializations, 40 skills)
- Helper functions:
  - `calculate_xp_for_level(level)` - Exponential XP curve
  - `calculate_stats_for_level(level, class)` - Class-specific stats
  - Specialization helpers (name, description, skills, etc.)

## Important Patterns

### Plugin Pattern
Each major feature is a Bevy plugin:
- `LoginPlugin`, `CharacterSelectionPlugin`, `CharacterCreationPlugin`
- `GameUIPlugin`, `PausePlugin`, `SettingsPlugin`
- `PlayerPlugin`, `CameraPlugin`, `NetworkingPlugin`
- `NpcPlugin`, `InteractionPlugin`, `NpcDialogPlugin`
- `CollisionPlugin`, `UIStackPlugin`

### State-Based Systems
Systems run only in specific GameStates:
```rust
.add_systems(OnEnter(GameState::Login), setup_login)
.add_systems(OnExit(GameState::Login), cleanup_login)
.add_systems(Update, login_input.run_if(in_state(GameState::Login)))
```

### Marker Components for Cleanup
```rust
#[derive(Component)]
struct GameWorld;  // Marks all game entities
#[derive(Component)]
struct LoginUI;    // Marks login UI entities
```
Used in cleanup systems to despawn all entities of a type.

## File Size Reference
- Large files (>1000 lines): collision.rs (1850)
- Medium files (200-500 lines): login.rs (380), npc_dialog.rs (250), npc.rs (200)
- Most files: 100-200 lines
- Small helpers: <100 lines
