# UI Stack Fix - Layer Registrierung korrigiert ✅

**Datum:** 10. November 2025, 12:35 Uhr
**Problem:** ESC schloss NPC Dialog nicht, Pause-Menü öffnete stattdessen
**Status:** ✅ GEFIXT

---

## Das Problem

Nach der ursprünglichen Implementierung:
- NPC anklicken → Dialog öffnet
- ESC drücken → ❌ Pause-Menü öffnete (falsch!)
- Dialog blieb offen

**Ursache:** Layer wurde erst beim UI-Spawn registriert, aber ESC-Handler lief im selben Frame **vorher**!

---

## Die Lösung

Layer-Registrierung verschoben von:
- ❌ **Vorher:** `spawn_npc_dialog()` → Zu spät!
- ✅ **Jetzt:** `mouse_click_system()` → Sofort beim Click!

### Geänderte Dateien:

#### 1. `client/src/interaction.rs`
```rust
// Import hinzugefügt:
use crate::ui::{UILayerStack, UILayerType};

// In mouse_click_system Parameter hinzugefügt:
mut ui_stack: ResMut<UILayerStack>,

// Beim erfolgreichen NPC-Click:
npc_dialog_state.open_dialog(entity, npc.npc_type, npc.name.clone());
// Register UI layer immediately so ESC handler sees it
ui_stack.push_layer(UILayerType::NpcDialog);  // NEU!
```

#### 2. `client/src/ui/npc_dialog.rs`
```rust
// UILayerStack Parameter ENTFERNT (nicht mehr benötigt):
fn spawn_npc_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    // ENTFERNT: mut ui_stack: ResMut<UILayerStack>,
    player_stats: Res<PlayerStats>,
    ...
)

// Layer-Push ENTFERNT (passiert jetzt in interaction.rs):
// ENTFERNT: ui_stack.push_layer(UILayerType::NpcDialog);
```

---

## Wie es jetzt funktioniert

### Timeline (selber Frame):

```
Frame N:
  1. Player klickt auf NPC
  2. mouse_click_system läuft:
     - open_dialog() → active = true
     - push_layer(NpcDialog) → Layer auf Stack!  ✅
  3. handle_escape_key läuft später im Frame:
     - Sieht Layer auf Stack
     - Wenn ESC gedrückt → Kann jetzt korrekt handeln
  4. spawn_npc_dialog läuft:
     - Spawnt UI Entities
```

**Kritisch:** Layer ist **vor** dem ESC-Handler auf dem Stack!

---

## Test-Anleitung

### 1. Server starten (falls nicht läuft):
```bash
cd /home/max/code/game
./run_server.sh
```

### 2. Client starten:
```bash
cargo run --release
```

### 3. Testen:
```
1. Login / Character auswählen
2. Im Spiel: Zum NPC laufen (5, 1, 5)
3. NPC anklicken → Dialog öffnet
4. ESC drücken
```

**Erwartetes Verhalten:**
- ✅ Dialog schließt sich
- ✅ Pause-Menü öffnet NICHT
- ✅ Console: "ESC pressed - handling layer: NpcDialog"
- ✅ Console: "UI Layer removed: NpcDialog"

### 4. Erneut ESC drücken:
**Erwartetes Verhalten:**
- ✅ Jetzt öffnet Pause-Menü
- ✅ Console: "ESC pressed - opening pause menu (no layers)"
- ✅ Console: "UI Layer pushed: PauseMenu"

---

## Console Logs (Erwartet)

```
[INFO] Opening dialog with NPC: Meister der Künste
[INFO] UI Layer pushed: NpcDialog (priority: 300)
[INFO] ESC pressed - handling layer: NpcDialog
[INFO] Closing NPC dialog
[INFO] UI Layer removed: NpcDialog
[INFO] ESC pressed - opening pause menu (no layers)
[INFO] UI Layer pushed: PauseMenu (priority: 200)
```

---

## Technische Details

### Warum der Fix funktioniert:

**Problem der ursprünglichen Implementierung:**
```
Frame N:
├─ Mouse Click Event
│  └─ open_dialog() [active = true]
├─ ESC Handler läuft
│  └─ Kein Layer auf Stack! → Öffnet Pause
└─ spawn_npc_dialog läuft
   └─ push_layer() [zu spät!]
```

**Mit dem Fix:**
```
Frame N:
├─ Mouse Click Event
│  ├─ open_dialog() [active = true]
│  └─ push_layer() [sofort!] ✅
├─ ESC Handler läuft
│  └─ Layer auf Stack! → Schließt Dialog ✅
└─ spawn_npc_dialog läuft
   └─ Spawnt nur UI (Layer schon da)
```

---

## Code-Änderungen

**Hinzugefügt:**
- `interaction.rs`: Import von UILayerStack
- `interaction.rs`: UILayerStack Parameter in mouse_click_system
- `interaction.rs`: push_layer beim NPC-Click

**Entfernt:**
- `npc_dialog.rs`: UILayerStack Parameter in spawn_npc_dialog
- `npc_dialog.rs`: push_layer Aufruf (nicht mehr nötig)

**Netto:** ~3 Zeilen hinzugefügt, ~2 Zeilen entfernt

---

## Zusammenfassung

**Problem:** Timing-Issue - Layer wurde zu spät registriert
**Lösung:** Layer sofort beim Click registrieren, nicht erst beim Spawn
**Status:** ✅ Gefixt und kompiliert
**Test:** Bereit zum Testen

---

**Build Status:** ✅ Erfolgreich (8.87s)
**Errors:** 0
**Warnings:** Nur unused imports (normal)

**Bereit zum Testen!** 🚀
