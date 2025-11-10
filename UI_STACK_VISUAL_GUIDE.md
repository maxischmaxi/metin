# UI Stack System - Visual Guide

## Current Problem Visualization

```
❌ BEFORE (Broken):

Player presses ESC with NPC dialog open:
┌─────────────────────────────────────┐
│  [Pause Menu - VISIBLE]             │ ← Opens in background
│                                     │
│  ┌───────────────────────────────┐ │
│  │ [NPC Dialog - STILL OPEN]     │ │ ← Stays open!
│  │                               │ │
│  │  "Choose your spec..."        │ │
│  │                               │ │
│  │  [Close]                      │ │
│  └───────────────────────────────┘ │
│                                     │
│  [Resume] [Settings] [Quit]         │
└─────────────────────────────────────┘
    ↑
    Confusing! Which UI is active?
```

---

## Solution Visualization

```
✅ AFTER (Fixed):

Player presses ESC with NPC dialog open:

Step 1 - Dialog Open:
┌─────────────────────────────────────┐
│  Game World (blurred)               │
│                                     │
│  ┌───────────────────────────────┐ │
│  │ [NPC Dialog]   Priority: 300  │ │ ← Top of stack
│  │                               │ │
│  │  "Choose your spec..."        │ │
│  │                               │ │
│  │  [Close]                      │ │
│  └───────────────────────────────┘ │
│                                     │
└─────────────────────────────────────┘

Press ESC → Closes dialog (top layer)

Step 2 - Dialog Closed:
┌─────────────────────────────────────┐
│  Game World (active)                │
│                                     │
│  [HP] [MP] [ST]                     │ ← GameUI (Priority: 100)
│                                     │
│                                     │
└─────────────────────────────────────┘

Press ESC → Opens pause menu

Step 3 - Pause Open:
┌─────────────────────────────────────┐
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ │
│  ┃ [Pause Menu]  Priority: 200   ┃ │ ← Top of stack
│  ┃                                ┃ │
│  ┃  [Resume]                      ┃ │
│  ┃  [Settings]                    ┃ │
│  ┃  [Quit]                        ┃ │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │
└─────────────────────────────────────┘

Press ESC → Closes pause, resumes game
```

---

## UI Layer Stack Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     UILayerStack                            │
│                                                             │
│  Pop ←                   LIFO Stack                   → Push│
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Layer 3: NpcDialog    (Priority: 300) [Blocks]     │◄─┼─ Top
│  ├──────────────────────────────────────────────────────┤  │
│  │  Layer 2: PauseMenu    (Priority: 200) [Blocks]     │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  Layer 1: GameUI       (Priority: 100) [Allows]     │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  ESC Handler checks top_layer() and processes accordingly  │
└─────────────────────────────────────────────────────────────┘
```

---

## State Flow Diagram

```
┌─────────────┐
│   InGame    │
│             │
│  GameUI:100 │◄────────────────────┐
└─────────────┘                     │
      │                             │
      │ Press ESC                   │ Press ESC
      │                             │
      ▼                             │
┌─────────────┐                     │
│   Paused    │                     │
│             │                     │
│  GameUI:100 │                     │
│  Pause :200 │◄──────────┐         │
└─────────────┘           │         │
      │                   │         │
      │ Click Settings    │ ESC     │
      │                   │         │
      ▼                   │         │
┌─────────────┐           │         │
│  Settings   │           │         │
│             │           │         │
│  GameUI:100 │           │         │
│  Pause :200 │───────────┘         │
│  Settings:250│                     │
└─────────────┘                     │
                                    │
┌─────────────────────────────────┐ │
│  InGame with NPC Dialog         │ │
│                                 │ │
│  GameUI    :100                 │ │
│  NpcDialog :300                 │─┘
│                                 │
│  Press ESC → Dialog closes      │
│  Press ESC → Pause opens        │
└─────────────────────────────────┘
```

---

## ESC Key Decision Tree

```
                    ESC Pressed
                         │
                         ▼
              ┌──────────────────────┐
              │ UILayerStack.is_empty()?│
              └──────────────────────┘
                    /        \
                  Yes         No
                  /             \
                 ▼               ▼
         ┌─────────────┐   ┌─────────────────┐
         │ In InGame?  │   │ Get top_layer() │
         └─────────────┘   └─────────────────┘
               / \                    │
             Yes  No                  ▼
             /     \          ┌──────────────────┐
            ▼       ▼         │ Match layer_type │
      ┌─────────┐  Do        └──────────────────┘
      │ Open    │  Nothing         /    |    \
      │ Pause   │               NPC  Pause Settings
      └─────────┘                /     |      \
                                ▼      ▼       ▼
                        Close    Resume   Back to
                        Dialog   Game     Pause
```

---

## Code Flow Example

### Scenario: NPC Dialog Open, Press ESC

```rust
1. User presses ESC
   └─> handle_escape_key() in ui_stack.rs

2. Check UILayerStack
   └─> ui_stack.top_layer() returns Some(NpcDialog)

3. Match on layer type
   └─> UILayerType::NpcDialog arm executes

4. Actions:
   ├─> npc_dialog_state.close_dialog()
   │   └─> Sets active = false in NpcDialogState
   │
   └─> ui_stack.remove_layer(NpcDialog)
       └─> Removes from stack

5. Cleanup system detects inactive dialog
   └─> cleanup_closed_dialog() in npc_dialog.rs
       └─> Despawns dialog entities

6. Result: Dialog closed, game continues
```

### Scenario: In Game, No Dialog, Press ESC

```rust
1. User presses ESC
   └─> handle_escape_key() in ui_stack.rs

2. Check UILayerStack
   └─> ui_stack.top_layer() returns Some(GameUI)

3. Match on layer type
   └─> UILayerType::GameUI arm executes

4. Check current state
   └─> State is InGame

5. Actions:
   ├─> next_state.set(GameState::Paused)
   │   └─> State transition to Paused
   │
   └─> ui_stack.push_layer(PauseMenu)
       └─> Adds PauseMenu to stack

6. OnEnter(Paused) triggers
   └─> setup_pause() spawns pause UI

7. Result: Pause menu opens
```

---

## Data Structure Visualization

### UILayerStack Resource

```rust
UILayerStack {
    layers: Vec<UILayer> = [
        // Sorted by priority (highest last)
        UILayer {
            layer_type: GameUI,
            priority: 100,
            blocks_input: false,  // Allows game input
        },
        UILayer {
            layer_type: PauseMenu,
            priority: 200,
            blocks_input: true,   // Blocks game input
        },
        UILayer {
            layer_type: NpcDialog,
            priority: 300,
            blocks_input: true,   // Blocks everything
        },
    ]
}

Methods:
- push_layer()    → Adds to vec, keeps sorted
- pop_layer()     → Removes last (highest priority)
- remove_layer()  → Removes specific layer
- top_layer()     → Returns &last without removing
- has_layer()     → Check if layer exists
- clear()         → Remove all layers
```

---

## Lifecycle Visualization

### NPC Dialog Lifecycle

```
┌─────────────────────────────────────────────────────┐
│ 1. Player clicks NPC                                │
│    └─> mouse_click_system() in interaction.rs      │
│        └─> npc_dialog_state.open_dialog()          │
│            └─> active = true                        │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│ 2. Dialog state changed                             │
│    └─> spawn_npc_dialog() detects active=true      │
│        ├─> ui_stack.push_layer(NpcDialog)          │
│        │   └─> Layer added with priority 300       │
│        └─> Spawns dialog UI entities               │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│ 3. Dialog visible, player presses ESC              │
│    └─> handle_escape_key() in ui_stack.rs          │
│        └─> Checks top_layer() = NpcDialog          │
│            ├─> npc_dialog_state.close_dialog()     │
│            │   └─> active = false                   │
│            └─> ui_stack.remove_layer(NpcDialog)    │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│ 4. Dialog state changed again                       │
│    └─> cleanup_closed_dialog() detects active=false│
│        └─> Despawns all dialog UI entities         │
└─────────────────────────────────────────────────────┘
```

---

## Priority Levels Explained

```
Priority 300+  ┌────────────────────────────────┐
Critical UI    │  Error Dialogs                 │
               │  Confirmation Prompts          │
               └────────────────────────────────┘

Priority 300   ┌────────────────────────────────┐
Dialogs        │  NPC Dialog                    │
               │  Quest Dialog                  │
               │  Trade Window                  │
               └────────────────────────────────┘

Priority 250   ┌────────────────────────────────┐
System UI      │  Settings Menu                 │
               │  Graphics Options              │
               │  Audio Options                 │
               └────────────────────────────────┘

Priority 200   ┌────────────────────────────────┐
Menus          │  Pause Menu                    │
               │  Character Menu                │
               │  Map (fullscreen)              │
               └────────────────────────────────┘

Priority 150   ┌────────────────────────────────┐
Overlays       │  Inventory (future)            │
               │  Skills (future)               │
               │  Map (overlay)                 │
               └────────────────────────────────┘

Priority 100   ┌────────────────────────────────┐
Base UI        │  Health/Mana/Stamina Bars      │
               │  XP Bar                        │
               │  Nameplate                     │
               │  Controls Hints                │
               └────────────────────────────────┘
```

---

## Console Log Flow

### Typical Session Logs

```bash
# Game starts
[INFO] Game initialized

# Enter InGame state
[INFO] UI Layer pushed: GameUI (priority: 100)

# Walk to NPC, click
[INFO] Opening dialog with NPC: Meister der Künste
[INFO] UI Layer pushed: NpcDialog (priority: 300)

# Press ESC
[INFO] ESC pressed - handling layer: NpcDialog
[INFO] Closing NPC dialog
[INFO] UI Layer removed: NpcDialog

# Press ESC again
[INFO] ESC pressed - opening pause menu (no layers)
[INFO] UI Layer pushed: PauseMenu (priority: 200)

# Press ESC in pause
[INFO] ESC pressed - handling layer: PauseMenu
[INFO] UI Layer removed: PauseMenu

# Exit game
[INFO] UI Stack cleared (1 layers)
```

---

## File Organization

```
client/src/
├── main.rs
│   └── Registers UIStackPlugin (before other UI plugins)
│
├── ui/
│   ├── mod.rs
│   │   └── Exports UIStackPlugin, UILayerStack, UILayerType
│   │
│   ├── ui_stack.rs          ⭐ NEW FILE
│   │   ├── UILayerStack      (Resource)
│   │   ├── UILayer           (Struct)
│   │   ├── UILayerType       (Enum)
│   │   ├── UIStackPlugin     (Plugin)
│   │   └── handle_escape_key (System)
│   │
│   ├── npc_dialog.rs         ✏️ MODIFIED
│   │   ├── spawn_npc_dialog  → Adds push_layer(NpcDialog)
│   │   └── cleanup_closed    → Adds remove_layer(NpcDialog)
│   │
│   ├── pause.rs              ✏️ MODIFIED
│   │   ├── setup_pause       → Adds push_layer(PauseMenu)
│   │   ├── pause_buttons     → Removes ESC handler
│   │   └── cleanup_pause     → Adds remove_layer(PauseMenu)
│   │
│   ├── settings.rs           ✏️ MODIFIED
│   │   ├── setup_settings    → Adds push_layer(Settings)
│   │   ├── settings_buttons  → Removes ESC handler
│   │   └── cleanup_settings  → Adds remove_layer(Settings)
│   │
│   └── game_ui.rs            ✏️ MODIFIED
│       ├── setup_game_ui     → Adds push_layer(GameUI)
│       ├── update_instructions → Removes ESC handler
│       └── cleanup_game_ui   → Adds remove_layer(GameUI)
│
└── interaction.rs
    └── NpcDialogState (Resource - already exists)
```

---

## Benefits Visualization

### Before: Scattered ESC Handlers

```
┌──────────────┐
│  game_ui.rs  │──┐
└──────────────┘  │
                  │   All independently
┌──────────────┐  │   handle ESC key
│   pause.rs   │──┼─► No coordination
└──────────────┘  │   Conflicts possible
                  │   Hard to debug
┌──────────────┐  │
│ settings.rs  │──┤
└──────────────┘  │
                  │
┌──────────────┐  │
│npc_dialog.rs │──┘ (missing!)
└──────────────┘
```

### After: Centralized Management

```
┌──────────────────────────────────┐
│      ui_stack.rs                 │
│                                  │
│  ✓ Single ESC handler            │
│  ✓ Priority-based processing     │
│  ✓ Clear layer hierarchy         │
│  ✓ Console logging               │
│  ✓ Easy to debug                 │
└──────────────────────────────────┘
          ▲         ▲        ▲
          │         │        │
    ┌─────┘    ┌────┘   └────┐
    │          │             │
┌───┴───┐  ┌──┴──┐      ┌──┴───┐
│ Dialog│  │Pause│      │Settings│
└───────┘  └─────┘      └────────┘
    All register/unregister layers
    No ESC handlers needed
```

---

## Testing Matrix

| Scenario | Initial State | Action | Expected Result | Verify |
|----------|--------------|--------|-----------------|--------|
| 1 | Dialog open | ESC | Dialog closes | ✓ Log: "Layer removed: NpcDialog" |
| 2 | Dialog closed | ESC | Pause opens | ✓ Log: "Layer pushed: PauseMenu" |
| 3 | Pause open | ESC | Game resumes | ✓ Log: "Layer removed: PauseMenu" |
| 4 | Settings open | ESC | Back to pause | ✓ Log: "Layer removed: Settings" |
| 5 | Dialog + ESC twice | ESC ESC | Close dialog, then pause | ✓ Two separate actions |
| 6 | State change | Change state | Stack clears | ✓ No stale layers |

---

## Quick Reference

### For Developers

**Adding a new UI layer:**
```rust
1. Add variant to UILayerType enum
2. Add priority in UILayer::new()
3. Push layer in spawn function
4. Remove layer in cleanup function
5. Add ESC behavior to handle_escape_key() if needed
```

**Debugging UI issues:**
```bash
# Look for these logs:
grep "UI Layer" server.log
grep "ESC pressed" server.log

# Stack should be balanced:
pushed count == removed count
```

---

**Ready for implementation!** 🚀

Refer to this guide while implementing for visual context.
