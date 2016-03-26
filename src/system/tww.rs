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

pub fn dmeter_rupy_init(addr: Addr) {
    let dmeter_rupy_init = unsafe { transmute::<Addr, extern "C" fn(Addr)>(0x801F7868) };
    dmeter_rupy_init(addr);
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

pub fn get_layer_by_id(id: i32) -> Addr {
    let fpcly_layer = unsafe { transmute::<Addr, extern "C" fn(i32) -> Addr>(0x8003b92c) };
    fpcly_layer(id)
}

pub fn set_current_layer(addr: Addr) {
    let fpcly_set_current_layer = unsafe { transmute::<Addr, extern "C" fn(Addr)>(0x8003b8cc) };
    fpcly_set_current_layer(addr)
}

pub fn get_current_layer() -> Addr {
    let fpcly_current_layer = unsafe { transmute::<Addr, extern "C" fn() -> Addr>(0x8003b8d4) };
    fpcly_current_layer()
}

pub fn get_root_layer() -> Addr {
    read(0x80365B7C)
}

use game::actor::{ActorTemplate, ActorMemory};

pub fn dstage_actor_create(template: *const ActorTemplate, memory: *mut ActorMemory) {
    let dstage_actor_create = unsafe {
        transmute::<Addr, extern "C" fn(*const ActorTemplate, *mut ActorMemory)>(0x8003f484)
    };
    dstage_actor_create(template, memory);
}

pub fn fopacm_create_append() -> &'static mut ActorMemory {
    let fopacm_create_append = unsafe {
        transmute::<Addr, extern "C" fn() -> *mut ActorMemory>(0x80023f3c)
    };
    let actor_memory = fopacm_create_append();
    unsafe { &mut *actor_memory }
}
