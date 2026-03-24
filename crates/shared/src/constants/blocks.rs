//! Block type definitions and properties

/// Maximum block state ID
pub const MAX_BLOCK_STATES: u32 = 16_384;

/// Total number of block types
pub const BLOCK_TYPE_COUNT: u16 = 256;

/// Air block ID (no collision, no rendering)
pub const BLOCK_AIR: u16 = 0;

/// Stone block ID
pub const BLOCK_STONE: u16 = 1;

/// Grass block ID
pub const BLOCK_GRASS: u16 = 2;

/// Dirt block ID
pub const BLOCK_DIRT: u16 = 3;

/// Cobblestone block ID
pub const BLOCK_COBBLESTONE: u16 = 4;

/// Planks block ID (generic wood planks)
pub const BLOCK_PLANKS: u16 = 5;

/// Sand block ID
pub const BLOCK_SAND: u16 = 12;

/// Water block ID
pub const BLOCK_WATER: u16 = 8;

/// Lava block ID
pub const BLOCK_LAVA: u16 = 10;

/// Oak log block ID
pub const BLOCK_OAK_LOG: u16 = 17;

/// Oak leaves block ID
pub const BLOCK_OAK_LEAVES: u16 = 18;

/// Bedrock block ID (indestructible)
pub const BLOCK_BEDROCK: u16 = 7;

/// Coal ore block ID
pub const BLOCK_COAL_ORE: u16 = 16;

/// Iron ore block ID
pub const BLOCK_IRON_ORE: u16 = 15;

/// Gold ore block ID
pub const BLOCK_GOLD_ORE: u16 = 14;

/// Diamond ore block ID
pub const BLOCK_DIAMOND_ORE: u16 = 56;

/// Emerald ore block ID
pub const BLOCK_EMERALD_ORE: u16 = 129;

/// Copper ore block ID
pub const BLOCK_COPPER_ORE: u16 = 23;

/// Oak sapling block ID
pub const BLOCK_OAK_SAPLING: u16 = 6;

/// Snow block ID
pub const BLOCK_SNOW: u16 = 80;

/// Ice block ID
pub const BLOCK_ICE: u16 = 79;

/// Clay block ID
pub const BLOCK_CLAY: u16 = 82;

/// Gravel block ID
pub const BLOCK_GRAVEL: u16 = 13;

/// Granite block ID
pub const BLOCK_GRANITE: u16 = 1; // Reserved for future

/// Block name to ID mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockDefinition {
    pub id: u16,
    pub name: &'static str,
    pub solid: bool,
    pub transparent: bool,
    pub liquid: bool,
}

impl BlockDefinition {
    pub const fn new(id: u16, name: &'static str, solid: bool, transparent: bool, liquid: bool) -> Self {
        Self { id, name, solid, transparent, liquid }
    }
}

/// Get block definition by ID
pub fn get_block(id: u16) -> Option<BlockDefinition> {
    Some(match id {
        BLOCK_AIR => BlockDefinition::new(BLOCK_AIR, "air", false, true, false),
        BLOCK_STONE => BlockDefinition::new(BLOCK_STONE, "stone", true, false, false),
        BLOCK_GRASS => BlockDefinition::new(BLOCK_GRASS, "grass_block", true, false, false),
        BLOCK_DIRT => BlockDefinition::new(BLOCK_DIRT, "dirt", true, false, false),
        BLOCK_COBBLESTONE => BlockDefinition::new(BLOCK_COBBLESTONE, "cobblestone", true, false, false),
        BLOCK_PLANKS => BlockDefinition::new(BLOCK_PLANKS, "oak_planks", true, false, false),
        BLOCK_SAND => BlockDefinition::new(BLOCK_SAND, "sand", true, false, false),
        BLOCK_WATER => BlockDefinition::new(BLOCK_WATER, "water", false, true, true),
        BLOCK_LAVA => BlockDefinition::new(BLOCK_LAVA, "lava", false, true, true),
        BLOCK_OAK_LOG => BlockDefinition::new(BLOCK_OAK_LOG, "oak_log", true, false, false),
        BLOCK_OAK_LEAVES => BlockDefinition::new(BLOCK_OAK_LEAVES, "oak_leaves", false, true, false),
        BLOCK_BEDROCK => BlockDefinition::new(BLOCK_BEDROCK, "bedrock", true, false, false),
        BLOCK_COAL_ORE => BlockDefinition::new(BLOCK_COAL_ORE, "coal_ore", true, false, false),
        BLOCK_IRON_ORE => BlockDefinition::new(BLOCK_IRON_ORE, "iron_ore", true, false, false),
        BLOCK_GOLD_ORE => BlockDefinition::new(BLOCK_GOLD_ORE, "gold_ore", true, false, false),
        BLOCK_DIAMOND_ORE => BlockDefinition::new(BLOCK_DIAMOND_ORE, "diamond_ore", true, false, false),
        BLOCK_EMERALD_ORE => BlockDefinition::new(BLOCK_EMERALD_ORE, "emerald_ore", true, false, false),
        BLOCK_COPPER_ORE => BlockDefinition::new(BLOCK_COPPER_ORE, "copper_ore", true, false, false),
        BLOCK_OAK_SAPLING => BlockDefinition::new(BLOCK_OAK_SAPLING, "oak_sapling", false, true, false),
        BLOCK_SNOW => BlockDefinition::new(BLOCK_SNOW, "snow_block", true, false, false),
        BLOCK_ICE => BlockDefinition::new(BLOCK_ICE, "ice", true, false, false),
        BLOCK_CLAY => BlockDefinition::new(BLOCK_CLAY, "clay", true, false, false),
        BLOCK_GRAVEL => BlockDefinition::new(BLOCK_GRAVEL, "gravel", true, false, false),
        _ => return None,
    })
}
