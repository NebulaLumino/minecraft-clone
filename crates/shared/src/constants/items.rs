//! Item type definitions

/// Maximum item stack size
pub const MAX_STACK_SIZE: u8 = 64;

/// Diamond item ID
pub const ITEM_DIAMOND: u16 = 264;

/// Iron ingot item ID
pub const ITEM_IRON_INGOT: u16 = 265;

/// Gold ingot item ID
pub const ITEM_GOLD_INGOT: u16 = 266;

/// Coal item ID
pub const ITEM_COAL: u16 = 263;

/// Charcoal item ID
pub const ITEM_CHARCOAL: u16 = 263; // Same ID with different damage value

/// Wooden plank item ID
pub const ITEM_OAK_PLANKS: u16 = 5;

/// Stick item ID
pub const ITEM_STICK: u16 = 280;

/// Torch item ID
pub const ITEM_TORCH: u16 = 50;

/// Diamond sword item ID
pub const ITEM_DIAMOND_SWORD: u16 = 276;

/// Diamond pickaxe item ID
pub const ITEM_DIAMOND_PICKAXE: u16 = 278;

/// Iron sword item ID
pub const ITEM_IRON_SWORD: u16 = 267;

/// Iron pickaxe item ID
pub const ITEM_IRON_PICKAXE: u16 = 256;

/// Wooden sword item ID
pub const ITEM_WOODEN_SWORD: u16 = 268;

/// Wooden pickaxe item ID
pub const ITEM_WOODEN_PICKAXE: u16 = 269;

/// Stone sword item ID
pub const ITEM_STONE_SWORD: u16 = 272;

/// Stone pickaxe item ID
pub const ITEM_STONE_PICKAXE: u16 = 273;

/// Gold sword item ID
pub const ITEM_GOLDEN_SWORD: u16 = 283;

/// Gold pickaxe item ID
pub const ITEM_GOLDEN_PICKAXE: u16 = 284;

/// Item definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemDefinition {
    pub id: u16,
    pub name: &'static str,
    pub stack_size: u8,
    pub tool_type: Option<ToolType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    Sword,
    Pickaxe,
    Axe,
    Shovel,
    Hoe,
}

impl ItemDefinition {
    pub const fn new(id: u16, name: &'static str, stack_size: u8, tool_type: Option<ToolType>) -> Self {
        Self { id, name, stack_size, tool_type }
    }
}

/// Get item definition by ID
pub fn get_item(id: u16) -> Option<ItemDefinition> {
    Some(match id {
        ITEM_DIAMOND => ItemDefinition::new(ITEM_DIAMOND, "diamond", 64, None),
        ITEM_IRON_INGOT => ItemDefinition::new(ITEM_IRON_INGOT, "iron_ingot", 64, None),
        ITEM_GOLD_INGOT => ItemDefinition::new(ITEM_GOLD_INGOT, "gold_ingot", 64, None),
        ITEM_COAL => ItemDefinition::new(ITEM_COAL, "coal", 64, None),
        ITEM_OAK_PLANKS => ItemDefinition::new(ITEM_OAK_PLANKS, "oak_planks", 64, None),
        ITEM_STICK => ItemDefinition::new(ITEM_STICK, "stick", 64, None),
        ITEM_TORCH => ItemDefinition::new(ITEM_TORCH, "torch", 64, None),
        ITEM_DIAMOND_SWORD => ItemDefinition::new(ITEM_DIAMOND_SWORD, "diamond_sword", 1, Some(ToolType::Sword)),
        ITEM_DIAMOND_PICKAXE => ItemDefinition::new(ITEM_DIAMOND_PICKAXE, "diamond_pickaxe", 1, Some(ToolType::Pickaxe)),
        ITEM_IRON_SWORD => ItemDefinition::new(ITEM_IRON_SWORD, "iron_sword", 1, Some(ToolType::Sword)),
        ITEM_IRON_PICKAXE => ItemDefinition::new(ITEM_IRON_PICKAXE, "iron_pickaxe", 1, Some(ToolType::Pickaxe)),
        ITEM_WOODEN_SWORD => ItemDefinition::new(ITEM_WOODEN_SWORD, "wooden_sword", 1, Some(ToolType::Sword)),
        ITEM_WOODEN_PICKAXE => ItemDefinition::new(ITEM_WOODEN_PICKAXE, "wooden_pickaxe", 1, Some(ToolType::Pickaxe)),
        ITEM_STONE_SWORD => ItemDefinition::new(ITEM_STONE_SWORD, "stone_sword", 1, Some(ToolType::Sword)),
        ITEM_STONE_PICKAXE => ItemDefinition::new(ITEM_STONE_PICKAXE, "stone_pickaxe", 1, Some(ToolType::Pickaxe)),
        ITEM_GOLDEN_SWORD => ItemDefinition::new(ITEM_GOLDEN_SWORD, "golden_sword", 1, Some(ToolType::Sword)),
        ITEM_GOLDEN_PICKAXE => ItemDefinition::new(ITEM_GOLDEN_PICKAXE, "golden_pickaxe", 1, Some(ToolType::Pickaxe)),
        _ => return None,
    })
}
