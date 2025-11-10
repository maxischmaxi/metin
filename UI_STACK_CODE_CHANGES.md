# UI Stack System - Exact Code Changes

This document shows the **exact code changes** needed for each file.

---

## Summary of Changes

| Phase | File | Action | Lines |
|-------|------|--------|-------|
| 1 | `ui/ui_stack.rs` | CREATE | ~150 |
| 2 | `ui/mod.rs` | MODIFY | +3 |
| 3 | `main.rs` | MODIFY | +1 |
| 4 | `ui/npc_dialog.rs` | MODIFY | +10 |
| 5 | `ui/pause.rs` | MODIFY | +8, -5 |
| 6 | `ui/settings.rs` | MODIFY | +8, -5 |
| 7 | `ui/game_ui.rs` | MODIFY | +8, -4 |

---

## Phase 1: Create UI Stack Module

**File:** `client/src/ui/ui_stack.rs` (NEW FILE)

<details>
<summary>Click to expand full file content (~150 lines)</summary>

```rust
use bevy::prelude::*;

/// Priority-based UI layer management system
#[derive(Resource, Default)]
pub struct UILayerStack {
    layers: Vec<UILayer>,
}

impl UILayerStack {
    /// Add a layer to the stack (sorted by priority)
    pub fn push_layer(&mut self, layer_type: UILayerType) {
        // Don't add if already exists
        if self.has_layer(layer_type) {
            warn!("Layer {:?} already exists in stack", layer_type);
            return;
        }
        
        let layer = UILayer::new(layer_type);
        
        // Keep sorted by priority (highest first)
        let insert_pos = self.layers
            .iter()
            .position(|l| l.priority < layer.priority)
            .unwrap_or(self.layers.len());
        
        self.layers.insert(insert_pos, layer);
        info!("UI Layer pushed: {:?} (priority: {})", layer_type, layer.priority);
    }
    
    /// Remove and return the top layer
    pub fn pop_layer(&mut self) -> Option<UILayer> {
        let layer = self.layers.pop();
        if let Some(ref l) = layer {
            info!("UI Layer popped: {:?}", l.layer_type);
        }
        layer
    }
    
    /// Remove a specific layer from the stack
    pub fn remove_layer(&mut self, layer_type: UILayerType) {
        let len_before = self.layers.len();
        self.layers.retain(|l| l.layer_type != layer_type);
        
        if self.layers.len() < len_before {
            info!("UI Layer removed: {:?}", layer_type);
        }
    }
    
    /// Get the topmost layer without removing it
    pub fn top_layer(&self) -> Option<&UILayer> {
        self.layers.last()
    }
    
    /// Check if a specific layer exists in the stack
    pub fn has_layer(&self, layer_type: UILayerType) -> bool {
        self.layers.iter().any(|l| l.layer_type == layer_type)
    }
    
    /// Clear all layers
    pub fn clear(&mut self) {
        if !self.layers.is_empty() {
            info!("UI Stack cleared ({} layers)", self.layers.len());
            self.layers.clear();
        }
    }
    
    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
    
    /// Get number of layers
    pub fn len(&self) -> usize {
        self.layers.len()
    }
}

/// Types of UI layers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UILayerType {
    GameUI,        // Base in-game UI (health bars, etc.)
    NpcDialog,     // NPC conversation dialogs
    PauseMenu,     // Pause menu
    Settings,      // Settings menu
}

/// A single UI layer with metadata
#[derive(Debug, Clone, Copy)]
pub struct UILayer {
    pub layer_type: UILayerType,
    pub priority: i32,
    pub blocks_input: bool,
}

impl UILayer {
    pub fn new(layer_type: UILayerType) -> Self {
        let (priority, blocks_input) = match layer_type {
            UILayerType::GameUI => (100, false),      // Base layer, doesn't block
            UILayerType::PauseMenu => (200, true),    // Blocks game input
            UILayerType::Settings => (250, true),     // Blocks everything below
            UILayerType::NpcDialog => (300, true),    // Highest priority overlay
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

/// Centralized ESC key handler - processes topmost layer first
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
    
    // Handle topmost layer first (LIFO)
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

</details>

---

## Phase 2: Update UI Module

**File:** `client/src/ui/mod.rs`

**Line 7** - Add after `mod settings;`:
```rust
mod ui_stack;
```

**Line 15** - Add after `pub use settings::SettingsPlugin;`:
```rust
pub use ui_stack::{UIStackPlugin, UILayerStack, UILayerType};
```

---

## Phase 3: Register Plugin

**File:** `client/src/main.rs`

Find the `.add_plugins()` section and add `UIStackPlugin` **before** other UI plugins:

```rust
.add_plugins((
    // ... existing plugins
    UIStackPlugin,  // ADD THIS - Must be before other UI plugins
    LoginPlugin,
    CharacterCreationPlugin,
    CharacterSelectionPlugin,
    GameUIPlugin,
    NpcDialogPlugin,
    PausePlugin,
    SettingsPlugin,
))
```

---

## Phase 4: Update NPC Dialog

**File:** `client/src/ui/npc_dialog.rs`

**Line 12** - Add import after other use statements:
```rust
use super::{UILayerStack, UILayerType};
```

**Line 36** - Add parameter to `spawn_npc_dialog`:
```rust
fn spawn_npc_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
    player_stats: Res<PlayerStats>,
    auth_state: Res<AuthState>,
    font: Res<GameFont>,
    existing_dialog: Query<Entity, With<NpcDialogUI>>,
)
```

**Line 45** - Add after the return statement:
```rust
// Only spawn if dialog is active and doesn't exist yet
if !dialog_state.active || !existing_dialog.is_empty() {
    return;
}

// Register layer
ui_stack.push_layer(UILayerType::NpcDialog);  // ADD THIS
```

**Line 301** - Add parameter to `cleanup_closed_dialog`:
```rust
fn cleanup_closed_dialog(
    mut commands: Commands,
    dialog_state: Res<NpcDialogState>,
    dialog_query: Query<Entity, With<NpcDialogUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line 305** - Add before the for loop:
```rust
if !dialog_state.active && !dialog_query.is_empty() {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::NpcDialog);  // ADD THIS
    
    for entity in dialog_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

---

## Phase 5: Update Pause Menu

**File:** `client/src/ui/pause.rs`

**Line 6** - Add import:
```rust
use super::{button_system, NORMAL_BUTTON, UILayerStack, UILayerType};
```

**Line 33** - Add parameter to `setup_pause`:
```rust
fn setup_pause(
    mut commands: Commands,
    font: Res<GameFont>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line 35** - Add after line that gets font_handle:
```rust
fn setup_pause(mut commands: Commands, font: Res<GameFont>, mut ui_stack: ResMut<UILayerStack>) {
    // Register layer
    ui_stack.push_layer(UILayerType::PauseMenu);  // ADD THIS
    
    let font_handle = font.0.clone();
```

**Line 227** - Modify `pause_buttons` signature (REMOVE keyboard parameter):
```rust
fn pause_buttons(
    mut interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<AuthState>,
    mut exit: EventWriter<AppExit>,
    // REMOVE THIS: keyboard: Res<ButtonInput<KeyCode>>,
)
```

**Lines 234-238** - DELETE the ESC key handler:
```rust
// DELETE THESE LINES:
// // ESC key to resume
// if keyboard.just_pressed(KeyCode::Escape) {
//     next_state.set(GameState::InGame);
//     return;
// }
```

**Line 268** - Add parameter to `cleanup_pause`:
```rust
fn cleanup_pause(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line 271** - Add before the for loop:
```rust
fn cleanup_pause(...) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::PauseMenu);  // ADD THIS
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

---

## Phase 6: Update Settings Menu

**File:** `client/src/ui/settings.rs`

**Line ~10** - Add to imports:
```rust
use super::{button_system, NORMAL_BUTTON, UILayerStack, UILayerType};
```

**Line ~75** - Add parameter to `setup_settings`:
```rust
fn setup_settings(
    mut commands: Commands,
    font: Res<GameFont>,
    settings: Res<MMOSettings>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line ~77** - Add after line that gets font_handle:
```rust
fn setup_settings(...) {
    // Register layer
    ui_stack.push_layer(UILayerType::Settings);  // ADD THIS
    
    let font_handle = font.0.clone();
```

**Line 325** - Modify `settings_buttons` signature (REMOVE keyboard parameter):
```rust
fn settings_buttons(
    mut interaction_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<MMOSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    // REMOVE THIS: keyboard: Res<ButtonInput<KeyCode>>,
)
```

**Lines 332-336** - DELETE the ESC key handler:
```rust
// DELETE THESE LINES:
// // ESC key to go back to pause menu
// if keyboard.just_pressed(KeyCode::Escape) {
//     next_state.set(GameState::Paused);
//     return;
// }
```

**Line ~440** - Add parameter to `cleanup_settings`:
```rust
fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line ~443** - Add before the for loop:
```rust
fn cleanup_settings(...) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::Settings);  // ADD THIS
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

---

## Phase 7: Update Game UI

**File:** `client/src/ui/game_ui.rs`

**Line ~12** - Add to imports:
```rust
use super::{NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, UILayerStack, UILayerType};
```

**Line ~100** - Add parameter to `setup_game_ui`:
```rust
fn setup_game_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line ~102** - Add as first line in function:
```rust
fn setup_game_ui(...) {
    // Register base game UI layer
    ui_stack.push_layer(UILayerType::GameUI);  // ADD THIS
    
    // ... rest of function
```

**Line 569** - Modify `update_instructions` signature (REMOVE next_state parameter):
```rust
fn update_instructions(
    keyboard: Res<ButtonInput<KeyCode>>,
    // REMOVE THIS: mut next_state: ResMut<NextState<GameState>>,
)
```

**Lines 573-575** - DELETE the ESC key handler:
```rust
// DELETE THESE LINES:
// if keyboard.just_pressed(KeyCode::Escape) {
//     next_state.set(GameState::Paused);
// }
```

**Line ~640** - Add parameter to `cleanup_game_ui`:
```rust
fn cleanup_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameUI>>,
    mut ui_stack: ResMut<UILayerStack>,  // ADD THIS
)
```

**Line ~643** - Add before the for loop:
```rust
fn cleanup_game_ui(...) {
    // Remove from stack
    ui_stack.remove_layer(UILayerType::GameUI);  // ADD THIS
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

---

## Verification Checklist

After making changes, verify:

- [ ] `ui/ui_stack.rs` exists and compiles
- [ ] All 7 files have been modified correctly
- [ ] No compilation errors: `cargo build --release`
- [ ] No warnings about unused parameters
- [ ] ESC handlers removed from individual systems
- [ ] Layer push/remove calls added to spawn/cleanup functions
- [ ] UIStackPlugin registered in main.rs BEFORE other UI plugins

---

## Build & Test

```bash
cd /home/max/code/game

# Build
cargo build --release

# If successful, test:
./run_client.sh

# Watch for these console logs:
# [INFO] UI Layer pushed: GameUI (priority: 100)
# [INFO] UI Layer pushed: NpcDialog (priority: 300)
# [INFO] ESC pressed - handling layer: NpcDialog
# [INFO] UI Layer removed: NpcDialog
```

---

## Common Issues & Solutions

### Issue: "cannot find UILayerStack in scope"
**Solution:** Make sure `ui/mod.rs` exports it and main.rs imports it

### Issue: "ESC still opens pause with dialog open"
**Solution:** Verify UIStackPlugin is registered BEFORE other UI plugins

### Issue: "Layer not removed on cleanup"
**Solution:** Check that cleanup functions have UILayerStack parameter

### Issue: Compilation errors about removed parameters
**Solution:** Update system signatures to remove keyboard/next_state parameters

---

## Success Indicators

✅ **Code compiles without errors**
✅ **No unused parameter warnings**
✅ **Console shows "UI Layer pushed/removed" messages**
✅ **ESC closes dialog before opening pause menu**
✅ **All existing functionality still works**

---

**Ready to implement!** Follow the phases in order for smooth integration.
