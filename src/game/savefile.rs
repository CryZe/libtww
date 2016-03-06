use system::memory::{read, reference};
use warping::Entrance;

pub fn is_new_game_plus() -> bool {
    read(0x803B82A8)
}

pub fn get_picture_count() -> u8 {
    read(0x803B8170)
}

pub fn get_triforce_set() -> u8 {
    read(0x803B82A9)
}

pub fn get_entrance() -> &'static mut Entrance {
    reference(0x803B8138)
}
