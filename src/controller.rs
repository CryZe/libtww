use memory::read as mem_read;

pub const DPAD_LEFT: u16 = 0x0001;
pub const DPAD_RIGHT: u16 = 0x0002;
pub const DPAD_DOWN: u16 = 0x0004;
pub const DPAD_UP: u16 = 0x0008;
pub const Z: u16 = 0x0010;
pub const R: u16 = 0x0020;
pub const L: u16 = 0x0040;
pub const A: u16 = 0x0100;
pub const B: u16 = 0x0200;
pub const X: u16 = 0x0400;
pub const Y: u16 = 0x0800;
pub const START: u16 = 0x1000;

pub fn read() -> u16 {
    mem_read(0x803E0D2A)
}

pub fn is_pressed(buttons: u16) -> bool {
    read() == buttons
}
