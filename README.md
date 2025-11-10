# MMORPG - Rust & Bevy Game

Ein vollständiges MMORPG mit Rust, Bevy Engine, SQLite DB, JWT Auth und UDP Networking.

## 🚀 Quick Start

```bash
./run_server.sh  # Port 5000
./run_client.sh  # Bevy Client
```

## 📁 Struktur

```
server/  → Auth (JWT+bcrypt), DB (SQLite), UDP Server
client/  → Bevy 0.14, UI, 3D-Welt, Collision
shared/  → Messages, CharacterData, Skills
```

## ✅ Implementierte Features

### Core Systems
- **Auth:** Registration, Login (JWT 24h), Session Management
- **Database:** SQLite mit sqlx, Users & Characters Tabellen, Migrations
- **Networking:** UDP Client-Server (bincode), Real-time Updates

### Character System  
- **4 Klassen:** Krieger, Ninja, Sura, Schamane (Metin2-Style)
- **8 Spezialisierungen:** 2 pro Klasse (PvM/PvP)
- **40 Skills:** 5 Skills pro Spec, freigeschaltet bei Lvl 5/10/15/25/40
- **Level 1-100:** Exponentielle XP-Kurve (100 * level^2.8)
- **Klassenspezifische Stats:** HP/Mana/Stamina pro Level unterschiedlich

### Gameplay
- **3D-Welt:** PBR Rendering, Orbit Camera (RMB+Maus, Mausrad Zoom)
- **Movement:** WASD kamera-relativ, Shift-Sprint
- **Collision:** Auto-Collider System mit 3 Detail-Levels (Low/Medium/High)
  - Performance Cache (35% faster)
  - LOD System (2-3x faster mit vielen Objekten)
  - Visual Debug (F1-F4 Wireframes)
- **Free Camera:** F5 Dev-Mode, WASD+Space/Ctrl, Shift-Boost
- **NPCs:** "Meister der Künste" bei (5,1,5), 3m Interaction Range

### UI System
- **States:** Login → CharSelect → CharCreate/InGame → Paused → Settings
- **UI Stack:** Priority-basiertes Layer-Management (ESC-Key handling)
- **Nameplate:** 3D→2D Konvertierung, Level + Name über Spieler
- **Dev Tools:** F3 Panel, K-Taste (+1000 XP), +/-Level Buttons

## 🎮 Steuerung

**In-Game:**
- WASD: Bewegen | RMB+Maus: Kamera | Mausrad: Zoom
- K: +1000 XP (Dev) | F3: Dev Panel | F5: Free Cam
- ESC: Pause Menu | F1-F4: Collision Debug

**NPC Interaction:**
- Linksklick auf NPC (<3m) → Dialog
- Bei Level 5+: Spezialisierung wählen (permanent!)

## 🗄️ Datenbank Schema

```sql
users: id, username(unique), password_hash, email, created_at
characters: id, user_id, name(unique), class, level, experience, 
            specialization, pos_x/y/z, skin/hair_color, created_at
```

## 🔧 Entwicklung

**Dependencies:**
- Bevy 0.14, sqlx 0.8 (SQLite), bcrypt 0.15, jsonwebtoken 9
- parry3d 0.17 (Collision), rayon (Parallelization)

**Architecture:**
- Client: ECS mit Plugins (Player, Camera, UI, Collision, Interaction)
- Server: Async Tokio, Auth Module, DB Module
- Shared: Messages (ClientMessage, ServerMessage), Enums (CharacterClass, Specialization, SkillId)

**Key Files:**
- `client/src/collision.rs` (1850 lines) - Auto-Collider System
- `server/src/auth/` - Password, JWT, Session, Handlers
- `server/src/db/` - Users, Characters CRUD
- `shared/src/lib.rs` - 40 Skills, 8 Specs, Messages

## 📚 Dokumentation (Archiviert)

**Aktuelle Systeme:** Siehe AGENTS.md (vollständige Projekt-Dokumentation)

**Legacy Docs (für Details):**
- DATABASE_PHASE1-3.md - DB Setup & Auth
- LEVEL_SYSTEM.md - XP-Kurve & Stats
- SKILL_SYSTEM_DESIGN.md - Alle 40 Skills detailliert
- COLLISION_README.md - Auto-Collider API
- NPC_IMPLEMENTATION_SUMMARY.md - NPC System
- UI_STACK_* - UI Layer Management
- SPECIALIZATION_QUICKSTART.md - Spec-Wahl Anleitung

## ⚙️ Build & Test

```bash
cargo build --release           # Beide
cargo test -p server            # Server Tests (19 passed)
RUST_LOG=info ./run_server.sh  # Mit Logs
```

**Windows:** Siehe WINDOWS_BUILD.md

## 🎯 Status

- **Kompiliert:** ✅ Client & Server Release
- **Tests:** ✅ 19/19 Auth Tests, 3/3 DB Tests
- **Features:** ✅ Auth, DB, Level, Skills, Collision, NPC
- **Production:** ⚠️ JWT Secret hardcoded, keine TLS

## 📝 Schnellreferenz

| Taste | Funktion | Taste | Funktion |
|-------|----------|-------|----------|
| WASD | Bewegen | K | +1000 XP |
| RMB | Kamera | F3 | Dev Panel |
| ESC | Pause | F5 | Free Cam |
| F1-F4 | Collision Debug | 1-5 | Skills (geplant) |

**Klassen:** Krieger (Tank) • Ninja (Agil) • Sura (Balanced) • Schamane (Healer)
**Specs:** 2 pro Klasse, wählbar ab Lvl 5 (permanent!)
