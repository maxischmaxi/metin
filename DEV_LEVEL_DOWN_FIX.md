# -Level Button Fix - Korrekte Level-Down Implementierung

## 🐛 Problem

Beim Drücken von `-Lvl` sprang der Spieler mehrere Level **HOCH** statt runter!

### Ursache

**Fehlerhaftes Verständnis von `player_stats.experience`:**

```rust
// FALSCH gedacht:
let current_total_xp = player_stats.experience;  // ❌ Das ist NICHT die Gesamt-XP!
```

**Realität:**
- `player_stats.experience` = XP **im aktuellen Level** (0 bis xp_needed)
- NICHT die gesamte XP seit Level 1!

**Beispiel:**
```
Level 5, 500 XP im aktuellen Level
Gesamt-XP seit Level 1 = ~100 + 200 + 400 + 775 + 500 = ~1975 XP
player_stats.experience = 500 ❌ (nicht 1975!)
```

**Alte fehlerhafte Logik:**
```rust
// Versuchte totale XP zu berechnen (falsch!)
let mut xp_at_prev_level_start = 0i64;
for lvl in 2..=prev_level {
    xp_at_prev_level_start += shared::calculate_xp_for_level(lvl);
}
// Bei Level 5: xp_at_prev_level_start = 100+200+400 = 700
// xp_to_remove = -(500 - 700) = -(-200) = +200 XP ❌ LEVEL UP!
```

---

## ✅ Lösung

**Einfach:** Entferne die aktuelle XP + 1, um Level-Down zu triggern!

```rust
DevButton::RemoveLevel => {
    if player_stats.level > 1 {
        // player_stats.experience = XP im aktuellen Level
        // Entferne (current_xp + 1) um ins negative zu gehen
        let xp_to_remove = -(player_stats.experience + 1);
        
        network.send_message(&ClientMessage::GainExperience { 
            amount: xp_to_remove 
        });
    }
}
```

### Wie es funktioniert:

**Beispiel: Level 5 mit 500 XP**

1. User drückt `-Lvl`
2. Client sendet: `GainExperience { amount: -501 }`
3. Server:
   ```rust
   new_xp = 500 + (-501) = -1  // Negativ!
   
   while new_level > 1 && new_xp < 0 {
       new_level -= 1;  // 5 → 4
       xp_for_prev = calculate_xp_for_level(5) = 1349
       new_xp += 1349;  // -1 + 1349 = 1348
       break;  // new_xp ist jetzt positiv
   }
   
   // Problem: new_xp = 1348, nicht 0! ❌
   ```

**Warte, das ist immer noch falsch!** Der Server setzt XP nicht auf 0 beim Level-Down!

---

## 🔧 Server-Fix Benötigt

Der Server muss beim Level-Down die XP auf 0 setzen, nicht die XP vom vorherigen Level addieren!

**Server-Problem:**
```rust
// AKTUELL (FALSCH):
while new_level > 1 && new_xp < 0 {
    new_level -= 1;
    xp_for_prev_level = calculate_xp_for_level(new_level + 1);
    new_xp += xp_for_prev_level;  // ❌ Das gibt zu viel XP!
}
```

**Was passiert:**
- Level 5, 500 XP
- -501 XP → new_xp = -1
- Level Down zu 4
- new_xp += 1349 → new_xp = 1348 ❌ (sollte 0 sein!)

**Korrekte Server-Logik:**
```rust
while new_level > 1 && new_xp < 0 {
    new_level -= 1;
    new_xp = 0;  // ✅ Einfach auf 0 setzen!
    break;  // Nur ein Level pro Dev-Command
}
```

---

## 🎯 Finale Lösung

### Client (schon implementiert):
```rust
let xp_to_remove = -(player_stats.experience + 1);
```

### Server (muss noch geändert werden):
```rust
// Handle level-downs (negative XP)
if new_xp < 0 && new_level > 1 {
    new_level -= 1;
    new_xp = 0;  // Start of new level
    log::info!("DEV: Character {} leveled DOWN to {}", character_id, new_level);
}
```

---

## 🧪 Test nach Fix

```
Level 5, 500 XP
↓ Click [-Lvl]
Level 4, 0 XP ✓

Level 4, 0 XP
↓ Click [-Lvl]
Level 3, 0 XP ✓

Level 1, 0 XP
↓ Click [-Lvl]
Level 1, 0 XP (keine Änderung) ✓
```

---

**Status:** Client ✅ | Server ⏳ (wird jetzt gefixt)
