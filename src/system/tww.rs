use Addr;
use std::mem::transmute;
use system::memory::read;

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