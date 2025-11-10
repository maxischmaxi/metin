# UI Stack/Priority System Implementation Plan

## Problem Statement

**Current Issue:**
- When NPC dialog is open and ESC is pressed, the Pause menu opens in the background
- The dialog remains open, creating UI confusion
- Multiple ESC handlers compete without coordination
- No priority/layer system for UI management

**Problematic ESC Handlers:**
1. `client/src/ui/game_ui.rs:573` - Opens pause menu from InGame
2. `client/src/ui/pause.rs:235` - Returns to InGame from Paused  
3. `client/src/ui/settings.rs:333` - Returns to Paused from Settings
4. `client/src/ui/npc_dialog.rs` - No ESC handler (dialog stays open!)

**Desired Behavior:**
```
ESC while NPC dialog open → Close dialog
ESC while no dialog → Open pause menu
ESC in pause menu → Resume game
ESC in settings → Back to pause menu
```

## Solution Architecture

### Core Concept: UI Layer Stack

A centralized resource that tracks active UI layers in priority order (stack-based LIFO).

```rust
// Higher priority = rendered on top, handled first
┌─────────────────────────────┐
│  NpcDialog (Priority: 300)  │ ← ESC closes this first
├─────────────────────────────┤
│  PauseMenu (Priority: 200)  │ ← Then this
├─────────────────────────────┤
│  GameUI (Priority: 100)     │ ← Base layer
└─────────────────────────────┘
```

### Components

#### 1. UILayerStack Resource
```rust
#[derive(Resource, Default)]
pub struct UILayerStack {
    layers: Vec<UILayer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UILayerType {
    GameUI,           // Base in-game UI (health bars, etc.)
    NpcDialog,        // NPC conversation dialogs
    PauseMenu,        // Pause menu
    Settings,         // Settings menu
    // Future: Inventory, Map, Skills, etc.
}

#[derive(Debug, Clone, Copy)]
pub struct UILayer {
    layer_type: UILayerType,
    priority: i32,
    blocks_input: bool,  // Whether this layer blocks lower layers
}
```

**Priority Levels:**
- 100: Base game UI (always present)
- 200: Pause menu
- 250: Settings menu
- 300: NPC dialogs
- 400+: Critical UI (confirmations, errors)

#### 2. Centralized ESC Handler
```rust
fn handle_escape_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_stack: ResMut<UILayerStack>,
    mut next_state: ResMut<NextState<GameState>>,
    mut npc_dialog_state: ResMut<NpcDialogState>,
    current_state: Res<State<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    
    // Get topmost layer
    if let Some(top_layer) = ui_stack.pop_layer() {
        match top_layer.layer_type {
            UILayerType::NpcDialog => {
                // Close dialog
                npc_dialog_state.close_dialog();
            }
            UILayerType::Settings => {
                // Back to pause
                next_state.set(GameState::Paused);
            }
            UILayerType::PauseMenu => {
                // Resume game
                next_state.set(GameState::InGame);
            }
            UILayerType::GameUI => {
                // Open pause menu
                next_state.set(GameState::Paused);
            }
        }
    } else {
        // No layers, default behavior: open pause if in game
        if *current_state.get() == GameState::InGame {
            next_state.set(GameState::Paused);
        }
    }
}
```

## Implementation Steps

### Phase 1: Create UI Stack Module

**File:** `client/src/ui/ui_stack.rs` (NEW)

**Contents:**
```rust
use bevy::prelude::*;

/// Priority-based UI layer management
#[derive(Resource, Default)]
pub struct UILayerStack {
    layers: Vec<UILayer>,
}

impl UILayerStack {
    pub fn push_layer(&mut self, layer_type: UILayerType) {
        let layer = UILayer::new(layer_type);
        // Keep sorted by priority (highest first)
        let insert_pos = self.layers
            .iter()
            .position(|l| l.priority < layer.priority)
            .unwrap_or(self.layers.len());
        self.layers.insert(insert_pos, layer);
        info!("UI Layer pushed: {:?} (priority: {})", layer_type, layer.priority);
    }
    
    pub fn pop_layer(&mut self) -> Option<UILayer> {
        self.layers.pop()
    }
    
    pub fn remove_layer(&mut self, layer_type: UILayerType) {
        self.layers.retain(|l| l.layer_type != layer_type);
        info!("UI Layer removed: {:?}", layer_type);
    }
    
    pub fn top_layer(&self) -> Option<&UILayer> {
        self.layers.last()
    }
    
    pub fn has_layer(&self, layer_type: UILayerType) -> bool {
        self.layers.iter().any(|l| l.layer_type == layer_type)
    }
    
    pub fn clear(&mut self) {
        self.layers.clear();
    }
    
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UILayerType {
    GameUI,
    NpcDialog,
    PauseMenu,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub struct UILayer {
    pub layer_type: UILayerType,
    pub priority: i32,
    pub blocks_input: bool,
}

impl UILayer {
    pub fn new(layer_type: UILayerType) -> Self {
        let (priority, blocks_input) = match layer_type {
            UILayerType::GameUI => (100, false),
            UILayerType::PauseMenu => (200, true),
            UILayerType::Settings => (250, true),
            UILayerType::NpcDialog => (300, true),
        };
        
        Self {
            layer_type,
            priority,
            blocks_input,
        }
    }
}

/// Plugin for UI stack management
pub struct UIStackPlugin;

impl Plugin for UIStackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UILayerStack>()
            .add_systems(Update, handle_escape_key);
    }
}

/// Centralized ESC key handler
fn handle_escape_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_stack: ResMut<UILayerStack>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut npc_dialog_state: ResMut<crate::interaction::NpcDialogState>,
    current_state: Res<State<crate::GameState>>,
) {
    use crate::GameState;
    
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    
    // Handle topmost layer first
    if let Some(top_layer) = ui_stack.top_layer() {
        info!("ESC pressed - handling layer: {:?}", top_layer.layer_type);
        
        match top_layer.layer_type {
            UILayerType::NpcDialog => {
                // Close NPC dialog
                npc_dialog_state.close_dialog();
                ui_stack.remove_layer(UILayerType::NpcDialog);
            }
            UILayerType::Settings => {
                // Back to pause menu
                next_state.set(GameState::Paused);
                ui_stack.remove_layer(UILayerType::Settings);
            }
            UILayerType::PauseMenu => {
                // Resume game
                next_state.set(GameState::InGame);
                ui_stack.remove_layer(UILayerType::PauseMenu);
            }
            UILayerType::GameUI => {
                // Open pause menu (only if in game)
                if *current_state.get() == GameState::InGame {
                    next_state.set(GameState::Paused);
                    ui_stack.push_layer(UILayerType::PauseMenu);
                }
            }
        }
    } else {
        // No layers on stack - default behavior
        if *current_state.get() == GameState::InGame {
            info!("ESC pressed - opening pause menu (no layers)");
            next_state.set(GameState::Paused);
            ui_stack.push_layer(UILayerType::PauseMenu);
        }
    }
}
```

**Estimated Lines:** ~150

---

### Phase 2: Update UI Module

**File:** `client/src/ui/mod.rs`

**Changes:**
```rust
mod login;
mod character_creation;
mod character_selection;
mod game_ui;
mod npc_dialog;
mod pause;
mod settings;
mod ui_stack;  // NEW

pub use login::LoginPlugin;
pub use character_creation::CharacterCreationPlugin;
pub use character_selection::CharacterSelectionPlugin;
pub use game_ui::{GameUIPlugin, PlayerStats};
pub use npc_dialog::NpcDialogPlugin;
pub use pause::PausePlugin;
pub use settings::SettingsPlugin;
pub use ui_stack::{UIStackPlugin, UILayerStack, UILayerType};  // NEW

// ... rest unchanged
```

**Estimated Changes:** +3 lines

---

### Phase 3: Update NPC Dialog Plugin

**File:** `client/src/ui/npc_dialog.rs`

**Changes:**

1. **Import UI stack:**
```rust
use super::{UILayerStack, UILayerType};
```

2. **Push layer when dialog spawns:**
```rust
fn spawn_npc_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
    // ... rest
) {
    // Only spawn if dialog is active and doesn't exist yet
    if !dialog_state.active || !existing_dialog.is_empty() {
        return;
    }
    
    // Register layer
    ui_stack.push_layer(UILayerType::NpcDialog);  // NEW
    
    // ... rest of spawn logic unchanged
}
```

3. **Remove layer when dialog closes:**
```rust
fn cleanup_closed_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    dialog_query: Query<Entity, With<NpcDialogUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    if !dialog_state.active && !dialog_query.is_empty() {
        // Remove from stack
        ui_stack.remove_layer(UILayerType::NpcDialog);  // NEW
        
        for entity in dialog_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

**Estimated Changes:** +10 lines (2 imports, 2 params, 2 calls)

---

### Phase 4: Update Pause Plugin

**File:** `client/src/ui/pause.rs`

**Changes:**

1. **Import UI stack:**
```rust
use super::{UILayerStack, UILayerType};
```

2. **Push layer on enter:**
```rust
fn setup_pause(
    mut commands: Commands,
    font: Res<GameFont>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Register layer
    ui_stack.push_layer(UILayerType::PauseMenu);  // NEW
    
    // ... rest unchanged
}
```

3. **Remove ESC handler from pause_buttons():**
```rust
fn pause_buttons(
    mut interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<AuthState>,
    mut exit: EventWriter<AppExit>,
    // REMOVE: keyboard: Res<ButtonInput<KeyCode>>,
) {
    // REMOVE: ESC key handler (lines 234-238)
    // The centralized handler in ui_stack.rs now handles this
    
    for (interaction, button) in interaction_query.iter_mut() {
        // ... rest unchanged
    }
}
```

4. **Remove layer on exit:**
```rust
fn cleanup_pause(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::PauseMenu);  // NEW
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

**Estimated Changes:** +8 lines, -5 lines (net +3)

---

### Phase 5: Update Settings Plugin

**File:** `client/src/ui/settings.rs`

**Changes:**

1. **Import UI stack:**
```rust
use super::{UILayerStack, UILayerType};
```

2. **Push layer on enter:**
```rust
fn setup_settings(
    mut commands: Commands,
    font: Res<GameFont>,
    settings: Res<MMOSettings>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Register layer
    ui_stack.push_layer(UILayerType::Settings);  // NEW
    
    // ... rest unchanged
}
```

3. **Remove ESC handler from settings_buttons():**
```rust
fn settings_buttons(
    mut interaction_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<MMOSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    // REMOVE: keyboard: Res<ButtonInput<KeyCode>>,
) {
    // REMOVE: ESC key handler (lines 332-336)
    
    for (interaction, button) in interaction_query.iter_mut() {
        // ... rest unchanged
    }
}
```

4. **Remove layer on exit:**
```rust
fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::Settings);  // NEW
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

**Estimated Changes:** +8 lines, -5 lines (net +3)

---

### Phase 6: Update Game UI Plugin

**File:** `client/src/ui/game_ui.rs`

**Changes:**

1. **Import UI stack:**
```rust
use super::{UILayerStack, UILayerType};
```

2. **Push base layer on enter:**
```rust
fn setup_game_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Register base game UI layer
    ui_stack.push_layer(UILayerType::GameUI);  // NEW
    
    // ... rest unchanged
}
```

3. **Remove ESC handler from update_instructions():**
```rust
fn update_instructions(
    keyboard: Res<ButtonInput<KeyCode>>,
    // REMOVE: mut next_state: ResMut<NextState<GameState>>,
) {
    // REMOVE: ESC handler (lines 573-575)
    // The centralized handler in ui_stack.rs now handles this
}
```

4. **Remove layer on exit:**
```rust
fn cleanup_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // NEW
) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::GameUI);  // NEW
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

**Estimated Changes:** +8 lines, -4 lines (net +4)

---

### Phase 7: Register Plugin in Main

**File:** `client/src/main.rs`

**Changes:**

Find the plugin registration section and add:
```rust
.add_plugins((
    // ... existing plugins
    UIStackPlugin,  // NEW - Add before other UI plugins
    LoginPlugin,
    CharacterCreationPlugin,
    // ... rest
))
```

**Estimated Changes:** +1 line

---

## Testing Strategy

### Test 1: NPC Dialog ESC Handling
**Steps:**
1. Start game, login, select character
2. Walk to NPC (Meister der Künste at 5,1,5)
3. Click NPC to open dialog
4. Press ESC

**Expected:**
- Dialog closes
- Pause menu does NOT open
- Game remains active

**Verify:**
- Console log: "UI Layer removed: NpcDialog"
- No pause menu visible

---

### Test 2: Sequential ESC from Dialog to Pause
**Steps:**
1. Open NPC dialog (as above)
2. Press ESC (closes dialog)
3. Press ESC again

**Expected:**
- First ESC: Dialog closes
- Second ESC: Pause menu opens
- Clean transition

**Verify:**
- Console logs:
  - "UI Layer removed: NpcDialog"
  - "UI Layer pushed: PauseMenu"

---

### Test 3: Pause Menu ESC Handling
**Steps:**
1. In game (no dialog)
2. Press ESC (opens pause)
3. Press ESC again

**Expected:**
- First ESC: Pause menu opens
- Second ESC: Resume game
- No dialog appears

**Verify:**
- Console logs:
  - "UI Layer pushed: PauseMenu"
  - "UI Layer removed: PauseMenu"

---

### Test 4: Settings ESC Handling
**Steps:**
1. Open pause menu
2. Click "Einstellungen"
3. Press ESC

**Expected:**
- Returns to pause menu
- Does NOT resume game

**Verify:**
- Console log: "UI Layer removed: Settings"
- Pause menu visible

---

### Test 5: Multiple Layers Stack
**Steps:**
1. Press ESC (opens pause)
2. Open settings
3. Open NPC dialog (hypothetically)
4. Press ESC 3 times

**Expected:**
- Each ESC closes one layer
- Layers close in LIFO order: Dialog → Settings → Pause
- Finally returns to game

---

### Test 6: State Transition with Dialog Open
**Steps:**
1. Open NPC dialog
2. Click "Zum Hauptmenü" in pause (if accessible)

**Expected:**
- Dialog closes
- State changes correctly
- UI stack clears

---

## Edge Cases & Considerations

### 1. State Transitions
**Issue:** What if state changes while dialog is open?

**Solution:** Clear stack on state exit
```rust
fn on_exit_ingame(mut ui_stack: ResMut<UILayerStack>) {
    ui_stack.clear();
}
```

### 2. Multiple Dialogs
**Issue:** What if two dialogs try to open?

**Solution:** Check if layer already exists
```rust
if !ui_stack.has_layer(UILayerType::NpcDialog) {
    ui_stack.push_layer(UILayerType::NpcDialog);
}
```

### 3. Input Blocking
**Issue:** Should lower layers receive input?

**Solution:** Use `blocks_input` flag
```rust
if let Some(top) = ui_stack.top_layer() {
    if top.blocks_input {
        return; // Don't process input in lower systems
    }
}
```

### 4. Z-Index Coordination
**Issue:** Ensure UI z-index matches stack priority

**Current Z-indices:**
- NpcDialog: 200
- PauseMenu: (not set, default 0)
- GameUI: 100

**Recommendation:** Match z-index to priority
- GameUI: 100
- PauseMenu: 200
- Settings: 250
- NpcDialog: 300

---

## Benefits of This System

### 1. Centralized Control
- Single source of truth for ESC handling
- Easy to debug UI layer issues
- Clear hierarchy

### 2. Extensibility
- Easy to add new UI layers (Inventory, Map, Skills)
- Priority-based rendering
- Input blocking per layer

### 3. Maintainability
- No scattered ESC handlers
- Clear layer lifecycle (push on spawn, pop on cleanup)
- Console logging for debugging

### 4. User Experience
- Predictable ESC behavior
- No UI conflicts
- Clean layer management

---

## Future Enhancements

### 1. Input Blocking System
```rust
fn block_game_input(
    ui_stack: Res<UILayerStack>,
    mut player_query: Query<&mut Player>,
) {
    if let Some(top) = ui_stack.top_layer() {
        if top.blocks_input {
            // Disable player movement
            for mut player in player_query.iter_mut() {
                player.input_blocked = true;
            }
        }
    }
}
```

### 2. Layer-Specific Events
```rust
#[derive(Event)]
struct UILayerEvent {
    layer_type: UILayerType,
    action: UILayerAction,
}

enum UILayerAction {
    Opened,
    Closed,
}
```

### 3. Animation System
```rust
impl UILayer {
    pub fn transition_in(&self) -> AnimationType {
        match self.layer_type {
            UILayerType::NpcDialog => AnimationType::FadeIn,
            UILayerType::PauseMenu => AnimationType::SlideFromTop,
            _ => AnimationType::Instant,
        }
    }
}
```

### 4. Layer State Persistence
```rust
impl UILayerStack {
    pub fn save_state(&self) -> Vec<UILayerType> {
        self.layers.iter().map(|l| l.layer_type).collect()
    }
    
    pub fn restore_state(&mut self, state: Vec<UILayerType>) {
        for layer_type in state {
            self.push_layer(layer_type);
        }
    }
}
```

---

## Code Statistics

**New Files:** 1
- `client/src/ui/ui_stack.rs`: ~150 lines

**Modified Files:** 6
- `client/src/ui/mod.rs`: +3 lines
- `client/src/ui/npc_dialog.rs`: +10 lines
- `client/src/ui/pause.rs`: +8 -5 = +3 lines
- `client/src/ui/settings.rs`: +8 -5 = +3 lines
- `client/src/ui/game_ui.rs`: +8 -4 = +4 lines
- `client/src/main.rs`: +1 line

**Total Changes:** ~182 lines added, ~14 lines removed

**Net New Code:** ~168 lines

---

## Implementation Order

1. ✅ **Phase 1** - Create `ui_stack.rs` module (core functionality)
2. ✅ **Phase 2** - Update `ui/mod.rs` (exports)
3. ✅ **Phase 7** - Register plugin in `main.rs` (enable system)
4. ✅ **Phase 3** - Update NPC dialog (highest priority layer)
5. ✅ **Phase 4** - Update pause menu
6. ✅ **Phase 5** - Update settings menu
7. ✅ **Phase 6** - Update game UI (base layer)
8. ✅ **Testing** - Run all test scenarios

**Estimated Implementation Time:** 30-45 minutes

---

## Success Criteria

- ✅ ESC closes NPC dialog when open
- ✅ ESC opens pause menu when no dialog
- ✅ ESC in pause menu resumes game
- ✅ ESC in settings returns to pause
- ✅ No UI conflicts or state issues
- ✅ Console logs show correct layer operations
- ✅ Code is clean and well-documented
- ✅ All existing functionality preserved

---

## Risk Assessment

**Low Risk:**
- Centralized ESC handling is cleaner than scattered handlers
- Stack-based approach is well-proven pattern
- Changes are additive, not destructive
- Easy to rollback if needed

**Potential Issues:**
- State transition timing (solution: clear stack on exit)
- Multiple spawns (solution: check existence before push)
- Z-index conflicts (solution: coordinate with priority)

**Mitigation:**
- Comprehensive testing before commit
- Console logging for debugging
- Clear documentation

---

## Documentation

After implementation, create:
- `UI_STACK_SYSTEM.md` - System overview
- Update `AGENTS.md` - Add UI stack section
- Update `README.md` - Note improved UX

---

## Conclusion

This UI Stack system solves the ESC key conflict by introducing a priority-based layer management system. It's extensible, maintainable, and provides a foundation for future UI features like inventory, map, and skill windows.

**Status:** Ready for implementation
**Complexity:** Medium
**Impact:** High (significantly improves UX)
**Risk:** Low

Let's proceed with implementation! 🚀
