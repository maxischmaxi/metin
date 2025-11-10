# MMORPG Entwicklungs-Dokumentation

## Projekt-Übersicht

Ein vollständiges MMORPG mit Rust und Bevy Engine, bestehend aus Client-Server-Architektur, Authentifizierung, Charakterverwaltung und 3D-Gameplay.

### Projekt-Struktur (Monorepo)

```
game/
├── server/          # Game Server (UDP, Authentifizierung, Datenbank)
│   ├── src/
│   │   ├── main.rs           # Server Haupt-Loop
│   │   ├── lib.rs            # Server Library
│   │   ├── auth/             # Authentifizierungs-Systeme
│   │   │   ├── mod.rs        # Auth Modul
│   │   │   ├── handlers.rs   # Register/Login Handler
│   │   │   ├── jwt.rs        # JWT Token Erstellung/Validierung
│   │   │   ├── password.rs   # bcrypt Password Hashing
│   │   │   └── session.rs    # Session Management
│   │   └── db/               # Datenbank-Operationen
│   │       ├── mod.rs        # DB Initialisierung & Migrationen
│   │       ├── users.rs      # User CRUD
│   │       └── characters.rs # Character CRUD
│   ├── migrations/           # SQL Migrations
│   │   ├── 001_create_users.sql
│   │   └── 002_create_characters.sql
│   └── tests/               # Server Tests
│       ├── auth_test.rs
│       └── db_test.rs
│
├── client/          # Bevy Game Client
│   └── src/
│       ├── main.rs           # Client Entry Point & GameStates
│       ├── auth_state.rs     # Auth State Management
│       ├── networking.rs     # UDP Client & Message Handling
│       ├── camera.rs         # Orbit Camera System
│       ├── player.rs         # Player Movement & World
│       └── ui/               # UI Systeme
│           ├── mod.rs        # UI Exports & Button System
│           ├── login.rs      # Login/Register Screen
│           ├── character_selection.rs  # Charakter Auswahl
│           ├── character_creation.rs   # Charakter Erstellung
│           ├── game_ui.rs    # In-Game UI
│           ├── pause.rs      # Pause-Menü
│           └── settings.rs   # Settings-Menü
│
├── shared/          # Gemeinsame Datenstrukturen
│   └── src/
│       └── lib.rs   # Messages, Character, Settings
│
├── Cargo.toml       # Workspace Configuration
├── game.db          # SQLite Datenbank (auto-generiert)
├── run_server.sh    # Server Startup Script
└── run_client.sh    # Client Startup Script
```

---

## 🎯 Abgeschlossene Features

### Phase 1: Datenbank-Grundlage ✅

**Technologie:** SQLite mit sqlx

**Dateien:**
- `server/migrations/001_create_users.sql` - Users Tabelle
- `server/migrations/002_create_characters.sql` - Characters Tabelle
- `server/src/db/mod.rs` - DB Init mit Migrations
- `server/src/db/users.rs` - User CRUD Operationen
- `server/src/db/characters.rs` - Character CRUD Operationen

**Datenbank-Schema:**

```sql
-- Users Tabelle
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
);
CREATE INDEX idx_users_username ON users(username);

-- Characters Tabelle
CREATE TABLE characters (
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
    FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE INDEX idx_characters_user_id ON characters(user_id);
CREATE INDEX idx_characters_name ON characters(name);
```

**Wichtige Funktionen:**
- `init_database()` - Erstellt DB-Datei + Parent-Verzeichnis, führt Migrations aus
- `create_user()` - Erstellt User mit Validierung
- `find_by_username()` - Findet User für Login
- `create_character()` - Erstellt Character mit Appearance
- `get_user_characters()` - Lädt alle Characters eines Users
- `load_character()` - Lädt vollständige Character-Daten
- `delete_character()` - Löscht Character (nur wenn Owner)

**Tests:** 3/3 passed (in `server/tests/db_test.rs`)

---

### Phase 2: Authentifizierungs-System ✅

**Technologie:** bcrypt + JWT

**Dateien:**
- `server/src/auth/password.rs` - bcrypt Hashing (Cost: 8)
- `server/src/auth/jwt.rs` - JWT Token (24h Expiry, HS256)
- `server/src/auth/session.rs` - In-Memory Session Management
- `server/src/auth/handlers.rs` - Register/Login Logik

**JWT Secret:** `"your-secret-key"` (Hardcoded - für Produktion ändern!)

**Session-Daten:**
```rust
pub struct SessionData {
    pub user_id: i64,
    pub username: String,
    pub character_id: Option<i64>,  // Aktuell ausgewählter Character
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,  // 24 Stunden
}
```

**Wichtige Funktionen:**
- `hash_password(password)` -> String (bcrypt)
- `verify_password(password, hash)` -> bool
- `create_token(user_id, username)` -> String (JWT)
- `handle_register()` - User registrieren (dup check, bcrypt)
- `handle_login()` - User einloggen (verify password, create session)
- `cleanup_expired()` - Entfernt abgelaufene Sessions (läuft alle 60s)

**Tests:** 19/19 passed (in `server/tests/auth_test.rs`)

---

### Phase 3: Client-UI System ✅

**Technologie:** Bevy UI (egui-style)

**GameStates:**
```rust
pub enum GameState {
    Login,              // Login/Register Screen
    CharacterSelection, // Charakter Auswahl
    CharacterCreation,  // Charakter Erstellen
    InGame,            // Im Spiel
    Paused,            // Pause-Menü (ESC im Spiel)
    Settings,          // Einstellungen
}
```

**UI-Komponenten:**

#### 1. Login-Screen (`client/src/ui/login.rs`)

**Features:**
- Deutscher Text überall
- Dynamischer Submit-Button: "Einloggen" ↔ "Registrieren"
- Tab-Navigation zwischen Feldern
- Input-Cursor nur in aktivem Feld
- Validierung: Username ≥3, Password ≥8 Zeichen
- Status-Nachrichten mit Farben (grün=Erfolg, rot=Fehler)
- Nach Registrierung: Automatisch zurück zum Login

**Input-Felder:**
- Username (immer sichtbar)
- Password (immer sichtbar, maskiert mit *)
- Email (nur bei Registrierung, optional)

**Keyboard-Shortcuts:**
- Tab = Feld wechseln
- Enter = Submit
- Backspace = Zeichen löschen
- Buchstaben/Zahlen/Punkt/Minus

**System-Details:**
- `LoginState` Resource speichert Input
- Separate Systeme für Text-Updates (vermeidet Query-Konflikte):
  - `update_input_display()` - Input-Felder
  - `update_submit_button_text()` - Submit-Button
  - `update_status_display()` - Status-Nachrichten
  - `handle_auth_response_ui()` - Server-Antworten

#### 2. Charakter-Auswahl (`client/src/ui/character_selection.rs`)

**Features:**
- Zeigt alle Characters des Users als klickbare Karten
- Hover-Effekt: Blauer Rahmen → Hellblau beim Hover → Grün beim Click
- Character-Info: Name, Klasse, Level, Zuletzt gespielt
- "Klicken um diesen Charakter zu spielen" Hinweis

**Buttons:**
- Character-Karten (klickbar) → SelectCharacter Message → InGame
- "+ Neuen Charakter erstellen" (grün) → CharacterCreation
- "Ausloggen" → Login (AuthState.logout())
- "Spiel beenden" (rot) → Exit

**System-Details:**
- `SelectionButton::SelectCharacter(character_id)` speichert ID
- `character_card_hover_system()` für Border-Color Änderungen
- Sendet `ClientMessage::SelectCharacter` an Server
- Speichert selected_character_id in AuthState

#### 3. Charakter-Erstellung (`client/src/ui/character_creation.rs`)

**Features:**
- Namenseingabe (Standard: "Hero", max 20 Zeichen)
- Klassen-Buttons: Krieger, Magier, Schurke
- Anzeige der gewählten Klasse
- Blinkender Cursor im Namensfeld

**Buttons:**
- Klassen-Buttons → Wählt Klasse
- "Erstellen ✓" (grün) → Sendet CreateCharacter
- "← Zurück" → CharacterSelection

**Character-Daten:**
```rust
CharacterData {
    name: String,
    class: CharacterClass,  // Warrior, Mage, Rogue
    appearance: CharacterAppearance {
        skin_color: [f32; 3],
        hair_color: [f32; 3],
    }
}
```

#### 4. Pause-Menü (`client/src/ui/pause.rs`) ⭐ NEU

**Zugriff:** ESC im InGame-State

**Features:**
- Halbtransparenter Hintergrund (alpha: 0.95)
- Großer "Pause" Titel
- "Drücke ESC um fortzufahren" Hinweis

**Buttons:**
- "Weiterspielen" (grün) → InGame
- "Einstellungen" → Settings
- "Zum Hauptmenü" → CharacterSelection (Spieler bleibt!)
- "Ausloggen" → Login (Welt wird gelöscht)
- "Spiel beenden" (rot) → Exit

**Navigation:**
- ESC im Spiel → Pause-Menü
- ESC im Pause-Menü → Zurück ins Spiel

#### 5. Settings-Menü (`client/src/ui/settings.rs`) ⭐ NEU

**Zugriff:** "Einstellungen" im Pause-Menü

**Grafik-Einstellungen:**
- VSync: AN/AUS (ändert `window.present_mode` sofort)
- Vollbild: AN/AUS (ändert `window.mode` sofort)

**Audio-Einstellungen:**
- Gesamtlautstärke: 0-100% (±10% pro Click)
- Musik: 0-100%
- Soundeffekte: 0-100%

**Settings Resource:**
```rust
MMOSettings {
    graphics: GraphicsSettings {
        vsync: bool,
        fullscreen: bool,
        resolution: (u32, u32),  // 1280x720
    },
    audio: AudioSettings {
        master_volume: f32,  // 0.0 - 1.0
        music_volume: f32,
        sfx_volume: f32,
    }
}
```

**Buttons:**
- Toggle-Buttons (AN/AUS)
- +/- Buttons für Lautstärke
- "← Zurück" → Paused

**Navigation:**
- ESC im Settings → Pause-Menü
- Settings bleiben persistent während Session

**Update-System:**
- `update_setting_displays()` aktualisiert alle Displays bei Änderung
- Echtzeit-Feedback für alle Einstellungen

---

### Phase 4: Netzwerk-Integration ✅

**Technologie:** UDP mit bincode Serialisierung

**Dateien:**
- `client/src/networking.rs` - UDP Client
- `shared/src/lib.rs` - Message Protokoll
- `server/src/main.rs` - UDP Server

**Server-Adresse:** `127.0.0.1:5000`

**Netzwerk-Protokoll:**

```rust
// Client → Server Messages
pub enum ClientMessage {
    // Authentifizierung
    Auth(AuthMessage),
    
    // Character Management
    CreateCharacter { token: String, character: CharacterData },
    SelectCharacter { token: String, character_id: i64 },
    DeleteCharacter { token: String, character_id: i64 },
    
    // Gameplay (noch nicht voll implementiert)
    Join { character: CharacterData },
    Move { direction: Vec3 },
    Disconnect,
}

// Server → Client Messages
pub enum ServerMessage {
    // Auth Responses
    AuthResponse(AuthResponse),
    
    // Character Responses
    CharacterCreated { character_id: i64 },
    CharacterCreationFailed { reason: String },
    CharacterSelected { character_id: i64 },
    CharacterSelectionFailed { reason: String },
    CharacterDeleted { character_id: i64 },
    CharacterDeletionFailed { reason: String },
    
    // Gameplay (noch nicht implementiert)
    PlayerJoined { id: u64, character: CharacterData, position: Vec3 },
    PlayerLeft { id: u64 },
    PlayerMoved { id: u64, position: Vec3 },
    WorldState { players: Vec<PlayerState> },
}

pub enum AuthResponse {
    LoginSuccess { token: String, characters: Vec<CharacterSummary> },
    LoginFailed { reason: String },
    RegisterSuccess,
    RegisterFailed { reason: String },
}
```

**Client-Netzwerk-System:**

```rust
// NetworkClient Resource
pub struct NetworkClient {
    socket: Arc<Mutex<UdpSocket>>,  // Thread-safe Socket
    incoming_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
    server_addr: String,  // "127.0.0.1:5000"
}
```

**Wichtige Systeme:**
- `listen_for_messages()` - Background Thread für Empfang
- `process_incoming_messages()` - Verarbeitet Messages zu Events
- `handle_auth_responses()` - Auth Event → State Transitions

**Events:**
```rust
pub struct AuthResponseEvent(pub AuthResponse);

pub enum CharacterResponseEvent {
    Created { character_id: i64 },
    CreationFailed { reason: String },
    Selected { character_id: i64 },
    SelectionFailed { reason: String },
    Deleted { character_id: i64 },
    DeletionFailed { reason: String },
}
```

**Server Message-Handling:**

```rust
// In server/src/main.rs GameServer
async fn handle_client_message(&mut self, client_addr: SocketAddr, message: ClientMessage) {
    match message {
        ClientMessage::Auth(auth_msg) => self.handle_auth_message(),
        ClientMessage::CreateCharacter { token, character } => {
            // 1. Token validieren
            // 2. Character-Name prüfen (unique)
            // 3. Character in DB erstellen
            // 4. Response senden
        },
        ClientMessage::SelectCharacter { token, character_id } => {
            // 1. Token validieren
            // 2. Character ownership prüfen
            // 3. Session aktualisieren
            // 4. Response senden
        },
        // ... weitere Handler
    }
}
```

**Server Update Loop:**
- 60 FPS (~16ms pro Frame)
- Non-blocking UDP Receive
- Session Cleanup alle 60s

---

### Phase 5: 3D-Welt & Gameplay ✅

**Technologie:** Bevy 3D mit PBR Rendering

#### Kamera-System (`client/src/camera.rs`)

**Orbit Camera Features:**
- Folgt Spieler automatisch
- Rechte Maustaste + Mausbewegung = Rotation
- Mausrad = Zoom (2.0 - 20.0 Einheiten)
- Pitch-Limit: -1.5 bis 1.5 Radians

**Kamera-Persistenz:** ⭐ WICHTIG
```rust
// SavedCameraState Resource
struct SavedCameraState {
    camera: Option<OrbitCamera>,  // Gespeichert beim Verlassen von InGame
}

pub struct OrbitCamera {
    pub focus: Vec3,     // Wo die Kamera hinschaut (Spieler-Position)
    pub radius: f32,     // Zoom-Distanz
    pub pitch: f32,      // Vertikal-Rotation
    pub yaw: f32,        // Horizontal-Rotation
}
```

**Wichtiges Detail:**
- Beim Wechsel von InGame → Paused: Kamera-State wird gespeichert
- Beim Wechsel von Paused → InGame: Kamera-State wird wiederhergestellt
- **Verhindert:** Kamera-Reset beim ESC → Weiterspielen

**Systeme:**
- `save_camera_state()` - OnExit(InGame)
- `switch_to_3d_camera()` - OnEnter(InGame), lädt SavedState
- `orbit_camera_mouse()` - Rotation
- `orbit_camera_zoom()` - Zoom
- `update_camera_focus()` - Folgt Spieler

#### Spieler-System (`client/src/player.rs`)

**Spieler-Komponente:**
```rust
pub struct Player {
    pub speed: f32,  // 5.0 Einheiten/Sekunde
}
```

**GameWorld Marker:** ⭐ WICHTIG
```rust
#[derive(Component)]
struct GameWorld;  // Markiert alle Spiel-Entities
```

**Alle spawned Entities:**
- Spieler (Kapsel: 0.5 Radius, 1.5 Höhe, blau)
- Boden (50x50 Ebene, grün)
- Umgebungs-Objekte (7x7 Gitter, Würfel, braun)
- Licht (DirectionalLight mit Schatten)

**Wichtiges Detail - Spawn-Verhalten:** ⭐ KRITISCH

```rust
fn setup_player(..., player_query: Query<&Player>) {
    // NUR spawnen wenn noch kein Spieler existiert!
    if player_query.is_empty() {
        // Spawn Spieler + Welt
    }
}

// Cleanup beim Verlassen
fn cleanup_player(...) {
    // Löscht alle Entities mit GameWorld Marker
}
```

**Warum wichtig:**
- Verhindert doppelte Spieler bei ESC → Weiterspielen
- OnEnter(InGame) wird mehrfach aufgerufen
- Ohne Check: Jedes Mal neuer Spieler!

**Cleanup-Trigger:**
- OnEnter(CharacterSelection) - User wählt anderen Character
- OnEnter(Login) - User loggt aus

#### Bewegungs-System - Kamera-Relativ ⭐ WICHTIG

**WASD-Steuerung:**
```rust
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
    camera_query: Query<&OrbitCamera>,
) {
    let camera_yaw = camera_query.get_single().map(|cam| cam.yaw).unwrap_or(0.0);
    
    // Input in Kamera-Raum sammeln
    let mut input_direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { input_direction.z -= 1.0; } // Forward
    if keyboard.pressed(KeyCode::KeyS) { input_direction.z += 1.0; } // Backward
    if keyboard.pressed(KeyCode::KeyA) { input_direction.x -= 1.0; } // Left
    if keyboard.pressed(KeyCode::KeyD) { input_direction.x += 1.0; } // Right
    
    if input_direction.length() > 0.0 {
        input_direction = input_direction.normalize();
        
        // Rotiere Input mit Kamera-Yaw → Welt-Richtung
        let rotation = Quat::from_rotation_y(camera_yaw);
        let world_direction = rotation * input_direction;
        
        // Bewegung in Welt-Raum
        let movement = world_direction * player.speed * time.delta_seconds();
        transform.translation += movement;
        
        // Spieler dreht sich in Bewegungsrichtung
        let target_rotation = Quat::from_rotation_y(
            world_direction.x.atan2(-world_direction.z)
        );
        transform.rotation = target_rotation;
    }
}
```

**Wie es funktioniert:**
1. Kamera schaut nach Osten (90°)
2. W drücken → Input: (0, 0, -1)
3. Mit Yaw 90° rotieren → World: (1, 0, 0)
4. Spieler läuft nach Osten
5. Spieler-Kapsel dreht sich nach Osten

**Steuerung:**
- WASD = Bewegung relativ zur Kamera
- Rechte Maustaste = Kamera drehen
- Mausrad = Zoom
- ESC = Pause-Menü

---

## 🗂️ AuthState Management

**Client-seitige Resource:**

```rust
#[derive(Resource, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
    pub characters: Vec<CharacterSummary>,
    pub selected_character_id: Option<i64>,
}

impl AuthState {
    pub fn login(&mut self, token: String, username: String, characters: Vec<CharacterSummary>)
    pub fn logout(&mut self)  // Löscht alles
    pub fn select_character(&mut self, character_id: i64)
    pub fn get_selected_character(&self) -> Option<&CharacterSummary>
    pub fn is_authenticated(&self) -> bool
    pub fn get_token(&self) -> Option<&str>
}
```

**Verwendung:**
- Nach Login: `auth_state.login(token, username, characters)`
- Charakter wählen: `auth_state.select_character(id)`
- Logout: `auth_state.logout()` → Löscht Token + Characters + Selection
- Character-Requests: `auth_state.get_token()` für Authorization

---

## 🔧 Wichtige Bevy-Konzepte im Projekt

### States
```rust
#[derive(States)]
pub enum GameState { ... }

// Systems laufen nur in bestimmten States
.add_systems(Update, player_movement.run_if(in_state(GameState::InGame)))
.add_systems(OnEnter(GameState::Login), setup_login)
.add_systems(OnExit(GameState::Login), cleanup_login)
```

### Resources
```rust
#[derive(Resource)]
pub struct AuthState { ... }

// Init im App-Builder
app.init_resource::<AuthState>()

// Verwendung in Systems
fn my_system(auth: Res<AuthState>) { ... }
fn my_mut_system(mut auth: ResMut<AuthState>) { ... }
```

### Events
```rust
#[derive(Event)]
pub struct AuthResponseEvent(pub AuthResponse);

// Register
app.add_event::<AuthResponseEvent>()

// Write
fn sender(mut events: EventWriter<AuthResponseEvent>) {
    events.send(AuthResponseEvent(...));
}

// Read
fn receiver(mut events: EventReader<AuthResponseEvent>) {
    for event in events.read() { ... }
}
```

### Components
```rust
#[derive(Component)]
pub struct Player { pub speed: f32 }

// Spawn mit Component
commands.spawn((
    PbrBundle { ... },
    Player { speed: 5.0 },
));

// Query
fn system(query: Query<(&Transform, &Player)>) {
    for (transform, player) in query.iter() { ... }
}
```

### Query-Konflikte vermeiden

**Problem:**
```rust
// ❌ FEHLER: Beide greifen auf Text zu!
fn bad_system(
    mut query1: Query<&mut Text, With<DisplayA>>,
    mut query2: Query<&mut Text, With<DisplayB>>,
) { ... }
```

**Lösung 1 - Separate Systems:**
```rust
fn system1(mut query: Query<&mut Text, With<DisplayA>>) { ... }
fn system2(mut query: Query<&mut Text, With<DisplayB>>) { ... }
```

**Lösung 2 - Without Filter:**
```rust
fn system(
    mut query1: Query<&mut Text, (With<DisplayA>, Without<DisplayB>)>,
    mut query2: Query<&mut Text, (With<DisplayB>, Without<DisplayA>)>,
) { ... }
```

---

## 🐛 Behobene Bugs & Lessons Learned

### 1. Doppelte Spieler beim ESC → Weiterspielen

**Problem:**
- `setup_player()` wird bei jedem `OnEnter(InGame)` aufgerufen
- ESC → Settings → Zurück triggert OnEnter nochmal
- Resultat: Jedes Mal ein neuer Spieler

**Lösung:**
```rust
fn setup_player(..., player_query: Query<&Player>) {
    if player_query.is_empty() {
        // Nur spawnen wenn noch kein Spieler da
    }
}
```

**Zusatz:**
- `GameWorld` Marker für alle Spiel-Entities
- `cleanup_player()` bei Login/CharacterSelection

### 2. Kamera-Reset beim Pause-Menü

**Problem:**
- Kamera wird gelöscht bei InGame → Paused
- Neue Kamera mit Default-Werten bei Paused → InGame

**Lösung:**
```rust
#[derive(Resource, Default)]
struct SavedCameraState {
    camera: Option<OrbitCamera>,
}

// OnExit(InGame)
fn save_camera_state() { ... }

// OnEnter(InGame)
fn switch_to_3d_camera(saved_state: Res<SavedCameraState>) {
    let orbit = saved_state.camera.clone().unwrap_or_default();
    // Restore position
}
```

### 3. Query-Konflikte im Login-System

**Problem:**
```rust
Query<&mut Text, With<StatusDisplay>>,
Query<&mut Text, With<SubmitButtonText>>,
// ❌ Beide wollen &mut Text!
```

**Lösung:**
- Separate Systems für verschiedene Displays
- Status-Updates über Resource-Changes
```rust
fn update_status_display(login_state: Res<LoginState>, mut query: Query<&mut Text, With<StatusDisplay>>) {
    if login_state.is_changed() { ... }
}
```

### 4. Registrierung ging direkt zum Login

**Problem:**
- Nach RegisterSuccess wurde User automatisch eingeloggt
- Verwirrend für Nutzer

**Lösung:**
```rust
AuthResponse::RegisterSuccess => {
    login_state.status_message = "✓ Registrierung erfolgreich! Du kannst dich jetzt einloggen.";
    login_state.is_register_mode = false;  // Wechsle zu Login-Mode
    login_state.password.clear();  // Sicherheit
    
    // Verstecke E-Mail Feld
    for mut style in register_fields_query.iter_mut() {
        style.display = Display::None;
    }
}
```

### 5. Cursor blinkt in allen Input-Feldern

**Problem:**
- Cursor blinkte in allen Feldern gleichzeitig
- Auch in nicht-fokussierten Feldern

**Lösung:**
```rust
let is_active = login_state.active_field == InputField::Username;
let is_empty = login_state.username.is_empty();

text.sections[0].value = if is_empty {
    if is_active {
        format!("Benutzername eingeben{}", cursor)  // Mit Cursor
    } else {
        "Benutzername eingeben".to_string()  // Ohne Cursor
    }
} else {
    if is_active {
        format!("{}{}", login_state.username, cursor)
    } else {
        login_state.username.clone()
    }
};
```

---

## 📊 Netzwerk-Ablauf Beispiele

### User Registration Flow

```
Client                          Server                      Database
  |                               |                             |
  | 1. Auth(Register)            |                             |
  |----------------------------->|                             |
  |                              | 2. Check username exists    |
  |                              |---------------------------->|
  |                              |<----------------------------|
  |                              | 3. Hash password (bcrypt)   |
  |                              | 4. INSERT INTO users        |
  |                              |---------------------------->|
  |                              |<----------------------------|
  | 5. RegisterSuccess           |                             |
  |<-----------------------------|                             |
  |                              |                             |
  | UI: "✓ Registrierung erfolgreich!"                         |
  | Wechsel zu Login-Mode                                      |
```

### Login Flow

```
Client                          Server                      Database
  |                               |                             |
  | 1. Auth(Login)               |                             |
  |----------------------------->|                             |
  |                              | 2. SELECT * FROM users      |
  |                              |---------------------------->|
  |                              |<----------------------------|
  |                              | 3. verify_password()        |
  |                              | 4. create_token() (JWT)     |
  |                              | 5. create session           |
  |                              | 6. SELECT characters        |
  |                              |---------------------------->|
  |                              |<----------------------------|
  | 7. LoginSuccess{token, chars}|                             |
  |<-----------------------------|                             |
  |                              |                             |
  | AuthState.login()                                          |
  | GameState → CharacterSelection                             |
```

### Character Creation Flow

```
Client                          Server                      Database
  |                               |                             |
  | 1. CreateCharacter{token,data}|                            |
  |----------------------------->|                             |
  |                              | 2. Validate token           |
  |                              | 3. Check name unique        |
  |                              |---------------------------->|
  |                              |<----------------------------|
  |                              | 4. INSERT INTO characters   |
  |                              |---------------------------->|
  |                              |<----------------------------|
  | 5. CharacterCreated{id}      |                             |
  |<-----------------------------|                             |
  |                              |                             |
  | Event → CharacterResponseEvent::Created                    |
  | (Noch keine automatische Transition implementiert)         |
```

### Character Selection Flow

```
Client                          Server                      Database
  |                               |                             |
  | 1. SelectCharacter{token,id} |                             |
  |----------------------------->|                             |
  |                              | 2. Validate token           |
  |                              | 3. Load character           |
  |                              |---------------------------->|
  |                              |<----------------------------|
  |                              | 4. Verify ownership         |
  |                              | 5. session.set_character(id)|
  | 6. CharacterSelected{id}     |                             |
  |<-----------------------------|                             |
  |                              |                             |
  | AuthState.select_character(id)                             |
  | GameState → InGame                                         |
```

---

## 🎨 UI-Styling Konstanten

```rust
// In client/src/ui/mod.rs

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Hintergrundfarben
const BG_DARK: Color = Color::srgb(0.1, 0.1, 0.15);
const BG_ELEMENT: Color = Color::srgb(0.15, 0.15, 0.2);

// Text-Farben
const TEXT_WHITE: Color = Color::WHITE;
const TEXT_YELLOW: Color = Color::srgb(1.0, 1.0, 0.4);  // Input-Text
const TEXT_GRAY: Color = Color::srgb(0.6, 0.6, 0.6);    // Hints
const TEXT_GREEN: Color = Color::srgb(0.3, 1.0, 0.3);   // Success
const TEXT_RED: Color = Color::srgb(1.0, 0.3, 0.3);     // Error

// Button-Farben
const BUTTON_GREEN: Color = Color::srgb(0.2, 0.6, 0.2);  // Primary Actions
const BUTTON_RED: Color = Color::srgb(0.5, 0.1, 0.1);    // Dangerous Actions

// Border-Farben
const BORDER_BLUE: Color = Color::srgb(0.4, 0.6, 0.8);   // Input Fields
const BORDER_HOVER: Color = Color::srgb(0.6, 0.8, 1.0);  // Hover
const BORDER_ACTIVE: Color = Color::srgb(0.2, 0.8, 0.2); // Active/Pressed
```

---

## 🚀 Wie man das Spiel startet

### Server Starten

```bash
cd /home/max/code/game
./run_server.sh

# Oder manuell:
RUST_LOG=info cargo run --release -p server
```

**Server-Output:**
```
INFO Database initialized successfully
INFO Migration 001_create_users completed
INFO Migration 002_create_characters completed
INFO All migrations completed successfully
INFO Server started on 127.0.0.1:5000
INFO Game server running. Press Ctrl+C to stop.
```

### Client Starten

```bash
cd /home/max/code/game
./run_client.sh

# Oder manuell:
cargo run --release -p client
```

### Test-Ablauf

1. **Registrierung:**
   - Username: `testuser` (min. 3 Zeichen)
   - Password: `testpass123` (min. 8 Zeichen)
   - Email: Optional
   - Click "Neuen Account erstellen"
   - Warte auf "✓ Registrierung erfolgreich!"
   - Formular wechselt automatisch zu Login

2. **Login:**
   - Username: `testuser`
   - Password: `testpass123`
   - Click "Einloggen"
   - → Charakterauswahl

3. **Charakter Erstellen:**
   - Click "+ Neuen Charakter erstellen"
   - Name: z.B. "Gandalf"
   - Klasse wählen: Krieger/Magier/Schurke
   - Click "Erstellen ✓"
   - → Zurück zur Charakterauswahl

4. **Charakter Wählen:**
   - Click auf Charakter-Karte
   - → Im Spiel

5. **Im Spiel:**
   - WASD = Bewegen
   - Rechte Maustaste + Mausbewegung = Kamera drehen
   - Mausrad = Zoom
   - ESC = Pause-Menü

6. **Pause-Menü:**
   - "Weiterspielen" → Zurück ins Spiel
   - "Einstellungen" → Settings-Menü
   - "Zum Hauptmenü" → Charakterauswahl
   - "Ausloggen" → Login
   - "Spiel beenden" → Exit

7. **Settings:**
   - VSync AN/AUS
   - Vollbild AN/AUS
   - Lautstärke +/-
   - "← Zurück" → Pause-Menü

---

## 📝 TODOs & Bekannte Einschränkungen

### Nicht Implementiert / Unvollständig

1. **Multiplayer-Synchronisation**
   - Andere Spieler werden nicht angezeigt
   - PlayerJoined/PlayerLeft Messages definiert aber nicht verwendet
   - WorldState Updates nicht implementiert

2. **Character Selection nach Creation**
   - Nach Charakter-Erstellung: Kein automatischer Übergang
   - Muss manuell zurück → Charakter auswählen

3. **Token-Persistenz**
   - Token geht beim Client-Neustart verloren
   - Kein "Remember Me" / Auto-Login
   - Könnte in lokaler Datei gespeichert werden

4. **Character Deletion**
   - UI-Button existiert nicht
   - Backend DeleteCharacter ist implementiert
   - Könnte in Charakterauswahl hinzugefügt werden

5. **Error-Handling in UI**
   - CharacterCreationFailed wird nicht angezeigt
   - Nur im Log sichtbar
   - Sollte Status-Message in UI zeigen

6. **Audio-System**
   - Lautstärke-Einstellungen haben keine Wirkung
   - Kein Audio implementiert
   - Settings werden gespeichert aber nicht verwendet

7. **Erweiterte Character-Customization**
   - Skin/Hair Color wird nicht im UI angezeigt
   - Nur Default-Werte werden verwendet
   - Könnte Farbauswahl hinzugefügt werden

8. **Character Stats/Inventory**
   - Level/XP werden nicht verwendet
   - Kein Inventar-System
   - Keine Items

9. **Position-Persistenz**
   - Character-Position wird nicht gespeichert beim Logout
   - Immer Spawn bei (0, 1, 0)
   - Database-Spalten vorhanden aber nicht verwendet

10. **JWT Secret**
    - Hardcoded "your-secret-key"
    - Sollte Environment Variable sein
    - Für Produktion KRITISCH ändern!

### Bekannte Bugs

**Keine kritischen Bugs bekannt** ✅

Alle größeren Bugs wurden behoben:
- ✅ Doppelte Spieler
- ✅ Kamera-Reset
- ✅ Query-Konflikte
- ✅ Cursor in allen Feldern
- ✅ Automatischer Login nach Registrierung

---

## 🔮 Nächste Schritte / Empfehlungen

### Kurzfristig (1-2 Sessions)

1. **Character Deletion UI**
   - "Löschen"-Button in Charakterauswahl
   - Bestätigungs-Dialog
   - CharacterDeleted Event → Character-Liste aktualisieren

2. **Error-Feedback verbessern**
   - CharacterCreationFailed in UI zeigen
   - CharacterSelectionFailed in UI zeigen
   - NetworkClient Fehler besser anzeigen

3. **Auto-Select nach Creation**
   - Nach CharacterCreated Event:
   - Automatisch selected_character_id setzen
   - Transition zu InGame
   - Oder: Zurück zu CharacterSelection mit "Neu erstellt!"-Marker

### Mittelfristig (3-5 Sessions)

4. **Multiplayer-Synchronisation**
   - Server: Track alle verbundenen Spieler
   - Broadcast WorldState alle 50ms
   - Client: Spawn/Update OtherPlayer Entities
   - Interpolation für smooth Movement

5. **Character Stats & Combat**
   - Health/Mana Bars in UI
   - Attack-System (Click = Attack)
   - Damage Calculation
   - Death & Respawn

6. **Inventory-System**
   - Items Tabelle in DB
   - Inventory UI (I-Taste)
   - Drag & Drop
   - Item-Pickups in Welt

### Langfristig (6+ Sessions)

7. **Erweiterte Welt**
   - Terrain-System
   - NPCs & Quests
   - Dungeons/Instanzen
   - Fast Travel / Waypoints

8. **Erweiterte Features**
   - Chat-System
   - Friends/Groups
   - Trading
   - Crafting

9. **Performance & Skalierung**
   - Server: Mehrere Game Regions
   - Client: Level-of-Detail
   - Database: Indices optimieren
   - Network: Prediction & Lag Compensation

---

## 🛠️ Entwickler-Hinweise

### Code-Style

- **Deutsche UI-Texte**, Englische Code-Kommentare
- Bevy Conventions: Systems mit `snake_case`, Components mit `PascalCase`
- Ausführliche Kommentare bei komplexen Systemen
- `info!()` für wichtige Events, `error!()` für Fehler

### Testing

```bash
# Server Tests
cargo test -p server

# Alle Tests
cargo test

# Mit Output
cargo test -- --nocapture
```

### Database Reset

```bash
# Komplett neu starten
rm game.db
cargo run -p server  # DB wird neu erstellt
```

### Network Debugging

```bash
# Server mit Logs
RUST_LOG=info cargo run --release -p server

# Client mit Logs
RUST_LOG=info cargo run --release -p client

# Nur Networking-Logs
RUST_LOG=client::networking=debug cargo run -p client
```

### Build-Zeiten optimieren

```bash
# Nur ändern was geändert wurde
cargo build -p client

# Parallel kompilieren
cargo build --release -j 8
```

---

## 📚 Wichtige Dependencies

### Server (`server/Cargo.toml`)

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "sqlite"] }
bcrypt = "0.15"
jsonwebtoken = "9"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.11"
shared = { path = "../shared" }
```

### Client (`client/Cargo.toml`)

```toml
[dependencies]
bevy = "0.14"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
shared = { path = "../shared" }
```

### Shared (`shared/Cargo.toml`)

```toml
[dependencies]
bevy = "0.14"
serde = { version = "1.0", features = ["derive"] }
```

---

## 🎓 Gelernte Bevy-Patterns

### 1. State-basierte UI

```rust
// Jeder Screen ist ein Plugin
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

### 2. Event-driven Architecture

```rust
// Networking → Events → State Changes
fn process_messages(network: Res<NetworkClient>, mut events: EventWriter<AuthResponseEvent>) {
    while let Some(msg) = network.get_message() {
        events.send(AuthResponseEvent(msg));
    }
}

fn handle_events(mut events: EventReader<AuthResponseEvent>, mut state: ResMut<NextState<GameState>>) {
    for event in events.read() {
        match event.0 {
            AuthResponse::LoginSuccess { .. } => state.set(GameState::CharacterSelection),
            // ...
        }
    }
}
```

### 3. Resource für persistente Daten

```rust
#[derive(Resource, Default)]
pub struct AuthState { /* ... */ }

// System kann darauf zugreifen
fn my_system(auth: Res<AuthState>) {
    if let Some(token) = auth.get_token() {
        // ...
    }
}
```

### 4. Marker Components für Cleanup

```rust
#[derive(Component)]
struct LoginUI;

fn cleanup(mut commands: Commands, query: Query<Entity, With<LoginUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

### 5. Thread-safe Networking

```rust
pub struct NetworkClient {
    socket: Arc<Mutex<UdpSocket>>,
    incoming_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
}

// Background Thread
std::thread::spawn(move || {
    listen_for_messages(socket_clone, messages_clone);
});
```

---

## 📞 Support & Weitere Infos

### Bevy Dokumentation
- https://bevyengine.org/learn/
- https://docs.rs/bevy/

### SQLx Dokumentation
- https://docs.rs/sqlx/

### Projekt-spezifische Docs
- `DATABASE_PHASE1.md` - Database Setup
- `DATABASE_PHASE2.md` - Auth System
- `DATABASE_PHASE3.md` - Client Integration

---

## ⚡ Quick Reference

### Server starten
```bash
cd /home/max/code/game && ./run_server.sh
```

### Client starten
```bash
cd /home/max/code/game && ./run_client.sh
```

### Kompilieren
```bash
cargo build --release
```

### Tests
```bash
cargo test -p server
```

### Database löschen
```bash
rm game.db
```

### Logs aktivieren
```bash
RUST_LOG=info cargo run -p client
```

---

## 🎯 Session-Ziele erreicht

- ✅ Datenbank mit Users & Characters
- ✅ Authentifizierung (bcrypt + JWT)
- ✅ Client-Server Kommunikation (UDP)
- ✅ Vollständige UI (Login, Character, Pause, Settings)
- ✅ 3D-Welt mit Spieler & Kamera
- ✅ Kamera-relative Bewegung
- ✅ Persistente Kamera-Position
- ✅ Keine doppelten Spieler
- ✅ Separates Pause- & Settings-Menü
- ✅ Deutsche UI-Texte
- ✅ Funktionierende Settings (VSync, Vollbild, Audio)

**Projekt-Status: Solide Grundlage für MMORPG vorhanden! 🚀**

---

_Letzte Aktualisierung: 2024-11-09_
_Nächste Session: Weiter mit Multiplayer-Sync oder Character Deletion UI_
