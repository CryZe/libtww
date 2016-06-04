use {Coord, system};
use system::memory;
use link::Link;
use game::layer;

pub const DEFAULT_ENEMY_ID: i16 = -1;
pub const DEFAULT_FLAG: u16 = 0;
pub const DEFAULT_PARAMS: u32 = 0;

macro_rules! actors {
    ($($mod_name:ident { $($name:ident: $actor_name:ident)* })*) => {
        $(
            pub mod $mod_name {
                $(
                    pub const $name: &'static str = stringify!($actor_name);
                )*
            }
        )*
    }
}

actors! {
    breakable {
        SIGN: Kanban
        BREAKABLE_CUP: MKoppu
        BREAKABLE_PLATE: MOsara
        BREAKABLE_JUG: MPot
        SKULL: Odokuro
        NUT: VigaH
        PILE_OF_LEAVES: Vochi
        SMALL_POT: kotubo
        LARGE_POT: ootubo1
    }

    door {
        KNOB00D: KNOB00D
        KNOB01D: KNOB01D
    }

    dungeon_boss {
        KALLE_DEMOS: Bkm
        GOHDAN: Bst
        GOHMA: Btd
        MOLGERA: Bwd
        GANONDORF: Gnd
        JALHALLA: big_pow
    }

    enemy_npc {
        KARGAROC: Bb
        BOKOBLIN: Bk
        QUILL: Bm1
        CANON: Canon
        BIG_OCTO: Daiocta
        PHANTOM_GANON: Fganon
        FIRE_KEESE: Fkeeth
        FLOOR_MASTER_2: Fmastr2
        GYORG: GyCtrl
        REDEAD: Rdead1
        DEXIVINE: Sss
        STALFOS: Stal
        DARKNUT: Tn
        BLADE_TRAP: Trap
        ARMOS: amos
        ARMOS_2: amos2
        BUBBLE: bable
        BOKO_BABA: bbaba
        BLACK_CHUCHU: c_black
        BLUE_CHUCHU: c_blue
        GREEN_CHUCHU: c_green
        YELLOW_CHUCHU: c_kiiro
        RED_CHUCHU: c_red
        KEESE: keeth
        MAGTAIL: magtail
        MOBLIN: mo2
        MOBLIN_STATUE: moZOU
        MOUSE: nezumi
        PEAHAT: p_hat
        POE: pow
        REDEAD_1: rdead1
        REGULAR_WIZZROBE: wiz_r
    }

    exit {
        DOOR_0: KNOB00
        DOOR_1: KNOB01
        GROTTO_ENTRANCE: Pitfall
    }

    foliage {
        PALM_TREE: Oyashi
        FLOWER: flower
        FLWR17: flwr17
        FLWR7: flwr7
        SMALL_ROCK_1: koisi1
        KUSAX1: kusax1
        KUSAX21: kusax21
        KUSAX7: kusax7
        LARGE_TREE: lwood
        PFLWRX7: pflwrx7
        SMALL_TREE_3: swood3
        SMALL_TREE_5: swood5
    }

    friendly_npc {
        STURGEON: Aj1
        GRANDMA: Ba1
        GREAT_FAIRY: BigElf
        RITO_POSTMAN_2: Bm2
        RITO_POSTMAN_4: Bm4
        RITO_POSTMAN_5: Bm5
        MAKAR: Cb1
        SEAGULL: Kamome
        ARYLL: Ls1
        MEDLI: Md1
        PIG: Pig
        TETRA: Zl1
        CRAB: kani
    }

    gameplay {
        ATTENTION_GRABBER: AttTag
        BOMB_FLOWER: BFlower
        HEART_CONTAINER_DUNGEON_BOSS_ITEM_DROP: Bitem
        VALOOS_TAIL: Dr2
        HOOKSHOT_TARGET: Hfuck1
        BREAKABLE_FLOOR_TILE: Hhyu1
        SPRING_ON_A_BLOCK_2: Hjump2
        WIND_COLUMN_GENERATOR: Hsen1
        GRAPPLE_POINT: Kui
        SOLIDIFIED_MAGMA_PLATFORM: Magrock
        WOODEN_BOX_WITH_BLACK_FRAME: Ospbox
        DANGLING_ROPE_WITH_LANTERN: RopeR
        POSTBOX: Tpost
        WARP_JAR_2: Warpts2
        JET_OF_MAGMA: Yfire00
        RING_OF_FIRE: Zenfire
        BRIDGE: bridge
        COLLECTIBLE_ITEM: item
        BABA_BUD: jbaba
        PUSHABLE_BLOCK_0: osiBLK0
        PUSHABLE_BLOCK_1: osiBLK1
    }

    lod_model {
        FORSAKEN_FORTRESS: LOD01
        STAR_ISLAND: LOD02
        NORTHERN_FAIRY_ISLE: LOD03
        GALE_ISLAND: LOD04
        CRESCENT_MOON_ISLE: LOD05
        SEVEN_STAR_ISLES: LOD06
        OVERLOOK_ISLAND: LOD07
        FOUR_EYE_REEF: LOD08
        MOTHER_AND_CHILD_ISLES: LOD09
        SPECTACLE_ISLAND: LOD10
        WINDFALL_ISLAND: LOD11
        PAWPRINT_ISLE: LOD12
        DRAGON_ROOST_ISLAND: LOD13
        FLIGHT_CONTROL_PLATFORM: LOD14
        WESTERN_FAIRY_ISLE: LOD15
        ROCK_SPIRE_ISLE: LOD16
        TINGLE_ISLAND: LOD17
        NORTHERN_TRIANGLE_ISLAND: LOD18
        EASTERN_FAIRY_ISLE: LOD19
        FIRE_MOUNTAIN: LOD20
        STAR_BELT_ARCHIPELAGO: LOD21
        THREE_EYE_REEF: LOD22
        GREATFISH_ISLE: LOD23
        CYCLOPS_REEF: LOD24
        SIX_EYE_REEF: LOD25
        TOWER_OF_THE_GODS: LOD26
        EASTERN_TRIANGLE_ISLAND: LOD27
        THORNED_FAIRY_ISLE: LOD28
        NEEDLEPOINT_ISLAND: LOD29
        ISLET_OF_STEEL: LOD30
        STONE_WATCHER_ISLAND: LOD31
        SOUTHERN_TRIANGLE_ISLAND: LOD32
        PRIVATE_OASIS: LOD33
        BOMB_ISLAND: LOD34
        BIRDS_PEAK_ISLAND: LOD35
        DIAMOND_STEPPE_ISLAND: LOD36
        FIVE_EYE_REEF: LOD37
        SHARK_ISLAND: LOD38
        SOUTHERN_FAIRY_ISLE: LOD39
        ICE_RING_ISLE: LOD40
        FOREST_HAVEN: LOD41
        CLIFF_PLATEAU_ISLES: LOD42
        HORSESHOE_ISLAND: LOD43
        OUTSET_ISLAND: LOD44
        HEADSTONE_ISLAND: LOD45
        TWO_EYE_REEF: LOD46
        ANGULAR_ISLES: LOD47
        BOAT_RACE_ISLAND: LOD48
        FIVE_STAR_ISLES: LOD49
    }

    large_object {
        STALL_A: RotenA
        STALL_B: RotenB
        STALL_C: RotenC
        TOWER_OF_THE_GODS_EXTERIOR: X_tower
        LINK_STATUE_INSIDE_HYRULE_CASTLE: YLzou
    }

    mechanics {
        SEED_PLANTING_SPOT_FOR_MAKAR: VmcBS
    }

    obstacle {
        IRON_BARS: Ashut
        LARGE_ROCK: Ebrock
        SPIKE: Htoge1
        EYE_VINE_BLOCKER: Ss
        TINGLE: Tc
    }

    storyline {
        TRIANGLE_ISLAND_STATUE: Doguu
        ZEPHOS_AND_CYCLOS: Hr
        DIN_STATUE: MegamiD
        FARORE_STATUE: MegamiF
        NAYRU_STATUE: MegamiN
        GANONS_TOWER_4_BOSS_DOOR: VgnFD
    }

    switch {
        ALL_ENEMIES_KILLED_SWITCH: ALLdie
        SWITCH_BUFFER_0: AND_SW0
        SWITCH_BUFFER_2: AND_SW2
        WIND_SWITCH: Hpbot1
        FLOOR_SWITCH_A: Kbota_A
        PROXIMITY_SWITCH: SW_C00
        CRYSTAL_SWITCH: SW_HIT0
        WIND_WAKER_SONG_SWITCH_B: SWtactB
        TINGLE_C_SWITCH: agbCSW
    }

    tg_door {
        KNOB00D: KNOB00D
        KNOB01D: KNOB01D
        KNOB03D: KNOB03D
        ZENS12: ZenS12
        DUNGEON_BARRED_DOOR: Zenshut
        NORMAL_DUNGEON_DOOR: door10
        NORMAL_EARTH_AND_WIND_TEMPLE_DOOR: door12
        BOSS_DUNGEON_DOOR: door20
        FORBIDDEN_WOODS_BOSS_DOOR: doorKD
        BARRED_EARTH_AND_WIND_TEMPLE_DOOR: doorSH
        LOCKED_EARTH_AND_WIND_TEMPLE_DOOR: keyS12
        DUNGEON_LOCKED_DOOR: keyshut
    }

    treasure_chest {
        TREASURE_CHEST: takara
        TREASURE_CHEST_2: takara2
        TAKARA3: takara3
        TREASURE_CHEST_3: takara3
        TREASURE_CHEST_4: takara4
        TREASURE_CHEST_5: takara5
        TREASURE_CHEST_6: takara6
        TREASURE_CHEST_7: takara7
        TREASURE_CHEST_8: takara8
        TREASURE_I: takaraI
        TREASURE_K: takaraK
        TREASURE_M: takaraM
        TREASURE_AGC: tkrAGc
        TREASURE_AIK: tkrAIk
        TREASURE_AKD: tkrAKd
        TREASURE_AOC: tkrAOc
        TREASURE_AOS: tkrAOs
        TREASURE_A_SWITCH: tkrASw
        TREASURE_CHEST_UNLOCKED_BY_LIGHT_BEAM: tkrBMs
        TREASURE_CTF: tkrCTf
    }

    trigger {
        EVENT_TRIGGER: TagEv
        HINT_TRIGGER: TagHt
        HINT_TRIGGER_2: TagHt2
        TEXT_EVENT_TRIGGER: TagMsg
        WEATHER_TRIGGER_0: ky_tag0
        WEATHER_TRIGGER_1: ky_tag1
        WEATHER_TRIGGER_2: ky_tag2
        WEATHER_TRIGGER_3: ky_tag3
        WEATHER_TRIGGER_4: kytag4
        WEATHER_TRIGGER_6: kytag6
    }

    uncategorized {
        ATDOOR: ATdoor
        AC1: Ac1
        AH: Ah
        INVISIBLE_WALL: Akabe
        AKABE10: Akabe10
        APZL: Apzl
        ASTOP: Astop
        ATTENTION_GRABBER_B: AttTagB
        AYGR: Aygr
        AYUSH: Ayush
        BLK_CR: BLK_CR
        HELMAROC_KING_OBJECT_GIBS: Bdkobj
        BITA: Bita
        BJ1: Bj1
        BJ2: Bj2
        BJ3: Bj3
        BJ4: Bj4
        BJ5: Bj5
        BJ6: Bj6
        BJ7: Bj7
        BJ8: Bj8
        BJ9: Bj9
        BLIFT: Blift
        BM3: Bm3
        BMCON1: Bmcon1
        BMCON2: Bmcon2
        BMSW: Bmsw
        BS1: Bs1
        BS2: Bs2
        BTSW2: Btsw2
        CAFE_LAMP: Cafelmp
        CMTRAP: CmTrap
        CO1: Co1
        COM_A: Com_A
        COM_C: Com_C
        CRTRM1: CrTrM1
        CRTRM2: CrTrM2
        CRTRS3: CrTrS3
        CRTRS4: CrTrS4
        CRTRS5: CrTrS5
        DBLK0: DBLK0
        DKKIBA: DKkiba
        DEMO_DK: Demo_Dk
        DK: Dk
        DS1: Ds1
        DSAKU: Dsaku
        EAYOGN: Eayogn
        EBOMZO: Ebomzo
        EBROCK2: Ebrock2
        ECUBE: Ecube
        EKAO: Ekao
        EKSKZ: Ekskz
        ESEKH: Esekh
        ESEKH2: Esekh2
        ESKBAN: Eskban
        EVSW: Evsw
        FTREE: FTree
        F_PLATFORM_FLIGHT_PLATFORM: Fdai
        FIGURE: Figure
        FIRE: Fire
        FLOOR_MASTER: Fmaster
        FLOOR_MASTER_1: Fmastr1
        GBOARD: GBoard
        GASHIP1: Gaship1
        GASHIP2: Gaship2
        GBRG00: Gbrg00
        GDEMO20: Gdemo20
        GFLAG: Gflag
        YELLOW_OCEAN_WARP: Ghrwp
        GICEL: GiceL
        GK1: Gk1
        GKAI00: Gkai00
        GNBTAKI: Gnbtaki
        GNTAKIE: Gntakie
        GNTAKIS: Gntakis
        GP1: Gp1
        GRYW00: Gryw00
        GTAKI: Gtaki
        GYCTRLB: GyCtrlB
        HAMI1: Hami1
        HAMI2: Hami2
        HAMI3: Hami3
        HAMI4: Hami4
        HAMIY: HamiY
        HBOX1: Hbox1
        HBOX2: Hbox2
        HBOX2S: Hbox2S
        HBRF1: Hbrf1
        HCBH: Hcbh
        HDAI1: Hdai1
        HDAI2: Hdai2
        HDAI3: Hdai3
        HFBOT1A: Hfbot1A
        HFBOT1B: Hfbot1B
        HFBOT1C: Hfbot1C
        HHA: Hha
        HHBOT1: Hhbot1
        HHBOT1N: Hhbot1N
        SPRING_ON_A_BLOCK_1: Hjump1
        HKIKAI1: Hkikai1
        HMLIF: Hmlif
        HMON1: Hmon1
        HMON1D: Hmon1d
        HMON2: Hmon2
        HMON2D: Hmon2d
        HMOS1: Hmos1
        HMOS2: Hmos2
        HMOS3: Hmos3
        HO: Ho
        HOMEN1: Homen1
        HOMEN2: Homen2
        HPU1: Hpu1
        HPU2: Hpu2
        HR2: Hr2
        HSEKI1: Hseki1
        HSEKI2: Hseki2
        HSEKI3: Hseki3
        HSEKI4: Hseki4
        HSEKI5: Hseki5
        HSEKI6: Hseki6
        HSEKI7: Hseki7
        HSEN2: Hsen2
        HSEN3: Hsen3
        HSH: Hsh
        HSH2: Hsh2
        HTETU1: Htetu1
        HTOBI1: Htobi1
        HTOBI2: Htobi2
        HTOBI3: Htobi3
        HUMI0Z: Humi0z
        HUMI2Z: Humi2z
        HUMI3Z: Humi3z
        HUMI4Z: Humi4z
        HUMI5Z: Humi5z
        HYOIKAM: HyoiKam
        HYS: Hys
        HYS2: Hys2
        HYUF1: Hyuf1
        HYUF2: Hyuf2
        ITAT00: ITat00
        IKADA: Ikada
        IKARI: Ikari
        IKORI: Ikori
        JI1: Ji1
        KGBDOR: KGBdor
        DOOR_2: KNOB02
        DOOR_3: KNOB03
        KANAT: Kanat
        KBOTAC: KbotaC
        KBOTA_B: Kbota_B
        KF1: Kf1
        KG1: Kg1
        KG2: Kg2
        KITA: Kita
        KK1: Kk1
        KKIBA: Kkiba
        KKIBAB: KkibaB
        FORBBIDEN_WOODS_LIFT: Klft
        KM1: Km1
        KMI00: Kmi00
        KMI02: Kmi02
        KMTUB: Kmtub
        KO1: Ko1
        KO2: Ko2
        KOKIIE: Kokiie
        KP1: Kp1
        KROCK00: Krock00
        KRYU00: Kryu00
        KSAKU: Ksaku
        KTARU: Ktaru
        KTARUO: Ktaruo
        KTARUR: Ktarur
        KTARUX: Ktarux
        REFLECTABLE_LIGHT_BEAM_0: LTag0
        REFLECTABLE_LIGHT_BEAM_1: LTag1
        LTAGR0: LTagR0
        LAMP: Lamp
        MKANOK2: MKanok2
        MKANOKE: MKanoke
        MCRTN: Mcrtn
        MCUBE: Mcube
        MCUBE10: Mcube10
        MCYLN: Mcyln
        MFLFT: Mflft
        MHMRSW0: MhmrSW0
        MHSG12: Mhsg12
        MHSG15: Mhsg15
        MHSG4H: Mhsg4h
        MHSG6: Mhsg6
        MHSG9: Mhsg9
        MJDOOR: MjDoor
        MK: Mk
        MKDAN1: Mkdan1
        MKIEBA: MkieBA
        MKIEBAB: MkieBAB
        MKIEBB: MkieBB
        MKIEK: MkieK
        MKNJD: MknjD
        MMRR: Mmrr
        MMUSIC: Mmusic
        MN: Mn
        MORI1: Mori1
        MPWRB: MpwrB
        MSDAN: Msdan
        MSDAN2: Msdan2
        MSUSW: MsuSW
        MSUSWB: MsuSWB
        MSWING: Mswing
        MT: Mt
        MTFLAG: MtFlag
        MTORISU: MtoriSU
        TRIANGULAR_PRISM_BLOCK: MtryB
        TRIANGULAR_PRISM_BLOCK_TARGET_LOCATION: MtryBCr
        MWTRSB: MwtrSB
        MYGNSB: MygnSB
        NBOX: NBOX
        NBOX10: NBOX10
        NH: Nh
        NPCSO: NpcSo
        NZFALL: Nzfall
        OB1: Ob1
        TIMER: ObjTime
        OCANON: Ocanon
        OCLOUD: Ocloud
        OHATCH: Ohatch
        OJTREE: Ojtree
        OKIOKE: Okioke
        OLIFT: Olift
        OQ: Oq
        OQW: Oqw
        OS: Os
        OS1: Os1
        OS2: Os2
        OSHIP: Oship
        OSTOOL: Ostool
        OTANA: Otana
        OTBLE: Otble
        OTBLEL: OtbleL
        OWATER: Owater
        P1A: P1a
        P1B: P1b
        P2A: P2a
        P2B: P2b
        P2C: P2c
        PSCNCHG: PScnChg
        PAPER: Paper
        PBCO: Pbco
        PBKA: Pbka
        PF1: Pf1
        PIRATES: Pirates
        PIWA: Piwa
        PLANT: Plant
        PM1: Pm1
        PO: Po
        PPOS: Ppos
        PTCO: Ptco
        PTCU: Ptcu
        PTUBO: Ptubo
        PUTI: Puti
        QDGHD: Qdghd
        QTKHD: Qtkhd
        QUAKE: Quake
        RCLOUD: Rcloud
        RDEAD2: Rdead2
        RETAG0: ReTag0
        RFLW: Rflw
        RFORCE: Rforce
        ROTEN2: Roten2
        ROTEN3: Roten3
        ROTEN4: Roten4
        SMBDOR: SMBdor
        SMTOGE: SMtoge
        SPITEM: SPitem
        SWTDOOR: SWTdoor
        SWAT00: SWat00
        WIND_WAKER_SONG_SWITCH: SWtact
        CHANDELIER: SYAN
        SA1: Sa1
        SA2: Sa2
        SA3: Sa3
        SA4: Sa4
        SA5: Sa5
        SALVFM: SalvFM
        SALVAG2: Salvag2
        SALVAGE_E: SalvagE
        SALVAGN: SalvagN
        SALVAGE: Salvage
        SARACE: Sarace
        SEARCH: Search
        SFAIRY: Sfairy
        KING_OF_RED_LIONS_SHIP_FORM_PROP: Ship
        SHMRGRD: Shmrgrd
        SIEFLAG: SieFlag
        SITEM: Sitem
        SKANRAN: Skanran
        STDOORL: Stdoorl
        STDOORR: Stdoorr
        STGATE: Stgate
        STOUDAI: Stoudai
        STTOGE: Sttoge
        SV0: Sv0
        SV1: Sv1
        SV2: Sv2
        SV3: Sv3
        SVSP: Svsp
        SALVAGE_SWITCH: SwSlvg
        TABLE: Table
        TAGCB1: TagCb1
        TAGCB11: TagCb11
        TAGCB12: TagCb12
        TAGCB13: TagCb13
        TAGCB14: TagCb14
        TAGD1: TagD1
        TAGD2: TagD2
        TAGD3: TagD3
        TAGD4: TagD4
        TAGDM: TagDM
        TAGISL: TagIsl
        TAGKB: TagKb
        TAGMSO: TagMSo
        TAGMD: TagMd
        TAGMD1: TagMd1
        TAGMD11: TagMd11
        TAGMD12: TagMd12
        TAGMD13: TagMd13
        TAGMD14: TagMd14
        TAGMD15: TagMd15
        TAGMD16: TagMd16
        TAGMK: TagMk
        TAGPO: TagPo
        TAGSO: TagSo
        TAGWP: TagWp
        TENMADO: Tenmado
        TESTPO: TestPo
        LASER_BARRIER_INISIDE_HYRULE_CASTLE: TnTrap
        TPOTA: Tpota
        TRFLAG: TrFlag
        TURU: Turu
        TURU2: Turu2
        TURU3: Turu3
        UB1: Ub1
        UB2: Ub2
        UB3: Ub3
        UB4: Ub4
        UG1: Ug1
        UG2: Ug2
        UM1: Um1
        UM2: Um2
        UM3: Um3
        UO1: Uo1
        UO2: Uo2
        UO3: Uo3
        USOVMC: Usovmc
        UW1: Uw1
        UW2: Uw2
        VBAKH: VbakH
        VDORA: Vdora
        VDS: Vds
        VFAN: Vfan
        PEDASTAL_OF_TIME: VmsDZ
        MASTER_SWORD_MODEL_FROM_HYRULE_CASTLE_BASEMENT: VmsMS
        VOLTAG: VolTag
        VPBOT: Vpbot
        VTENG: Vteng
        VTIL1: Vtil1
        VTIL2: Vtil2
        VTIL3: Vtil3
        VTIL4: Vtil4
        VTIL5: Vtil5
        VYASI: Vyasi
        WLVTAG: WLvTag
        WALL: Wall
        DUNGEON_WARP_EXIT: Warpf
        WARPFO: Warpfo
        WARPGN: Warpgn
        WARPNT: Warpnt
        WARPT: Warpt
        WARP_JAR_1: Warpts1
        WARP_JAR_3: Warpts3
        WIND_COLUMN: WindTag
        YBGAF00: Ybgaf00
        YBOIL00: Yboil00
        MAGICAL_BARRIER: Ycage00
        YFRLT00: Yfrlt00
        YGCWP: Ygcwp
        YGSTP00: Ygstp00
        YGUSH00: Ygush00
        YGUSH01: Ygush01
        YGUSH02: Ygush02
        YKGROFF: YkgrOFF
        YKGRON: YkgrON
        YKZYG: Ykzyg
        YLKIC: Ylkic
        YLLIC: Yllic
        YLSIC: Ylsic
        YM1: Ym1
        YM2: Ym2
        SHAFT_OF_LIGHT_WARP: Ysdls00
        YTRND00: Ytrnd00
        YW1: Yw1
        YWARP00: Ywarp00
        ZK1: Zk1
        AGBA: agbA
        AGBA2: agbA2
        AGBAT: agbAT
        AGBB: agbB
        AGBD: agbD
        AGBF: agbF
        AGBF2: agbF2
        AGBFA: agbFA
        AGBMARK: agbMARK
        AGBMW: agbMW
        AGBR: agbR
        AGBTBOX: agbTBOX
        TORCH: bonbori
        DMGROOM: dmgroom
        DRAGON: dragon
        FLOWER: flower
        FLWR7: flwr7
        FROCK: frock
        GMOS: gmos
        LOWERCASE_HO: ho
        IKADAS: ikadaS
        BEEDLES_SHOPSHIP: ikada_h
        IKADA_U: ikada_u
        KT: kt
        KURO_S: kuro_s
        KURO_T: kuro_t
        KUSAX1: kusax1
        KUSAX21: kusax21
        KUSAX7: kusax7
        KY00YOU: ky00you
        KYTAG00: kytag00
        KYTAG5: kytag5
        MOUSE_HOLE: nezuana
        PFLOWER: pflower
        S_TURU: s_turu
        SEA: sea
        SPEAKUN: speakun
        SPOTBX1: spotbx1
        SWOOD: swood
        SWOOD3: swood3
        WOODB: woodb
        WOODBX: woodbx
        KNIGHT_STATUE: zouK
        KNIGHT_STATUE_1: zouK1
        KNIGHT_STATUE_2: zouK2
        KNIGHT_STATUE_3: zouK3
        KNIGHT_STATUE_4: zouK4
        TRIFORCE_FLAG: HcFlag
        FORSAKEN_FORTRESS_FLAG: MjFlag
        JET_OF_STEAM_0: Ystm0
        JET_OF_STEAM_1: Ystm1
        MAGMA: magma
    }
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
