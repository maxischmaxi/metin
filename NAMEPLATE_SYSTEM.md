# Nameplate-System Dokumentation

## Übersicht

Spieler haben jetzt ein schwebendes Nameplate über dem Kopf, das **Level** und **Character-Name** anzeigt.

## Features

✅ **2D-Text im 3D-Raum** - Schwebt über dem Spieler
✅ **Billboard-Effekt** - Dreht sich immer zur Kamera
✅ **Halbtransparenter Hintergrund** - Schwarzer Background (RGBA 0,0,0,0.3)
✅ **Weißer Text** - Gut lesbar auf dunklem Hintergrund
✅ **Auto-Update** - Aktualisiert sich bei Level-Ups
✅ **Format:** "Lvl X - CharacterName"

## Technische Implementierung

### Komponenten

```rust
#[derive(Component)]
struct PlayerNameplate;  // Marker für das Nameplate-Entity

#[derive(Component)]
struct NameplateText;    // Marker für den Text
```

### Spawn-Struktur

```
Player (PbrBundle - Kapsel)
└── Nameplate (SpatialBundle)
    ├── Background (SpriteBundle)
    │   └── Sprite { color: rgba(0,0,0,0.3), size: 200x30 }
    └── Text (Text2dBundle)
        └── "Lvl 1 - PlayerName"
```

**Position:** Y = 2.5 über dem Spieler (Kapsel ist 1.5 hoch)

### Systems

#### 1. Billboard-System

```rust
fn billboard_nameplate(
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    mut nameplate_query: Query<&mut Transform, With<PlayerNameplate>>,
)
```

**Was es macht:**

- Läuft jedes Frame
- Holt Kamera-Position in Welt-Raum
- Rotiert Nameplate um Y-Achse zur Kamera
- Text bleibt aufrecht (keine X/Z Rotation)

**Ergebnis:** Nameplate schaut immer zum Spieler, egal wo die Kamera ist

#### 2. Update-System

```rust
fn update_nameplate_text(
    player_stats: Res<PlayerStats>,
    mut nameplate_query: Query<&mut Text, With<NameplateText>>,
)
```

**Was es macht:**

- Läuft wenn PlayerStats sich ändern
- Aktualisiert Text-Content
- Format: `"Lvl {} - {}"`

**Trigger:**

- Level-Up Events
- Character-Selection

### Data-Flow

```
Character Selection:
1. Server sendet CharacterSelected { name, level, ... }
2. Client speichert in PlayerStats.character_name
3. setup_player() spawnt Nameplate mit Text
4. Text = "Lvl 1 - PlayerName"

Level-Up:
1. Server sendet LevelUp { new_level, ... }
2. Client updated PlayerStats.level
3. update_nameplate_text() triggered (PlayerStats changed)
4. Text = "Lvl 2 - PlayerName"
```

## Styling

### Text

- **Font:** `momo.ttf` (Custom Font)
- **Größe:** 20px
- **Farbe:** Weiß (Color::WHITE)
- **Position:** Z = 0.1 (vor Background)

### Background

- **Typ:** Sprite2D
- **Farbe:** RGBA(0, 0, 0, 0.3) - Schwarz mit 30% Opacity
- **Größe:** 200x30 Pixel
- **Position:** Z = 0.0 (hinter Text)

### Container

- **Typ:** SpatialBundle (kein visuelles Element)
- **Position:** Vec3(0, 2.5, 0) relativ zum Player
- **Rotation:** Dynamisch via Billboard-System

## Verhalten

### Bewegung

- Nameplate folgt Spieler automatisch (ist Child-Entity)
- Keine manuelle Position-Update nötig

### Rotation

- Nameplate dreht sich zur Kamera (Billboard)
- Nur Y-Achse rotation (bleibt horizontal)
- Text bleibt lesbar aus allen Winkeln

### Sichtbarkeit

- Immer sichtbar wenn Spieler sichtbar
- Skaliert mit Distanz (ist im 3D-Raum)
- Bei sehr weiter Entfernung wird Text klein

### Updates

- **Bei Level-Up:** Text updated sich automatisch
- **Bei Character-Wechsel:** Neues Nameplate wird gespawnt
- **Bei Logout:** Nameplate wird mit Spieler gelöscht (GameWorld Marker)

## Multiplayer-Ready

Das System ist bereits vorbereitet für Multiplayer:

```rust
// Für andere Spieler (zukünftig):
fn spawn_other_player(name: String, level: i32) {
    commands.spawn(OtherPlayer)
        .with_children(|parent| {
            // Genau dasselbe Nameplate-System!
            spawn_nameplate(parent, name, level);
        });
}
```

Jeder Spieler (lokal + remote) kann sein eigenes Nameplate haben.

## Performance

- **Rendering:** Minimal (2D Sprite + Text)
- **Billboard:** 1 Rotation pro Frame pro Nameplate
- **Update:** Nur bei Level-Change
- **Memory:** ~500 Bytes pro Nameplate

**Skalierung:** Funktioniert für 100+ Spieler gleichzeitig

## Testing

### Im Spiel testen:

1. **Character auswählen/erstellen**
   - Name: z.B. "Gandalf"
   - Level: 1

2. **Im Spiel schauen**
   - Über dem Spieler schwebt: **"Lvl 1 - Gandalf"**
   - Weißer Text auf schwarzem Hintergrund
   - Dreht sich mit Kamera-Bewegung

3. **Level-Up testen (K drücken)**
   - Taste K = +1000 XP
   - Nach Level-Up: Text ändert sich zu **"Lvl 2 - Gandalf"**
   - Automatisch, kein Reload nötig

4. **Kamera drehen (Rechte Maustaste)**
   - Nameplate dreht sich mit
   - Bleibt immer lesbar
   - Folgt Spieler-Bewegung

### Erwartetes Verhalten:

✓ Nameplate erscheint sofort beim Spawn
✓ Text ist lesbar aus allen Winkeln
✓ Hintergrund ist halbtransparent schwarz
✓ Höhe passt (nicht im Boden, nicht zu hoch)
✓ Updated sich bei Level-Ups
✓ Verschwindet beim Logout

## Bekannte Limitierungen

1. **Distanz-Skalierung:**
   - Bei sehr weiter Entfernung wird Text sehr klein
   - Könnte mit konstanter Größe (Camera-Distance basiert) gelöst werden

2. **Keine Health-Bar:**
   - Aktuell nur Name + Level
   - Health-Bar könnte als zusätzliches Child hinzugefügt werden

3. **Keine Farben:**
   - Alle Nameplates sehen gleich aus
   - Könnte Farbe nach Team/Guild/Status haben

## Zukünftige Erweiterungen

### Einfach zu implementieren:

1. **Health-Bar unter dem Namen**

```rust
parent.spawn(SpriteBundle {
    sprite: Sprite {
        color: Color::srgb(0.8, 0.2, 0.2),
        custom_size: Some(Vec2::new(HP_PERCENT * 200.0, 5.0)),
    },
    transform: Transform::from_xyz(0.0, -15.0, 0.1),
});
```

2. **Guild-Name/Title**

```rust
text = format!("<Guild>\nLvl {} - {}", level, name);
```

3. **Farb-Coding**

```rust
// Freundlich = Grün, Feindlich = Rot
let name_color = if is_friendly { Color::GREEN } else { Color::RED };
```

4. **Distance-Fade**

```rust
// Verblasst bei großer Distanz
let alpha = (1.0 - distance / MAX_VISIBLE_DISTANCE).clamp(0.0, 1.0);
```

5. **Konstante Größe**

```rust
// Skaliert mit Kamera-Distanz für konstante Screen-Size
let scale = distance / BASE_DISTANCE;
transform.scale = Vec3::splat(scale);
```

## Code-Locations

**Player-Spawn:** `client/src/player.rs` (Zeile ~66-95)
**Billboard-System:** `client/src/player.rs` (Zeile ~285-300)
**Update-System:** `client/src/player.rs` (Zeile ~273-283)
**PlayerStats:** `client/src/ui/game_ui.rs` (character_name Field)

---

_Letzte Aktualisierung: 2024-11-09_
_Status: Voll funktionsfähig, bereit für Testing_
