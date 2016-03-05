use system::memory::{ptr, write, write_str};
use Addr;

pub const ADDR_X_COORD: Addr = 0x80491A60;
pub const ADDR_Y_COORD: Addr = 0x80491A64;
pub const ADDR_BACKGROUND_COLOR: Addr = 0x80491A80;
pub const ADDR_VISIBILITY: Addr = 0x80491A84;

pub const ADDR_LINE1: Addr = 0x80491A89;
pub const ADDR_LINE2_VISIBILITY: Addr = 0x80491AC6;

pub fn activate() {
    set_x(32);
    set_y(112);
    write::<u32>(ADDR_Y_COORD, 112);
    write::<u32>(ADDR_BACKGROUND_COLOR, 0x00000000);
    write(ADDR_VISIBILITY, true);
    write::<u8>(ADDR_LINE2_VISIBILITY, 0x00);
    write_line1("");
}

pub fn x() -> &'static mut u32 {
    unsafe { &mut *ptr::<u32>(ADDR_X_COORD) }
}

pub fn y() -> &'static mut u32 {
    unsafe { &mut *ptr::<u32>(ADDR_Y_COORD) }
}

pub fn set_x(x: u32) {
    write(ADDR_X_COORD, x);
}

pub fn set_y(y: u32) {
    write(ADDR_Y_COORD, y);
}

pub fn write_line1<T: AsRef<str>>(text: T) {
    write_str(ptr(ADDR_LINE1), text);
}
