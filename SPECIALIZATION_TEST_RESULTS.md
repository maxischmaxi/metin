# Specialization System - Test Results

## ✅ Changes Made (This Session)

### 1. Character Class Now Sent in CharacterSelected
**Problem**: `auth_state.class` was `None`, so NPC dialog couldn't show specialization options.

**Solution**: Modified server to send `character_class` in `CharacterSelected` message.

**Files Changed**:
- `shared/src/lib.rs` - Added `character_class: CharacterClass` to `CharacterSelected`
- `server/src/main.rs` - Include `char_class` in response
- `client/src/networking.rs` - Store `character_class` in AuthState
- `client/src/ui/npc_dialog.rs` - Improved class detection with fallback

### 2. Character Creation Flow Fixed (Previous Session)
**Problem**: Character name wasn't shown after creation, devtools didn't work.

**Solution**: Wait for server response before transitioning to InGame.

---

## 🧪 Test Procedure

### Test 1: New Character - Level to 5 - Choose Specialization

#### Steps:
1. **Login/Register**
   ```
   Username: spectest
   Password: testpass123
   ```

2. **Create New Character**
   - Name: "SpecTester"
   - Class: **Krieger** (Warrior)
   - Click "Erstellen ✓"
   - Should wait briefly, then auto-select character
   - Should show in-game with correct name

3. **Level to 5**
   - Press `K` key ~20-30 times
   - Watch level increase: 1 → 2 → 3 → 4 → 5
   - Watch XP bar and level display
   - Console should show level-ups

4. **Find NPC**
   - Walk to position (5, 1, 5) using WASD
   - Look for golden NPC "Meister der Künste"
   - NPC should glow when you're within 3m

5. **Talk to NPC**
   - Get close (< 3m)
   - Left-click on NPC
   - **Expected**: Dialog shows:
     ```
     Wähle deine Spezialisierung
     ───────────────────────────
     Du hast Level 5 erreicht!
     Es ist Zeit, deinen Pfad zu wählen.
     
     [Leibwächter]    [Gladiator]
     PvM Tank         PvP Damage
     [Wählen]         [Wählen]
     
     [Schließen]
     ```

6. **Choose Specialization**
   - Click "Wählen" on one (e.g., Leibwächter)
   - **Expected**: 
     - Dialog closes
     - Console shows: "Player chose specialization: Leibwächter"
     - Console shows: "Sent specialization choice to server"

7. **Verify Persistence**
   - Talk to NPC again
   - **Expected**: Dialog shows:
     ```
     Meister der Künste
     ──────────────────
     Du hast bereits eine Spezialisierung gewählt:
     
     Leibwächter
     
     Dieser Pfad ist nun dein Schicksal.
     
     [Schließen]
     ```

8. **Logout & Login**
   - ESC → Zum Hauptmenü → Ausloggen
   - Login again
   - Select character
   - Talk to NPC
   - **Expected**: Still shows "bereits gewählt"

---

### Test 2: Different Classes - Different Specs

#### Krieger (Warrior)
- **Spec 1**: Leibwächter (PvM Tank)
- **Spec 2**: Gladiator (PvP Damage)

#### Ninja
- **Spec 1**: Bogenschütze (Fernkampf)
- **Spec 2**: Attentäter (Nahkampf)

#### Sura
- **Spec 1**: Dämonen-Jäger (PvM)
- **Spec 2**: Blutkrieger (PvP)

#### Schamane (Shaman)
- **Spec 1**: Lebenshüter (Support)
- **Spec 2**: Sturmrufer (PvP Damage)

**Test**: Create characters of each class, verify correct specializations are shown.

---

### Test 3: Edge Cases

#### Test 3.1: Level < 5
1. Create new character
2. Don't level (stay Level 1)
3. Talk to NPC
4. **Expected**: Dialog shows:
   ```
   Meister der Künste
   ──────────────────
   Du musst Level 5 erreichen, um eine Spezialisierung zu wählen.
   
   Kehre zurück, wenn du stärker geworden bist.
   
   [Schließen]
   ```

#### Test 3.2: Out of Range
1. Stand > 3m away from NPC
2. Try to click NPC
3. **Expected**: No dialog opens, no interaction

#### Test 3.3: Server Validation
1. Try to choose specialization at Level 4 (if possible to bypass client check)
2. **Expected**: Server rejects with "must be level 5"

3. Try to choose specialization twice
4. **Expected**: Server rejects with "already chosen"

---

## 📊 Results

### ✅ What Should Work

- [ ] Character creation shows name immediately
- [ ] Character selection sets `auth_state.class` correctly
- [ ] Level < 5: NPC shows "must reach level 5" message
- [ ] Level 5+, no spec: NPC shows 2 specialization options
- [ ] Specialization buttons match character class
- [ ] Clicking "Wählen" sends to server
- [ ] Server validates and saves to database
- [ ] Dialog closes after successful choice
- [ ] Talking to NPC again shows "already chosen"
- [ ] Specialization persists after logout/login
- [ ] NPC glow effect works within 3m range

### 🐛 Known Issues (If Any)

Record any issues found during testing here:

1. 
2. 
3. 

---

## 🔍 Debugging

### Client Console Logs to Look For

**Character Selection:**
```
Character selected: SpecTester (ID: X)
AuthState class set to: Krieger
```

**NPC Dialog Opening (Level 5+):**
```
Opening dialog with NPC: Meister der Künste
Dialog state: Show specialization options
Available specs for Krieger: Leibwächter, Gladiator
```

**Specialization Choice:**
```
Player chose specialization: Leibwächter
Sent specialization choice to server
```

**Server Response:**
```
Received SpecializationChosen
AuthState specialization set to: Leibwächter
```

### Server Logs to Look For

**Specialization Request:**
```
[INFO] Received ChooseSpecialization for character X: Leibwächter
[INFO] Character X validated: Level 5, no existing spec, class matches
[INFO] Updated specialization in database for character X
[INFO] Sent SpecializationChosen response
```

**Validation Failures (if any):**
```
[WARN] Specialization choice failed: Must be level 5 or higher
[WARN] Specialization choice failed: Character already has a specialization
[WARN] Specialization choice failed: Specialization not valid for class
```

---

## 🎯 Success Criteria

All tests pass when:
1. ✅ Character class is correctly identified in NPC dialog
2. ✅ Appropriate specializations shown for each class
3. ✅ Server validates level, existing spec, and class compatibility
4. ✅ Specialization is saved to database
5. ✅ Specialization persists across sessions
6. ✅ NPC dialog changes after specialization is chosen
7. ✅ Cannot choose specialization twice (permanent choice)

---

## 📝 Test Log

### Test Run 1 - [Date/Time]

**Tester**: 
**Character**: 
**Class**: 
**Result**: 

Notes:


### Test Run 2 - [Date/Time]

**Tester**: 
**Character**: 
**Class**: 
**Result**: 

Notes:


---

## 🚀 Next Steps After Testing

If all tests pass:
- [ ] Document any edge cases found
- [ ] Create SPECIALIZATION_QUICKSTART.md guide
- [ ] Update AGENTS.md with specialization system details
- [ ] Consider implementing skill UI based on chosen specialization

If issues found:
- [ ] Document specific failures
- [ ] Check server logs for validation errors
- [ ] Verify auth_state.class is being set
- [ ] Check NPC dialog class detection logic
