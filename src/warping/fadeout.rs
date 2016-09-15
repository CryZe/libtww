#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FadeOut {
    NormalBlack = 0,
    NormalWhite = 1,
    Wobble = 2,
    CutToBlackCrash = 3, // Crash?
    BlurAway = 4,
    CutToBlack = 5,
    CutToBlack2 = 6,
    CutToWhite = 7,
    NormalBlack2 = 8,
    NormalWhite2 = 9,
    CutToBlack3 = 10,
    CutToWhite3 = 11,
    Last = 255,
}
