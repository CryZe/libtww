use Addr;
use core::mem::transmute;

pub fn random() -> f64 {
    let _random = unsafe { transmute::<Addr, extern "C" fn() -> f64>(0x80243b40) };
    _random()
}

pub fn cdyl_init_async() {
    let _cdyl_init_async = unsafe { transmute::<Addr, extern "C" fn()>(0x80022A88) };
    _cdyl_init_async();
}