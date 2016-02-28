use core::mem;
use Addr;

pub fn ptr<T>(addr: Addr) -> *mut T {
    unsafe { mem::transmute(addr) }
}

pub fn read<T: Copy>(addr: Addr) -> T {
    unsafe { *ptr(addr) }
}

pub fn write<T>(addr: Addr, value: T) {
    unsafe {
        *ptr(addr) = value;
    }
}

pub fn write_str(ptr: *mut u8, value: &str) {
    let mut dst = ptr;
    for &byte in value.as_bytes().iter() {
        unsafe {
            *dst = byte;
            dst = dst.offset(1);
        }
        if byte == 0 {
            return;
        }
    }
    unsafe {
        *dst = 0;
    }
}
