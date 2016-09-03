use system::memory::reference;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Sword {
    None = 0,
    HerosSword = 1,
    UnchargedMasterSword = 2,
    HalfChargedMasterSword = 4,
    FullyChargedMasterSword = 8,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Shield {
    None = 0,
    HerosShield = 1,
    MirrorShield = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HerosCharm {
    None = 0,
    Disabled = 1,
    Enabled = 2,
}

#[repr(C, packed)]
pub struct QuestItems {
    pub sword: Sword,
    pub shield: Shield,
    pub has_power_bracelets: bool,
    pub has_pirates_charm: bool,
    pub heros_charm: HerosCharm,
}

impl QuestItems {
    pub fn get() -> &'static mut QuestItems {
        reference(0x803B81BC)
    }
}

impl Sword {
    pub fn item_id(self) -> u8 {
        use self::Sword::*;
        use super::item::*;

        match self {
            None => EMPTY,
            HerosSword => HEROS_SWORD,
            UnchargedMasterSword => UNCHARGED_MASTER_SWORD,
            HalfChargedMasterSword => HALF_CHARGED_MASTER_SWORD,
            FullyChargedMasterSword => FULLY_CHARGED_MASTER_SWORD,
        }
    }
}

impl Shield {
    pub fn item_id(self) -> u8 {
        use self::Shield::*;
        use super::item::*;

        match self {
            None => EMPTY,
            HerosShield => HEROS_SHIELD,
            MirrorShield => MIRROR_SHIELD,
        }
    }
}
