use system::memory::{reference, write, write_str};
use warping::FadeOut;
use warping::Entrance;

pub const NO_LAYER_OVERRIDE: i8 = -1;

#[repr(C, packed)]
#[derive(Clone)]
pub struct Warp {
    pub entrance: Entrance,
    pub layer_override: i8,
    pub enabled: bool,
    pub fadeout: FadeOut,
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
            entrance: Entrance {
                stage: [0; 8],
                entrance: entrance,
                room: room,
            },
            layer_override: layer_override,
            enabled: enabled,
            fadeout: fadeout,
        };
        write_str(warp.entrance.stage.as_mut_ptr(), stage);
        warp
    }

    pub fn execute(self) {
        write(0x803BD248, self);
    }

    pub fn last_exit() -> &'static Warp {
        reference(0x803BD248)
    }
}
