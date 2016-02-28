use memory::{write, write_str};

pub const NO_LAYER_OVERRIDE: i8 = -1;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FadeOut {
    Default = 0,
}

#[repr(C)]
#[derive(Clone)]
pub struct Warp {
    stage: [u8; 8],
    entrance: u16,
    room: u8,
    layer_override: i8,
    enabled: bool,
    fadeout: FadeOut,
}

impl Warp {
    pub fn new(stage: &str,
               entrance: u16,
               room: u8,
               layer_override: i8,
               fadeout: FadeOut,
               enabled: bool)
               -> Self {
        let mut warp = Warp {
            stage: [0; 8],
            entrance: entrance,
            room: room,
            layer_override: layer_override,
            enabled: enabled,
            fadeout: fadeout,
        };
        write_str(warp.stage.as_mut_ptr(), stage);
        warp
    }

    pub fn execute(self) {
        write(0x803BD248, self);
    }
}
