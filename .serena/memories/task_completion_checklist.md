# Task Completion Checklist

When a coding task is completed, follow these steps:

## 1. Build & Compilation
```bash
# Always build in release mode for accurate performance
cargo build --release

# Check for errors
# Warnings are acceptable if they're pre-existing (unused code, etc.)
```

**Expected:** 0 errors, warnings OK if pre-existing

## 2. Testing

### Server Changes
```bash
cargo test -p server
```
**Expected:** All tests pass (19/19 auth tests, 3/3 db tests)

### Client Changes
```bash
# Manual testing required (no automated tests yet)
./run_server.sh  # Terminal 1
./run_client.sh  # Terminal 2
```

## 3. Manual Testing Scenarios

### Authentication Flow
1. Start server, start client
2. Register new account (username ≥3, password ≥8)
3. Login with credentials
4. Verify token, character list loads

### Character Management
1. Create character (name, class selection)
2. Verify character appears in list
3. Select character → enters game
4. Logout → character persists

### In-Game
1. WASD movement works
2. Camera controls (RMB, mouse wheel)
3. ESC opens pause menu
4. Settings changes apply (VSync, Fullscreen)

### NPC Interaction (if applicable)
1. Walk to NPC at (5, 1, 5)
2. NPC glows when < 3m
3. Click NPC → dialog opens
4. At level 5+: specialization choice works

## 4. Database Integrity
```bash
# Check if migrations ran
sqlite3 game.db ".tables"
# Expected: users, characters tables exist

# Check data integrity
sqlite3 game.db "SELECT COUNT(*) FROM users;"
sqlite3 game.db "SELECT COUNT(*) FROM characters;"
```

## 5. Log Verification

### Server Logs
```bash
# Check for errors
grep -i error server.log
# Expected: No errors (or only expected errors)

# Check startup
grep -i "server started" server.log
# Expected: "Server started on 127.0.0.1:5000"
```

### Client Logs
Look for in console:
- No panic! messages
- FPS should be ~60
- State transitions log correctly

## 6. Performance Check
- Client FPS: Should be 60 (or monitor refresh rate)
- Server CPU: Should be low when idle
- Memory: No obvious leaks (stable over time)

## 7. Code Quality

### Before Commit
```bash
# Optional: Run clippy if available
cargo clippy

# Optional: Format code
cargo fmt
```

### Review
- [ ] No commented-out debug code
- [ ] No `println!` debug statements (use `info!`, `debug!` instead)
- [ ] No hardcoded test values in production code
- [ ] German UI text, English code comments
- [ ] Proper error handling (no unwrap() in critical paths)

## 8. Documentation

### Update if needed
- [ ] AGENTS.md - Major feature changes
- [ ] README.md - New commands, features
- [ ] Create session notes if complex changes

### Session Summary
For significant changes, document:
- What was changed
- Why it was changed
- Any new patterns/lessons learned
- Known issues or TODOs

## 9. Git Commit (Optional)
```bash
git status
git add <relevant files>
git commit -m "Clear, descriptive message"
```

## 10. Known Issues to Accept
These are acceptable and don't block completion:
- ✅ Unused code warnings (dead_code for future features)
- ✅ JWT secret hardcoded (documented security warning)
- ✅ Audio settings don't work (no audio system yet)
- ✅ Multiplayer not implemented (WorldState messages defined but unused)
- ✅ Some missing UI error displays (logged instead)

## Red Flags (Must Fix)
- ❌ Compilation errors
- ❌ Test failures (regression)
- ❌ Server crashes on startup
- ❌ Client crashes on startup
- ❌ Database corruption
- ❌ Unable to login after changes
- ❌ Unable to create/select characters
- ❌ Player can't move in-game

## Quick Checklist
- [ ] `cargo build --release` succeeds
- [ ] Server starts without errors
- [ ] Client starts and shows UI
- [ ] Can register, login, create character, enter game
- [ ] WASD movement works
- [ ] No critical errors in logs
- [ ] Performance acceptable (60 FPS)
