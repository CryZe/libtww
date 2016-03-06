use std::convert::From;
use game::flag::Flag;

#[derive(Copy, Clone)]
pub struct Pearl(pub u8);

impl From<Pearl> for Flag {
    fn from(t: Pearl) -> Flag {
        Flag(0x803B81C7, t.0)
    }
}

pub const NAYRUS_PEARL: Pearl = Pearl(1 << 0);
pub const DINS_PEARL: Pearl = Pearl(1 << 1);
pub const FARORES_PEARL: Pearl = Pearl(1 << 2);

impl Pearl {
    pub fn unlock(&self) {
        let flag: Flag = self.clone().into();
        flag.activate();
    }

    pub fn lock(&self) {
        let flag: Flag = self.clone().into();
        flag.deactivate();
    }

    pub fn is_unlocked(&self) -> bool {
        let flag: Flag = self.clone().into();
        flag.is_active()
    }
}
