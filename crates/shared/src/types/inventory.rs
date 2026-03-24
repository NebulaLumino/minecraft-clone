//! Inventory and item types

use serde::{Deserialize, Serialize};

/// Maximum slots in a player inventory (36 = 27 main + 9 armor/hotbar)
pub const PLAYER_INVENTORY_SIZE: usize = 36;

/// Maximum slots in a chest (27)
pub const CHEST_INVENTORY_SIZE: usize = 27;

/// Hotbar size
pub const HOTBAR_SIZE: usize = 9;

/// Item ID type
pub type ItemId = u16;

/// Inventory slot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Slot {
    pub item_id: ItemId,
    pub count: u8,
    pub damage: u16,
}

impl Slot {
    pub const EMPTY: Self = Self { item_id: 0, count: 0, damage: 0 };

    pub const fn new(item_id: ItemId, count: u8, damage: u16) -> Self {
        Self { item_id, count, damage }
    }

    pub fn is_empty(&self) -> bool {
        self.item_id == 0 || self.count == 0
    }
}

/// Item stack with slot position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item_id: ItemId,
    pub count: u8,
    pub damage: u16,
    pub nbt: Option<Vec<u8>>,
}

impl ItemStack {
    pub const EMPTY: Self = Self {
        item_id: 0,
        count: 0,
        damage: 0,
        nbt: None,
    };

    pub const fn new(item_id: ItemId, count: u8) -> Self {
        Self {
            item_id,
            count,
            damage: 0,
            nbt: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.item_id == 0 || self.count == 0
    }
}

/// Inventory container type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryType {
    Player,
    Hotbar,
    Chest,
    Furnace,
    CraftingTable,
    EnderChest,
}

impl InventoryType {
    pub fn slot_count(&self) -> usize {
        match self {
            Self::Player => PLAYER_INVENTORY_SIZE,
            Self::Hotbar => HOTBAR_SIZE,
            Self::Chest => CHEST_INVENTORY_SIZE,
            Self::Furnace => 3,
            Self::CraftingTable => 10,
            Self::EnderChest => 27,
        }
    }
}

/// Inventory with type and slots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub inventory_type: InventoryType,
    pub slots: Vec<Slot>,
}

impl Inventory {
    pub fn new(inventory_type: InventoryType) -> Self {
        let slot_count = inventory_type.slot_count();
        Self {
            inventory_type,
            slots: vec![Slot::EMPTY; slot_count],
        }
    }

    pub fn get_slot(&self, index: usize) -> Option<Slot> {
        self.slots.get(index).copied()
    }

    pub fn set_slot(&mut self, index: usize, slot: Slot) -> Option<Slot> {
        if index < self.slots.len() {
            let old = self.slots[index];
            self.slots[index] = slot;
            Some(old)
        } else {
            None
        }
    }
}
