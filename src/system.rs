use libc::{c_int, c_void, size_t};
use Addr;
use core::mem::transmute;
use core::ptr::null_mut;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let _memalign = unsafe {
        transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88)
    };
    _memalign(0xFFFFFFFC, size)
}

#[no_mangle]
pub extern "C" fn posix_memalign(memptr: *mut *mut c_void,
                                 alignment: size_t,
                                 size: size_t)
                                 -> c_int {
    let _memalign = unsafe {
        transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88)
    };
    unsafe {
        *memptr = _memalign(alignment, size);
    }
    0
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    let _free = unsafe { transmute::<Addr, extern "C" fn(*mut c_void)>(0x8023eac0) };
    _free(ptr);
}

#[no_mangle]
pub extern "C" fn memset(ptr: *mut c_void, value: c_int, num: size_t) -> *mut c_void {
    let _memset = unsafe {
        transmute::<Addr, extern "C" fn(*mut c_void, c_int, size_t) -> *mut c_void>(0x80250054)
    };
    _memset(ptr, value, num)
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let new_data = malloc(size);

    if ptr != null_mut() {
        let mut dst = new_data as *mut u8;
        let mut src = ptr as *mut u8;

        for _ in 0..size {
            unsafe {
                *dst = *src;
                dst = dst.offset(1);
                src = src.offset(1);
            }
        }

        free(ptr); // TODO Test
    }

    new_data
}

#[no_mangle]
pub extern "C" fn memmove(destination: *mut c_void,
                          source: *const c_void,
                          num: size_t)
                          -> *mut c_void {
    let _memmove = unsafe {
        transmute::<Addr,
                    extern "C" fn(*mut c_void,
                                  *const c_void,
                                  size_t)
                                  -> *mut c_void>(0x80328f4c)
    };
    _memmove(destination, source, num) // TODO Test
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume() {
    // FIXME
}
