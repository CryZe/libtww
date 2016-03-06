use system::memory::{reference, read_str};

#[repr(C)]
#[derive(Clone)]
pub struct Entrance {
    pub stage: [u8; 8],
    pub entrance: u16,
    pub room: u8,
}

impl Entrance {
    pub fn last_entrance() -> &'static mut Entrance {
        reference(0x803BD23C)
    }

    pub fn stage_name(&self) -> &'static str {
        read_str(self.stage.as_ptr())
    }
}
