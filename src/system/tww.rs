use Addr;
use std::mem::transmute;
use system::memory::{read, write};

pub fn random() -> f64 {
    let random = unsafe { transmute::<Addr, extern "C" fn() -> f64>(0x80243b40) };
    random()
}

pub fn cdyl_init_async() {
    let cdyl_init_async = unsafe { transmute::<Addr, extern "C" fn()>(0x80022A88) };
    cdyl_init_async();
}

pub fn get_frame_count() -> u32 {
    read(0x80396218)
}

pub fn is_pause_menu_up() -> bool {
    read(0x803EA537) // alternative: 0x80396228
}

pub fn set_wind(direction: u8) {
    write(0x803D894A, direction << 5);
}