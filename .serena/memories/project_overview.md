# MMORPG Game - Project Overview

## Purpose
Ein vollständiges MMORPG mit Rust und Bevy Engine. Client-Server-Architektur mit Authentifizierung, Charakterverwaltung, 3D-Gameplay, Level-System (1-100), 4 Klassen, 8 Spezialisierungen, 40 Skills, NPC-System und vollständigem Collision-System.

## Tech Stack

### Core
- **Language:** Rust Edition 2021
- **Engine:** Bevy 0.14 (ECS-basiert)
- **Database:** SQLite mit sqlx 0.8
- **Network:** UDP mit bincode Serialisierung
- **Authentication:** JWT (jsonwebtoken 9) + bcrypt 0.15

### Server Dependencies
- tokio 1.0 (async runtime, full features)
- sqlx 0.8 (SQLite, runtime-tokio-native-tls)
- bcrypt 0.15 (password hashing, cost: 8)
- jsonwebtoken 9 (HS256, 24h expiry)
- chrono 0.4 (timestamps)
- log 0.4, env_logger 0.11

### Client Dependencies
- bevy 0.14 (default features)
- parry3d 0.17 (collision detection, Quickhull)
- rayon (multi-threading für collision)
- serde 1.0, bincode 1.3

## Architecture

### Monorepo Structure
```
game/
├── server/   - UDP Server, Auth (JWT+bcrypt), DB (SQLite), Game Logic
├── client/   - Bevy ECS, 3D Rendering, UI, Collision, Networking
└── shared/   - Messages, Enums, Common Data Structures
```

### Key Systems
1. **Auth System:** JWT tokens (24h), bcrypt password hashing, session management
2. **Database:** SQLite, migrations, users & characters tables
3. **Networking:** UDP client-server, bincode serialization
4. **Character System:** 4 classes (Krieger, Ninja, Sura, Schamane), 8 specializations, 40 skills
5. **Level System:** Level 1-100, exponential XP curve (100 * level^2.8)
6. **Collision System:** Auto-collider with 3 detail levels, LOD, visual debugging (F1-F4)
7. **NPC System:** Interaction system, dialogs, specialization choice at level 5
8. **UI Stack:** Priority-based layer management, ESC key handling

## Project Status
- ✅ Fully functional auth, database, networking
- ✅ Complete UI system (Login, CharacterSelection, CharacterCreation, InGame, Paused, Settings)
- ✅ 3D world with camera system (orbit, zoom, camera-relative movement)
- ✅ Collision detection (Auto-Collider, LOD, performance cache)
- ✅ Level & XP system
- ✅ NPC interaction
- ⚠️ Production warnings: JWT secret hardcoded, no TLS
