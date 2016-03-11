use system::memory::{read, write};

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

pub fn buttons_down() -> u16 {
    read(0x803E0D2A)
}

pub fn buttons_pressed() -> u16 {
    read(0x803E0D2E)
}

pub fn set_buttons_down(buttons: u16) {
    write(0x803E0D2A, buttons)
}

pub fn set_buttons_pressed(buttons: u16) {
    write(0x803E0D2E, buttons)
}

pub fn is_down(buttons: u16) -> bool {
    buttons_down() & buttons == buttons
}

pub fn is_pressed(buttons: u16) -> bool {
    buttons_pressed() & buttons == buttons
}
