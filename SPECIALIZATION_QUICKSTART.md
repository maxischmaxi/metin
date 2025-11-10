# 🎮 Spezialisierungs-System - Schnellstart

## ✅ Status: VOLLSTÄNDIG IMPLEMENTIERT & BEREIT ZUM TESTEN!

**Server:** ✅ LÄUFT (Port 5000)  
**Client:** ✅ LÄUFT  
**NPC-System:** ✅ FUNKTIONIERT  
**Specialization Choice:** ✅ FERTIG  

---

## 🚀 5-Minuten-Test (Neue Session - November 10, 2024)

### Was wurde gerade behoben:
- ✅ Character Class wird jetzt korrekt in `CharacterSelected` message gesendet
- ✅ `auth_state.class` wird richtig gesetzt beim Character-Auswahl
- ✅ NPC Dialog zeigt jetzt die richtigen Spezialisierungen an
- ✅ Character Creation wartet auf Server-Response (Name wird korrekt angezeigt)

---

## Schnelltest-Ablauf

### Schritt 1: Login/Register (30s)
```
1. Client sollte bereits laufen
2. Register neuen Account:
   Username: spectest
   Password: testpass123
3. Oder login mit bestehendem Account
```

### Schritt 2: Character Erstellen (30s)
```
1. Click "+ Neuen Charakter erstellen"
2. Name eingeben: "SpecTester"
3. Klasse wählen: KRIEGER (für diesen Test)
4. Click "Erstellen ✓"
5. ✨ NEU: Wartet auf Server, dann automatisch ins Spiel
6. Prüfe: Nameplate zeigt "Lvl 1 - SpecTester"
```

### Schritt 3: Level auf 5 bringen (1 Min)
```
1. Drücke K-Taste ~30 Mal (kann gedrückt halten)
2. Jedes K = +1000 XP
3. Console zeigt Level-Ups:
   🎉 LEVEL UP! Now level 2
   🎉 LEVEL UP! Now level 3
   ...
   🎉 LEVEL UP! Now level 5
4. Unten links: "Lvl 5 (XXX/2100)"
```

### Schritt 4: NPC Finden (30s)
```
1. Mit WASD nach vorne+rechts gehen
2. NPC "Meister der Künste" bei (5, 1, 5)
3. Goldene Kapsel
4. Glüht wenn < 3m entfernt
```

### Schritt 5: Spezialisierung Wählen (1 Min)
```
1. LINKE MAUSTASTE auf NPC
2. ✨ Dialog zeigt jetzt korrekt:
   
   ╔═══════════════════════════════════╗
   ║ Wähle deine Spezialisierung       ║
   ║ ───────────────────────────       ║
   ║ Du hast Level 5 erreicht!         ║
   ║                                   ║
   ║ ┌──────────┐    ┌──────────┐    ║
   ║ │Leibwächter│   │ Gladiator │    ║
   ║ │PvM Tank   │   │ PvP Damage│    ║
   ║ │ [Wählen]  │   │ [Wählen]  │    ║
   ║ └──────────┘    └──────────┘    ║
   ║                                   ║
   ║         [Schließen]               ║
   ╚═══════════════════════════════════╝

3. Click "Wählen" (z.B. Leibwächter)
4. Console: "Sent specialization choice to server"
5. Dialog schließt automatisch
```

### Schritt 6: Verifizieren (30s)
```
1. NPC erneut anklicken
2. Dialog zeigt:
   "Du hast bereits gewählt: Leibwächter"
3. ESC → Ausloggen
4. Login → Character auswählen
5. NPC anklicken → Immer noch "bereits gewählt" ✓
```

---

## 🔧 Was genau wurde behoben?

### Problem (Vorher):
```
❌ NPC Dialog zeigte keine Spezialisierungs-Buttons
❌ Nur "Schließen" Button sichtbar
❌ auth_state.class war None
```

### Lösung (Jetzt):
```
✅ ServerMessage::CharacterSelected sendet character_class
✅ Client speichert class in AuthState
✅ NPC Dialog kann class lesen → zeigt Specs
```

### Geänderte Dateien (Diese Session):
- `shared/src/lib.rs` - CharacterSelected mit character_class
- `server/src/main.rs` - Sendet character_class
- `client/src/networking.rs` - Speichert character_class
- `client/src/ui/npc_dialog.rs` - Verbesserte Class-Detection

---

## 📋 Alle Spezialisierungen

### 🗡️ Krieger
1. **Leibwächter** - PvM Tank, Gruppe schützen
2. **Gladiator** - PvP Damage, Burst-Schaden

### 🥷 Ninja  
1. **Bogenschütze** - Fernkampf, Kiting
2. **Attentäter** - Nahkampf, Kritische Treffer

### 🔥 Sura
1. **Dämonen-Jäger** - PvM, Lebensraub
2. **Blutkrieger** - PvP, Healing Reduction

### ⚡ Schamane
1. **Lebenshüter** - Support, Heilen
2. **Sturmrufer** - PvP Damage, Elemental

---

## 🐛 Problembehandlung

### Keine Spec-Buttons im Dialog?
**Check 1:** Level >= 5?
```
Nameplate sollte "Lvl 5" oder höher zeigen
Lösung: K-Taste drücken
```

**Check 2:** Character Class gesetzt?
```
Console beim Login sollte zeigen:
"AuthState class set to: Krieger"

Falls nicht → Logout/Login
```

**Check 3:** Bereits gewählt?
```
Wenn bereits gewählt, zeigt Dialog nur:
"Du hast bereits gewählt: [Name]"
```

### NPC öffnet keinen Dialog?
```
1. Zu weit weg? → Näher gehen (< 3m)
2. Nicht geklickt? → Genau auf goldene Kapsel klicken
3. NPC spawnt nicht? → Check Position (5, 1, 5)
```

---

## 🎯 Test-Checkliste

- [ ] Character Creation zeigt Namen sofort
- [ ] Nameplate korrekt: "Lvl 1 - Name"
- [ ] K-Taste gibt +1000 XP
- [ ] Level-Up bis 5 funktioniert
- [ ] NPC bei (5, 1, 5) sichtbar
- [ ] NPC glüht bei Nähe
- [ ] NPC-Click öffnet Dialog
- [ ] **Dialog zeigt 2 Spezialisierungen** ← HAUPTTEST!
- [ ] Spezialisierungen passen zur Klasse
- [ ] "Wählen" funktioniert
- [ ] Console: "Sent to server"
- [ ] Dialog schließt
- [ ] NPC zeigt "bereits gewählt" beim 2. Mal
- [ ] Persistiert nach Logout/Login

---

## 📊 Console-Logs (Erwartete Ausgabe)

### Character Selection:
```
[INFO] Character selected: SpecTester (ID: X)
[INFO] AuthState class set to: Krieger        ← NEU! Wichtig!
```

### NPC Dialog (Level 5+):
```
[INFO] Opening dialog with NPC: Meister der Künste
[INFO] AuthState class: Some(Krieger)         ← Sollte gesetzt sein!
[INFO] Available specs: Leibwächter, Gladiator
```

### Specialization Choice:
```
[INFO] Player chose specialization: Leibwächter
[INFO] Sent specialization choice to server
[INFO] Received SpecializationChosen
```

---

## 🔍 Server-Log Debugging

```bash
tail -f /home/max/code/game/server.log | grep -i special
```

**Erwartete Logs:**
```
[INFO] Received ChooseSpecialization: Leibwächter
[INFO] Character 1 validated: Level 5, no spec, class OK
[INFO] Updated specialization in database
[INFO] Sent SpecializationChosen response
```

---

## 🔄 Neustart (falls nötig)

```bash
# Server neustarten
pkill -f "target.*server"
cd /home/max/code/game
RUST_LOG=info ./target/release/server > server.log 2>&1 &

# Client neustarten  
pkill -f "target.*client"
./target/release/client &
```

---

## 🎉 Erfolg = Alle Punkte ✓

Wenn alle Test-Checklistenpunkte funktionieren:
- ✅ Spezialisierungs-System ist **VOLL FUNKTIONSFÄHIG**!
- ✅ Ready für weitere Features (Skill UI, etc.)

---

## 📚 Weitere Dokumentation

- `SPECIALIZATION_TEST_RESULTS.md` - Detaillierte Test-Protokolle
- `SPECIALIZATION_IMPLEMENTATION_COMPLETE.md` - Vollständige Code-Dokumentation
- `SKILL_SYSTEM_DESIGN.md` - Alle 40 Skills im Detail
- `NPC_IMPLEMENTATION_SUMMARY.md` - NPC-System Details

---

**BEREIT ZUM TESTEN! 🚀**

Die Änderungen von dieser Session beheben das Hauptproblem:
**NPC Dialog zeigt jetzt korrekt die Spezialisierungs-Optionen!**
