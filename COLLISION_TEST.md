# Collision System - Test-Anleitung (Phase 1 & 2)

## ✅ Status: Detection & Resolution funktionieren!

## 🎮 Schnellstart

### 1. Server starten
```bash
cd /home/max/code/game
./run_server.sh
```

### 2. Client starten
```bash
cd /home/max/code/game
./run_client.sh
# Oder neu kompiliert:
./target/release/client
```

---

## 🧪 Test-Szenarien

### Test 1: Player-NPC Collision

**Objekt:** Goldener NPC "Meister der Künste"  
**Position:** (5, 1, 5) - 5 Meter rechts, 5 Meter vorne

**Schritte:**
1. Login/Register
2. Character erstellen/wählen
3. Im Spiel spawnen (Position: 0, 1, 0)
4. **Mit W+D** zum goldenen NPC laufen (rechts-vorne)
5. Direkt auf den NPC zu laufen

**Erwartetes Ergebnis:**
- Console Log erscheint:
  ```
  [INFO client::collision] Collision started: Entity(3v1) <-> Entity(5v1) (penetration: 0.XX)
  ```
- Penetration-Wert zeigt wie tief die Entities überlappen
- Beim Weglaufen:
  ```
  [INFO client::collision] Collision ended: Entity(3v1) <-> Entity(5v1)
  ```

**Status:** ✅ Collision Detection & Resolution funktionieren!  
**Ergebnis:** Player stoppt vor NPC (~0.9m Abstand) und kann nicht durchlaufen

---

### Test 2: Player-Baum Collision

**Objekt:** Brauner Zylinder (Baum)  
**Position:** (-3, 1, 3) - 3 Meter links, 3 Meter vorne

**Schritte:**
1. Im Spiel sein
2. **Mit W+A** nach links-vorne laufen
3. Direkt auf den braunen Baum zu

**Erwartetes Ergebnis:**
- Console Log: Collision detected
- Baum-Collider:
  - Shape: Cylinder (radius: 0.3, height: 2.0)
  - Type: Static

---

### Test 3: Player-Stein Collision

**Objekt:** Grauer Sphere (Stein)  
**Position:** (3, 0.5, -3) - 3 Meter rechts, 3 Meter hinten

**Schritte:**
1. Im Spiel sein
2. **Mit S+D** nach rechts-hinten laufen
3. Zum grauen Stein laufen

**Erwartetes Ergebnis:**
- Console Log: Collision detected
- Stein-Collider:
  - Shape: Sphere (radius: 0.5)
  - Type: Static

---

### Test 4: Player-Wand Collision

**Objekt:** Beige Box (Wand)  
**Position:** (0, 1, -8) - 8 Meter nach hinten (Norden)

**Schritte:**
1. Im Spiel sein
2. **Mit S** direkt nach hinten laufen
3. Zur beigen Wand laufen

**Erwartetes Ergebnis:**
- Console Log: Collision detected
- Wand-Collider:
  - Shape: Box (6m breit × 2m hoch × 0.5m tief)
  - Type: Static

---

## 🗺️ Welt-Layout

```
           N (Z-)
           ↑
           |
    
    Baum   |    
    (-3,3) |
           |
           |
W ← ───────┼───────0,0──── → E
           |     (Spawn)
           |
           |        NPC
           |       (5,5)
           |
           |    Stein
           ↓    (3,-3)
           S (Z+)

   Wand bei (0, -8): ======
```

**Objekte:**
- **Spawn Point:** (0, 1, 0) - Blaue Kapsel (Player)
- **NPC:** (5, 1, 5) - Goldene Kapsel
- **Baum:** (-3, 1, 3) - Brauner Zylinder
- **Stein:** (3, 0.5, -3) - Grauer Sphere
- **Wand:** (0, 1, -8) - Beige Box

---

## 📊 Console Output Verstehen

### Collision Started
```
[INFO client::collision] Collision started: Entity(3v1) <-> Entity(5v1) (penetration: 0.42)
```

**Bedeutung:**
- `Entity(3v1)` - Player Entity ID
- `Entity(5v1)` - NPC/Objekt Entity ID
- `penetration: 0.42` - Wie tief Entities überlappen (in Metern)

**Penetration-Werte:**
- `0.01 - 0.1` - Leichte Berührung
- `0.1 - 0.5` - Mittlere Überlappung
- `0.5+` - Starke Überlappung (Entity ist weit "innen")

### Collision Ended
```
[INFO client::collision] Collision ended: Entity(3v1) <-> Entity(5v1)
```

**Bedeutung:**
- Entities berühren sich nicht mehr
- CollidingWith wurde aktualisiert

---

## 🔍 Debug-Tipps

### 1. Collision Count

Zähle wie viele Collisions aktiv sind:
```rust
// Optional in collision.rs hinzufügen:
fn debug_collision_count(query: Query<&CollidingWith>) {
    let total: usize = query.iter().map(|c| c.entities.len()).sum();
    if total > 0 {
        info!("Total active collisions: {}", total / 2); // /2 weil beide Entities tracken
    }
}
```

### 2. Wer kollidiert mit wem?

```rust
fn debug_colliding_entities(
    query: Query<(Entity, &CollidingWith), With<Player>>,
) {
    for (entity, colliding) in query.iter() {
        if !colliding.entities.is_empty() {
            info!("Player {:?} colliding with: {:?}", entity, colliding.entities);
        }
    }
}
```

### 3. Collision Position

Die Contact-Point Information ist im `CollisionStarted` Event:
```rust
fn debug_collision_position(
    mut events: EventReader<CollisionStarted>,
) {
    for event in events.read() {
        info!("Collision at position: {:?}", event.contact_point);
    }
}
```

---

## ⚠️ Bekannte Verhaltensweisen

### 1. ~~Player läuft durch Objekte~~ ✅ BEHOBEN!

**War das ein Bug?** ✅ Ja, ist jetzt behoben!

**Erklärung:**
- Phase 1 hatte nur **Collision Detection**
- Phase 2 hat **Collision Resolution** hinzugefügt
- Collision wird erkannt UND resolved!

**Sichtbar:**
- Console Logs erscheinen ✅
- Player stoppt vor Objekten ✅
- Keine Überlappung mehr ✅

### 2. Viele Collision Events beim Durchlaufen

**Ist das ein Bug?** ❌ Nein!

**Erklärung:**
- `CollisionStarted` beim Eintritt
- Jedes Frame wird State aktualisiert
- `CollisionEnded` beim Verlassen

**Normal:**
```
[INFO] Collision started: ... (penetration: 0.05)
[INFO] Collision started: ... (penetration: 0.23)  <- wird tiefer
[INFO] Collision started: ... (penetration: 0.41)
[INFO] Collision started: ... (penetration: 0.12)  <- wird flacher
[INFO] Collision ended: ...
```

### 3. Penetration-Wert ändert sich

**Ist das ein Bug?** ❌ Nein!

**Erklärung:**
- Penetration = wie tief Entities überlappen
- Ändert sich während Bewegung
- Maximum wenn Entities direkt übereinander

---

## 🎯 Erfolgs-Kriterien

Phase 1 & 2 sind erfolgreich wenn:

**Phase 1 - Detection:**
- ✅ Console Logs erscheinen beim Berühren von Objekten
- ✅ `CollisionStarted` Event beim Eintritt
- ✅ `CollisionEnded` Event beim Verlassen
- ✅ Penetration-Werte sind sinnvoll (0.01 - 1.0)
- ✅ Kein Crash beim Kollidieren
- ✅ Alle 4 Test-Objekte funktionieren

**Phase 2 - Resolution:**
- ✅ Player stoppt vor Objekten
- ✅ Keine Überlappung mehr
- ✅ Push-back funktioniert
- ✅ Player kann um Objekte herum laufen
- ✅ Kein "Durchschlüpfen" möglich

**NICHT erwartbar in Phase 2:**
- ⏳ Automatisches Wall Sliding (Phase 2.5)
- ⏳ Spatial Partitioning (Phase 3)
- ⏳ Collision Layers (Phase 3)

---

## 🚀 Nächste Schritte

Nach erfolgreichem Test von Phase 1 & 2:

### ~~Phase 2: Collision Resolution~~ ✅ KOMPLETT!
- ~~Player stoppt vor Static Objects~~ ✅
- ~~Dynamic-Dynamic Push-back~~ ✅
- ~~Push-back Strength~~ ✅

### Phase 2.5: Wall Sliding (Optional)
- Automatisches Gleiten an Wänden
- Tangent-Projection
- Smoothere Bewegung

### Phase 3: Optimierung
- Spatial Partitioning (Grid)
- Collision Layers/Masks
- Broad Phase AABB
- Performance: 1000+ Entities

---

## 📝 Test-Checkliste

**Phase 1 - Detection:**
- [ ] Server läuft
- [ ] Client startet ohne Fehler
- [ ] Player spawnt korrekt
- [ ] NPC-Collision wird geloggt
- [ ] Baum-Collision wird geloggt
- [ ] Stein-Collision wird geloggt
- [ ] Wand-Collision wird geloggt
- [ ] CollisionEnded wird geloggt
- [ ] Penetration-Werte sind positiv
- [ ] Kein Crash beim Kollidieren

**Phase 2 - Resolution:**
- [ ] Player stoppt vor NPC (~0.9m Abstand)
- [ ] Player kann NICHT durch NPC laufen
- [ ] Player stoppt vor Baum
- [ ] Player stoppt vor Stein
- [ ] Player stoppt vor Wand
- [ ] Player kann um Objekte herum laufen
- [ ] Kein "Durchschlüpfen" möglich
- [ ] Console Logs wie vorher

**Alle Checkboxen erfüllt?** → Phase 1 & 2 sind KOMPLETT! ✅

---

_Erstellt: 2024-11-10_
_Letztes Update: 2024-11-10_
_Status: Phase 1 & 2 - Detection & Resolution ✅_
