//! Inventory module
//!
//! Player inventory and hotbar.

use voxel_shared::types::Slot;

/// Player inventory
pub struct Inventory {
    pub hotbar: [Slot; 9],
    pub main: [Slot; 27],
    pub armor: [Slot; 4],
    pub selected_slot: usize,
}

impl Inventory {
    /// Create new inventory
    pub fn new() -> Self {
        Self {
            hotbar: [Slot::EMPTY; 9],
            main: [Slot::EMPTY; 27],
            armor: [Slot::EMPTY; 4],
            selected_slot: 0,
        }
    }

    /// Get selected item
    pub fn selected(&self) -> Slot {
        self.hotbar[self.selected_slot]
    }

    /// Select hotbar slot
    pub fn select(&mut self, slot: usize) {
        if slot < 9 {
            self.selected_slot = slot;
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}
