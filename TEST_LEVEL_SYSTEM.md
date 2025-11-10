# Level-System Test-Anleitung

## Quick Start

### 1. Server ist bereits gestartet ✓
```bash
# Check:
ps aux | grep target/release/server
# Sollte laufen auf Port 5000
```

### 2. Client starten
```bash
cd /home/max/code/game
./target/release/client
# Oder:
cargo run --release -p client
```

## Test-Szenario

### Neuer Character (Empfohlen)

1. **Login/Register**
   - Username: `testlevel`
   - Password: `testpass123`

2. **Character erstellen**
   - Name: z.B. "Grinder"
   - Klasse wählen:
     - **Warrior** = Viel HP (2080 bei Lvl 100)
     - **Mage** = Viel Mana (1882 bei Lvl 100)
     - **Rogue** = Viel Stamina (1585 bei Lvl 100)
   - "Erstellen ✓"

3. **Im Spiel**
   - Unten links siehst du:
     ```
     Lvl 1 (0/100)
     HP  [████████░░] 100/100  (oder 100 je nach Klasse)
     MP  [██████████] 100/100
     ST  [██████████] 100/100
     XP  [░░░░░░░░░░] <- Goldener Bar (leer)
     ```

### XP-Gain testen

**Taste K drücken = +1000 XP**

#### Level 1→2 (100 XP needed)
```bash
K drücken
# Console:
# [INFO] +1000 XP! (0/200)
# [INFO] 🎉 LEVEL UP! Now level 2
# [INFO]   HP: 100 → 120  (Warrior)
# [INFO]   Mana: 100 → 105
# [INFO]   Stamina: 100 → 112
```

**Was passiert:**
- XP-Bar füllt sich INSTANT auf 100%, dann Level-Up
- Level-Display: "Lvl 1" → "Lvl 2"
- Neue XP-Bar: (0/200) weil Level 2→3 braucht ~200 XP
- HP/Mana/Stamina Bars werden länger (Max-Werte erhöht)
- Alle Bars sind voll (Heilung beim Level-Up!)

#### Level 2→3 (400 XP needed)
```bash
K K K K  (4x drücken)
# [INFO] +1000 XP! (200/775)
# [INFO] +1000 XP! (400/775)
# [INFO] +1000 XP! (600/775)
# [INFO] +1000 XP! (25/1349)  <- Automatischer Level-Up!
# [INFO] 🎉 LEVEL UP! Now level 3
```

#### Schnell auf Level 10

**30-50x K drücken** (kann gedrückt halten!)

```
Level  1→ 2:  1x  K (100 XP)
Level  2→ 3:  4x  K (400 XP)
Level  3→ 4:  7x  K (775 XP)
Level  4→ 5: 11x  K (1349 XP)
Level  5→ 6: 17x  K (2100 XP)
...
Level  9→10: 50x  K (~50k XP)
```

**Bei Level 10:**
- **Warrior:** 280 HP, 145 Mana, 208 Stamina
- **Mage:** 172 HP, 262 Mana, 154 Stamina
- **Rogue:** 208 HP, 172 Mana, 235 Stamina

### Persistenz testen

1. **Im Spiel** (z.B. Level 5)
2. **ESC** → Zum Hauptmenü
3. **Ausloggen**
4. **Wieder einloggen**
5. **Charakter auswählen**
6. **Im Spiel:** Level & XP sind gespeichert! ✓

### Mehrere Level-Ups

**Spamme K 100-200x** (schnell drücken!)

Du wirst sehen:
```
[INFO] 🎉 LEVEL UP! Now level 11
[INFO] 🎉 LEVEL UP! Now level 12  <- Mehrere Level-Ups hintereinander!
[INFO] 🎉 LEVEL UP! Now level 13
```

Das ist normal! Wenn du viel XP auf einmal bekommst, kannst du mehrere Level überspringen.

## Was beobachten

### UI-Updates in Echtzeit

- **XP-Bar**: Füllt sich gold von links nach rechts
- **Level-Text**: "Lvl X (current/needed)"
- **Stat-Bars**: Werden länger bei Level-Up
- **HP/Mana/Stamina**: Werden voll (Heilung)

### Console-Output

```
[INFO client] Sent +1000 XP request (Dev Key 'K')
[INFO client] +1000 XP! (500/775)
[INFO client] +1000 XP! (725/1349)
[INFO client] 🎉 LEVEL UP! Now level 3
[INFO client]   HP: 120 → 140
[INFO client]   Mana: 105 → 110
[INFO client]   Stamina: 112 → 124
```

### Server-Log

```bash
tail -f /home/max/code/game/server.log
```

```
[INFO server] Character 1 gained 1000 XP (total: 1000)
[INFO server] Character 1 leveled up to 2!
[INFO server] Character 1 is now level 2 (HP: 120, Mana: 105, Stamina: 112)
[INFO server] Updated level/XP in database for character 1
```

## Erwartete Werte

### XP-Kurve testen

| Level | XP benötigt | K-Drücke | Gesamt K-Drücke |
|-------|-------------|----------|-----------------|
| 1→2   | 100         | 1        | 1               |
| 2→3   | 400         | 1        | 2               |
| 5→6   | 2,100       | 3        | ~10             |
| 10→11 | 25,000      | 25       | ~50             |
| 20→21 | 410,000     | 410      | ~2,000          |

### Stats-Kurve (Warrior)

| Level | HP   | Mana | Stamina |
|-------|------|------|---------|
| 1     | 100  | 100  | 100     |
| 10    | 280  | 145  | 208     |
| 25    | 580  | 220  | 388     |
| 50    | 1080 | 345  | 688     |
| 100   | 2080 | 595  | 1288    |

## Bekannte Verhaltensweisen

✓ **Mehrfache Level-Ups:** Normal bei viel XP auf einmal
✓ **XP Overflow:** Überschüssige XP wird ins nächste Level übertragen
✓ **Max Level 100:** Bei Level 100 zeigt XP-Bar "100%", xp_needed = 0
✓ **Heilung bei Level-Up:** HP/Mana/Stamina werden voll aufgefüllt
✓ **DB-Speicherung:** Automatisch bei jedem Level-Up

## Fehlersuche

### "Keine Reaktion auf K"
- Bist du im InGame State? (nicht im Menü)
- Check Console für "Sent +1000 XP request"
- Server läuft? (`ps aux | grep server`)

### "XP-Bar bewegt sich nicht"
- Check PlayerStats Resource
- EventReader funktioniert?
- Server sendet ExperienceGained?

### "Stats ändern sich nicht"
- LevelUp Event kommt an?
- Check Console für "LEVEL UP!"
- Max-Stats werden aktualisiert?

## Performance-Test

**Extremer XP-Gain:**
```bash
# Im Spiel: K 500x schnell drücken
# Server sollte mehrere Level-Ups verarbeiten
# Client sollte smooth updaten
# Kein Lag/Freeze
```

**Erwartetes Ergebnis:**
- ~50+ Level-Ups in Sekunden
- Alle Updates korrekt
- Keine Crashes
- DB korrekt updated

---

**Viel Spaß beim Grinden! 🎮**
