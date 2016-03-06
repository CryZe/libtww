use {Addr, Coord};
use system::memory::{ptr, read, write, read_str};

pub mod inventory;
pub mod item;
pub mod song;
pub mod triforce;
pub mod pearl;

pub const OFFSET: Addr = 0x803B8108;
pub const POSITION_OFFSET: Addr = 0x803d78fc;

#[repr(C)]
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
        unsafe { &mut *ptr(OFFSET) }
    }

    pub fn position() -> &'static Coord {
        unsafe { &mut *ptr(POSITION_OFFSET) }
    }

    pub fn name() -> &'static str {
        read_str(ptr(0x803B8264))
    }

    pub fn activate_storage() {
        write(0x803BD3A3, true);
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
}

#[derive(Copy, Clone)]
pub enum CollisionType {
    Default,
    ChestStorage,
    DoorCancel,
}
