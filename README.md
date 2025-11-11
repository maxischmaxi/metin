# MMORPG - Rust & Bevy Game

Ein vollständiges MMORPG mit Rust, Bevy Engine, SQLite DB, JWT Auth, UDP Networking und **Rapier Physics Engine**.

## 🚀 Quick Start

```bash
./run_server.sh  # Port 5000
./run_client.sh  # Bevy Client
```

## 📁 Projekt-Struktur

```
server/  → Auth (JWT+bcrypt), DB (SQLite), UDP Server
client/  → Bevy 0.14, UI, 3D-Welt, Rapier Physics
shared/  → Messages, CharacterData, Skills
```

## ✅ Implementierte Features

### Core Systems
- **Auth:** Registration, Login (JWT 24h), Session Management
- **Database:** SQLite mit sqlx, Users & Characters Tabellen, Migrations
- **Networking:** UDP Client-Server (bincode), Real-time Position Updates
- **Physics:** bevy_rapier3d - Professional Collision & Gravity
- **Day/Night Cycle:** Server-controlled time system with dynamic sun movement (15min cycle) ⭐ NEW

### Character System  
- **4 Klassen:** Krieger, Ninja, Sura, Schamane (Metin2-Style)
- **8 Spezialisierungen:** 2 pro Klasse (PvM/PvP)
- **40 Skills:** 5 Skills pro Spec, freigeschaltet bei Lvl 5/10/15/25/40
- **Level 1-100:** Exponentielle XP-Kurve (100 * level^2.8)
- **Klassenspezifische Stats:** HP/Mana/Stamina pro Level unterschiedlich

### Gameplay & Physics
- **3D-Welt:** PBR Rendering, Medieval City mit 17+ Gebäuden
- **Orbit Camera:** RMB+Maus Rotation, Mausrad Zoom, Kamera-relative Bewegung
- **Day/Night Cycle:** ⭐ NEW
  - ☀️ Dynamic sun movement across the sky
  - 🌅 Realistic lighting transitions (dawn/day/dusk/night)
  - ⏰ Server-controlled time (12:00 start, 15min cycle)
  - 🎨 Time-based ambient lighting and sun intensity
  - 🌍 Visual sun sphere with emissive glow
- **Rapier Physics Engine:**
  - ✅ Realistische Gravitation (-9.81 m/s²)
  - ✅ Dynamic RigidBody für Spieler
  - ✅ Fixed RigidBody für Gebäude & Terrain
  - ✅ Automatic Collision Resolution
  - ✅ Keine Penetration durch Wände
  - ✅ Velocity-based Movement
  - ✅ Friction & Damping
  - ✅ Locked Rotation (kein Umkippen)
- **Movement:** WASD kamera-relativ, Velocity-based (5 m/s)
- **Spawn System:** Player spawnt bei Y=3.0 und fällt auf Boden
- **Free Camera:** F5 Dev-Mode, WASD+Space/Ctrl, Shift-Boost
- **NPCs:** "Meister der Künste" bei (5,1,5), 3m Interaction Range

### UI System
- **States:** Login → CharSelect → CharCreate/InGame → Paused → Settings
- **UI Stack:** Priority-basiertes Layer-Management (ESC-Key handling)
- **Nameplate:** 3D→2D Konvertierung, Level + Name über Spieler
- **Dev Tools:** F3 Panel, K-Taste (+1000 XP), +/-Level Buttons

## 🎮 Steuerung

**In-Game:**
- WASD: Bewegen (Velocity-based)
- RMB+Maus: Kamera drehen (💡 Drehe Kamera um die Sonne zu sehen!)
- Mausrad: Zoom
- K: +1000 XP (Dev)
- F1: Rapier Debug Wireframes
- F3: Dev Panel
- F5: Free Cam
- ESC: Pause Menu

**Sonne finden:** Schaue nach OBEN bei 12:00 Mittag (Serverstart)! ☀️

**NPC Interaction:**
- Linksklick auf NPC (<3m) → Dialog
- Bei Level 5+: Spezialisierung wählen (permanent!)

## 🗄️ Datenbank Schema

```sql
users: id, username(unique), password_hash, email, created_at, last_login
characters: id, user_id, name(unique), class, level, experience, 
            specialization, pos_x/y/z, skin/hair_color, created_at, last_played
```

## 🏗️ Architektur

### Client (Bevy 0.14)
```
Plugins:
- PhysicsPlugin (Rapier)        → Gravity, Collision, Physics
- PlayerPlugin                  → Movement, Spawning
- CameraPlugin                  → Orbit + Free Camera
- BuildingPlugin                → City Generation
- UIStackPlugin                 → Layer Management
- NetworkingPlugin              → UDP Communication
- InteractionPlugin             → NPC Dialogs
- NpcPlugin                     → NPC Spawning
```

### Server (Tokio Async)
```
Modules:
- auth/                         → JWT, bcrypt, Sessions
  ├─ handlers.rs               → Register/Login Logic
  ├─ jwt.rs                    → Token Creation/Validation
  ├─ password.rs               → bcrypt Hashing
  └─ session.rs                → In-Memory Session Store
- db/                          → SQLite Operations
  ├─ users.rs                  → User CRUD
  └─ characters.rs             → Character CRUD
```

### Shared
```
Messages: ClientMessage, ServerMessage
Enums: CharacterClass, Specialization, SkillId
Data: CharacterData, PlayerState
```

## 🔧 Technologie-Stack

**Core:**
- Rust 1.75+
- Bevy 0.14 (Game Engine)
- bevy_rapier3d 0.27 (Physics)
- Server-authoritative Time System (Custom)

**Server:**
- tokio (Async Runtime)
- sqlx 0.8 (Database)
- bcrypt 0.15 (Password Hashing)
- jsonwebtoken 9 (JWT)
- bincode 1.3 (Serialization)

**Client:**
- bevy_rapier3d (Physics & Collision)
- parry3d 0.17 (Convex Hull - Legacy)
- rayon (Multi-Threading - Legacy)

## 🎯 Rapier Physics Details

### Player Configuration
```rust
RigidBody::Dynamic              // Affected by gravity
Collider::capsule_y(0.75, 0.5)  // Height, Radius
Velocity::default()             // Movement via velocity
LockedAxes::ROTATION_LOCKED     // Don't tip over
GravityScale(1.0)               // Full gravity
Damping {
    linear_damping: 0.5,        // Air resistance
    angular_damping: 1.0,       // No spinning
}
Friction {
    coefficient: 0.7,           // Realistic ground friction
}
```

### Building Colliders
```rust
RigidBody::Fixed                // Static geometry
Collider::cuboid(w/2, h/2, d/2) // Box shape
```

### Physics Loop (60 FPS)
```
1. Player spawns at Y=3.0 (in air)
2. Gravity applies: velocity.y -= 9.81 * dt
3. Position updates: Y decreases
4. Collision Detection: Player vs Ground
5. Collision Resolution: Rapier stops fall
6. Player rests on ground (velocity.y = 0)
7. WASD input → velocity.x/z changes
8. Rapier handles all collisions automatically
```

## 📚 Code-Highlights

### Key Files
- `client/src/physics.rs` - Rapier Physics Plugin
- `client/src/player.rs` - Player Movement (Velocity-based)
- `client/src/building/city.rs` - 17 Buildings mit Rapier Colliders
- `client/src/camera.rs` - Orbit + Free Camera
- `client/src/skybox.rs` - Day/Night Cycle System ⭐ NEW
- `server/src/auth/` - Complete Auth System
- `server/src/db/` - Database Operations
- `shared/src/lib.rs` - 40 Skills, Network Messages, TimeUpdate

### Movement System (Velocity-based)
```rust
// Only change horizontal velocity, preserve Y for gravity
velocity.linvel.x = direction.x * speed;
velocity.linvel.z = direction.z * speed;
// velocity.linvel.y unchanged (gravity controls it)
```

### Collision Detection
```rust
// Rapier handles everything automatically:
// - Player vs Buildings → Blocked
// - Player vs Ground → Stopped
// - No tunneling, no glitches
```

## ⚙️ Build & Test

```bash
# Development
cargo build                     # Debug build
cargo build --release           # Release build

# Testing
cargo test -p server            # Server tests (19 passed)
cargo test                      # All tests

# Run
RUST_LOG=info ./run_server.sh  # Server mit Logs
./run_client.sh                 # Client
```

## 🐛 Debugging

**Physics Debug:**
- F1: Toggle Rapier Debug Wireframes (grüne Linien)

**Logs:**
```bash
RUST_LOG=debug cargo run -p client   # Alle Logs
RUST_LOG=info cargo run -p server    # Server Logs
```

## 🎯 Status

- **Kompiliert:** ✅ Client & Server Release
- **Tests:** ✅ 19/19 Auth Tests, 3/3 DB Tests
- **Physics:** ✅ Rapier Integration Complete
- **Day/Night:** ✅ Full 24h Cycle with Dynamic Lighting ⭐ NEW
- **Features:** ✅ Auth, DB, Level, Skills, Physics, NPC, Time System
- **Production:** ⚠️ JWT Secret hardcoded, keine TLS

## 🚀 Feature History

### Day/Night Cycle System (Latest - 2025-11-11)
**Feature:** Vollständiges Tag-Nacht-System mit dynamischer Sonnenbeleuchtung.

**Implementation:**
- ✅ Server-authoritative Zeit (Start: 12:00 Mittag)
- ✅ 15 Minuten Echtzeit = 24 Stunden Spielzeit (96x Speed)
- ✅ Dynamische Sonnenbewegung (Ost → Süd → West → Nord)
- ✅ Realistische Lichtübergänge (500-10000 lux)
- ✅ Sichtbare Sonne mit Emissive Material
- ✅ Zeit-basierte Ambient Light Anpassung
- ✅ 1 Hz Time-Update Broadcast vom Server

**Resultat:** Vollständig immersive Tag/Nacht-Atmosphäre mit Server-Synchronisation!

### Rapier Physics Migration (2025-11-10)
**Problem:** Custom collision system hatte Race Conditions, Frame Delays, und Tunneling-Bugs.

**Lösung:** Migration zu bevy_rapier3d
- ✅ Alle Gebäude: Fixed RigidBody + Collider
- ✅ Player: Dynamic RigidBody mit Gravity
- ✅ Movement: Velocity-based statt Transform
- ✅ Automatic Collision Resolution
- ✅ No more manual collision code

**Resultat:** 100% stabile Kollision, realistische Physik, keine Bugs mehr!

## 📝 Schnellreferenz

| Taste | Funktion | Taste | Funktion |
|-------|----------|-------|----------|
| WASD | Bewegen | K | +1000 XP |
| RMB | Kamera | F3 | Dev Panel |
| ESC | Pause | F5 | Free Cam |
| F1 | Rapier Debug | 1-5 | Skills (geplant) |

**Klassen:** Krieger (Tank) • Ninja (Agil) • Sura (Balanced) • Schamane (Healer)
**Specs:** 2 pro Klasse, wählbar ab Lvl 5 (permanent!)

## 🔮 Nächste Schritte

### Geplant
- [ ] Jump Implementation (`Space` Taste)
- [ ] Weather System (Regen, Schnee, Nebel)
- [ ] Moon & Stars bei Nacht
- [ ] Dynamische Skybox-Farben (Gradient)
- [ ] Skill System (1-5 Hotkeys)
- [ ] Monster Spawning
- [ ] Combat System
- [ ] Inventory & Items
- [ ] Multiplayer Synchronisation

### Einfach hinzuzufügen (Rapier)
```rust
// Jump
if keyboard.just_pressed(KeyCode::Space) && is_grounded() {
    velocity.linvel.y = 8.0;  // Jump force
}

// Grounded Check
let is_grounded = velocity.linvel.y.abs() < 0.1;
```

## 📄 Dokumentation

Für detaillierte Informationen siehe:
- `DAY_NIGHT_CYCLE.md` - Vollständige Tag/Nacht-System Dokumentation ⭐ NEW
- `DAYNIGHT_QUICKSTART.md` - Quick-Start Guide für Tag/Nacht-Zyklus ⭐ NEW
- `AGENTS.md` - Vollständige Entwickler-Dokumentation (Legacy, archiviert)
- Inline Code-Kommentare in allen wichtigen Systemen

## 🏆 Credits

- **Engine:** Bevy 0.14
- **Physics:** bevy_rapier3d (Rapier Physics Engine)
- **Inspiration:** Metin2 (Klassen, Skills, Gameplay)

---

**Version:** 0.5.0 (Day/Night Cycle)
**Last Updated:** 2025-11-11
**Status:** ✅ Playable Alpha mit Physik-Engine und dynamischem Tag/Nacht-System
