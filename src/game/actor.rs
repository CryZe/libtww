use {Coord, system};
use system::memory;
use link::Link;
use game::layer;

pub const DEFAULT_ENEMY_ID: i16 = -1;
pub const DEFAULT_FLAG: u16 = 0;
pub const DEFAULT_PARAMS: u32 = 0;

pub mod breakable {
    pub const SIGN: &'static str = "Kanban";
    pub const BREAKABLE_CUP: &'static str = "MKoppu";
    pub const BREAKABLE_PLATE: &'static str = "MOsara";
    pub const BREAKABLE_JUG: &'static str = "MPot";
    pub const SKULL: &'static str = "Odokuro";
    pub const NUT: &'static str = "VigaH";
    pub const PILE_OF_LEAVES: &'static str = "Vochi";
    pub const SMALL_POT: &'static str = "kotubo";
    pub const LARGE_POT: &'static str = "ootubo1";
}

pub mod door {
    pub const KNOB00D: &'static str = "KNOB00D";
    pub const KNOB01D: &'static str = "KNOB01D";
}

pub mod dungeon_boss {
    pub const KALLE_DEMOS: &'static str = "Bkm";
    pub const GOHDAN: &'static str = "Bst";
    pub const GOHMA: &'static str = "Btd";
    pub const MOLGERA: &'static str = "Bwd";
    pub const GANONDORF: &'static str = "Gnd";
    pub const JALHALLA: &'static str = "big_pow";
}

pub mod enemy_npc {
    pub const KARGAROC: &'static str = "Bb";
    pub const BOKOBLIN: &'static str = "Bk";
    pub const QUILL: &'static str = "Bm1";
    pub const CANON: &'static str = "Canon";
    pub const BIG_OCTO: &'static str = "Daiocta";
    pub const PHANTOM_GANON: &'static str = "Fganon";
    pub const FIRE_KEESE: &'static str = "Fkeeth";
    pub const FLOOR_MASTER_2: &'static str = "Fmastr2";
    pub const GYORG: &'static str = "GyCtrl";
    pub const REDEAD: &'static str = "Rdead1";
    pub const DEXIVINE: &'static str = "Sss";
    pub const STALFOS: &'static str = "Stal";
    pub const DARKNUT: &'static str = "Tn";
    pub const BLADE_TRAP: &'static str = "Trap";
    pub const ARMOS: &'static str = "amos";
    pub const ARMOS_2: &'static str = "amos2";
    pub const BUBBLE: &'static str = "bable";
    pub const BOKO_BABA: &'static str = "bbaba";
    pub const BLACK_CHUCHU: &'static str = "c_black";
    pub const BLUE_CHUCHU: &'static str = "c_blue";
    pub const GREEN_CHUCHU: &'static str = "c_green";
    pub const YELLOW_CHUCHU: &'static str = "c_kiiro";
    pub const RED_CHUCHU: &'static str = "c_red";
    pub const KEESE: &'static str = "keeth";
    pub const MAGTAIL: &'static str = "magtail";
    pub const MOBLIN: &'static str = "mo2";
    pub const MOBLIN_STATUE: &'static str = "moZOU";
    pub const MOUSE: &'static str = "nezumi";
    pub const PEAHAT: &'static str = "p_hat";
    pub const POE: &'static str = "pow";
    pub const REDEAD_1: &'static str = "rdead1";
    pub const REGULAR_WIZZROBE: &'static str = "wiz_r";
}

pub mod exit {
    pub const DOOR_0: &'static str = "KNOB00";
    pub const DOOR_1: &'static str = "KNOB01";
    pub const GROTTO_ENTRANCE: &'static str = "Pitfall";
}

pub mod foliage {
    pub const PALM_TREE: &'static str = "Oyashi";
    pub const FLOWER: &'static str = "flower";
    pub const FLWR17: &'static str = "flwr17";
    pub const FLWR7: &'static str = "flwr7";
    pub const SMALL_ROCK_1: &'static str = "koisi1";
    pub const KUSAX1: &'static str = "kusax1";
    pub const KUSAX21: &'static str = "kusax21";
    pub const KUSAX7: &'static str = "kusax7";
    pub const LARGE_TREE: &'static str = "lwood";
    pub const PFLWRX7: &'static str = "pflwrx7";
    pub const SMALL_TREE_3: &'static str = "swood3";
    pub const SMALL_TREE_5: &'static str = "swood5";
}

pub mod friendly_npc {
    pub const STURGEON: &'static str = "Aj1";
    pub const GRANDMA: &'static str = "Ba1";
    pub const GREAT_FAIRY: &'static str = "BigElf";
    pub const RITO_POSTMAN_2: &'static str = "Bm2";
    pub const RITO_POSTMAN_4: &'static str = "Bm4";
    pub const RITO_POSTMAN_5: &'static str = "Bm5";
    pub const MAKAR: &'static str = "Cb1";
    pub const SEAGULL: &'static str = "Kamome";
    pub const ARYLL: &'static str = "Ls1";
    pub const MEDLI: &'static str = "Md1";
    pub const PIG: &'static str = "Pig";
    pub const TETRA: &'static str = "Zl1";
    pub const CRAB: &'static str = "kani";
}

pub mod gameplay {
    pub const ATTENTION_GRABBER: &'static str = "AttTag";
    pub const BOMB_FLOWER: &'static str = "BFlower";
    pub const HEART_CONTAINER_DUNGEON_BOSS_ITEM_DROP: &'static str = "Bitem";
    pub const VALOOS_TAIL: &'static str = "Dr2";
    pub const HOOKSHOT_TARGET: &'static str = "Hfuck1";
    pub const BREAKABLE_FLOOR_TILE: &'static str = "Hhyu1";
    pub const SPRING_ON_A_BLOCK_2: &'static str = "Hjump2";
    pub const WIND_COLUMN_GENERATOR: &'static str = "Hsen1";
    pub const GRAPPLE_POINT: &'static str = "Kui";
    pub const SOLIDIFIED_MAGMA_PLATFORM: &'static str = "Magrock";
    pub const WOODEN_BOX_WITH_BLACK_FRAME: &'static str = "Ospbox";
    pub const DANGLING_ROPE_WITH_LANTERN: &'static str = "RopeR";
    pub const POSTBOX: &'static str = "Tpost";
    pub const WARP_JAR_2: &'static str = "Warpts2";
    pub const JET_OF_MAGMA: &'static str = "Yfire00";
    pub const RING_OF_FIRE: &'static str = "Zenfire";
    pub const BRIDGE: &'static str = "bridge";
    pub const COLLECTIBLE_ITEM: &'static str = "item";
    pub const BABA_BUD: &'static str = "jbaba";
    pub const PUSHABLE_BLOCK_0: &'static str = "osiBLK0";
    pub const PUSHABLE_BLOCK_1: &'static str = "osiBLK1";
}

pub mod lod_model {
    pub const FORSAKEN_FORTRESS: &'static str = "LOD01";
    pub const STAR_ISLAND: &'static str = "LOD02";
    pub const NORTHERN_FAIRY_ISLE: &'static str = "LOD03";
    pub const GALE_ISLAND: &'static str = "LOD04";
    pub const CRESCENT_MOON_ISLE: &'static str = "LOD05";
    pub const SEVEN_STAR_ISLES: &'static str = "LOD06";
    pub const OVERLOOK_ISLAND: &'static str = "LOD07";
    pub const FOUR_EYE_REEF: &'static str = "LOD08";
    pub const MOTHER_AND_CHILD_ISLES: &'static str = "LOD09";
    pub const SPECTACLE_ISLAND: &'static str = "LOD10";
    pub const WINDFALL_ISLAND: &'static str = "LOD11";
    pub const PAWPRINT_ISLE: &'static str = "LOD12";
    pub const DRAGON_ROOST_ISLAND: &'static str = "LOD13";
    pub const FLIGHT_CONTROL_PLATFORM: &'static str = "LOD14";
    pub const WESTERN_FAIRY_ISLE: &'static str = "LOD15";
    pub const ROCK_SPIRE_ISLE: &'static str = "LOD16";
    pub const TINGLE_ISLAND: &'static str = "LOD17";
    pub const NORTHERN_TRIANGLE_ISLAND: &'static str = "LOD18";
    pub const EASTERN_FAIRY_ISLE: &'static str = "LOD19";
    pub const FIRE_MOUNTAIN: &'static str = "LOD20";
    pub const STAR_BELT_ARCHIPELAGO: &'static str = "LOD21";
    pub const THREE_EYE_REEF: &'static str = "LOD22";
    pub const GREATFISH_ISLE: &'static str = "LOD23";
    pub const CYCLOPS_REEF: &'static str = "LOD24";
    pub const SIX_EYE_REEF: &'static str = "LOD25";
    pub const TOWER_OF_THE_GODS: &'static str = "LOD26";
    pub const EASTERN_TRIANGLE_ISLAND: &'static str = "LOD27";
    pub const THORNED_FAIRY_ISLE: &'static str = "LOD28";
    pub const NEEDLEPOINT_ISLAND: &'static str = "LOD29";
    pub const ISLET_OF_STEEL: &'static str = "LOD30";
    pub const STONE_WATCHER_ISLAND: &'static str = "LOD31";
    pub const SOUTHERN_TRIANGLE_ISLAND: &'static str = "LOD32";
    pub const PRIVATE_OASIS: &'static str = "LOD33";
    pub const BOMB_ISLAND: &'static str = "LOD34";
    pub const BIRDS_PEAK_ISLAND: &'static str = "LOD35";
    pub const DIAMOND_STEPPE_ISLAND: &'static str = "LOD36";
    pub const FIVE_EYE_REEF: &'static str = "LOD37";
    pub const SHARK_ISLAND: &'static str = "LOD38";
    pub const SOUTHERN_FAIRY_ISLE: &'static str = "LOD39";
    pub const ICE_RING_ISLE: &'static str = "LOD40";
    pub const FOREST_HAVEN: &'static str = "LOD41";
    pub const CLIFF_PLATEAU_ISLES: &'static str = "LOD42";
    pub const HORSESHOE_ISLAND: &'static str = "LOD43";
    pub const OUTSET_ISLAND: &'static str = "LOD44";
    pub const HEADSTONE_ISLAND: &'static str = "LOD45";
    pub const TWO_EYE_REEF: &'static str = "LOD46";
    pub const ANGULAR_ISLES: &'static str = "LOD47";
    pub const BOAT_RACE_ISLAND: &'static str = "LOD48";
    pub const FIVE_STAR_ISLES: &'static str = "LOD49";
}

pub mod large_object {
    pub const STALL_A: &'static str = "RotenA";
    pub const STALL_B: &'static str = "RotenB";
    pub const STALL_C: &'static str = "RotenC";
    pub const TOWER_OF_THE_GODS_EXTERIOR: &'static str = "X_tower";
    pub const LINK_STATUE_INSIDE_HYRULE_CASTLE: &'static str = "YLzou";
}

pub mod mechanics {
    pub const SEED_PLANTING_SPOT_FOR_MAKAR: &'static str = "VmcBS";
}

pub mod obstacle {
    pub const IRON_BARS: &'static str = "Ashut";
    pub const LARGE_ROCK: &'static str = "Ebrock";
    pub const SPIKE: &'static str = "Htoge1";
    pub const EYE_VINE_BLOCKER: &'static str = "Ss";
    pub const TINGLE: &'static str = "Tc";
}

pub mod storyline {
    pub const TRIANGLE_ISLAND_STATUE: &'static str = "Doguu";
    pub const ZEPHOS_AND_CYCLOS: &'static str = "Hr";
    pub const DIN_STATUE: &'static str = "MegamiD";
    pub const FARORE_STATUE: &'static str = "MegamiF";
    pub const NAYRU_STATUE: &'static str = "MegamiN";
    pub const GANONS_TOWER_4_BOSS_DOOR: &'static str = "VgnFD";
}

pub mod switch {
    pub const ALL_ENEMIES_KILLED_SWITCH: &'static str = "ALLdie";
    pub const SWITCH_BUFFER_0: &'static str = "AND_SW0";
    pub const SWITCH_BUFFER_2: &'static str = "AND_SW2";
    pub const WIND_SWITCH: &'static str = "Hpbot1";
    pub const FLOOR_SWITCH_A: &'static str = "Kbota_A";
    pub const PROXIMITY_SWITCH: &'static str = "SW_C00";
    pub const CRYSTAL_SWITCH: &'static str = "SW_HIT0";
    pub const WIND_WAKER_SONG_SWITCH_B: &'static str = "SWtactB";
    pub const TINGLE_C_SWITCH: &'static str = "agbCSW";
}

pub mod tg_door {
    pub const KNOB00D: &'static str = "KNOB00D";
    pub const KNOB01D: &'static str = "KNOB01D";
    pub const KNOB03D: &'static str = "KNOB03D";
    pub const ZENS12: &'static str = "ZenS12";
    pub const DUNGEON_BARRED_DOOR: &'static str = "Zenshut";
    pub const NORMAL_DUNGEON_DOOR: &'static str = "door10";
    pub const NORMAL_EARTH_AND_WIND_TEMPLE_DOOR: &'static str = "door12";
    pub const BOSS_DUNGEON_DOOR: &'static str = "door20";
    pub const FORBIDDEN_WOODS_BOSS_DOOR: &'static str = "doorKD";
    pub const BARRED_EARTH_AND_WIND_TEMPLE_DOOR: &'static str = "doorSH";
    pub const LOCKED_EARTH_AND_WIND_TEMPLE_DOOR: &'static str = "keyS12";
    pub const DUNGEON_LOCKED_DOOR: &'static str = "keyshut";
}

pub mod treasure_chest {
    pub const TREASURE_CHEST: &'static str = "takara";
    pub const TREASURE_CHEST_2: &'static str = "takara2";
    pub const TAKARA3: &'static str = "takara3";
    pub const TREASURE_CHEST_3: &'static str = "takara3";
    pub const TREASURE_CHEST_4: &'static str = "takara4";
    pub const TREASURE_CHEST_5: &'static str = "takara5";
    pub const TREASURE_CHEST_6: &'static str = "takara6";
    pub const TREASURE_CHEST_7: &'static str = "takara7";
    pub const TREASURE_CHEST_8: &'static str = "takara8";
    pub const TREASURE_I: &'static str = "takaraI";
    pub const TREASURE_K: &'static str = "takaraK";
    pub const TREASURE_M: &'static str = "takaraM";
    pub const TREASURE_AGC: &'static str = "tkrAGc";
    pub const TREASURE_AIK: &'static str = "tkrAIk";
    pub const TREASURE_AKD: &'static str = "tkrAKd";
    pub const TREASURE_AOC: &'static str = "tkrAOc";
    pub const TREASURE_AOS: &'static str = "tkrAOs";
    pub const TREASURE_A_SWITCH: &'static str = "tkrASw";
    pub const TREASURE_CHEST_UNLOCKED_BY_LIGHT_BEAM: &'static str = "tkrBMs";
    pub const TREASURE_CTF: &'static str = "tkrCTf";
}

pub mod trigger {
    pub const EVENT_TRIGGER: &'static str = "TagEv";
    pub const HINT_TRIGGER: &'static str = "TagHt";
    pub const HINT_TRIGGER_2: &'static str = "TagHt2";
    pub const TEXT_EVENT_TRIGGER: &'static str = "TagMsg";
    pub const WEATHER_TRIGGER_0: &'static str = "ky_tag0";
    pub const WEATHER_TRIGGER_1: &'static str = "ky_tag1";
    pub const WEATHER_TRIGGER_2: &'static str = "ky_tag2";
    pub const WEATHER_TRIGGER_3: &'static str = "ky_tag3";
    pub const WEATHER_TRIGGER_4: &'static str = "kytag4";
    pub const WEATHER_TRIGGER_6: &'static str = "kytag6";
}

pub mod uncategorized {
    pub const ATDOOR: &'static str = "ATdoor";
    pub const AC1: &'static str = "Ac1";
    pub const AH: &'static str = "Ah";
    pub const INVISIBLE_WALL: &'static str = "Akabe";
    pub const AKABE10: &'static str = "Akabe10";
    pub const APZL: &'static str = "Apzl";
    pub const ASTOP: &'static str = "Astop";
    pub const ATTENTION_GRABBER_B: &'static str = "AttTagB";
    pub const AYGR: &'static str = "Aygr";
    pub const AYUSH: &'static str = "Ayush";
    pub const BLK_CR: &'static str = "BLK_CR";
    pub const HELMAROC_KING_OBJECT_GIBS: &'static str = "Bdkobj";
    pub const BITA: &'static str = "Bita";
    pub const BJ1: &'static str = "Bj1";
    pub const BJ2: &'static str = "Bj2";
    pub const BJ3: &'static str = "Bj3";
    pub const BJ4: &'static str = "Bj4";
    pub const BJ5: &'static str = "Bj5";
    pub const BJ6: &'static str = "Bj6";
    pub const BJ7: &'static str = "Bj7";
    pub const BJ8: &'static str = "Bj8";
    pub const BJ9: &'static str = "Bj9";
    pub const BLIFT: &'static str = "Blift";
    pub const BM3: &'static str = "Bm3";
    pub const BMCON1: &'static str = "Bmcon1";
    pub const BMCON2: &'static str = "Bmcon2";
    pub const BMSW: &'static str = "Bmsw";
    pub const BS1: &'static str = "Bs1";
    pub const BS2: &'static str = "Bs2";
    pub const BTSW2: &'static str = "Btsw2";
    pub const CAFE_LAMP: &'static str = "Cafelmp";
    pub const CMTRAP: &'static str = "CmTrap";
    pub const CO1: &'static str = "Co1";
    pub const COM_A: &'static str = "Com_A";
    pub const COM_C: &'static str = "Com_C";
    pub const CRTRM1: &'static str = "CrTrM1";
    pub const CRTRM2: &'static str = "CrTrM2";
    pub const CRTRS3: &'static str = "CrTrS3";
    pub const CRTRS4: &'static str = "CrTrS4";
    pub const CRTRS5: &'static str = "CrTrS5";
    pub const DBLK0: &'static str = "DBLK0";
    pub const DKKIBA: &'static str = "DKkiba";
    pub const DEMO_DK: &'static str = "Demo_Dk";
    pub const DK: &'static str = "Dk";
    pub const DS1: &'static str = "Ds1";
    pub const DSAKU: &'static str = "Dsaku";
    pub const EAYOGN: &'static str = "Eayogn";
    pub const EBOMZO: &'static str = "Ebomzo";
    pub const EBROCK2: &'static str = "Ebrock2";
    pub const ECUBE: &'static str = "Ecube";
    pub const EKAO: &'static str = "Ekao";
    pub const EKSKZ: &'static str = "Ekskz";
    pub const ESEKH: &'static str = "Esekh";
    pub const ESEKH2: &'static str = "Esekh2";
    pub const ESKBAN: &'static str = "Eskban";
    pub const EVSW: &'static str = "Evsw";
    pub const FTREE: &'static str = "FTree";
    pub const F_PLATFORM_FLIGHT_PLATFORM: &'static str = "Fdai";
    pub const FIGURE: &'static str = "Figure";
    pub const FIRE: &'static str = "Fire";
    pub const FLOOR_MASTER: &'static str = "Fmaster";
    pub const FLOOR_MASTER_1: &'static str = "Fmastr1";
    pub const GBOARD: &'static str = "GBoard";
    pub const GASHIP1: &'static str = "Gaship1";
    pub const GASHIP2: &'static str = "Gaship2";
    pub const GBRG00: &'static str = "Gbrg00";
    pub const GDEMO20: &'static str = "Gdemo20";
    pub const GFLAG: &'static str = "Gflag";
    pub const YELLOW_OCEAN_WARP: &'static str = "Ghrwp";
    pub const GICEL: &'static str = "GiceL";
    pub const GK1: &'static str = "Gk1";
    pub const GKAI00: &'static str = "Gkai00";
    pub const GNBTAKI: &'static str = "Gnbtaki";
    pub const GNTAKIE: &'static str = "Gntakie";
    pub const GNTAKIS: &'static str = "Gntakis";
    pub const GP1: &'static str = "Gp1";
    pub const GRYW00: &'static str = "Gryw00";
    pub const GTAKI: &'static str = "Gtaki";
    pub const GYCTRLB: &'static str = "GyCtrlB";
    pub const HAMI1: &'static str = "Hami1";
    pub const HAMI2: &'static str = "Hami2";
    pub const HAMI3: &'static str = "Hami3";
    pub const HAMI4: &'static str = "Hami4";
    pub const HAMIY: &'static str = "HamiY";
    pub const HBOX1: &'static str = "Hbox1";
    pub const HBOX2: &'static str = "Hbox2";
    pub const HBOX2S: &'static str = "Hbox2S";
    pub const HBRF1: &'static str = "Hbrf1";
    pub const HCBH: &'static str = "Hcbh";
    pub const HDAI1: &'static str = "Hdai1";
    pub const HDAI2: &'static str = "Hdai2";
    pub const HDAI3: &'static str = "Hdai3";
    pub const HFBOT1A: &'static str = "Hfbot1A";
    pub const HFBOT1B: &'static str = "Hfbot1B";
    pub const HFBOT1C: &'static str = "Hfbot1C";
    pub const HHA: &'static str = "Hha";
    pub const HHBOT1: &'static str = "Hhbot1";
    pub const HHBOT1N: &'static str = "Hhbot1N";
    pub const SPRING_ON_A_BLOCK_1: &'static str = "Hjump1";
    pub const HKIKAI1: &'static str = "Hkikai1";
    pub const HMLIF: &'static str = "Hmlif";
    pub const HMON1: &'static str = "Hmon1";
    pub const HMON1D: &'static str = "Hmon1d";
    pub const HMON2: &'static str = "Hmon2";
    pub const HMON2D: &'static str = "Hmon2d";
    pub const HMOS1: &'static str = "Hmos1";
    pub const HMOS2: &'static str = "Hmos2";
    pub const HMOS3: &'static str = "Hmos3";
    pub const HO: &'static str = "Ho";
    pub const HOMEN1: &'static str = "Homen1";
    pub const HOMEN2: &'static str = "Homen2";
    pub const HPU1: &'static str = "Hpu1";
    pub const HPU2: &'static str = "Hpu2";
    pub const HR2: &'static str = "Hr2";
    pub const HSEKI1: &'static str = "Hseki1";
    pub const HSEKI2: &'static str = "Hseki2";
    pub const HSEKI3: &'static str = "Hseki3";
    pub const HSEKI4: &'static str = "Hseki4";
    pub const HSEKI5: &'static str = "Hseki5";
    pub const HSEKI6: &'static str = "Hseki6";
    pub const HSEKI7: &'static str = "Hseki7";
    pub const HSEN2: &'static str = "Hsen2";
    pub const HSEN3: &'static str = "Hsen3";
    pub const HSH: &'static str = "Hsh";
    pub const HSH2: &'static str = "Hsh2";
    pub const HTETU1: &'static str = "Htetu1";
    pub const HTOBI1: &'static str = "Htobi1";
    pub const HTOBI2: &'static str = "Htobi2";
    pub const HTOBI3: &'static str = "Htobi3";
    pub const HUMI0Z: &'static str = "Humi0z";
    pub const HUMI2Z: &'static str = "Humi2z";
    pub const HUMI3Z: &'static str = "Humi3z";
    pub const HUMI4Z: &'static str = "Humi4z";
    pub const HUMI5Z: &'static str = "Humi5z";
    pub const HYOIKAM: &'static str = "HyoiKam";
    pub const HYS: &'static str = "Hys";
    pub const HYS2: &'static str = "Hys2";
    pub const HYUF1: &'static str = "Hyuf1";
    pub const HYUF2: &'static str = "Hyuf2";
    pub const ITAT00: &'static str = "ITat00";
    pub const IKADA: &'static str = "Ikada";
    pub const IKARI: &'static str = "Ikari";
    pub const IKORI: &'static str = "Ikori";
    pub const JI1: &'static str = "Ji1";
    pub const KGBDOR: &'static str = "KGBdor";
    pub const DOOR_2: &'static str = "KNOB02";
    pub const DOOR_3: &'static str = "KNOB03";
    pub const KANAT: &'static str = "Kanat";
    pub const KBOTAC: &'static str = "KbotaC";
    pub const KBOTA_B: &'static str = "Kbota_B";
    pub const KF1: &'static str = "Kf1";
    pub const KG1: &'static str = "Kg1";
    pub const KG2: &'static str = "Kg2";
    pub const KITA: &'static str = "Kita";
    pub const KK1: &'static str = "Kk1";
    pub const KKIBA: &'static str = "Kkiba";
    pub const KKIBAB: &'static str = "KkibaB";
    pub const FORBBIDEN_WOODS_LIFT: &'static str = "Klft";
    pub const KM1: &'static str = "Km1";
    pub const KMI00: &'static str = "Kmi00";
    pub const KMI02: &'static str = "Kmi02";
    pub const KMTUB: &'static str = "Kmtub";
    pub const KO1: &'static str = "Ko1";
    pub const KO2: &'static str = "Ko2";
    pub const KOKIIE: &'static str = "Kokiie";
    pub const KP1: &'static str = "Kp1";
    pub const KROCK00: &'static str = "Krock00";
    pub const KRYU00: &'static str = "Kryu00";
    pub const KSAKU: &'static str = "Ksaku";
    pub const KTARU: &'static str = "Ktaru";
    pub const KTARUO: &'static str = "Ktaruo";
    pub const KTARUR: &'static str = "Ktarur";
    pub const KTARUX: &'static str = "Ktarux";
    pub const REFLECTABLE_LIGHT_BEAM_0: &'static str = "LTag0";
    pub const REFLECTABLE_LIGHT_BEAM_1: &'static str = "LTag1";
    pub const LTAGR0: &'static str = "LTagR0";
    pub const LAMP: &'static str = "Lamp";
    pub const MKANOK2: &'static str = "MKanok2";
    pub const MKANOKE: &'static str = "MKanoke";
    pub const MCRTN: &'static str = "Mcrtn";
    pub const MCUBE: &'static str = "Mcube";
    pub const MCUBE10: &'static str = "Mcube10";
    pub const MCYLN: &'static str = "Mcyln";
    pub const MFLFT: &'static str = "Mflft";
    pub const MHMRSW0: &'static str = "MhmrSW0";
    pub const MHSG12: &'static str = "Mhsg12";
    pub const MHSG15: &'static str = "Mhsg15";
    pub const MHSG4H: &'static str = "Mhsg4h";
    pub const MHSG6: &'static str = "Mhsg6";
    pub const MHSG9: &'static str = "Mhsg9";
    pub const MJDOOR: &'static str = "MjDoor";
    pub const MK: &'static str = "Mk";
    pub const MKDAN1: &'static str = "Mkdan1";
    pub const MKIEBA: &'static str = "MkieBA";
    pub const MKIEBAB: &'static str = "MkieBAB";
    pub const MKIEBB: &'static str = "MkieBB";
    pub const MKIEK: &'static str = "MkieK";
    pub const MKNJD: &'static str = "MknjD";
    pub const MMRR: &'static str = "Mmrr";
    pub const MMUSIC: &'static str = "Mmusic";
    pub const MN: &'static str = "Mn";
    pub const MORI1: &'static str = "Mori1";
    pub const MPWRB: &'static str = "MpwrB";
    pub const MSDAN: &'static str = "Msdan";
    pub const MSDAN2: &'static str = "Msdan2";
    pub const MSUSW: &'static str = "MsuSW";
    pub const MSUSWB: &'static str = "MsuSWB";
    pub const MSWING: &'static str = "Mswing";
    pub const MT: &'static str = "Mt";
    pub const MTFLAG: &'static str = "MtFlag";
    pub const MTORISU: &'static str = "MtoriSU";
    pub const TRIANGULAR_PRISM_BLOCK: &'static str = "MtryB";
    pub const TRIANGULAR_PRISM_BLOCK_TARGET_LOCATION: &'static str = "MtryBCr";
    pub const MWTRSB: &'static str = "MwtrSB";
    pub const MYGNSB: &'static str = "MygnSB";
    pub const NBOX: &'static str = "NBOX";
    pub const NBOX10: &'static str = "NBOX10";
    pub const NH: &'static str = "Nh";
    pub const NPCSO: &'static str = "NpcSo";
    pub const NZFALL: &'static str = "Nzfall";
    pub const OB1: &'static str = "Ob1";
    pub const TIMER: &'static str = "ObjTime";
    pub const OCANON: &'static str = "Ocanon";
    pub const OCLOUD: &'static str = "Ocloud";
    pub const OHATCH: &'static str = "Ohatch";
    pub const OJTREE: &'static str = "Ojtree";
    pub const OKIOKE: &'static str = "Okioke";
    pub const OLIFT: &'static str = "Olift";
    pub const OQ: &'static str = "Oq";
    pub const OQW: &'static str = "Oqw";
    pub const OS: &'static str = "Os";
    pub const OS1: &'static str = "Os1";
    pub const OS2: &'static str = "Os2";
    pub const OSHIP: &'static str = "Oship";
    pub const OSTOOL: &'static str = "Ostool";
    pub const OTANA: &'static str = "Otana";
    pub const OTBLE: &'static str = "Otble";
    pub const OTBLEL: &'static str = "OtbleL";
    pub const OWATER: &'static str = "Owater";
    pub const P1A: &'static str = "P1a";
    pub const P1B: &'static str = "P1b";
    pub const P2A: &'static str = "P2a";
    pub const P2B: &'static str = "P2b";
    pub const P2C: &'static str = "P2c";
    pub const PSCNCHG: &'static str = "PScnChg";
    pub const PAPER: &'static str = "Paper";
    pub const PBCO: &'static str = "Pbco";
    pub const PBKA: &'static str = "Pbka";
    pub const PF1: &'static str = "Pf1";
    pub const PIRATES: &'static str = "Pirates";
    pub const PIWA: &'static str = "Piwa";
    pub const PLANT: &'static str = "Plant";
    pub const PM1: &'static str = "Pm1";
    pub const PO: &'static str = "Po";
    pub const PPOS: &'static str = "Ppos";
    pub const PTCO: &'static str = "Ptco";
    pub const PTCU: &'static str = "Ptcu";
    pub const PTUBO: &'static str = "Ptubo";
    pub const PUTI: &'static str = "Puti";
    pub const QDGHD: &'static str = "Qdghd";
    pub const QTKHD: &'static str = "Qtkhd";
    pub const QUAKE: &'static str = "Quake";
    pub const RCLOUD: &'static str = "Rcloud";
    pub const RDEAD2: &'static str = "Rdead2";
    pub const RETAG0: &'static str = "ReTag0";
    pub const RFLW: &'static str = "Rflw";
    pub const RFORCE: &'static str = "Rforce";
    pub const ROTEN2: &'static str = "Roten2";
    pub const ROTEN3: &'static str = "Roten3";
    pub const ROTEN4: &'static str = "Roten4";
    pub const SMBDOR: &'static str = "SMBdor";
    pub const SMTOGE: &'static str = "SMtoge";
    pub const SPITEM: &'static str = "SPitem";
    pub const SWTDOOR: &'static str = "SWTdoor";
    pub const SWAT00: &'static str = "SWat00";
    pub const WIND_WAKER_SONG_SWITCH: &'static str = "SWtact";
    pub const CHANDELIER: &'static str = "SYAN";
    pub const SA1: &'static str = "Sa1";
    pub const SA2: &'static str = "Sa2";
    pub const SA3: &'static str = "Sa3";
    pub const SA4: &'static str = "Sa4";
    pub const SA5: &'static str = "Sa5";
    pub const SALVFM: &'static str = "SalvFM";
    pub const SALVAG2: &'static str = "Salvag2";
    pub const SALVAGE_E: &'static str = "SalvagE";
    pub const SALVAGN: &'static str = "SalvagN";
    pub const SALVAGE: &'static str = "Salvage";
    pub const SARACE: &'static str = "Sarace";
    pub const SEARCH: &'static str = "Search";
    pub const SFAIRY: &'static str = "Sfairy";
    pub const KING_OF_RED_LIONS_SHIP_FORM_PROP: &'static str = "Ship";
    pub const SHMRGRD: &'static str = "Shmrgrd";
    pub const SIEFLAG: &'static str = "SieFlag";
    pub const SITEM: &'static str = "Sitem";
    pub const SKANRAN: &'static str = "Skanran";
    pub const STDOORL: &'static str = "Stdoorl";
    pub const STDOORR: &'static str = "Stdoorr";
    pub const STGATE: &'static str = "Stgate";
    pub const STOUDAI: &'static str = "Stoudai";
    pub const STTOGE: &'static str = "Sttoge";
    pub const SV0: &'static str = "Sv0";
    pub const SV1: &'static str = "Sv1";
    pub const SV2: &'static str = "Sv2";
    pub const SV3: &'static str = "Sv3";
    pub const SVSP: &'static str = "Svsp";
    pub const SALVAGE_SWITCH: &'static str = "SwSlvg";
    pub const TABLE: &'static str = "Table";
    pub const TAGCB1: &'static str = "TagCb1";
    pub const TAGCB11: &'static str = "TagCb11";
    pub const TAGCB12: &'static str = "TagCb12";
    pub const TAGCB13: &'static str = "TagCb13";
    pub const TAGCB14: &'static str = "TagCb14";
    pub const TAGD1: &'static str = "TagD1";
    pub const TAGD2: &'static str = "TagD2";
    pub const TAGD3: &'static str = "TagD3";
    pub const TAGD4: &'static str = "TagD4";
    pub const TAGDM: &'static str = "TagDM";
    pub const TAGISL: &'static str = "TagIsl";
    pub const TAGKB: &'static str = "TagKb";
    pub const TAGMSO: &'static str = "TagMSo";
    pub const TAGMD: &'static str = "TagMd";
    pub const TAGMD1: &'static str = "TagMd1";
    pub const TAGMD11: &'static str = "TagMd11";
    pub const TAGMD12: &'static str = "TagMd12";
    pub const TAGMD13: &'static str = "TagMd13";
    pub const TAGMD14: &'static str = "TagMd14";
    pub const TAGMD15: &'static str = "TagMd15";
    pub const TAGMD16: &'static str = "TagMd16";
    pub const TAGMK: &'static str = "TagMk";
    pub const TAGPO: &'static str = "TagPo";
    pub const TAGSO: &'static str = "TagSo";
    pub const TAGWP: &'static str = "TagWp";
    pub const TENMADO: &'static str = "Tenmado";
    pub const TESTPO: &'static str = "TestPo";
    pub const LASER_BARRIER_INISIDE_HYRULE_CASTLE: &'static str = "TnTrap";
    pub const TPOTA: &'static str = "Tpota";
    pub const TRFLAG: &'static str = "TrFlag";
    pub const TURU: &'static str = "Turu";
    pub const TURU2: &'static str = "Turu2";
    pub const TURU3: &'static str = "Turu3";
    pub const UB1: &'static str = "Ub1";
    pub const UB2: &'static str = "Ub2";
    pub const UB3: &'static str = "Ub3";
    pub const UB4: &'static str = "Ub4";
    pub const UG1: &'static str = "Ug1";
    pub const UG2: &'static str = "Ug2";
    pub const UM1: &'static str = "Um1";
    pub const UM2: &'static str = "Um2";
    pub const UM3: &'static str = "Um3";
    pub const UO1: &'static str = "Uo1";
    pub const UO2: &'static str = "Uo2";
    pub const UO3: &'static str = "Uo3";
    pub const USOVMC: &'static str = "Usovmc";
    pub const UW1: &'static str = "Uw1";
    pub const UW2: &'static str = "Uw2";
    pub const VBAKH: &'static str = "VbakH";
    pub const VDORA: &'static str = "Vdora";
    pub const VDS: &'static str = "Vds";
    pub const VFAN: &'static str = "Vfan";
    pub const PEDASTAL_OF_TIME: &'static str = "VmsDZ";
    pub const MASTER_SWORD_MODEL_FROM_HYRULE_CASTLE_BASEMENT: &'static str = "VmsMS";
    pub const VOLTAG: &'static str = "VolTag";
    pub const VPBOT: &'static str = "Vpbot";
    pub const VTENG: &'static str = "Vteng";
    pub const VTIL1: &'static str = "Vtil1";
    pub const VTIL2: &'static str = "Vtil2";
    pub const VTIL3: &'static str = "Vtil3";
    pub const VTIL4: &'static str = "Vtil4";
    pub const VTIL5: &'static str = "Vtil5";
    pub const VYASI: &'static str = "Vyasi";
    pub const WLVTAG: &'static str = "WLvTag";
    pub const WALL: &'static str = "Wall";
    pub const DUNGEON_WARP_EXIT: &'static str = "Warpf";
    pub const WARPFO: &'static str = "Warpfo";
    pub const WARPGN: &'static str = "Warpgn";
    pub const WARPNT: &'static str = "Warpnt";
    pub const WARPT: &'static str = "Warpt";
    pub const WARP_JAR_1: &'static str = "Warpts1";
    pub const WARP_JAR_3: &'static str = "Warpts3";
    pub const WIND_COLUMN: &'static str = "WindTag";
    pub const YBGAF00: &'static str = "Ybgaf00";
    pub const YBOIL00: &'static str = "Yboil00";
    pub const MAGICAL_BARRIER: &'static str = "Ycage00";
    pub const YFRLT00: &'static str = "Yfrlt00";
    pub const YGCWP: &'static str = "Ygcwp";
    pub const YGSTP00: &'static str = "Ygstp00";
    pub const YGUSH00: &'static str = "Ygush00";
    pub const YGUSH01: &'static str = "Ygush01";
    pub const YGUSH02: &'static str = "Ygush02";
    pub const YKGROFF: &'static str = "YkgrOFF";
    pub const YKGRON: &'static str = "YkgrON";
    pub const YKZYG: &'static str = "Ykzyg";
    pub const YLKIC: &'static str = "Ylkic";
    pub const YLLIC: &'static str = "Yllic";
    pub const YLSIC: &'static str = "Ylsic";
    pub const YM1: &'static str = "Ym1";
    pub const YM2: &'static str = "Ym2";
    pub const SHAFT_OF_LIGHT_WARP: &'static str = "Ysdls00";
    pub const YTRND00: &'static str = "Ytrnd00";
    pub const YW1: &'static str = "Yw1";
    pub const YWARP00: &'static str = "Ywarp00";
    pub const ZK1: &'static str = "Zk1";
    pub const AGBA: &'static str = "agbA";
    pub const AGBA2: &'static str = "agbA2";
    pub const AGBAT: &'static str = "agbAT";
    pub const AGBB: &'static str = "agbB";
    pub const AGBD: &'static str = "agbD";
    pub const AGBF: &'static str = "agbF";
    pub const AGBF2: &'static str = "agbF2";
    pub const AGBFA: &'static str = "agbFA";
    pub const AGBMARK: &'static str = "agbMARK";
    pub const AGBMW: &'static str = "agbMW";
    pub const AGBR: &'static str = "agbR";
    pub const AGBTBOX: &'static str = "agbTBOX";
    pub const TORCH: &'static str = "bonbori";
    pub const DMGROOM: &'static str = "dmgroom";
    pub const DRAGON: &'static str = "dragon";
    pub const FLOWER: &'static str = "flower";
    pub const FLWR7: &'static str = "flwr7";
    pub const FROCK: &'static str = "frock";
    pub const GMOS: &'static str = "gmos";
    pub const LOWERCASE_HO: &'static str = "ho";
    pub const IKADAS: &'static str = "ikadaS";
    pub const BEEDLES_SHOPSHIP: &'static str = "ikada_h";
    pub const IKADA_U: &'static str = "ikada_u";
    pub const KT: &'static str = "kt";
    pub const KURO_S: &'static str = "kuro_s";
    pub const KURO_T: &'static str = "kuro_t";
    pub const KUSAX1: &'static str = "kusax1";
    pub const KUSAX21: &'static str = "kusax21";
    pub const KUSAX7: &'static str = "kusax7";
    pub const KY00YOU: &'static str = "ky00you";
    pub const KYTAG00: &'static str = "kytag00";
    pub const KYTAG5: &'static str = "kytag5";
    pub const MOUSE_HOLE: &'static str = "nezuana";
    pub const PFLOWER: &'static str = "pflower";
    pub const S_TURU: &'static str = "s_turu";
    pub const SEA: &'static str = "sea";
    pub const SPEAKUN: &'static str = "speakun";
    pub const SPOTBX1: &'static str = "spotbx1";
    pub const SWOOD: &'static str = "swood";
    pub const SWOOD3: &'static str = "swood3";
    pub const WOODB: &'static str = "woodb";
    pub const WOODBX: &'static str = "woodbx";
    pub const KNIGHT_STATUE: &'static str = "zouK";
    pub const KNIGHT_STATUE_1: &'static str = "zouK1";
    pub const KNIGHT_STATUE_2: &'static str = "zouK2";
    pub const KNIGHT_STATUE_3: &'static str = "zouK3";
    pub const KNIGHT_STATUE_4: &'static str = "zouK4";
    pub const TRIFORCE_FLAG: &'static str = "HcFlag";
    pub const FORSAKEN_FORTRESS_FLAG: &'static str = "MjFlag";
    pub const JET_OF_STEAM_0: &'static str = "Ystm0";
    pub const JET_OF_STEAM_1: &'static str = "Ystm1";
    pub const MAGMA: &'static str = "magma";
}

#[repr(C, packed)]
pub struct ActorTemplate {
    pub name: [u8; 8],
    pub params: u32,
    pub coord: Coord,
    pub rotation: [u16; 2],
    pub flag: u16,
    pub enemy_id: i16,
}

#[repr(C, packed)]
pub struct ActorMemory {
    pub params: u32,
    pub coord: Coord,
    pub rotation: [u16; 2],
    pub flag: u16,
    pub enemy_id: i16,
    pub flags: [u8; 9],
    pub room_id: u8,
    pub padding: [u8; 2],
}

impl ActorMemory {
    fn new() -> &'static mut ActorMemory {
        system::fopacm_create_append()
    }

    fn write_actor(&mut self, actor: &ActorTemplate) {
        self.params = actor.params;
        self.coord = actor.coord.clone();
        self.rotation[0] = actor.rotation[0];
        self.rotation[1] = actor.rotation[1];
        self.flag = actor.flag;
        self.enemy_id = actor.enemy_id;
    }
}

impl ActorTemplate {
    pub fn new(name: &str, coord: Coord, rotation: [u16; 2]) -> Self {
        let mut actor = ActorTemplate {
            name: [0; 8],
            params: DEFAULT_PARAMS,
            coord: coord,
            rotation: rotation,
            flag: DEFAULT_FLAG,
            enemy_id: DEFAULT_ENEMY_ID,
        };
        memory::write_str(actor.name.as_mut_ptr(), name);
        actor
    }

    pub fn with_params(mut self, params: u32) -> Self {
        self.params = params;
        self
    }

    pub fn with_flag(mut self, flag: u16) -> Self {
        self.flag = flag;
        self
    }

    pub fn with_enemy_id(mut self, enemy_id: i16) -> Self {
        self.enemy_id = enemy_id;
        self
    }

    pub fn actor_name(&self) -> &str {
        memory::read_str(self.name.as_ptr())
    }

    pub fn spawn(&self) -> &'static mut ActorMemory {
        let memory = ActorMemory::new();
        memory.write_actor(self);
        memory.room_id = Link::room();

        layer::switch_to_safe_layer();

        system::dstage_actor_create(self, memory);

        memory
    }
}
