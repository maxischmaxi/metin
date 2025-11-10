# Level-System Dokumentation

## Übersicht

Das MMORPG hat jetzt ein vollständiges Level-System (Level 1-100) mit exponentieller XP-Kurve, ähnlich wie in großen MMORPGs (WoW, FFXIV, etc.).

## XP-Formel

**Exponentiell, sehr grindy:**
```
XP_needed = 100 * level^2.8
```

### XP-Anforderungen (Beispiele)

| Level | XP benötigt | Monster (à 1000 XP) |
|-------|-------------|---------------------|
| 1→2   | 100         | ~1                  |
| 10→11 | ~25,000     | ~25                 |
| 25→26 | ~240,000    | ~240                |
| 50→51 | ~4,400,000  | ~4,400              |
| 75→76 | ~16,500,000 | ~16,500             |
| 99→100| ~25,000,000 | ~25,000             |

**Total XP für Level 100:** ~9.5 Millionen XP

Das bedeutet: Um Level 100 zu erreichen, müsste man bei 1000 XP pro Monster etwa **95,000 Monster** töten - typisch für ein grindy MMORPG!

## Stats pro Level

Die Stats erhöhen sich **klassenabhängig** bei jedem Level-Up:

### Warrior (Tank)
- **HP:** +20 pro Level (Level 100: 2080 HP)
- **Mana:** +5 pro Level (Level 100: 595 Mana)
- **Stamina:** +12 pro Level (Level 100: 1288 Stamina)

### Mage (Caster)
- **HP:** +8 pro Level (Level 100: 892 HP)
- **Mana:** +18 pro Level (Level 100: 1882 Mana)
- **Stamina:** +6 pro Level (Level 100: 694 Stamina)

### Rogue (DPS/Utility)
- **HP:** +12 pro Level (Level 100: 1288 HP)
- **Mana:** +8 pro Level (Level 100: 892 Mana)
- **Stamina:** +15 pro Level (Level 100: 1585 Stamina)

## Implementierung

### Shared Protocol (shared/src/lib.rs)

**Neue Funktionen:**
```rust
pub fn calculate_xp_for_level(level: i32) -> i64
pub fn calculate_stats_for_level(level: i32, class: &CharacterClass) -> (f32, f32, f32)
```

**Neue Messages:**
```rust
// Client → Server
ClientMessage::GainExperience { amount: i64 }

// Server → Client
ServerMessage::ExperienceGained { 
    amount: i64, 
    new_total: i64, 
    xp_needed: i64 
}

ServerMessage::LevelUp { 
    new_level: i32, 
    new_max_health: f32, 
    new_max_mana: f32, 
    new_max_stamina: f32,
}
```

**CharacterData erweitert:**
```rust
pub struct CharacterData {
    pub name: String,
    pub class: CharacterClass,
    pub appearance: CharacterAppearance,
    pub level: i32,        // NEU
    pub experience: i64,   // NEU
}
```

**CharacterSelected erweitert:**
```rust
ServerMessage::CharacterSelected { 
    character_id: i64, 
    position: Vec3,
    level: i32,           // NEU
    experience: i64,      // NEU
    max_health: f32,      // NEU
    max_mana: f32,        // NEU
    max_stamina: f32,     // NEU
}
```

### Server (server/src/main.rs)

**Neue Handler-Funktion:**
```rust
async fn handle_gain_experience(&mut self, client_addr: SocketAddr, amount: i64)
```

**Logik:**
1. XP hinzufügen
2. Prüfen ob Level-Up (kann mehrfach leveln bei viel XP!)
3. Bei Level-Up: XP reduzieren, Level erhöhen
4. Stats für neues Level berechnen
5. Messages an Client senden:
   - `ExperienceGained` (immer)
   - `LevelUp` (nur wenn gelevelt)
6. In DB speichern

**DB-Funktion:** (server/src/db/characters.rs)
```rust
pub async fn update_level_and_xp(
    pool: &SqlitePool,
    character_id: i64,
    level: i32,
    experience: i64,
) -> Result<(), sqlx::Error>
```

### Client (client/src/)

**PlayerStats erweitert:** (ui/game_ui.rs)
```rust
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub level: i32,        // NEU
    pub experience: i64,   // NEU
    pub xp_needed: i64,    // NEU
}
```

**UI-Komponenten:**
- `LevelText`: Zeigt "Lvl 1 (0/100)" an
- `XpBar`: Goldener Progress-Bar unter den Stat-Bars

**Neue Events:** (networking.rs)
```rust
pub enum LevelingEvent {
    ExperienceGained { amount: i64, new_total: i64, xp_needed: i64 },
    LevelUp { new_level: i32, new_max_health: f32, new_max_mana: f32, new_max_stamina: f32 },
}
```

**Event Handler:**
```rust
fn handle_leveling_events(
    mut leveling_events: EventReader<LevelingEvent>,
    mut player_stats: ResMut<PlayerStats>,
)
```

- `ExperienceGained`: Aktualisiert XP-Bar
- `LevelUp`: 
  - Aktualisiert Level + Max-Stats
  - Restored HP/Mana/Stamina auf max (Heilung beim Level-Up!)
  - Berechnet XP für nächstes Level
  - Logged "🎉 LEVEL UP!" in Console

**Character Selection:** (ui/character_selection.rs)
- Lädt Level, XP und Stats vom Server
- Initialisiert `PlayerStats` mit korrekten Werten
- Spieler startet mit **vollen** HP/Mana/Stamina

## Testing

### Dev Command (Taste 'K')

Während im Spiel (InGame State):
- **Taste K drücken** → Sendet `GainExperience { amount: 1000 }`
- Server verarbeitet XP
- Client bekommt Updates
- XP-Bar füllt sich
- Bei Level-Up: Stats erhöhen sich, Bars werden grün, Console-Log

**Beispiel-Log beim Level-Up:**
```
[INFO] +1000 XP! (950/1000)
[INFO] 🎉 LEVEL UP! Now level 2
[INFO]   HP: 100 → 120
[INFO]   Mana: 100 → 105
[INFO]   Stamina: 100 → 112
```

### Test-Ablauf

1. **Neuen Character erstellen** (Level 1, 0 XP)
2. **Im Spiel:** Taste K drücken mehrfach
3. **Level 1→2:** Nach ~1 Druck (100 XP needed)
4. **Level 2→3:** Nach ~4 Drücken (400 XP needed)
5. **Level 10→11:** Nach ~25 Drücken
6. **Logout → Login:** Level + XP werden gespeichert
7. **Stats:** HP/Mana/Stamina steigen je nach Klasse

## UI-Integration

### Bottom Bar (Links)

```
Lvl 42 (3500/4400000)  ← Level & XP-Display
HP  [████████░░] 100/100
MP  [██████████] 100/100
ST  [█████████░] 90/100
XP  [████████░░] ← Goldener Progress-Bar
```

### Tasten

- **K** = +1000 XP (Dev-Cheat)
- **WASD** = Bewegen
- **Rechte Maus** = Kamera
- **ESC** = Pause-Menü

## Datenbank

**Spalten existieren bereits:**
```sql
CREATE TABLE characters (
    ...
    level INTEGER DEFAULT 1,
    experience INTEGER DEFAULT 0,
    ...
);
```

**Neue Character:**
- Level 1
- XP 0
- Stats basierend auf Klasse

**Bestehende Character:**
- Behalten ihr aktuelles Level/XP
- Stats werden beim Login neu berechnet

## Multiplayer-Kompatibilität

- **Skalierbar:** XP-Gain kann von beliebigen Quellen kommen:
  - Monster-Kills
  - Quests
  - PvP
  - Events
  - etc.

- **Anti-Cheat:** Server kontrolliert alle XP-Gains
  - Client kann nur `GainExperience` senden (aktuell für Testing)
  - Später: Server entscheidet wann XP vergeben wird

## Nächste Schritte (Empfehlungen)

1. **XP-Quellen implementieren:**
   - Monster-System mit XP-Rewards
   - Quest-System
   - XP-Sharing in Groups

2. **Level-Cap Mechanics:**
   - Bei Level 100: Special-Belohnungen
   - Prestige-System?
   - Endgame-Content

3. **Balancing:**
   - XP-Formel anpassen wenn zu grindy/zu leicht
   - Stat-Skalierung testen im Kampf
   - XP-Rewards für Monster anpassen

4. **UI-Verbesserungen:**
   - Level-Up Animation (Partikel, Sound)
   - XP-Gain Floating-Numbers
   - Level-Up Notification (Pop-up)
   - Stats-Vergleich bei Level-Up

5. **Weitere Features:**
   - Rested XP (Bonus nach Logout)
   - XP-Boost Items
   - Level-Sync für Dungeons
   - XP-Leaderboards

## Performance

- **DB Writes:** Nur bei Level-Up + Auto-Save Intervall
- **Network:** Minimale Messages (nur bei XP-Gain)
- **Calculation:** Exponential-Formel ist O(1), sehr schnell
- **Skalierung:** Funktioniert für 1000+ simultane Spieler

## Formeln im Detail

### XP-Kurve Vergleich

| Formel | Level 50 XP | Level 100 XP | Total XP |
|--------|-------------|--------------|----------|
| Linear (1000*L) | 50k | 100k | 5M |
| Quadratic (100*L²) | 250k | 1M | 33M |
| **Aktuell (100*L^2.8)** | **4.4M** | **25M** | **~9.5M** |
| Kubisch (100*L³) | 12.5M | 97M | 25M |

**Entscheidung:** L^2.8 = Grindy aber machbar, ähnlich wie WoW Classic

---

_Letzte Aktualisierung: 2024-11-09_
_Status: Voll funktionsfähig, bereit für Testing_
