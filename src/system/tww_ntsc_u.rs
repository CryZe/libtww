use Addr;
use std::mem::transmute;
use system::memory::{read, write};
use prelude::*;

pub fn random_u32() -> u32 {
    // TODO Wrong Signature, takes u32
    let random_u32 = unsafe { transmute::<Addr, extern "C" fn() -> u32>(0x802ABDE0) };
    random_u32()
}

pub fn random() -> f64 {
    // TODO Wrong Signature, takes f32
    let random = unsafe { transmute::<Addr, extern "C" fn() -> f64>(0x802463B0) };
    random()
}

pub fn cdyl_init_async() {
    let cdyl_init_async = unsafe { transmute::<Addr, extern "C" fn()>(0x80022DF8) };
    cdyl_init_async();
}

pub fn dmeter_rupy_init(addr: Addr) {
    let dmeter_rupy_init = unsafe { transmute::<Addr, extern "C" fn(Addr)>(0x801FC190) };
    dmeter_rupy_init(addr);
}

pub fn get_frame_count() -> u32 {
    read(0x80402cf8)
}

pub fn is_pause_menu_up() -> bool {
    read(0x80402D08) // alternative: 0x80402D08
}

/*
pub fn set_wind(direction: u8) {
    write(0x803D894A, direction << 5);
}
*/

pub fn get_layer_by_id(id: u32) -> Addr {
    let fpcly_layer = unsafe { transmute::<Addr, extern "C" fn(u32) -> Addr>(0x8003d9c4) };
    fpcly_layer(id)
}

pub fn set_current_layer(addr: Addr) {
    let fpcly_set_current_layer =
        unsafe { transmute::<Addr, extern "C" fn(Addr)>(0x8003d964) };
    fpcly_set_current_layer(addr)
}

pub fn get_current_layer() -> Addr {
    let fpcly_current_layer =
        unsafe { transmute::<Addr, extern "C" fn() -> Addr>(0x8003d96c) };
    fpcly_current_layer()
}
/*
pub fn get_root_layer() -> Addr {
    read(0x80365B7C)
}
*/
use game::actor::{ActorMemory, ActorTemplate};

pub fn dstage_actor_create(template: *const ActorTemplate, memory: *mut ActorMemory) {
    let dstage_actor_create = unsafe {
        transmute::<Addr, extern "C" fn(*const ActorTemplate, *mut ActorMemory)>(0x80041628)
    };
    dstage_actor_create(template, memory);
}

pub fn fopacm_create_append() -> &'static mut ActorMemory {
    let fopacm_create_append =
        unsafe { transmute::<Addr, extern "C" fn() -> *mut ActorMemory>(0x800242ac) };
    let actor_memory = fopacm_create_append();
    unsafe { &mut *actor_memory }
}


pub fn layer_loader(dzr: Addr, layer: Addr, room_id: u8) {
    let layer_loader =
        unsafe { transmute::<Addr, extern "C" fn(Addr, Addr, u8)>(0x800430e0) };
    layer_loader(dzr, layer, room_id);
}

pub fn ground_cross(a: Addr, b: Addr) -> f32 {
    let ground_cross =
        unsafe { transmute::<Addr, extern "C" fn(Addr, Addr) -> f32>(0x802468e4) };
    ground_cross(a, b)
}

pub fn fopmsgm_message_set(message_id: u16) {
    let fopmsgm_message_set = unsafe { transmute::<Addr, extern "C" fn(u16)>(0x8002b8a4) };
    fopmsgm_message_set(message_id)
}

#[allow(non_snake_case)]
pub fn dStage_dt_c_stageLoader(a: Addr, b: Addr) {
    let stage_loader = unsafe { transmute::<Addr, extern "C" fn(Addr, Addr)>(0x8004313c) };
    stage_loader(a, b)
}

#[allow(non_snake_case)]
pub fn dSv_player_get_item_c_onItem(dSv_player_get_item_c: Addr, slot_id: i32, unknown: u8) {
    let on_item = unsafe { transmute::<Addr, extern "C" fn(Addr, i32, u8)>(0x800594c4) };
    on_item(dSv_player_get_item_c, slot_id, unknown)
}

#[allow(non_snake_case)]
pub fn dSv_player_return_place_c_set(
    dSv_player_return_place_c: Addr,
    stage: *const u8,
    room: i8,
    start_code: u8,
) {
    let set =
        unsafe { transmute::<Addr, extern "C" fn(Addr, *const u8, i8, u8)>(0x80058bc8) };
    set(dSv_player_return_place_c, stage, room, start_code)
}

pub struct JKRDvdFile;

impl JKRDvdFile {
    pub fn constructor(this: *mut u8) {
        let constructor = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802bc6b4) };
        constructor(this)
    }

    pub fn destructor(this: *mut u8) {
        let destructor = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802bc878) };
        destructor(this)
    }

    pub fn open(this: *mut u8, path: *const u8) {
        let open =
            unsafe { transmute::<Addr, extern "C" fn(*mut u8, *const u8)>(0x802bc980) };
        open(this, path)
    }

    pub fn read(this: *mut u8, buffer: *mut u8, len: i32, unknown: i32) {
        let read = unsafe {
            transmute::<Addr, extern "C" fn(*mut u8, *mut u8, i32, i32)>(0x802bcae0)
        };
        read(this, buffer, len, unknown)
    }

    pub fn close(this: *mut u8) {
        let close = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x802bca68) };
        close(this)
    }

    pub fn get_file_size(this: *mut u8) -> i32 {
        let get_file_size =
            unsafe { transmute::<Addr, extern "C" fn(*mut u8) -> i32>(0x802bccac) };
        get_file_size(this)
    }
}

pub struct OS;

impl OS {
    //TODO okay?
    pub fn allocate_thread() -> Box<[u8]> {
        vec![0xCE; 792 + 32].into_boxed_slice()
    }

    pub fn allocate_mutex() -> Box<[u8]> {
        vec![0xCE; 64 + 32].into_boxed_slice()
    }

    pub fn allocate_cond() -> Box<[u8]> {
        vec![0xCE; 32].into_boxed_slice()
    }

    pub fn get_current_thread() -> *const u8 {
        let get_current_thread =
            unsafe { transmute::<Addr, extern "C" fn() -> *const u8>(0x80307a34) };
        get_current_thread()
    }

    pub fn is_thread_terminated(this: *const u8) -> bool {
        let is_thread_terminated =
            unsafe { transmute::<Addr, extern "C" fn(*const u8) -> bool>(0x80307a40) };
        is_thread_terminated(this)
    }

    pub fn create_thread(
        this: *mut u8,
        entry: extern "C" fn(*mut u8) -> *mut u8,
        arg: *mut u8,
        stack: *mut u8,
        stack_size: usize,
        priority: i32,
        attr: i16,
    ) -> bool {
        let create_thread = unsafe {
            transmute::<
                Addr,
                extern "C" fn(
                    *mut u8,
                    extern "C" fn(*mut u8) -> *mut u8,
                    *mut u8,
                    *mut u8,
                    usize,
                    i32,
                    i16,
                ) -> bool,
            >(0x8030803c)
        };
        create_thread(this, entry, arg, stack, stack_size, priority, attr)
    }

    pub fn resume_thread(this: *const u8) -> i32 {
        let resume_thread =
            unsafe { transmute::<Addr, extern "C" fn(*const u8) -> i32>(0x803086a4) };
        resume_thread(this)
    }

    pub fn suspend_thread(this: *const u8) -> i32 {
        let suspend_thread =
            unsafe { transmute::<Addr, extern "C" fn(*const u8) -> i32>(0x8030892c) };
        suspend_thread(this)
    }

    pub fn join_thread(this: *const u8, ret_value: *mut *mut u8) -> bool {
        let join_thread = unsafe {
            transmute::<Addr, extern "C" fn(*const u8, *mut *mut u8) -> bool>(0x803084c4)
        };
        join_thread(this, ret_value)
    }

    pub fn yield_thread() {
        let yield_thread = unsafe { transmute::<Addr, extern "C" fn()>(0x80308000) };
        yield_thread()
    }

    pub fn init_mutex(this: *mut u8) {
        let init_mutex = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x80305e5c) };
        init_mutex(this)
    }

    pub fn lock_mutex(this: *const u8) {
        let lock_mutex = unsafe { transmute::<Addr, extern "C" fn(*const u8)>(0x80305e94) };
        lock_mutex(this)
    }

    pub fn unlock_mutex(this: *const u8) {
        let unlock_mutex =
            unsafe { transmute::<Addr, extern "C" fn(*const u8)>(0x80305f70) };
        unlock_mutex(this)
    }

    pub fn try_lock_mutex(this: *const u8) -> bool {
        let try_lock_mutex =
            unsafe { transmute::<Addr, extern "C" fn(*const u8) -> bool>(0x803060a8) };
        try_lock_mutex(this)
    }

    pub fn init_cond(this: *mut u8) {
        let init_cond = unsafe { transmute::<Addr, extern "C" fn(*mut u8)>(0x80306164) };
        init_cond(this)
    }

    pub fn wait_cond(this: *const u8, mutex: *const u8) {
        let wait_cond =
            unsafe { transmute::<Addr, extern "C" fn(*const u8, *const u8)>(0x80306184) };
        wait_cond(this, mutex)
    }

    pub fn signal_cond(this: *const u8) {
        let signal_cond = unsafe { transmute::<Addr, extern "C" fn(*const u8)>(0x80306258) };
        signal_cond(this)
    }

    pub fn get_time() -> i64 {
        let get_time = unsafe { transmute::<Addr, extern "C" fn() -> i64>(0x803095ec) };
        get_time()
    }

    pub fn report(text: *const u8) {
        let report = unsafe { transmute::<Addr, extern "C" fn(*const u8)>(0x80006950) };
        report(text)
    }

    pub fn panic(file: *const u8, line: i32, message: *const u8) {
        let panic = unsafe {
            transmute::<Addr, extern "C" fn(*const u8, i32, *const u8)>(0x80006c4c)
        };
        panic(file, line, message)
    }
}

pub struct JUTAssertion;

impl JUTAssertion {
    pub fn get_s_device() -> u32 {
        let get_s_device = unsafe { transmute::<Addr, extern "C" fn() -> u32>(0x802c7690) };
        get_s_device()
    }

    pub fn show_assert(s_device: u32, file: *const u8, line: i32, assertion: *const u8) {
        let show_assert = unsafe {
            transmute::<Addr, extern "C" fn(u32, *const u8, i32, *const u8)>(0x802c7788)
        };
        show_assert(s_device, file, line, assertion);
    }

    pub fn set_visible(visibility: bool) {
        let set_visible = unsafe { transmute::<Addr, extern "C" fn(bool)>(0x802c7c14) };
        set_visible(visibility)
    }
}
