use Addr;
use system::memory::{reference, read, write};

pub const OFFSET: Addr = 0x803B8144;

#[repr(C, packed)]
pub struct Inventory {
    pub telescope_slot: u8,
    pub sail_slot: u8,
    pub wind_waker_slot: u8,
    pub grappling_hook_slot: u8,
    pub spoils_bag_slot: u8,
    pub boomerang_slot: u8,
    pub deku_leaf_slot: u8,
    pub tingle_tuner_slot: u8,
    pub picto_box_slot: u8,
    pub iron_boots_slot: u8,
    pub magic_armor_slot: u8,
    pub bait_bag_slot: u8,
    pub bow_slot: u8,
    pub bombs_slot: u8,
    pub bottle1_slot: u8,
    pub bottle2_slot: u8,
    pub bottle3_slot: u8,
    pub bottle4_slot: u8,
    pub delivery_bag_slot: u8,
    pub hookshot_slot: u8,
    pub skull_hammer_slot: u8,
    _p0: [u8; 18],
    pub has_delivery_bag: bool, // 6B
    _p1: [u8; 5],
    pub arrow_count: u8, // 71
    pub bomb_count: u8, // 72
    _p2: [u8; 4],
    pub arrow_capacity: u8, // 77
    pub bomb_capacity: u8, // 78
    _p3: [u8; 5],
    pub spoils_bag: SpoilsBag, // 7E
    pub bait_bag: BaitBag, // 86
    pub delivery_bag: DeliveryBag, // 8E
}

#[repr(C, packed)]
pub struct SpoilsBag {
    pub items: [u8; 8],
}

#[repr(C, packed)]
pub struct BaitBag {
    pub items: [u8; 8],
}

#[repr(C, packed)]
pub struct DeliveryBag {
    pub items: [u8; 8],
}

impl Inventory {
    pub fn get() -> &'static mut Inventory {
        reference(OFFSET)
    }

    pub fn get_by_slot_id(slot_id: usize) -> u8 {
        read(OFFSET + slot_id)
    }

    pub fn set_by_slot_id(slot_id: usize, item_id: u8) {
        write(OFFSET + slot_id, item_id)
    }
}
