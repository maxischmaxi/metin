# UI Stack System - Quick Start Guide

## The Problem

When NPC dialog is open and you press ESC:
- ❌ Pause menu opens in background
- ❌ Dialog stays open
- ❌ Confusing UI state

## The Solution

Implement a **UI Layer Stack** with priority-based ESC handling.

## How It Works

```
Priority Stack (LIFO):
┌─────────────────────────────┐
│  NpcDialog (300)            │ ← ESC closes this first
├─────────────────────────────┤
│  Settings (250)             │
├─────────────────────────────┤
│  PauseMenu (200)            │
├─────────────────────────────┤
│  GameUI (100)               │ ← Base layer
└─────────────────────────────┘
```

## Implementation Checklist

### Phase 1: Core System ⏳
- [ ] Create `client/src/ui/ui_stack.rs` (~150 lines)
  - UILayerStack resource
  - UILayerType enum
  - Centralized ESC handler
  - UIStackPlugin

### Phase 2: Integration ⏳
- [ ] Update `client/src/ui/mod.rs` - Export ui_stack
- [ ] Update `client/src/main.rs` - Register UIStackPlugin

### Phase 3: Update UI Components ⏳
- [ ] `npc_dialog.rs` - Push/remove NpcDialog layer
- [ ] `pause.rs` - Push/remove PauseMenu layer, remove ESC handler
- [ ] `settings.rs` - Push/remove Settings layer, remove ESC handler  
- [ ] `game_ui.rs` - Push/remove GameUI layer, remove ESC handler

## Expected Behavior After Implementation

```
Scenario 1: NPC Dialog Open
→ Press ESC → Dialog closes
→ Press ESC → Pause menu opens

Scenario 2: In Game
→ Press ESC → Pause menu opens
→ Press ESC → Resume game

Scenario 3: In Settings
→ Press ESC → Back to pause menu
→ Press ESC → Resume game
```

## Testing Commands

```bash
# Build and run
cd /home/max/code/game
cargo build --release
./run_client.sh

# Test scenario:
1. Login
2. Select character
3. Walk to NPC (5, 1, 5)
4. Click NPC → Dialog opens
5. Press ESC → Dialog should close (NOT pause menu)
6. Press ESC → Pause menu opens
7. Press ESC → Game resumes
```

## Key Files Modified

| File | Changes | Description |
|------|---------|-------------|
| `ui/ui_stack.rs` | NEW (~150 lines) | Core system |
| `ui/mod.rs` | +3 lines | Exports |
| `main.rs` | +1 line | Plugin registration |
| `ui/npc_dialog.rs` | +10 lines | Layer registration |
| `ui/pause.rs` | +3 lines | Layer registration, remove ESC |
| `ui/settings.rs` | +3 lines | Layer registration, remove ESC |
| `ui/game_ui.rs` | +4 lines | Layer registration, remove ESC |

**Total:** ~174 new lines, ~14 removed lines

## Console Logs to Watch For

```
[INFO] UI Layer pushed: GameUI (priority: 100)
[INFO] Opening dialog with NPC: Meister der Künste
[INFO] UI Layer pushed: NpcDialog (priority: 300)
[INFO] ESC pressed - handling layer: NpcDialog
[INFO] UI Layer removed: NpcDialog
[INFO] ESC pressed - opening pause menu (no layers)
[INFO] UI Layer pushed: PauseMenu (priority: 200)
```

## Benefits

✅ **Centralized ESC handling** - Single source of truth
✅ **Priority-based layers** - Clear hierarchy
✅ **No UI conflicts** - Proper layer management
✅ **Extensible** - Easy to add Inventory, Map, Skills
✅ **Better UX** - Predictable behavior

## Next Steps

1. Read full plan: `UI_STACK_IMPLEMENTATION_PLAN.md`
2. Implement Phase 1 (core system)
3. Integrate with existing UI
4. Test all scenarios
5. Document in AGENTS.md

## Estimated Time

- **Implementation:** 30-45 minutes
- **Testing:** 15 minutes
- **Documentation:** 10 minutes
- **Total:** ~1 hour

## Success Criteria

- [x] Plan created
- [ ] Core system implemented
- [ ] All UI components updated
- [ ] ESC closes dialog first
- [ ] No pause menu behind dialog
- [ ] All tests pass
- [ ] Code compiles without warnings

---

**Status:** Ready for implementation
**Risk Level:** Low
**Complexity:** Medium
**Impact:** High

Let's build it! 🚀
