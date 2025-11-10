# UI Stack System - Complete Planning Package

## 📋 Planning Status: COMPLETE ✅

All planning and documentation for the UI Stack System implementation is complete and ready for execution.

---

## 📚 Documentation Package (2,485 lines)

| Document | Lines | Purpose |
|----------|-------|---------|
| **SESSION_STATUS.md** | 322 | Overall session status and context |
| **UI_STACK_IMPLEMENTATION_PLAN.md** | 879 | Comprehensive technical specification |
| **UI_STACK_CODE_CHANGES.md** | 572 | Exact code changes per file |
| **UI_STACK_QUICK_START.md** | 148 | Quick reference guide |
| **UI_STACK_VISUAL_GUIDE.md** | 564 | Visual diagrams and examples |

---

## 🎯 What This Solves

### The Problem
When NPC dialog is open and ESC is pressed:
- ❌ Pause menu opens in background
- ❌ Dialog stays open
- ❌ Confusing UI state

### The Solution
Implement priority-based UI layer stack:
- ✅ ESC closes topmost layer first
- ✅ Clean layer hierarchy
- ✅ No UI conflicts

---

## 🚀 Quick Start

1. **Read Planning**
   ```bash
   cd /home/max/code/game
   cat UI_STACK_QUICK_START.md
   ```

2. **Implement**
   ```bash
   # Follow code changes document
   cat UI_STACK_CODE_CHANGES.md
   ```

3. **Test**
   ```bash
   cargo build --release
   ./run_client.sh
   ```

---

## 📊 Implementation Overview

### Files to Create (1)
- `client/src/ui/ui_stack.rs` - ~150 lines (core system)

### Files to Modify (6)
- `client/src/ui/mod.rs` - +3 lines
- `client/src/main.rs` - +1 line
- `client/src/ui/npc_dialog.rs` - +10 lines
- `client/src/ui/pause.rs` - +8, -5 lines
- `client/src/ui/settings.rs` - +8, -5 lines
- `client/src/ui/game_ui.rs` - +8, -4 lines

**Total Changes:** ~182 new lines, ~14 removed lines

---

## ⏱️ Timeline

- **Implementation:** 30-45 minutes
- **Testing:** 15 minutes
- **Documentation:** 10 minutes
- **Total:** ~1 hour

---

## ✅ Implementation Phases

### Phase 1: Core System
- [ ] Create `client/src/ui/ui_stack.rs`
  - UILayerStack resource
  - UILayerType enum (GameUI, PauseMenu, Settings, NpcDialog)
  - Centralized ESC handler
  - UIStackPlugin

### Phase 2: Integration
- [ ] Update `client/src/ui/mod.rs` - Export ui_stack
- [ ] Update `client/src/main.rs` - Register UIStackPlugin

### Phase 3: UI Components
- [ ] Update `npc_dialog.rs` - Push/remove NpcDialog layer
- [ ] Update `pause.rs` - Push/remove PauseMenu, remove ESC handler
- [ ] Update `settings.rs` - Push/remove Settings, remove ESC handler
- [ ] Update `game_ui.rs` - Push/remove GameUI, remove ESC handler

### Phase 4: Testing
- [ ] Test: Dialog ESC closes dialog (not pause)
- [ ] Test: ESC sequence (close dialog → open pause → resume)
- [ ] Test: Pause menu ESC behavior
- [ ] Test: Settings ESC returns to pause

---

## 🎨 Architecture

```
UILayerStack (Priority-based LIFO)
├── NpcDialog (300)    ← ESC closes this first
├── Settings (250)
├── PauseMenu (200)
└── GameUI (100)       ← Base layer
```

---

## 📖 Document Guide

### For Understanding
1. Start with **UI_STACK_QUICK_START.md** - Overview
2. Read **UI_STACK_VISUAL_GUIDE.md** - Diagrams
3. Study **UI_STACK_IMPLEMENTATION_PLAN.md** - Full details

### For Coding
1. Open **UI_STACK_CODE_CHANGES.md** - Copy/paste code
2. Reference **UI_STACK_IMPLEMENTATION_PLAN.md** - Context
3. Check **SESSION_STATUS.md** - Overall status

---

## 🧪 Test Scenarios

### Scenario 1: Dialog ESC
```
1. Open NPC dialog
2. Press ESC
Expected: Dialog closes, pause menu does NOT open
```

### Scenario 2: Sequential ESC
```
1. Open dialog
2. Press ESC (closes dialog)
3. Press ESC (opens pause)
Expected: Two separate actions
```

### Scenario 3: Pause ESC
```
1. Press ESC (opens pause)
2. Press ESC (resumes game)
Expected: Normal pause behavior
```

### Scenario 4: Settings ESC
```
1. Open pause → Settings
2. Press ESC
Expected: Back to pause (not game)
```

---

## 🎯 Success Criteria

- [ ] Code compiles without errors
- [ ] No unused parameter warnings
- [ ] ESC closes dialog before pause
- [ ] Console logs show layer operations
- [ ] All test scenarios pass
- [ ] No UI state conflicts

---

## 🔧 Console Verification

Expected logs during testing:
```
[INFO] UI Layer pushed: GameUI (priority: 100)
[INFO] UI Layer pushed: NpcDialog (priority: 300)
[INFO] ESC pressed - handling layer: NpcDialog
[INFO] UI Layer removed: NpcDialog
[INFO] ESC pressed - opening pause menu (no layers)
[INFO] UI Layer pushed: PauseMenu (priority: 200)
```

---

## 📦 Benefits

### Immediate
- ✅ Fixes ESC key conflict
- ✅ Better UX
- ✅ No UI state confusion

### Long-term
- ✅ Foundation for Inventory, Map, Skills UI
- ✅ Centralized input management
- ✅ Easier debugging
- ✅ Cleaner codebase

---

## ⚠️ Risk Assessment

**Risk Level:** 🟢 Low

**Why:**
- Changes are additive
- Clear rollback path
- Well-tested pattern
- Comprehensive planning

---

## 🔗 Related Work

### Previous Session
- ✅ Collision System (4 phases, ~850 lines)
- ✅ Level System
- ✅ Skill System Design
- ✅ Character Classes Update

### Current Session
- 🎯 UI/UX Improvements (ESC key handling)

---

## 💡 Next Steps After Implementation

1. Update `AGENTS.md` with UI Stack info
2. Consider adding more layers:
   - Inventory (Priority 150)
   - Map (Priority 150)
   - Skills (Priority 150)
3. Implement input blocking system
4. Add layer animations

---

## 📞 Need Help?

### Reference Documents
- **Quick questions:** UI_STACK_QUICK_START.md
- **Visual help:** UI_STACK_VISUAL_GUIDE.md
- **Code help:** UI_STACK_CODE_CHANGES.md
- **Deep dive:** UI_STACK_IMPLEMENTATION_PLAN.md
- **Context:** SESSION_STATUS.md

### Common Issues
- "Cannot find UILayerStack" → Check ui/mod.rs exports
- "ESC still wrong" → Verify plugin order in main.rs
- "Layer not removed" → Check cleanup functions
- "Compilation errors" → Update system signatures

---

## ✨ Summary

**Status:** Planning Complete - Ready for Implementation
**Complexity:** Medium
**Impact:** High
**Risk:** Low
**Time:** ~1 hour

All planning complete. Documentation comprehensive. Code changes clearly defined. Test strategy solid.

**Let's build it!** 🚀

---

**Last Updated:** November 10, 2025
**Next Action:** Begin Phase 1 implementation
