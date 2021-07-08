use system::memory;

#[cfg(feature = "ntsc_j")]
pub fn event_cancel() -> bool {
    memory::read(0x803BD3A3)
}

#[cfg(feature = "ntsc_u")]
pub fn event_cancel() -> bool {
    memory::read(0x803C9EA3)
}

#[cfg(feature = "ntsc_j")]
pub fn set_event_cancel(b: bool) {
    memory::write(0x803BD3A3, b);
}

#[cfg(feature = "ntsc_u")]
pub fn set_event_cancel(b: bool) {
    memory::write(0x803C9EA3, b);
}
