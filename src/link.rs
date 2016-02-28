use Addr;
use memory::ptr;

pub const OFFSET: Addr = 0x803B8108;

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
