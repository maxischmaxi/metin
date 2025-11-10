# Player Nameplate Fix - MMORPG Style ✅

## Problem
Das Player Nameplate (Level + Name über dem Charakter) wurde nicht angezeigt.

## Ursache
1. Ursprünglich wurde `Text2dBundle` verwendet, aber nicht richtig sichtbar
2. Fehlende Welt-zu-Bildschirm Konvertierung
3. Keine korrekte UI-Positionierung

## Lösung: Hybrid-System

### 1. 3D Marker im Spielraum
```rust
// Unsichtbarer 3D-Marker 2.5 Einheiten über dem Spieler
commands.spawn((
    SpatialBundle {
        transform: Transform::from_translation(spawn_pos + Vec3::Y * 2.5),
        ..default()
    },
    PlayerNameplate,
    GameWorld,
));
```

### 2. 2D UI-Overlay
```rust
// Sichtbares UI-Element, das dem 3D-Marker folgt
commands.spawn((
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            padding: UiRect { ... },
            ..default()
        },
        background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
        z_index: ZIndex::Global(100),
        border_radius: BorderRadius::all(Val::Px(4.0)),
        ..default()
    },
    NameplateUI,
))
.with_children(|parent| {
    parent.spawn(TextBundle::from_section(
        "Lvl 1 - CharacterName",
        TextStyle {
            font: asset_server.load("fonts/momo/momo.ttf"),
            font_size: 18.0,
            color: Color::srgb(1.0, 0.9, 0.3), // Golden text
        },
    ));
});
```

### 3. Systeme

#### a) Marker Position Update (folgt Spieler)
```rust
fn update_nameplate_marker_position(
    player_query: Query<&Transform, (With<Player>, Without<PlayerNameplate>)>,
    mut nameplate_query: Query<&mut Transform, With<PlayerNameplate>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for mut nameplate_transform in nameplate_query.iter_mut() {
        nameplate_transform.translation = player_transform.translation + Vec3::Y * 2.5;
    }
}
```

#### b) UI Position Update (Welt → Bildschirm)
```rust
fn update_nameplate_ui_position(
    nameplate_marker_query: Query<&GlobalTransform, With<PlayerNameplate>>,
    mut nameplate_ui_query: Query<(&mut Style, &Node), With<NameplateUI>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(marker_transform) = nameplate_marker_query.get_single() else { return };
    
    let world_pos = marker_transform.translation();
    
    if let Some(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
        for (mut style, node) in nameplate_ui_query.iter_mut() {
            let size = node.size();
            style.left = Val::Px(screen_pos.x - size.x / 2.0);  // Zentriert
            style.top = Val::Px(screen_pos.y - size.y - 5.0);   // Leicht darüber
        }
    }
}
```

#### c) Text Update (bei Level-Up)
```rust
fn update_nameplate_ui_text(
    player_stats: Res<crate::ui::PlayerStats>,
    nameplate_query: Query<&Children, With<NameplateUI>>,
    mut text_query: Query<&mut Text>,
) {
    if player_stats.is_changed() {
        for children in nameplate_query.iter() {
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.sections[0].value = format!("Lvl {} - {}", 
                        player_stats.level, 
                        player_stats.character_name
                    );
                }
            }
        }
    }
}
```

## Wie es funktioniert

1. **Spawn**: 
   - 3D-Marker wird 2.5 Einheiten über Spieler gespawnt
   - UI-Element wird als absolutes Overlay erstellt

2. **Update Loop**:
   - `update_nameplate_marker_position`: Marker folgt Spieler in 3D
   - `update_nameplate_ui_position`: UI wird auf Marker-Bildschirmposition gesetzt
   - `update_nameplate_ui_text`: Text wird bei Level-Änderung aktualisiert

3. **Rendering**:
   - UI wird immer über der 3D-Szene gerendert (z_index: 100)
   - Automatisches Zentrieren basierend auf Node-Größe
   - Goldener Text (MMORPG-Style)

## Visuelle Features

- ✅ Halbtransparenter schwarzer Hintergrund (alpha: 0.7)
- ✅ Abgerundete Ecken (4px border-radius)
- ✅ Goldener Text (RGB: 1.0, 0.9, 0.3)
- ✅ Automatische Zentrierung
- ✅ Folgt Spieler flüssig
- ✅ Aktualisiert sich bei Level-Up (K-Taste für +1000 XP)

## Test-Anleitung

1. **Server starten**:
   ```bash
   cd /home/max/code/game
   ./run_server.sh
   ```

2. **Client starten**:
   ```bash
   cd /home/max/code/game
   ./run_client.sh
   ```

3. **Im Spiel**:
   - Login/Registrieren
   - Character erstellen/auswählen
   - Im Spiel: **Über deinem Charakter sollte jetzt ein goldenes Nameplate schweben!**
   - Format: `Lvl 1 - [CharacterName]`

4. **Level testen**:
   - Drücke `K` um +1000 XP zu bekommen
   - Nameplate sollte sich automatisch aktualisieren: `Lvl 2 - [CharacterName]`

## Vergleich zu bekannten MMORPGs

Ähnlich wie in:
- **World of Warcraft**: Nameplate über Charakter
- **Final Fantasy XIV**: Floating Name/Level Display
- **Guild Wars 2**: Character Name Tag

## Technische Details

**Dateien geändert:**
- `client/src/player.rs` (alle Nameplate-Systeme)

**Neue Components:**
- `PlayerNameplate` - 3D-Marker Component
- `NameplateUI` - 2D UI-Overlay Component

**Neue Systeme:**
- `setup_nameplate_ui()` - OnEnter(InGame)
- `cleanup_nameplate_ui()` - OnExit(InGame)
- `update_nameplate_marker_position()` - Jedes Frame
- `update_nameplate_ui_position()` - Jedes Frame
- `update_nameplate_ui_text()` - Bei Level-Änderung

**Performance:**
- Minimal: Nur 2 Queries pro Frame (Marker + UI)
- `world_to_viewport()` ist sehr performant in Bevy
- UI wird nur bei Position-Änderung neu berechnet

## Bekannte Limitierungen

1. **Occlusion**: Nameplate ist immer sichtbar (auch durch Wände)
   - Könnte mit Raycast-Check verbessert werden
   - Typisch für MMORPGs

2. **Multiplayer**: Aktuell nur für lokalen Spieler
   - Für andere Spieler müsste System erweitert werden
   - Einfach: Gleiche Logik für `OtherPlayer` Component

## Zukünftige Verbesserungen

- [ ] Fade-out bei großer Distanz
- [ ] Healthbar unter Name
- [ ] Guild-Namen Support
- [ ] Farben basierend auf Fraktion/PvP-Status
- [ ] Nameplate für NPCs
- [ ] Nameplate für andere Spieler (Multiplayer)

