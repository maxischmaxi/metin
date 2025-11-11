# 🌅 Tag-Nacht-Zyklus - Quick Start

## So siehst du die Sonne! ☀️

### 1. Starte Server und Client

**Terminal 1 (Server):**
```bash
./run_server.sh
```

**Terminal 2 (Client):**
```bash
./run_client.sh
```

### 2. Einloggen & Character auswählen
1. Login mit deinem Account
2. Wähle einen Character aus
3. Warte bis du im Spiel bist

### 3. Sonne finden! 🔍

Die Sonne ist **groß, gelb und leuchtet**!

**Wo schauen:**
- **12:00 (Start)**: Schaue nach **OBEN** (Mittag, Sonne direkt über dir)
- **15:00**: Schaue **WEST** (Sonne wandert nach Westen)
- **18:00**: Schaue zum **Horizont im Westen** (Sonnenuntergang)
- **06:00**: Schaue zum **Horizont im Osten** (Sonnenaufgang)

**Kamera-Steuerung:**
- **Rechte Maustaste + Maus bewegen**: Kamera drehen
- **Mausrad**: Zoom
- Drehe die Kamera komplett um den Charakter!

### 4. Zeit überprüfen

**Option 1: Warte und beobachte**
- Die Sonne bewegt sich LANGSAM
- 15 Minuten Echtzeit = 1 voller Tag
- In ~2 Minuten siehst du deutliche Bewegung

**Option 2: Logs ansehen**
```bash
# In einem neuen Terminal:
tail -f server.log | grep "☀️"
```

**Option 3: F3 Debug Panel**
- Drücke **F3** im Client
- Zeigt FPS und Position
- (Uhrzeit könnte später hinzugefügt werden)

### 5. Schnelltest - Zeit beschleunigen

Wenn du es SOFORT sehen willst, kannst du den Code temporär ändern:

**In `server/src/main.rs` Zeile 10:**
```rust
// Original (15 Minuten = 1 Tag):
const TIME_SPEED_MULTIPLIER: f32 = 96.0;

// Schnelltest (1 Minute = 1 Tag):
const TIME_SPEED_MULTIPLIER: f32 = 1440.0;
```

Dann neu kompilieren:
```bash
cargo build --release -p server
./run_server.sh
```

Jetzt bewegt sich die Sonne **15x schneller**! 🚀

## Was du sehen solltest:

### 12:00 (Start)
```
     ☀️ <- Sonne direkt oben
      |
      |
     🧍 <- Du
```

### 15:00 (Nachmittag)
```
         ☀️ <- Sonne wandert nach Westen
        /
       /
     🧍 <- Du
```

### 18:00 (Sonnenuntergang)
```
☀️____________ <- Sonne am Horizont
     🧍 <- Du
```

### 21:00 (Nacht)
```
           (Sonne unter Horizont)
🌑___________ <- Dunkel
     🧍 <- Du
```

## Visueller Check:

### Helligkeit ändert sich:
- ✅ **Mittag (12:00)**: SEHR hell, harte Schatten
- ✅ **Dämmerung (18:00)**: Orange/rötliches Licht
- ✅ **Nacht (00:00)**: Dunkel, bläuliches Licht
- ✅ **Morgengrauen (06:00)**: Orange/rötliches Licht

### Schatten bewegen sich:
- Die Schatten der Gebäude drehen sich mit der Sonne!
- Mittags: Kurze Schatten (Sonne oben)
- Abends: Lange Schatten (Sonne niedrig)

## Häufige Probleme:

### "Ich sehe keine Sonne!"
1. **Drehe die Kamera um!** (Rechte Maustaste + Maus)
2. Schaue nach OBEN (bei 12:00 Mittag)
3. Warte 1-2 Minuten, die Sonne bewegt sich
4. Prüfe Logs: `tail -f client.log | grep "☀️"`

### "Es ist zu dunkel!"
- Warte bis 12:00 Mittag (Start-Zeit)
- Oder warte 7.5 Minuten bis nächster Mittag

### "Beleuchtung ändert sich nicht"
- Server-Verbindung prüfen
- Logs checken: `RUST_LOG=info ./run_server.sh`
- TimeUpdate Messages sollten erscheinen

## Fun Facts:

- **1 Echtzeit-Sekunde** = 1.6 Spiel-Minuten
- **1 Echtzeit-Minute** = 96 Spiel-Minuten = 1.6 Spiel-Stunden
- **15 Echtzeit-Minuten** = 24 Spiel-Stunden = 1 voller Tag
- **Pro Stunde Spielzeit** = ~4 komplette Tag-Nacht-Zyklen!

## Nächste Schritte:

Sobald du die Sonne siehst:
- 🌅 Beobachte einen kompletten Sonnenaufgang (5:00-7:00)
- 🌇 Beobachte einen kompletten Sonnenuntergang (17:00-19:00)
- 🌙 Erkunde die Stadt bei Nacht
- ⏱️ Messe wie lange ein voller Zyklus dauert

**Viel Spaß!** 🎮🌞
