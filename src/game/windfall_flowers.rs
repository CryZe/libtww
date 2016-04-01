use system::memory::reference;
use link::inventory::Inventory;

#[repr(C, packed)]
pub struct WindfallFlowers {
    pub shop_left: u8,
    _p0: u8,
    pub bench_bush: u8,
    pub bench_tree: u8,
    pub bench_stone: u8,
    pub platform_right: u8,
    pub shop_right: u8,
    pub platform_left: u8,
    pub alley_tree: u8,
    _p1: [u8; 2],
    pub gate_center_left: u8,
    pub gate_left_left: u8,
    pub gate_left_right: u8,
    pub gate_center_right: u8,
    pub gate_right_right: u8,
    pub gate_right_left: u8,
}

impl WindfallFlowers {
    pub fn get() -> &'static mut WindfallFlowers {
        reference(0x803B8814)
    }

    pub fn activate_pedestals() {
        let inventory = Inventory::get();
        inventory.has_delivery_bag = true;
    }
}
