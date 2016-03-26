use {Addr, Coord};
use system::memory::{ptr, read, write, read_str, reference};
use std::fmt::Display;
use std::fmt;
use self::quest_items::{Sword, Shield, QuestItems};

pub mod equips;
pub mod inventory;
pub mod item;
pub mod quest_items;
pub mod song;
pub mod triforce;
pub mod pearl;

pub const OFFSET: Addr = 0x803B8108;
pub const POSITION_OFFSET: Addr = 0x803d78fc;

#[repr(C, packed)]
pub struct Link {
    _p0: [u8; 1],
    pub heart_pieces: u8, // 8109
    _p1: [u8; 1],
    pub heart_quarters: u8, // 810B
    pub rupees: u16, // 810C
    _p2: [u8; 8],
    pub sword_id: u8, // 8116
    pub shield_id: u8, // 8117
    _p3: [u8; 3],
    pub max_magic: u8, // 811B
    pub magic: u8, // 811C
}

impl Link {
    pub fn get() -> &'static mut Link {
        reference(OFFSET)
    }

    pub fn position() -> &'static mut Coord {
        reference(POSITION_OFFSET)
    }

    pub fn room() -> u8 {
        read(0x803B9230)
    }

    pub fn horizontal_movement_direction() -> u16 {
        read(0x803EA3CA)
    }

    pub fn air_meter() -> u16 {
        read(0x803BDC62)
    }

    pub fn set_air_meter(frames: u16) {
        write(0x803BDC62, frames);
    }

    pub fn name() -> &'static str {
        read_str(ptr(0x803B8264))
    }

    pub fn activate_storage() {
        write(0x803BD3A3, true);
    }

    pub fn set_sword(&mut self, sword: Sword) {
        let quest_items = QuestItems::get();
        quest_items.sword = sword;
        self.sword_id = sword.item_id();
    }

    pub fn set_shield(&mut self, shield: Shield) {
        let quest_items = QuestItems::get();
        quest_items.shield = shield;
        self.shield_id = shield.item_id();
    }

    pub fn set_collision(collision: CollisionType) {
        let ptr = ptr::<u16>(read::<Addr>(0x803BDC40) + (0x24B << 1));
        match collision {
            CollisionType::Default => unsafe {
                *ptr &= 0xFFFF ^ 0x4004;
            },
            CollisionType::ChestStorage => unsafe {
                *ptr = (*ptr & (0xFFFF ^ 0x4000)) | 0x4;
            },
            CollisionType::DoorCancel => unsafe {
                *ptr |= 0x4004;
            },
        }
    }

    pub fn collision() -> CollisionType {
        // I read the address stored at 0x803BDC40 add 0x24B << 1 to it 
        // and that's the address of the collision flags
        let data = read::<u16>(read::<Addr>(0x803BDC40) + (0x24B << 1));
        let door_cancel_bit = data & 0x4000 != 0;
        let chest_storage_bit = data & 0x4 != 0;
        match (door_cancel_bit, chest_storage_bit) {
            (true, true) => CollisionType::DoorCancel,
            (_, true) => CollisionType::ChestStorage,
            _ => CollisionType::Default,
        }
    }
}

#[derive(Copy, Clone)]
pub enum CollisionType {
    Default,
    ChestStorage,
    DoorCancel,
}

impl Display for CollisionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &CollisionType::ChestStorage => "Chest Storage",
            &CollisionType::DoorCancel => "Door Cancel",
            &CollisionType::Default => "Default",
        };
        write!(f, "{}", s)
    }
}
