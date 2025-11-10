# NPC-System Implementierungsplan

## 🎯 Ziel
Spezialisierungs-Trainer NPC implementieren, der bei Level 5 die Spezialisierungswahl ermöglicht.

---

## 📋 Requirements

1. **NPC in der Welt spawnen**
   - Position: Nahe Spawn-Point (z.B. bei 5, 1, 5)
   - 3D-Model: Würfel oder Kapsel (unterschiedliche Farbe vom Spieler)
   - Nameplate: "Meister der Künste" (Level-unabhängig)

2. **Interaktions-Range**
   - Globale Konstante: `NPC_INTERACTION_RANGE = 3.0` Meter
   - Gilt für alle NPCs im Spiel
   - Visual Feedback wenn in Range

3. **Maus-Interaktion**
   - Linke Maustaste zum Anklicken
   - Raycast von Kamera zum NPC
   - Nur wenn in Range → Dialog öffnet

4. **Dialog-System**
   - **Level < 5**: "Du musst Level 5 erreichen, um eine Spezialisierung zu wählen."
   - **Level 5+, keine Spec**: Zeigt 2 Spezialisierungs-Optionen
   - **Level 5+, hat Spec**: "Du hast bereits eine Spezialisierung gewählt: [Name]"

5. **Spezialisierungs-UI**
   - Overlay-Dialog mit Titel
   - 2 Buttons (eine Spec pro Klasse)
   - Beschreibung jeder Spec
   - "Wählen" Button + "Abbrechen" Button
   - Nach Wahl: Dialog schließt, Message an Server

---

## 🏗️ Architektur

### Neue Dateien

**client/src/npc.rs** (NEU)
```rust
// NPC System
- struct Npc { name, npc_type, position }
- enum NpcType { SpecializationTrainer, Merchant, QuestGiver }
- Component markers
- Spawn-System
- Nameplate-System (wie Player)
```

**client/src/ui/npc_dialog.rs** (NEU)
```rust
// NPC Dialog UI
- NpcDialogPlugin
- setup_npc_dialog() - Creates UI overlay
- handle_dialog_buttons() - Wählen/Abbrechen
- cleanup_npc_dialog()
- Components: NpcDialogUI, DialogButton
```

**client/src/interaction.rs** (NEU)
```rust
// Interaction System
- INTERACTION_RANGE constant (3.0)
- mouse_click_system() - Raycast auf NPCs
- check_interaction_range() - Distance check
- highlight_interactable_npcs() - Visual feedback
```

### Erweiterte Dateien

**client/src/main.rs**
- Add NpcPlugin
- Add InteractionPlugin
- Add NpcDialogPlugin

**client/src/player.rs**
- GameWorld marker auch für NPCs verwenden

**shared/src/lib.rs**
- ClientMessage::InteractWithNpc { npc_id }
- ServerMessage::NpcDialog { npc_id, dialog_type, options }

---

## 🔧 Implementation Details

### 1. NPC Spawning (client/src/npc.rs)

```rust
const NPC_SPAWN_POSITIONS: &[(Vec3, &str, NpcType)] = &[
    (Vec3::new(5.0, 1.0, 5.0), "Meister der Künste", NpcType::SpecializationTrainer),
];

#[derive(Component)]
struct Npc {
    name: String,
    npc_type: NpcType,
}

#[derive(Component)]
struct NpcNameplate;

fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (position, name, npc_type) in NPC_SPAWN_POSITIONS {
        // Spawn NPC model (golden capsule)
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.8)),
                material: materials.add(Color::srgb(0.9, 0.7, 0.2)), // Golden
                transform: Transform::from_translation(*position),
                ..default()
            },
            Npc {
                name: name.to_string(),
                npc_type: *npc_type,
            },
            GameWorld,
        ));
        
        // Spawn NPC nameplate (similar to player)
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(*position + Vec3::Y * 2.5),
                ..default()
            },
            NpcNameplate,
            GameWorld,
        ));
        
        // Spawn UI nameplate
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: Color::srgba(0.2, 0.1, 0.0, 0.7).into(),
                z_index: ZIndex::Global(99),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            NpcNameplateUI,
            GameWorld,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                name.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/momo/momo.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(1.0, 0.9, 0.3),
                },
            ));
        });
    }
}
```

### 2. Interaction System (client/src/interaction.rs)

```rust
pub const NPC_INTERACTION_RANGE: f32 = 3.0;

fn mouse_click_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    npc_query: Query<(Entity, &Transform, &Npc)>,
    player_query: Query<&Transform, With<Player>>,
    mut npc_dialog_state: ResMut<NpcDialogState>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor_position) = window.cursor_position() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(player_transform) = player_query.get_single() else { return };
    
    // Convert cursor to ray
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };
    
    // Check all NPCs for intersection
    for (entity, npc_transform, npc) in npc_query.iter() {
        let npc_pos = npc_transform.translation;
        let distance_to_player = player_transform.translation.distance(npc_pos);
        
        // Check if in interaction range
        if distance_to_player > NPC_INTERACTION_RANGE {
            continue;
        }
        
        // Simple sphere-ray intersection
        let to_npc = npc_pos - ray.origin;
        let projection = to_npc.dot(ray.direction);
        
        if projection > 0.0 {
            let closest_point = ray.origin + ray.direction * projection;
            let distance = (closest_point - npc_pos).length();
            
            if distance < 0.5 { // NPC radius
                // Clicked on NPC!
                npc_dialog_state.open_dialog(entity, npc.npc_type, npc.name.clone());
                info!("Interacting with NPC: {}", npc.name);
                break;
            }
        }
    }
}

fn highlight_nearby_npcs(
    player_query: Query<&Transform, With<Player>>,
    mut npc_query: Query<(&Transform, &mut Handle<StandardMaterial>), With<Npc>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for (npc_transform, material_handle) in npc_query.iter_mut() {
        let distance = player_transform.translation.distance(npc_transform.translation);
        
        if let Some(material) = materials.get_mut(&*material_handle) {
            if distance <= NPC_INTERACTION_RANGE {
                // Highlight: Brighter glow
                material.emissive = Color::srgb(0.5, 0.4, 0.1).into();
            } else {
                // Normal
                material.emissive = Color::BLACK.into();
            }
        }
    }
}
```

### 3. NPC Dialog UI (client/src/ui/npc_dialog.rs)

```rust
#[derive(Resource, Default)]
struct NpcDialogState {
    active: bool,
    npc_entity: Option<Entity>,
    npc_type: Option<NpcType>,
    npc_name: String,
}

impl NpcDialogState {
    fn open_dialog(&mut self, entity: Entity, npc_type: NpcType, name: String) {
        self.active = true;
        self.npc_entity = Some(entity);
        self.npc_type = Some(npc_type);
        self.npc_name = name;
    }
    
    fn close_dialog(&mut self) {
        self.active = false;
        self.npc_entity = None;
        self.npc_type = None;
    }
}

fn setup_npc_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    player_stats: Res<PlayerStats>,
    auth_state: Res<AuthState>,
    font: Res<GameFont>,
) {
    if !dialog_state.active {
        return;
    }
    
    let Some(NpcType::SpecializationTrainer) = dialog_state.npc_type else { return };
    
    // Determine dialog content
    let (title, message, show_spec_buttons) = if player_stats.level < 5 {
        (
            "Meister der Künste",
            "Du musst Level 5 erreichen, um eine Spezialisierung zu wählen.\n\nKehre zurück, wenn du stärker geworden bist.",
            false,
        )
    } else if let Some(spec) = auth_state.specialization {
        (
            "Meister der Künste",
            &format!("Du hast bereits eine Spezialisierung gewählt:\n\n{}\n\nDieser Pfad ist nun dein Schicksal.", spec.name()),
            false,
        )
    } else {
        (
            "Wähle deine Spezialisierung",
            "Du hast Level 5 erreicht! Es ist Zeit, deinen Pfad zu wählen.\n\nWähle weise, denn diese Entscheidung ist permanent!",
            true,
        )
    };
    
    // Build dialog UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
            z_index: ZIndex::Global(200),
            ..default()
        },
        NpcDialogUI,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(600.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            background_color: Color::srgb(0.15, 0.1, 0.05).into(),
            border_color: Color::srgb(0.6, 0.4, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: font.0.clone(),
                    font_size: 32.0,
                    color: Color::srgb(1.0, 0.9, 0.3),
                },
            ));
            
            // Message
            parent.spawn(TextBundle::from_section(
                message,
                TextStyle {
                    font: font.0.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            
            if show_spec_buttons {
                // Get specializations for player class
                let class = auth_state.class.unwrap(); // Should be set
                let (spec1_name, spec2_name) = class.specializations();
                let spec1 = Specialization::from_class_and_index(class, 0).unwrap();
                let spec2 = Specialization::from_class_and_index(class, 1).unwrap();
                
                // Spec buttons container
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    create_spec_button(parent, spec1, font.0.clone());
                    create_spec_button(parent, spec2, font.0.clone());
                });
            }
            
            // Close button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.2, 0.1).into(),
                    ..default()
                },
                DialogButton::Close,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Schließen",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
    });
}

fn create_spec_button(
    parent: &mut ChildBuilder,
    spec: Specialization,
    font: Handle<Font>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(10.0),
            ..default()
        },
        background_color: Color::srgb(0.2, 0.15, 0.1).into(),
        border_color: Color::srgb(0.5, 0.4, 0.2).into(),
        ..default()
    })
    .with_children(|parent| {
        // Spec name
        parent.spawn(TextBundle::from_section(
            spec.name(),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::srgb(1.0, 0.8, 0.2),
            },
        ));
        
        // Description
        parent.spawn(TextBundle::from_section(
            spec.description(),
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            },
        ));
        
        // Choose button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            DialogButton::ChooseSpec(spec),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Wählen",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}
```

---

## 📝 Implementation Checklist

### Phase 1: NPC System Grundlagen
- [ ] `client/src/npc.rs` erstellen
  - [ ] Npc struct/component
  - [ ] NpcType enum
  - [ ] spawn_npcs() system
  - [ ] NPC nameplate system (copy from player.rs)
  
- [ ] `client/src/interaction.rs` erstellen
  - [ ] INTERACTION_RANGE constant
  - [ ] mouse_click_system()
  - [ ] highlight_nearby_npcs()
  - [ ] Raycast-Logik

### Phase 2: Dialog System
- [ ] `client/src/ui/npc_dialog.rs` erstellen
  - [ ] NpcDialogState resource
  - [ ] setup_npc_dialog()
  - [ ] create_spec_button()
  - [ ] handle_dialog_buttons()
  - [ ] cleanup_npc_dialog()

### Phase 3: Integration
- [ ] `client/src/main.rs` erweitern
  - [ ] NpcPlugin hinzufügen
  - [ ] InteractionPlugin hinzufügen
  - [ ] NpcDialogPlugin hinzufügen

- [ ] `client/src/auth_state.rs` erweitern
  - [ ] specialization field
  - [ ] class field (für Spec-Wahl)

- [ ] `shared/src/lib.rs` erweitern (optional)
  - [ ] ClientMessage::InteractWithNpc
  - [ ] ServerMessage::NpcDialog

### Phase 4: Server-Integration
- [ ] Server: ChooseSpecialization Handler vollständig implementieren
- [ ] DB: Specialization speichern/laden
- [ ] Validation: Level >= 5, keine Spec gewählt, Spec passt zu Klasse

---

## 🎨 Visual Design

### NPC Appearance
- **Model**: Goldene Kapsel (0.4 Radius, 1.8 Höhe)
- **Farbe**: Gold (0.9, 0.7, 0.2)
- **Nameplate**: Braun-transparenter Hintergrund
- **Highlight**: Glüht bei Nähe (emissive material)

### Dialog UI
- **Background**: Halbtransparentes Schwarz (0.5 alpha)
- **Dialog Box**: Braun (0.15, 0.1, 0.05)
- **Border**: Gold (0.6, 0.4, 0.1)
- **Buttons**: Grün für "Wählen", Grau für "Schließen"

---

## 🧪 Test Plan

1. **Spawn Test**
   - NPC erscheint bei (5, 1, 5)
   - Nameplate ist sichtbar
   - Goldene Farbe

2. **Range Test**
   - Nahe gehen → NPC glüht
   - Weg gehen → Glühen stoppt
   - Exakt 3.0m Distanz testen

3. **Click Test (Level < 5)**
   - NPC anklicken
   - Dialog öffnet: "Du musst Level 5 erreichen..."
   - Schließen-Button funktioniert

4. **Click Test (Level 5, keine Spec)**
   - K-Taste drücken (bis Level 5)
   - NPC anklicken
   - 2 Spezialisierungs-Optionen erscheinen
   - Namen + Beschreibungen korrekt

5. **Spec Choice Test**
   - Spec wählen
   - Message an Server
   - Dialog schließt
   - Erneut anklicken → "Du hast bereits gewählt"

---

## 📊 Geschätzte Zeilen Code

- `npc.rs`: ~300 Zeilen
- `interaction.rs`: ~150 Zeilen
- `npc_dialog.rs`: ~400 Zeilen
- Integration: ~50 Zeilen
- **Total**: ~900 Zeilen neuer Code

---

## 🚀 Priorität

**HIGH** - Kernfeature für Spezialisierungs-System
- Blocker für Skill-System
- Wichtig für Game-Progression
- User-freundlicher als UI-Popup bei Level-Up

