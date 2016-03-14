use libc::{c_int, c_void, size_t, c_uchar};
use Addr;
use core::mem::transmute;
use core::ptr::null_mut;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
pub fn panic_fmt() -> ! {
    panic!();
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let memalign = unsafe {
        transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88)
    };
    memalign(0xFFFFFFFC, size)
}

#[no_mangle]
pub extern "C" fn posix_memalign(memptr: *mut *mut c_void,
                                 alignment: size_t,
                                 size: size_t)
                                 -> c_int {
    let memalign = unsafe {
        transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88)
    };
    unsafe {
        *memptr = memalign(alignment, size);
    }
    0
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    let free = unsafe { transmute::<Addr, extern "C" fn(*mut c_void)>(0x8023eac0) };
    free(ptr);
}

#[no_mangle]
pub extern "C" fn memset(ptr: *mut c_void, value: c_int, num: size_t) -> *mut c_void {
    let memset = unsafe {
        transmute::<Addr, extern "C" fn(*mut c_void, c_int, size_t) -> *mut c_void>(0x80250054)
    };
    memset(ptr, value, num)
}

#[no_mangle]
pub extern "C" fn memcpy(destination: *mut c_void,
                         source: *const c_void,
                         num: size_t)
                         -> *mut c_void {
    let memcpy = unsafe {
        transmute::<Addr,
                    extern "C" fn(*mut c_void,
                                  *const c_void,
                                  size_t)
                                  -> *mut c_void>(0x80250034)
    };
    memcpy(destination, source, num)
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
    let memmove = unsafe {
        transmute::<Addr,
                    extern "C" fn(*mut c_void,
                                  *const c_void,
                                  size_t)
                                  -> *mut c_void>(0x80328f4c)
    };
    memmove(destination, source, num) // TODO Test
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume() {
    // FIXME
}

#[no_mangle]
pub extern "C" fn strlen(string: *const u8) -> size_t {
    let mut counter = 0;
    let mut string = string;
    while unsafe { *string } != 0 {
        string = unsafe { string.offset(1) };
        counter += 1;
    }
    counter
}

#[no_mangle]
pub extern "C" fn memcmp(ptr1: *const c_void,
                         ptr2: *const c_void,
                         num: size_t) -> c_int {
    let mut p1 = ptr1 as *const c_uchar;
    let mut p2 = ptr2 as *const c_uchar;
    let mut n = num;
    while n > 0 {
        let u1 = unsafe {*p1};
        let u2 = unsafe {*p2};
        if u1 != u2 {
            return (u1 - u2) as c_int;
        }
        p1 = unsafe {p1.offset(1)};
        p2 = unsafe {p2.offset(1)};
        n -= 1;
    }
    return 0;
}
