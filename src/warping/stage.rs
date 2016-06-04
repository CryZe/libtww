macro_rules! stages {
    ($($group:ident { $($stage:ident: $value:ident)* })*) => {
        $(
            pub mod $group {
                $(
                    pub const $stage: &'static str = stringify!($value);
                )*
            }
        )*
    }
}

stages! {
    nintendo_gallery {
        GREAT_SEA: figureA
        WINDFALL_ISLAND: figureB
        OUTSET_ISLAND: figureC
        FORSAKEN_FORTRESS: figureD
        SECRET_CAVERN: figureE
        DRAGON_ROOST_ISLAND: figureF
        FOREST_HAVEN: figureG
        MAIN_ROOM: Pfigure
    }

    savage_labyrinth {
        ENTRANCE: Cave09
        ROOM11: Cave10
        ROOM32: Cave11
        END: Cave06
    }

    dev {
        INVISIBLE_ISLAND: A_nami // Still in TWW HD
        E3_FOREST: A_R00
        AMOS_T: Amos_T
        WIND_TEMPLE: Cave08 // Still in TWW HD (crashes)
        OUTSET_ISLAND: DmSpot0 // Still in TWW HD
        E3_BOATING_COURSE: E3ROOP // Still in TWW HD (black screen)
        ISLAND_WITH_HOUSE: Ebesso // Still in TWW HD
        GANONS_TOWER: GanonC // Still in TWW HD (crashes)
        PIG_CHAMBER: H_test
        FIRE_CAVERN_WITH_SWITCHES: ITest61 // Still in TWW HD
        ICE_RING_ISLE_CAVERN: ITest62 // Still in TWW HD
        FIRE_MOUNTAIN_CAVERN: I_SubAN
        BASIC_ACTIONS: I_TestM
        ROPE_ROOM: I_TestR
        BRIDGE_ROOM: KATA_HB
        LARGE_EMPTY_ROOM: KATA_RM
        FIRE_MOUNTAIN: kazan // Still in TWW HD
        K_TEST2: K_Test2
        K_TEST3: K_Test3
        K_TEST4: K_Test4
        K_TEST5: K_Test5
        K_TEST6: K_Test6
        K_TEST8: K_Test8
        K_TEST9: K_Test9
        K_TESTA: K_Testa
        K_TESTB: K_Testb
        K_TESTC: K_Testc
        K_TESTD: K_Testd
        K_TESTE: K_Teste
        CAMERA_TEST: morocam
        SMOKE_TEST_ROOM: Msmoke // Still in TWW HD (only Room 0 works)
        HEADSTONE_ISLAND: Mukao // Still in TWW HD
        DEV_ENDING: ENDumi // Still in TWW HD
        GHOST_SHIP_1: PShip2 // Still in TWW HD
        GHOST_SHIP_2: PShip3 // Still in TWW HD
        GHOST_SHIP_3: SubD45
        SHIP_CONTROL_TEST: sea_E // Still in TWW HD (can't warp to?)
        STONE_WATCHER_ISLAND_CAVERN: SubD44 // Still in TWW HD
        BOMB_ISLAND_CAVERN: SubD51 // Still in TWW HD
        DECORATIVE_PEDESTALS: TEST
        DARK_CAVERN_WITH_SWITCHES: TF_05
        GROTTO_WITH_DARKNUTS: TF_07
        TINGLES_ROOM: tincle // Still in TWW HD
        BASIC_ISLAND: VrTest
    }

    great_fairy {
        NORTH: Fairy01
        EAST: Fairy02
        WEST: Fairy03
        FOREST_OF_FAIRIES: Fairy04
        THORNED: Fairy05
        SOUTH: Fairy06
    }

    ganons_tower {
        ENTRANCE: GanonA
        ROOM_TOWARDS_GOHMA: GanonB
        ROOM_TOWARDS_MOLGERA: GanonC
        ROOM_TOWARDS_KALLE_DEMOS: GanonD
        ROOM_TOWARDS_JALHALLA: GanonE
        PHANTOM_GANONS_MAZE: GanonJ
        PUPPET_GANON: GanonK
        STAIRCASE_TOWARDS_PUPPET_GANON: GanonL
        MAIN_ROOM: GanonM
        STAIRCASE_TO_MAIN_ROOM: GanonN
        TOWER: GTower
        GOHMA: Xboss0
        KALLE_DEMOS: Xboss1
        JALHALLA: Xboss2
        MOLGERA: Xboss3
    }

    hyrule {
        CASTLE: Hyroom
        FIELD: Hyrule
        MASTER_SWORD_CHAMBER: kenroom
    }

    cavern {
        BOMB_ISLAND: Cave01
        STAR_ISLAND: Cave02
        CLIFF_PLATEAU_ISLES: Cave03
        ROCK_SPIRE_ISLE: Cave04
        HORSESHOE_ISLAND: Cave05
        PAWPRINT_ISLE_WIZZROBE: Cave07
        SHARK_ISLAND: ITest63
        ICE_RING_ISLE: MiniHyo
        FIRE_MOUNTAIN: MiniKaz
        NEEDLE_ROCK_ISLE: SubD42
        ANGULAR_ISLES: SubD43
        BOATING_COURSE: SubD71
        STONE_WATCHER_ISLAND: TF_01
        OVERLOOK_ISLAND: TF_02
        BIRDS_PEAK_ROCK: TF_03
        CABANA: TF_04
        DRAGON_ROOST_ISLAND: TF_06
        PAWPRINT_ISLE_CHUCHU: TyuTyu
        DIAMOND_STEPPE_ISLAND: WarpD
    }

    windfall {
        GAME_ROOM: Kaisen
        SCHOOL_OF_JOY: Nitiyou
        BOMB_SHOP: Obombh
        LENZOS_HOUSE: Ocmera
        CAFE_BAR: Opub
        HOUSE_OF_WEALTH: Orichh
        CHU_JELLY_JUICE_SHOP: Pdrgsh
        JAIL: Pnezumi
    }

    earth_temple {
        ENTRANCE: Edaichi
        TEMPLE: M_Dai
        BOSS: M_DaiB
        MINI_BOSS: M_DaiMB
    }

    wind_temple {
        ENTRANCE: Ekaze
        TEMPLE: kaze
        BOSS: kazeB
        MINI_BOSS: kazeMB
    }

    forbidden_woods {
        BOSS: kinBOSS
        DUNGEON: kindan
        MINI_BOSS: kinMB
    }

    outset {
        LINKS_HOUSE: LinkRM
        UNDER_LINKS_HOUSE: LinkUG
        FOREST_OF_FAIRIES: A_mori
        ORCAS_ROOM: Ojhous
        STURGEONS_ROOM: Ojhous2
        MESAS_HOUSE: Omasao
        ABE_AND_ROSES_HOUSE: Onobuta
        JABUNS_ROOM: Pjavdou
    }

    forsaken_fortress {
        GANONDORFS_ROOM: M2ganon
        FF1_TOWER: Mjtower
        FF2_TOWER: M2tower
        FF1_INTERIOR: majroom
        FF2_INTERIOR: ma2room
        FF3_INTERIOR: ma3room
        FF1: MajyuE
    }

    dragon_roost_cavern {
        DUNGEON: M_NewD2
        BOSS: M_DragB
        MINI_BOSS: M_Dra09
    }

    dragon_roost_island {
        POND: Adanmae
        KOMALIS_ROOM: Comori
        POSTAL_SERVICE: Atorizk
    }

    forest_haven {
        POTION_ROOM: Ocrogh
        FOREST_HAVEN: Omori
        MAKARS_HIDING_PLACE: Otkura
    }

    sea {
        SEA: sea
        TETRAS_SHIP_INSIDE: Asoko
        TETRAS_SHIP_OUTSIDE: A_umikz
        SUBMARINE_FIVE_STAR_ISLES: Abship
        CABANA: Abesso
        BOATING_COURSE: Ocean
        GHOST_SHIP: PShip
        ISLET_OF_STEEL: ShipD
        BEEDLES_SHOP_SHIP: Obshop
    }

    other {
        NAME_SELECT: Name
        ENDING: ENDING
        TITLE_SCREEN: sea_T
    }

    tower_of_the_gods {
        DUNGEON: Siren
        BOSS: SirenB
        MINI_BOSS: SirenMB
        OUTSIDE: ADMumi
    }
}
