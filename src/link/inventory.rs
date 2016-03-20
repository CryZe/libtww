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
    _p0: [u8; 24],
    pub arrow_count: u8,
    pub bomb_count: u8,
    _p1: [u8; 4],
    pub arrow_capacity: u8,
    pub bomb_capacity: u8,
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
