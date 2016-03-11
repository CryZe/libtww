use std::convert::From;
use game::flag::Flag;

#[derive(Copy, Clone)]
pub struct TriforcePiece(pub u8);

impl From<TriforcePiece> for Flag {
    fn from(t: TriforcePiece) -> Flag {
        Flag(0x803B81C6, t.0)
    }
}

pub const TRIFORCE_PIECE_1: TriforcePiece = TriforcePiece(1 << 0);
pub const TRIFORCE_PIECE_2: TriforcePiece = TriforcePiece(1 << 1);
pub const TRIFORCE_PIECE_3: TriforcePiece = TriforcePiece(1 << 2);
pub const TRIFORCE_PIECE_4: TriforcePiece = TriforcePiece(1 << 3);
pub const TRIFORCE_PIECE_5: TriforcePiece = TriforcePiece(1 << 4);
pub const TRIFORCE_PIECE_6: TriforcePiece = TriforcePiece(1 << 5);
pub const TRIFORCE_PIECE_7: TriforcePiece = TriforcePiece(1 << 6);
pub const TRIFORCE_PIECE_8: TriforcePiece = TriforcePiece(1 << 7);

impl TriforcePiece {
    pub fn unlock(self) {
        Flag::activate(self.into())
    }

    pub fn lock(self) {
        Flag::deactivate(self.into())
    }

    pub fn is_unlocked(self) -> bool {
        Flag::is_active(self.into())
    }
}
