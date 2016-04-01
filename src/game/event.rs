use system::memory;

pub fn event_cancel() -> bool {
    memory::read(0x803BD3A3)
}

pub fn set_event_cancel(b: bool) {
    memory::write(0x803BD3A3, b);
}
