use {Addr, Coord};
use system::memory::{ptr, read, write, read_str};

pub const OFFSET: Addr = 0x803B8108;
pub const POSITION_OFFSET: Addr = 0x803d78fc;

#[repr(C)]
pub struct Link {
    _p0: [u8; 1],
    pub heart_pieces: u8, // 8109
    //_p1: [u8; 1],
    pub heart_quarters: u8, // 810B
    pub rupees: u16, // 810C
    _p2: [u8; 0x10 - 3],
    pub max_magic: u8, // 811B
    pub magic: u8, // 811C
}

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

#[derive(Copy, Clone)]
pub enum CollisionType {
    Default,
    ChestStorage,
    DoorCancel,
}

pub fn set_collision(collision: CollisionType) {
    let ptr = ptr::<u16>(read::<Addr>(0x803BDC40) + (0x24B << 1));
    match collision {
        CollisionType::Default => {
            unsafe { *ptr &= 0xFFFF ^ 0x4004; }
        }
        CollisionType::ChestStorage => {
            unsafe { *ptr = (*ptr & (0xFFFF ^ 0x4000)) | 0x4; }
        }
        CollisionType::DoorCancel => {
            unsafe { *ptr |= 0x4004; }
        }
    }
}
