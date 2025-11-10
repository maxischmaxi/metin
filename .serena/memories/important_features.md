# Important Features & Systems

## 1. Character System

### Classes (4)
- **Krieger** (Warrior): Tank, +20 HP/level, low mana
- **Ninja** (Rogue): Agile, +15 stamina/level, high mobility
- **Sura**: Balanced magic warrior, +12 mana/level
- **Schamane** (Shaman): Healer, +18 mana/level, support

### Specializations (8)
Each class has 2 specializations (PvM vs PvP):
- Krieger: Leibwächter (PvM Tank) / Gladiator (PvP Damage)
- Ninja: Bogenschütze (Ranged) / Attentäter (Melee)
- Sura: Dämonen-Jäger (PvM) / Blutkrieger (PvP)
- Schamane: Lebenshüter (Support) / Sturmrufer (PvP Damage)

### Skills (40 total)
5 skills per specialization, unlocked at levels 5, 10, 15, 25, 40.
All 40 skills defined in `shared/src/lib.rs` with SkillInfo (name, description, cooldown, mana cost, effects).

### Level System
- Levels 1-100
- Exponential XP curve: `100 * level^2.8`
- Total XP to level 100: ~9.5 million
- Class-specific stat gains per level
- DB columns: level, experience (in characters table)

## 2. Authentication System

### JWT Tokens
- Secret: "your-secret-key" (⚠️ hardcoded - change for production!)
- Expiry: 24 hours
- Algorithm: HS256
- Claims: user_id, username, exp, iat

### Password Security
- bcrypt hashing with cost 8
- Never stored in plaintext
- Server-side validation
- Minimum 8 characters

### Session Management
- In-memory session store
- SessionData includes: user_id, username, character_id, token, timestamps
- Automatic cleanup every 60 seconds
- Token validation on all protected endpoints

## 3. Database Schema

### users table
- id (PK, AUTOINCREMENT)
- username (UNIQUE, NOT NULL)
- password_hash (NOT NULL)
- email (nullable)
- created_at, last_login (timestamps)
- Index on username

### characters table
- id (PK, AUTOINCREMENT)
- user_id (FK to users)
- name (UNIQUE, NOT NULL)
- class (TEXT: Krieger, Ninja, Sura, Schamane)
- level (default 1)
- experience (default 0)
- specialization (TEXT, nullable until level 5)
- pos_x, pos_y, pos_z (position in world)
- skin_color_r/g/b, hair_color_r/g/b (appearance)
- created_at, last_played (timestamps)
- Indices on user_id, name, specialization

### Migrations
- 001_create_users.sql
- 002_create_characters.sql
- 003_add_specialization.sql
- Run automatically on server startup

## 4. Collision System

### Auto-Collider
3 detail levels with automatic generation from mesh:
- **Low:** AABB/Sphere, <10µs, 60-70% accuracy
- **Medium:** Simplified hull, ~150µs, 85-90% accuracy
- **High:** parry3d Quickhull, ~500µs, 95-98% accuracy

### Performance Optimizations
- **CollisionCache:** 35% faster detection
- **CollisionLOD:** Distance-based detail switching (2-3x faster with many objects)
- Spatial partitioning
- Multi-threading with rayon

### Visual Debugging
- F1: Toggle debug mode
- F2: Toggle AABB boxes
- F3: Toggle collision shapes
- F4: Toggle cache spheres
- Color-coded by detail (Green=Low, Yellow=Medium, Red=High, Blue=Manual)

## 5. NPC System

### NPC Spawning
- "Meister der Künste" at position (5, 1, 5)
- Golden capsule model
- Nameplate system (same as player)
- Glow effect when player within 3m

### Interaction
- Left mouse click to interact
- Raycast from camera to NPC (sphere-ray intersection)
- Global interaction range: 3.0 meters
- Opens NPC dialog on successful interaction

### Specialization Choice
- Available at level 5+
- Shows 2 specialization options based on character class
- Permanent choice (stored in database)
- Dialog shows different messages based on level and existing specialization

## 6. UI Stack System

### Priority-Based Layer Management
- GameUI (100): Base in-game UI
- PauseMenu (200): Pause overlay
- Settings (250): Settings overlay
- NpcDialog (300): Highest priority overlay

### ESC Key Handling
Centralized handler in `ui_stack.rs`:
- Closes topmost layer first (LIFO)
- Prevents UI conflicts
- Example: ESC with dialog open → closes dialog (not pause menu)

## 7. Camera System

### Orbit Camera
- Follows player automatically
- Right mouse button + movement: Rotate camera
- Mouse wheel: Zoom (2.0 - 20.0 units)
- Pitch limit: -1.5 to 1.5 radians
- Camera-relative WASD movement

### Camera Persistence
- SavedCameraState resource
- Saved on exit from InGame state
- Restored on enter to InGame state
- Prevents camera reset when opening/closing pause menu

## 8. Network Protocol

### Client → Server Messages
- Auth (Login, Register)
- CreateCharacter, SelectCharacter, DeleteCharacter
- ChooseSpecialization
- GainExperience (dev command)
- Join, Move, Disconnect (planned)

### Server → Client Messages
- AuthResponse (LoginSuccess/Failed, RegisterSuccess/Failed)
- CharacterCreated/Failed
- CharacterSelected/Failed
- SpecializationChosen/Failed
- ExperienceGained, LevelUp
- PlayerJoined/Left/Moved, WorldState (planned)

### Transport
- UDP on port 5000
- bincode serialization
- Non-blocking server loop (60 FPS)
- Background thread for client message reception

## 9. Dev Tools

### In-Game Commands
- **K key:** +1000 XP (level up testing)
- **F3:** Toggle dev panel
- **F5:** Toggle free camera
- **F1-F4:** Collision debugging

### Dev Panel (F3)
- Compact 2x2 button grid
- +Lvl: Add one level
- -Lvl: Remove one level
- +1K: Add 1000 XP
- →1: Reset to level 1

## 10. Important Resources

### Client Resources
- **AuthState:** Token, username, character list, selected character
- **PlayerStats:** HP, mana, stamina, level, XP, character name
- **NetworkClient:** UDP socket, incoming message queue
- **MMOSettings:** Graphics (vsync, fullscreen), Audio (volumes)
- **SavedCameraState:** Camera position persistence
- **UILayerStack:** UI layer priority management
- **NpcDialogState:** Active NPC dialog state

### GameStates
- Login
- CharacterSelection
- CharacterCreation
- InGame
- Paused
- Settings
