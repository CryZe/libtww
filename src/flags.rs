pub use Addr;

#[derive(Copy, Clone)]
pub struct Flag(Addr, u8);

pub const ANIMATION_SET_2: Flag = Flag(0x803B8759, 0x01);
pub const MEDLI_ON_BOAT: Flag = Flag(0x803B8742, 0x28); // FIXME 2 Flags
pub const MEDLI_IN_EARTH_TEMPLE: Flag = Flag(0x803B8755, 0x20);
pub const MEDLI_IN_EARTH_TEMPLE_ENTRANCE: Flag = Flag(0x803B875A, 0x04);

pub fn activate(flag: Flag) {
    let Flag(addr, value) = flag;
    let ptr = ::memory::ptr::<u8>(addr);
    unsafe {
        *ptr |= value;
    }
}

pub fn deactivate(flag: Flag) {
    let Flag(addr, value) = flag;
    let ptr = ::memory::ptr::<u8>(addr);
    unsafe {
        *ptr &= 0xFF ^ value;
    }
}
