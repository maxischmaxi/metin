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
        
        // Keep sorted by priority (lowest first, highest last)
        // This way .last() returns the highest priority layer
        let insert_pos = self.layers
            .iter()
            .position(|l| l.priority > layer.priority)
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
    #[allow(dead_code)]
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
