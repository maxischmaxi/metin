# MMORPG - Rust & Bevy Game

Ein grundlegendes MMORPG implementiert mit Rust und der Bevy Engine.

## Projektstruktur

Das Projekt verwendet ein Monorepo mit drei Crates:

```
game/
├── server/     # Game Server
├── client/     # Game Client (Bevy)
└── shared/     # Gemeinsame Datenstrukturen
```

## Features

### Implementiert ✅

- **Character-System**
  - Character-Erstellung mit Name und Klasse (Warrior, Mage, Rogue)
  - Character-Auswahl-Menü
  
- **Gameplay**
  - 3D-Spielwelt mit Terrain
  - Player-Spawn in der Map
  - WASD-Steuerung für Bewegung
  - Orbit-Camera (Rechte Maustaste zum Rotieren, Mausrad zum Zoomen)
  
- **UI-System**
  - Character-Erstellungsmenü
  - Character-Auswahlmenü
  - Einstellungsmenü (Grafik & Audio)
  - In-Game-UI mit Steuerungshinweisen

- **Einstellungen**
  - VSync an/aus
  - Vollbild-Modus
  - Audio-Lautstärkeregler (Master, Musik, SFX)

### Geplant 🚧

- Multiplayer-Networking (aktuell offline)
- Combat-System
- Inventory-System
- NPC & Quests

## Installation & Build

### Voraussetzungen

- Rust (neueste stabile Version)
- Cargo

### Compilieren

```bash
# Gesamtes Workspace bauen
cargo build --release

# Nur Client bauen
cargo build --release -p client

# Nur Server bauen
cargo build --release -p server
```

## Ausführen

### Schnellstart

```bash
# Client starten (empfohlen)
./run_client.sh

# Oder Server starten (optional, für zukünftige Multiplayer-Features)
./run_server.sh
```

### Manuell starten

```bash
# Client starten
cargo run --release -p client

# Server starten (vorbereitet für zukünftige Networking-Features)
RUST_LOG=info cargo run --release -p server
```

## Steuerung

### Menüs
- **Linke Maustaste**: Buttons klicken
- **ESC**: Zum Einstellungsmenü (im Spiel)
- **Quit Game Button**: Spiel beenden (im Hauptmenü und Einstellungen)

### Character-Erstellung
- **Buchstaben A-Z**: Name eingeben
- **Zahlen 0-9**: Zahlen hinzufügen
- **Shift + Buchstabe**: Großbuchstaben
- **Space**: Leerzeichen
- **Backspace**: Zeichen löschen
- **Linke Maustaste**: Klassen-Buttons und Erstellen

### Im Spiel
- **W/A/S/D**: Player bewegen
- **Rechte Maustaste gedrückt + Mausbewegung**: Kamera rotieren
- **Mausrad**: Kamera Zoom
- **ESC**: Einstellungsmenü öffnen

## Technologie-Stack

- **Engine**: Bevy 0.14
- **Sprache**: Rust (Edition 2021)
- **Serialisierung**: Serde + Bincode
- **Networking**: UDP-basiert (in Vorbereitung)

## Architektur

### Client

Der Client nutzt das Bevy ECS-System mit folgenden Plugins:

- `PlayerPlugin`: Player-Logik und Bewegung
- `CameraPlugin`: Orbit-Camera-System
- `NetworkingPlugin`: Netzwerk-Kommunikation (simplified)
- `CharacterSelectionPlugin`: Character-Auswahl-UI
- `CharacterCreationPlugin`: Character-Erstellung-UI
- `GameUIPlugin`: In-Game-UI
- `SettingsPlugin`: Einstellungsmenü

### Server

Einfacher UDP-basierter Game Server:

- Empfängt Client-Nachrichten
- Verwaltet Player-States
- Synchronisiert Spielwelt (in Entwicklung)

### Shared

Gemeinsame Datenstrukturen zwischen Client und Server:

- `CharacterData`: Character-Informationen
- `ClientMessage`: Client → Server Nachrichten
- `ServerMessage`: Server → Client Nachrichten
- `MMOSettings`: Spiel-Einstellungen

## Game States

Der Client verwendet folgende Game States:

1. `CharacterSelection`: Hauptmenü mit Character-Auswahl
2. `CharacterCreation`: Character-Erstellungsbildschirm
3. `InGame`: Aktives Gameplay
4. `Settings`: Einstellungsmenü

## Entwicklung

### Code-Struktur

```
client/src/
├── main.rs              # Entry point
├── camera.rs            # Orbit camera system
├── player.rs            # Player movement & logic
├── networking.rs        # Network client
└── ui/
    ├── mod.rs           # UI common code
    ├── character_creation.rs
    ├── character_selection.rs
    ├── game_ui.rs
    └── settings.rs
```

### Neues Feature hinzufügen

1. Gemeinsame Datenstrukturen in `shared/src/lib.rs` definieren
2. Server-Logik in `server/src/main.rs` implementieren
3. Client-Plugin in `client/src/` erstellen
4. Plugin in `client/src/main.rs` registrieren

## Lizenz

Dieses Projekt ist ein Beispielprojekt für Lernzwecke.
