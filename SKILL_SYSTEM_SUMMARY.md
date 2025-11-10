# 🎮 Skill-System & Spezialisierungen - Vollständige Übersicht

## ✅ Status: Design & Grundlagen Implementiert

Das komplette Skill-System wurde **designt und teilweise implementiert**. Die Grundlage steht, die volle Funktionalität folgt in den nächsten Schritten.

---

## 📊 Übersicht aller Klassen & Spezialisierungen

### 🗡️ KRIEGER
| Spezialisierung | Typ | Fokus | Skills (5) |
|----------------|-----|-------|------------|
| **Leibwächter** | PvM Tank | Monster tanken, Gruppe schützen | Schildwall, Provokation, Erderschütterung, Eiserne Haut, Letzte Bastion |
| **Gladiator** | PvP Damage | Burst-Schaden gegen Spieler | Wirbelsturm, Kriegsschrei, Hinrichtung, Raserei, Tödlicher Stoß |

### 🥷 NINJA
| Spezialisierung | Typ | Fokus | Skills (5) |
|----------------|-----|-------|------------|
| **Bogenschütze** | Fernkampf | Distanz-DPS, Kiting | Präzisionsschuss, Pfeilhagel, Giftpfeil, Rückwärtssprung, Durchschlag |
| **Attentäter** | Nahkampf | Burst, Kritische Treffer | Schattenschritt, Dolchwirbel, Tödliche Gifte, Unsichtbarkeit, Gnadenstoß |

### 🔥 SURA
| Spezialisierung | Typ | Fokus | Skills (5) |
|----------------|-----|-------|------------|
| **Dämonen-Jäger** | PvM | Monster-Damage, Lebensraub | Flammenschlag, Seelenraub, Zauberklinge, Dunkler Schutz, Dämonische Verwandlung |
| **Blutkrieger** | PvP | Healing Reduction, Damage | Blutgier, Seelenketten, Vampirschlag, Furchtaura, Seelenernte |

### ⚡ SCHAMANE
| Spezialisierung | Typ | Fokus | Skills (5) |
|----------------|-----|-------|------------|
| **Lebenshüter** | Support | Gruppe heilen, Buffs | Heilende Welle, Naturschild, Erneuerung, Segnung der Natur, Wiedergeburt |
| **Sturmrufer** | PvP Damage | Elemental-Schaden, CC | Blitzschlag, Kettenblitz, Tornado, Erdspieße, Zorn der Elemente |

**Gesamt:** 4 Klassen × 2 Specs = **8 Spezialisierungen** × 5 Skills = **40 einzigartige Skills**

---

## 🎯 Skill-Freischaltung

| Skill-Slot | Level | Beispiel (Leibwächter) |
|------------|-------|------------------------|
| Slot 1 | 5 | Schildwall |
| Slot 2 | 10 | Provokation |
| Slot 3 | 15 | Erderschütterung |
| Slot 4 | 25 | Eiserne Haut |
| Slot 5 | 40 | Letzte Bastion |

**Regel:** Spezialisierung wird **ab Level 5** gewählt und ist **permanent**!

---

## 📖 Beispiel-Skills (Detailliert)

### Leibwächter (Krieger PvM)

#### 1. Schildwall (Lvl 5)
```
Cooldown: 15s | Mana: 20
Effekt: -50% eingehender Schaden für 5s
Visuell: Blauer Schild-Effekt
```

#### 2. Provokation (Lvl 10)
```
Cooldown: 10s | Mana: 25
Effekt: Zwingt Monster (10m Radius) den Spieler anzugreifen
Visuell: Roter Kreis-Puls
```

#### 5. Letzte Bastion (Lvl 40 - Ultimate)
```
Cooldown: 60s | Mana: 80
Effekt: Bei tödlichem Schaden → Überlebt mit 1 HP, +100% Verteidigung (5s)
Visuell: Goldener Wiederauferstehungs-Effekt
```

### Sturmrufer (Schamane PvP)

#### 1. Blitzschlag (Lvl 5)
```
Cooldown: 6s | Mana: 25
Effekt: Ruft Blitz auf Feind, 150% Schaden, 20% Betäubung (1s)
Visuell: Blau-weißer Blitz vom Himmel
```

#### 5. Zorn der Elemente (Lvl 40 - Ultimate)
```
Cooldown: 45s | Mana: 80
Effekt: Kanalisiert 5s, 8 Meteore fallen (12m Radius), je 150 Schaden
Visuell: Feurige Meteore, Explosionen, Rauch
```

---

## 🛠️ Implementierungs-Status

### ✅ Komplett implementiert:

1. **Datenstrukturen** (shared/src/lib.rs)
   - `enum Specialization` (8 Spezialisierungen)
   - `enum SkillId` (40 Skills)
   - `struct SkillInfo` (Name, Beschreibung, Cooldown, Mana, etc.)
   - `enum SkillEffect` (Damage, Buffs, Debuffs, CC, etc.)
   
2. **CharacterClass Erweiterungen**
   - `specializations()` → Gibt beide Spec-Namen zurück
   
3. **Specialization Methods**
   - `from_class_and_index()` → Wählt Spec basierend auf Klasse & Index
   - `name()` → Display-Name (z.B. "Leibwächter")
   - `description()` → Kurzbeschreibung
   - `skills()` → Vec<SkillId> (5 Skills)
   
4. **SkillId Methods**
   - `info()` → Gibt vollständige SkillInfo zurück
   - Cooldown, Mana-Cost, Required Level, Damage-Multiplier, Effect
   
5. **Network Messages**
   - `ClientMessage::ChooseSpecialization`
   - `ServerMessage::SpecializationChosen/Failed`
   - CharacterData.specialization
   - CharacterSummary.specialization
   
6. **Datenbank**
   - Migration 003: ALTER TABLE characters ADD specialization
   - Index für schnelle Lookups

### 🔄 In Arbeit:

- Server-side Specialization Choice Handler
- Client-side Specialization Choice UI
- Skill-Bar im Game-UI

### 📅 Geplant:

- Skill-Aktivierung (Hotkeys 1-5)
- Cooldown-System
- Visual Effects
- Damage-Calculations

---

## 🎨 UI-Konzept

### Spezialisierungs-Wahl (Level 5)
```
┌─────────────────────────────────────────────────────────┐
│  Glückwunsch! Du hast Level 5 erreicht!                │
│  Wähle deine Spezialisierung (permanent!):             │
│                                                         │
│  ┌──────────────────┐     ┌──────────────────┐        │
│  │  LEIBWÄCHTER     │     │   GLADIATOR      │        │
│  │  ────────────    │     │   ─────────      │        │
│  │  PvM Tank        │     │   PvP Damage     │        │
│  │                  │     │                  │        │
│  │  Monster tanken, │     │  Burst-Schaden   │        │
│  │  Gruppe schützen │     │  gegen Spieler   │        │
│  │                  │     │                  │        │
│  │  [Wählen]        │     │  [Wählen]        │        │
│  └──────────────────┘     └──────────────────┘        │
│                                                         │
│  Skills: Schildwall, Provokation, ...                  │
└─────────────────────────────────────────────────────────┘
```

### Skill-Bar im Spiel
```
Bottom-Bar (bereits vorhanden) + Skill-Slots:

[HP][MP][ST]  [1][2][3][4][5][6][7][8][9]  [Map][Inv][Menu]
              ↑─── Ability Slots ────↑
                   (bereits da)
                   
→ Slots 1-5 werden mit Spec-Skills gefüllt
→ Grau = Noch nicht freigeschaltet (Level zu niedrig)
→ Grün = Verfügbar
→ Cooldown: Rote Überlagerung mit Countdown
```

---

## 🔥 Coole Features

1. **Permanente Wahl**: Einmal gewählt, bleibt Spezialisierung für immer
2. **Stufen-Freischaltung**: Skills schalten bei Level 5, 10, 15, 25, 40 frei
3. **Balancierung**: Jede Spec hat klare Stärken/Schwächen
4. **Vielfalt**: 8 völlig unterschiedliche Spielstile
5. **PvM vs PvP**: Klare Trennung der Spezialisierungen

---

## 📚 Dokumentation

- **SKILL_SYSTEM_DESIGN.md** - Vollständige Skill-Beschreibungen
- **SKILL_SYSTEM_IMPLEMENTATION.md** - TODO-Liste für Implementation
- **CHARACTER_CLASSES_UPDATE.md** - Klassen-Übersicht

---

## 🚀 Wie man es testet (wenn fertig)

1. **Character auf Level 5 bringen**
   ```bash
   # Im Spiel: K-Taste drücken (mehrmals für +1000 XP)
   # Bis Level 5
   ```

2. **Spezialisierung wählen**
   ```
   # Bei Level 5: UI öffnet sich automatisch
   # Wähle zwischen 2 Optionen
   # Click "Wählen"
   ```

3. **Skills testen**
   ```
   # Drücke 1-5 auf Tastatur
   # Nur Skills für aktuelles Level sind aktiv
   # Cooldowns werden angezeigt
   ```

---

## 🎯 Design-Philosophie

- **Metin2-inspiriert**: 2 Specs pro Klasse
- **WoW-ähnlich**: Skill-Trees mit Level-Freischaltung
- **Diablo-Style**: Action-Bar mit Hotkeys
- **MMORPG-Standard**: Cooldowns, Mana-Kosten, Effekte

---

**Status:** System designt, Grundlagen implementiert, UI & Logik folgen! 🎉

