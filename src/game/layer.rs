use {system, Addr};

pub fn get_layer_by_id(id: u32) -> Addr {
    system::get_layer_by_id(id)
}

pub fn set(addr: Addr) {
    system::set_current_layer(addr)
}

pub fn get() -> Addr {
    system::get_current_layer()
}

pub fn root_layer() -> Addr {
    system::get_root_layer()
}

pub fn safe_layer() -> Addr {
    system::memory::read(root_layer() + 8)
}

pub fn switch_to_safe_layer() {
    set(safe_layer());
}
