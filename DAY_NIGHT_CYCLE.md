# 🌅 Tag-Nacht-Zyklus System

## Übersicht

Der Tag-Nacht-Zyklus ist vollständig **server-authoritative** - alle Clients sehen immer exakt die gleiche Tageszeit.

## Technische Spezifikationen

### Zeitrechnung
- **Start**: 12:00 Mittags (Noon) beim Serverstart
- **Zykluslänge**: 15 Minuten Echtzeit = 24 Stunden Spielzeit
- **Geschwindigkeit**: 96x beschleunigt (1 reale Sekunde = 1.6 Spielminuten)
- **Update-Frequenz**: Server sendet 1x pro Sekunde `TimeUpdate` an alle Clients

### Sonnenposition
Die Sonne bewegt sich auf einer kreisförmigen Bahn um die Welt:

| Uhrzeit | Position | Beschreibung |
|---------|----------|--------------|
| 06:00 | Osten, Horizont | Sonnenaufgang |
| 12:00 | Süden, Direkt über Kopf | Mittag (höchster Punkt) |
| 18:00 | Westen, Horizont | Sonnenuntergang |
| 00:00 | Norden, Unter Horizont | Mitternacht |

### Licht-Intensitäten

#### Tageszeit-Phasen:
1. **Tiefe Nacht** (00:00 - 05:00)
   - Illuminance: 500 lux
   - Farbe: Kühles Blau (0.5, 0.6, 0.8)
   - Ambient: 50 brightness

2. **Morgengrauen** (05:00 - 07:00)
   - Illuminance: 500 → 10000 lux (Übergang)
   - Farbe: Warm-orange → Gelb-weiß
   - Ambient: 50 → 300 brightness

3. **Tag** (07:00 - 17:00)
   - Illuminance: 10000 lux
   - Farbe: Warmes Sonnenlicht (1.0, 0.95, 0.8)
   - Ambient: 300 brightness

4. **Abenddämmerung** (17:00 - 19:00)
   - Illuminance: 10000 → 500 lux (Übergang)
   - Farbe: Gelb-weiß → Rot-orange
   - Ambient: 300 → 50 brightness

5. **Nacht** (19:00 - 24:00)
   - Illuminance: 500 lux
   - Farbe: Kühles Blau (0.5, 0.6, 0.8)
   - Ambient: 50 brightness

## Implementierung

### Server (`server/src/main.rs`)

```rust
struct GameTime {
    hour: f32,              // 0.0 - 24.0
    start_time: Instant,    // Wann Server gestartet wurde
}

const TIME_SPEED_MULTIPLIER: f32 = 96.0;  // 96x schneller als Echtzeit
const TIME_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
```

**Update-Logik:**
1. Berechnet verstrichene Echtzeit seit Serverstart
2. Multipliziert mit 96.0 für Spielzeit
3. Addiert zu Start-Zeit (12.0)
4. Sendet Update an alle Clients

### Client (`client/src/skybox.rs`)

**Komponenten:**
- `Sun` (Component) - Directional Light für Beleuchtung
- `SunVisual` (Component) - Sichtbares gelbes Sphere-Mesh
- `GameTime` (Resource) - Aktuelle Spielzeit (von Server synchronisiert)

**Systeme:**
1. `setup_skybox` - Spawnt Himmel, Sonne (Light + Visual), Ambient Light
2. `update_sun_position` - Bewegt Sonne basierend auf Uhrzeit
3. `update_ambient_light` - Passt Licht-Intensität und Farbe an

### Shared (`shared/src/lib.rs`)

```rust
ServerMessage::TimeUpdate { hour: f32 }  // 0.0 - 24.0
```

## Visuelle Effekte

### Skybox
- **Typ**: Große Sphere (Radius 500m)
- **Farbe**: Hellblau (0.53, 0.81, 0.92)
- **Material**: Unlit, double-sided
- **Zweck**: Umgibt die gesamte Welt

### Sonne (Visual)
- **Typ**: Sphere (Radius 10m)
- **Farbe**: Gelb (1.0, 0.9, 0.6)
- **Emissive**: Starkes Leuchten (2.0, 1.8, 1.0)
- **Material**: Unlit (immer voll hell)
- **Position**: Synchron mit DirectionalLight

### Sonne (Light)
- **Typ**: DirectionalLight
- **Shadows**: Aktiviert
- **Intensität**: 500-10000 lux (je nach Tageszeit)
- **Farbe**: Dynamisch (warm bei Tag, kühl bei Nacht)

## Debugging

### Logs aktivieren:
```bash
RUST_LOG=info ./run_server.sh
RUST_LOG=info ./run_client.sh
```

### Im Client:
- **F3**: Dev Panel (zeigt FPS, Position)
- Sonnen-Position wird jede Sekunde geloggt
- Format: `☀️ Sun at HH.H:00 - Position: (X, Y, Z)`

### Erwartete Logs:

**Server:**
```
INFO  GameTime: 12.0 hours (Noon)
INFO  Broadcasting time update to 1 players
```

**Client:**
```
INFO  🌅 Setting up skybox and sun...
INFO  ☀️ Sun spawned at Vec3(0.0, 100.0, -50.0)
INFO  ✅ Skybox setup complete!
INFO  ☀️ Sun at 12.0:00 - Position: (0.0, 150.0, -50.0)
INFO  🌞 Light at 12.0:00 - Intensity: 10000 lux
```

## Zeitberechnung

### Server → Client:
```
Echtzeit seit Start: 60 Sekunden
Spielzeit: 60s * 96 = 5760 Sekunden = 96 Minuten = 1.6 Stunden
Start: 12.0 + 1.6 = 13.6 (13:36 Uhr)
```

### Vollständiger Zyklus:
```
15 Minuten Echtzeit = 900 Sekunden
900s * 96 = 86400 Sekunden = 24 Stunden Spielzeit ✅
```

## Koordinatensystem

```
        Y (Up)
        |
        |
        |_______ X (East)
       /
      /
     Z (North)
```

**Sonnenbahn:**
- X-Achse: Ost-West Bewegung
- Y-Achse: Höhe (0 = Horizont)
- Z-Achse: Konstant bei -50 (leicht südlich für bessere Sichtbarkeit)

## Erweiterungsmöglichkeiten

### Zukünftige Features:
- [ ] Dynamische Skybox-Farbe (Gradient je nach Tageszeit)
- [ ] Sterne bei Nacht
- [ ] Mond (gegenüber der Sonne)
- [ ] Nebel in der Dämmerung
- [ ] Partikel-Effekte (Sonnenstrahlen)
- [ ] Wettersystem (Regen, Wolken)
- [ ] Jahreszeiten (längere Tage im Sommer)

## Troubleshooting

### Problem: Sonne nicht sichtbar
**Lösung:** 
- F3 drücken für Debug-Info
- Logs checken: `RUST_LOG=info`
- Prüfen ob SunVisual-Mesh spawned wurde
- Kamera-Position überprüfen (nicht im Boden)

### Problem: Zeit synchronisiert nicht
**Lösung:**
- Server-Logs checken: Broadcast läuft?
- Client-Logs checken: TimeUpdate empfangen?
- Netzwerk-Verbindung testen

### Problem: Beleuchtung zu dunkel/hell
**Lösung:**
- Ambient Light brightness in `skybox.rs` anpassen
- DirectionalLight illuminance anpassen
- Tageszeit im Log überprüfen

## Performance

**Overhead:**
- Server: ~1 KB/s Netzwerk-Traffic (1 Message/Sekunde)
- Client: 2 Update-Systeme pro Frame (minimal)
- Skybox: 1 Sphere Mesh (statisch, keine Updates)
- Sonne: 2 Entities (Light + Visual, Position-Updates)

**Optimierungen:**
- Server sendet nur bei Änderung (aktuell: immer, könnte optimiert werden)
- Client-Systeme nutzen `Res<GameTime>` (keine Query-Overhead)
- Visuelle Sonne nur 1 Mesh (kein Billboard-Sprite)

## Code-Referenzen

**Server:** `server/src/main.rs:7-18` (GameTime Struct)  
**Server:** `server/src/main.rs:548-571` (Time Update Logic)  
**Client:** `client/src/skybox.rs` (Komplettes System)  
**Shared:** `shared/src/lib.rs:456` (TimeUpdate Message)  
**Networking:** `client/src/networking.rs:186-189` (Message Handler)

---

**Version:** 1.0  
**Erstellt:** 2025-11-11  
**Autor:** OpenCode AI  
**Status:** ✅ Production Ready
