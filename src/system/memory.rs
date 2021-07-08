use std::{mem, slice, str};
use Addr;
// use system::libc;
use libc;

pub fn ptr<T>(addr: Addr) -> *mut T {
    unsafe { mem::transmute(addr) }
}

pub fn reference<T>(addr: Addr) -> &'static mut T {
    unsafe { &mut *ptr(addr) }
}

pub fn read<T: Copy>(addr: Addr) -> T {
    unsafe { *ptr(addr) }
}

pub fn write<T>(addr: Addr, value: T) {
    unsafe {
        *ptr(addr) = value;
    }
}

pub fn write_str<T: AsRef<str>>(ptr: *mut u8, value: T) {
    let mut dst = ptr;
    for &byte in value.as_ref().as_bytes().iter() {
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

pub fn read_str(ptr: *const u8) -> &'static str {
    unsafe {
        let slice = slice::from_raw_parts(ptr, libc::strlen(ptr));
        str::from_utf8(slice).unwrap_or("")
    }
}
