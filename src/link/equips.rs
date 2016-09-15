use system::memory::reference;
use super::inventory::Inventory;

#[repr(C, packed)]
pub struct Equips {
    pub x_index: u8,
    pub y_index: u8,
    pub z_index: u8,
}

impl Equips {
    pub fn get() -> &'static mut Equips {
        reference(0x803B8111)
    }

    pub fn x_item_id(&self) -> u8 {
        Inventory::get_by_slot_id(self.x_index as usize)
    }

    pub fn y_item_id(&self) -> u8 {
        Inventory::get_by_slot_id(self.y_index as usize)
    }

    pub fn z_item_id(&self) -> u8 {
        Inventory::get_by_slot_id(self.z_index as usize)
    }
}
