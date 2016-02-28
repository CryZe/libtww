#[repr(C)]
pub struct Link {
    pub heart_pieces: u8, // 8109
    _p1: [u8; 1],
    pub heart_quarters: u8, // 810B
    pub rupees: [u8; 2], // 810C
    _p2: [u8; 0x10 - 3],
    pub max_magic: u8, // 811B
    pub magic: u8, // 811C
}
