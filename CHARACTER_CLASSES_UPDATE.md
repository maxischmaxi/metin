# Charakterklassen Update - Metin2 Style ✅

## Änderungen

### Alte Klassen → Neue Klassen

| Alt      | Neu       | Beschreibung                    |
|----------|-----------|----------------------------------|
| Warrior  | Krieger   | Tanky warrior, low mana         |
| Mage     | Ninja     | Agile assassin, high stamina    |
| Rogue    | Sura      | Balanced magic warrior          |
| -        | Schamane  | Shaman healer, high mana (NEU)  |

## Stats pro Level

### Krieger (Warrior)
- **HP/Level**: +20 (am meisten!)
- **Mana/Level**: +5 (am wenigsten)
- **Stamina/Level**: +12
- **Rolle**: Tank, Nahkampf

### Ninja (ehemals Mage)
- **HP/Level**: +12
- **Mana/Level**: +8
- **Stamina/Level**: +15 (am meisten!)
- **Rolle**: Agiler Assassine, schnelle Angriffe

### Sura (ehemals Rogue)
- **HP/Level**: +15
- **Mana/Level**: +12
- **Stamina/Level**: +10
- **Rolle**: Ausgewogener Magie-Krieger

### Schamane (NEU)
- **HP/Level**: +8 (am wenigsten)
- **Mana/Level**: +18 (am meisten!)
- **Stamina/Level**: +8
- **Rolle**: Heiler/Support, viel Mana

## Geänderte Dateien

1. **shared/src/lib.rs**
   - `CharacterClass` enum erweitert
   - `calculate_stats_for_level()` angepasst
   - `as_str()` aktualisiert

2. **client/src/ui/character_creation.rs**
   - 4 Klassen-Buttons statt 3
   - Button-Namen aktualisiert
   - Class-Display aktualisiert

3. **server/src/db/characters.rs**
   - DB-zu-Enum Konvertierung angepasst

4. **server/src/main.rs**
   - Klassen-Konvertierung bei SelectCharacter
   - Klassen-Konvertierung bei GainExperience

5. **server/src/auth/handlers.rs**
   - Klassen-Konvertierung bei Login

## Datenbank-Migration

Bestehende Characters wurden automatisch konvertiert:
```sql
UPDATE characters SET class = 'Krieger' WHERE class = 'Warrior';
UPDATE characters SET class = 'Ninja' WHERE class = 'Mage';
UPDATE characters SET class = 'Sura' WHERE class = 'Rogue';
```

## Character Creation UI

Jetzt mit **4 Buttons**:

```
┌──────────────────────────────────────┐
│     Charakter erstellen              │
│                                      │
│  Name: [Hero_]                       │
│                                      │
│  Klasse wählen:                      │
│  [Krieger] [Ninja] [Sura] [Schamane]│
│                                      │
│  Gewählte Klasse: Krieger            │
│                                      │
│  [Erstellen ✓]    [← Zurück]         │
└──────────────────────────────────────┘
```

## Test-Anleitung

1. **Server neu starten** (damit neue Klassen geladen werden):
   ```bash
   pkill -f "target.*server"
   cd /home/max/code/game
   ./run_server.sh
   ```

2. **Client starten**:
   ```bash
   ./run_client.sh
   ```

3. **Neuen Character erstellen**:
   - Login
   - "Neuen Charakter erstellen"
   - Wähle eine der 4 Klassen:
     - **Krieger** (viel HP)
     - **Ninja** (viel Stamina)
     - **Sura** (ausgewogen)
     - **Schamane** (viel Mana)

4. **Stats testen**:
   - Im Spiel: Drücke `K` für +1000 XP
   - Bei Level-Up: Verschiedene Klassen haben unterschiedliche Stat-Zuwächse
   - Krieger bekommt +20 HP, Schamane nur +8 HP aber +18 Mana

## Stat-Beispiele (Level 10)

### Krieger Lvl 10
- HP: 100 + (9 × 20) = **280**
- Mana: 100 + (9 × 5) = **145**
- Stamina: 100 + (9 × 12) = **208**

### Ninja Lvl 10
- HP: 100 + (9 × 12) = **208**
- Mana: 100 + (9 × 8) = **172**
- Stamina: 100 + (9 × 15) = **235**

### Sura Lvl 10
- HP: 100 + (9 × 15) = **235**
- Mana: 100 + (9 × 12) = **208**
- Stamina: 100 + (9 × 10) = **190**

### Schamane Lvl 10
- HP: 100 + (9 × 8) = **172**
- Mana: 100 + (9 × 18) = **262**
- Stamina: 100 + (9 × 8) = **172**

## Kompatibilität

- ✅ Bestehende Characters wurden konvertiert
- ✅ Alte "Warrior" → "Krieger"
- ✅ Alle Server-Endpoints aktualisiert
- ✅ Client-UI zeigt neue Namen

## Zukünftige Erweiterungen

- [ ] Klassen-spezifische Skills
- [ ] Klassen-spezifische Waffen
- [ ] Klassen-spezifische Rüstungen
- [ ] Unterschiedliche Startpositionen
- [ ] Klassen-Quest-Lines

