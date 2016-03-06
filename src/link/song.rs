use std::convert::From;
use game::flag::Flag;

#[derive(Copy, Clone)]
pub struct Song(pub u8);

impl From<Song> for Flag {
    fn from(s: Song) -> Flag {
        Flag(0x803B81C5, s.0)
    }
}

pub const WINDS_REQUIEM: Song = Song(1 << 0);
pub const BALLAD_OF_GALES: Song = Song(1 << 1);
pub const COMMAND_MELODY: Song = Song(1 << 2);
pub const EARH_GODS_LYRIC: Song = Song(1 << 3);
pub const WIND_GODS_ARIA: Song = Song(1 << 4);
pub const SONG_OF_PASSING: Song = Song(1 << 5);

impl Song {
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
