# 🎮 Quick Test Guide - Phase 2

## Schnelltest (5 Minuten)

### 1. Server starten
```bash
cd /home/max/code/game
./run_server.sh
```

### 2. Client starten
```bash
cd /home/max/code/game
./run_client.sh
```

### 3. Im Spiel

1. **Login/Register**
2. **Character erstellen/wählen**
3. **Im Spiel spawnen**

---

## ✅ Test: NPC Collision

**Ziel:** Goldener NPC bei (5, 1, 5)

**Mit W+D zum NPC laufen (rechts-vorne)**

**Erwartung:**
- ✅ Console: "Collision started..."
- ✅ Player **stoppt** vor NPC
- ✅ Abstand ~0.9m
- ✅ Player kann **nicht** durchlaufen
- ✅ Player kann um NPC herum laufen

**Wenn das funktioniert → Phase 2 ist fertig! 🎉**

---

## Zusätzliche Tests (optional)

### Test 2: Baum
- Position: (-3, 1, 3) - links-vorne
- Mit W+A hinlaufen
- Player sollte stoppen

### Test 3: Stein
- Position: (3, 0.5, -3) - rechts-hinten
- Mit S+D hinlaufen
- Player sollte stoppen

### Test 4: Wand
- Position: (0, 1, -8) - nach hinten
- Mit S hinlaufen
- Player sollte stoppen

---

## 🐛 Troubleshooting

### Player läuft immer noch durch
**Check:**
1. Wurde neu kompiliert? (`cargo build --release -p client`)
2. Läuft der neue Client? (nicht alter)
3. Console Logs erscheinen?

### Kein Collision Log
**Check:**
1. Ist Player wirklich nah genug? (< 1m)
2. Läuft Server?
3. RUST_LOG=info gesetzt?

### Player "zittert" vor Objekt
**Das ist OK!** Minimale Oszillation ist normal bei:
- Ecken/Kanten
- Multiple simultane Collisions
- Wird in Phase 3 optimiert

---

## ✅ Erfolg wenn:
- Player stoppt vor Objekten ✅
- Keine Überlappung ✅
- Console Logs erscheinen ✅
- Kein Crash ✅

**Alles gut? → Phase 2 KOMPLETT! 🚀**
