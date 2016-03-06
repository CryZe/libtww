use Addr;
use system::memory::ptr;

pub const OFFSET: Addr = 0x803B8144;

#[repr(C)]
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
}

pub fn get() -> &'static mut Inventory {
    unsafe { &mut *ptr(OFFSET) }
}
