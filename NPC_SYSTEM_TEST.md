# NPC System - Test Plan

## ✅ Implementiert

### Neue Dateien
1. `client/src/npc.rs` - NPC Spawning & Nameplate System
2. `client/src/interaction.rs` - Mouse Raycast & Range Check
3. `client/src/ui/npc_dialog.rs` - Dialog UI System

### Erweiterte Dateien
- `client/src/main.rs` - Plugin Registration
- `client/src/auth_state.rs` - class & specialization fields
- `client/src/player.rs` - GameWorld public
- `client/src/ui/mod.rs` - NpcDialogPlugin export
- `client/src/ui/character_selection.rs` - Store class/spec

## 🎮 Test Szenarien

### 1. NPC Spawn Test
**Schritte:**
1. Server starten
2. Client starten
3. Login & Character auswählen
4. Im Spiel spawnen

**Erwartetes Ergebnis:**
- ✅ Goldener NPC bei Position (5, 1, 5)
- ✅ Nameplate "Meister der Künste" über NPC
- ✅ NPC ist sichtbar

### 2. Range Highlight Test
**Schritte:**
1. Im Spiel sein
2. Zum NPC laufen (näher als 3m)
3. Vom NPC weglaufen (weiter als 3m)

**Erwartetes Ergebnis:**
- ✅ NPC glüht golden wenn < 3m entfernt
- ✅ NPC normal wenn > 3m entfernt

### 3. Click Test - Level 1 (unter Level 5)
**Schritte:**
1. Im Spiel sein (Level 1)
2. Zum NPC laufen
3. NPC mit linker Maustaste anklicken

**Erwartetes Ergebnis:**
- ✅ Dialog öffnet sich
- ✅ Titel: "Meister der Künste"
- ✅ Nachricht: "Du musst Level 5 erreichen..."
- ✅ Nur "Schließen" Button sichtbar
- ✅ Keine Spezialisierungs-Buttons

### 4. Click Test - Level 5 (keine Spec gewählt)
**Schritte:**
1. K-Taste drücken (mehrmals für +1000 XP)
2. Bis Level 5 kommen
3. Zum NPC laufen
4. NPC anklicken

**Erwartetes Ergebnis:**
- ✅ Dialog öffnet sich
- ✅ Titel: "Wähle deine Spezialisierung"
- ✅ 2 Spezialisierungs-Karten sichtbar
- ✅ Namen korrekt (z.B. "Leibwächter", "Gladiator" für Krieger)
- ✅ Beschreibungen sichtbar
- ✅ "Wählen" Buttons für beide Specs
- ✅ "Schließen" Button

### 5. Spezialisierungs-Wahl Test
**Schritte:**
1. Dialog offen (Level 5, keine Spec)
2. Eine Spezialisierung wählen (z.B. Leibwächter)
3. "Wählen" Button klicken

**Erwartetes Ergebnis:**
- ✅ Message an Server gesendet
- ✅ Dialog schließt sich
- ✅ Console Log: "Sent specialization choice to server"

### 6. Click Test - Nach Spezialisierungs-Wahl
**Schritte:**
1. Spezialisierung gewählt
2. NPC erneut anklicken

**Erwartetes Ergebnis:**
- ✅ Dialog öffnet sich
- ✅ Titel: "Meister der Künste"
- ✅ Nachricht: "Du hast bereits eine Spezialisierung gewählt: [Name]"
- ✅ Nur "Schließen" Button
- ✅ Keine Spezialisierungs-Buttons mehr

### 7. Range Click Test
**Schritte:**
1. Weit vom NPC entfernt sein (> 3m)
2. NPC anklicken (auch wenn er im Bild ist)

**Erwartetes Ergebnis:**
- ✅ Dialog öffnet NICHT
- ✅ Keine Interaktion möglich
- ✅ Console Log: Nichts

## 🎯 NPC Details

**Position:** (5, 1, 5) - 5 Meter rechts, 5 Meter vorne vom Spawn (0, 1, 0)

**Aussehen:**
- Goldene Kapsel (0.4 Radius, 1.8 Höhe)
- Farbe: RGB(0.9, 0.7, 0.2)
- Glüht bei Nähe: Emissive RGB(0.5, 0.4, 0.1)

**Nameplate:**
- Text: "Meister der Künste"
- Farbe: Golden RGB(1.0, 0.9, 0.3)
- Hintergrund: Braun-transparent RGBA(0.2, 0.1, 0.0, 0.7)
- Position: 2.5 Einheiten über NPC-Kopf

**Interaktions-Range:** 3.0 Meter (global für alle NPCs)

## 📊 Implementierte Features

### NPC System
- ✅ NPC Spawning
- ✅ NPC Nameplate (wie Player)
- ✅ World-to-Screen Position Conversion

### Interaction System
- ✅ Mouse Raycast (Linke Maustaste)
- ✅ Sphere-Ray Intersection Test
- ✅ Range Check (3.0m)
- ✅ Highlight bei Nähe (Emissive Glow)

### Dialog System
- ✅ NpcDialogState Resource
- ✅ Level-basierte Dialoge
- ✅ Spezialisierungs-Auswahl UI
- ✅ Button Handling
- ✅ Auto-Cleanup bei Dialog-Close

### Integration
- ✅ AuthState.class
- ✅ AuthState.specialization
- ✅ Character Selection speichert class/spec
- ✅ NetworkClient Message (ChooseSpecialization)

## 🐛 Bekannte Einschränkungen

1. **Server-Side noch nicht vollständig**
   - Server sendet nur "Not implemented yet"
   - DB speichert Specialization noch nicht
   - Validation fehlt

2. **Nur Spezialisierungs-Trainer**
   - Andere NPC-Typen (Merchant, QuestGiver) noch nicht implementiert
   - Nur ein NPC in der Welt

3. **Visual Feedback**
   - Kein Cursor-Change beim Hover über NPC
   - Keine Partikel-Effekte

## 🚀 Nächste Schritte

1. **Server-Integration (PRIORITÄT)**
   - DB Migration ausführen
   - ChooseSpecialization Handler implementieren
   - Validation (Level >= 5, keine Spec, Spec passt zu Klasse)
   - Specialization in DB speichern & laden

2. **Visual Enhancements**
   - Cursor wechselt zu Hand-Symbol über NPC
   - Partikel-Effekt um NPC
   - Soundeffekt bei Dialog-Öffnung

3. **Mehr NPCs**
   - Quest-Giver NPC
   - Merchant NPC
   - Verschiedene NPC-Typen an verschiedenen Orten

## 📝 Logs zum Debuggen

Bei korrekter Funktionsweise sollten folgende Logs erscheinen:

```
# NPC Spawn
INFO Spawning NPC 'Meister der Künste' at Vec3(5.0, 1.0, 5.0)

# NPC Click (in Range)
INFO Opening dialog with NPC: Meister der Künste

# Spec Wahl
INFO Player chose specialization: Leibwächter
INFO Sent specialization choice to server

# Dialog Close
INFO Closing NPC dialog
```

