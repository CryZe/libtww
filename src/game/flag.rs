use Addr;
use system::memory::{ptr, read};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flag(pub Addr, pub u8);

pub const HAS_SEEN_HELMAROC_ARRIVING_AT_OUTSET: Flag = Flag(0x803B872C, 1 << 0);
pub const TALKED_TO_ORCA_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B872C, 1 << 1);
pub const FOREST_OF_FAIRIES_BOKOBLINS_SPAWNED: Flag = Flag(0x803B872C, 1 << 2);
pub const TALKED_TO_ABE_ON_OUTSET_AFTER_HELMAROC: Flag = Flag(0x803B872C, 1 << 3);
pub const TALKED_TO_MESA_ON_OUTSET_WITH_SWORD: Flag = Flag(0x803B872C, 1 << 4);
pub const TALKED_TO_MESA_ON_OUTSET_WITHOUT_SWORD: Flag = Flag(0x803B872C, 1 << 5);
pub const TALKED_TO_ABE_ON_OUTSET_BEFORE_HELMAROC: Flag = Flag(0x803B872C, 1 << 7);
pub const RESCUED_TETRA: Flag = Flag(0x803B872D, 1 << 0);
pub const GOT_A_RUPEE_ON_THE_OUTSET_ROCKS: Flag = Flag(0x803B872D, 1 << 2);
pub const TALKED_TO_ORCA_AFTER_HELMAROC_ARRIVED: Flag = Flag(0x803B872D, 1 << 3);
pub const TALKED_TO_SUE_BELLE_WITH_PIRATES_ON_OUTSET: Flag = Flag(0x803B872D, 1 << 5);
pub const TALKED_TO_SUE_BELLE_ON_OUTSET_AFTER_HELMAROC: Flag = Flag(0x803B872D, 1 << 6);
pub const TALKED_TO_SUE_BELLE_ON_OUTSET_BEFORE_HELMAROC: Flag = Flag(0x803B872D, 1 << 7);
pub const SAW_A_LIGHT_OPERATOR_BOKOBLIN: Flag = Flag(0x803B872E, 1 << 0);
pub const TALKED_TO_JOEL_ON_OUTSET: Flag = Flag(0x803B872E, 1 << 4);
pub const TALKED_TO_ZILL_ON_OUTSET_EAST: Flag = Flag(0x803B872E, 1 << 5);
pub const TALKED_TO_KIDS_WITH_PIRATES_ON_OUTSET: Flag = Flag(0x803B872E, 1 << 6);
pub const SAW_TETRA_IN_FOREST_OF_FAIRIES: Flag = Flag(0x803B872E, 1 << 7);
pub const KILLED_ONE_FOREST_OF_FAIRIES_BOKOBLIN: Flag = Flag(0x803B872F, 1 << 0);
pub const BROUGHT_BACK_1_PIG_TO_ROSE_ON_OUTSET: Flag = Flag(0x803B872F, 1 << 1);
pub const TALKED_TO_ROSE_ON_OUTSET_2: Flag = Flag(0x803B872F, 1 << 2);
pub const TALKED_TO_ROSE_WITH_PIRATES_ON_OUTSET: Flag = Flag(0x803B872F, 1 << 3); // Unsure if actually Rose
pub const PIRATE_SHIP_ARRIVING_ON_OUTSET: Flag = Flag(0x803B872F, 1 << 4);
pub const SAW_LAVA_PLATFORMS_FORMING: Flag = Flag(0x803B872F, 1 << 7);
pub const PICKED_UP_FIRST_BARREL_IN_FF1: Flag = Flag(0x803B8730, 1 << 0); // Don't show Jail Text anymore
pub const GRABBED_FIRST_ROPE_IN_FF1: Flag = Flag(0x803B8730, 1 << 1);
pub const GOT_THROWN_INTO_JAIL_IN_FF1: Flag = Flag(0x803B8730, 1 << 2);
pub const GRAPPLED_VALOOS_TAIL: Flag = Flag(0x803B8730, 1 << 5);
pub const KILLED_BOTH_FOREST_OF_FAIRIES_BOKOBLINS: Flag = Flag(0x803B8730, 1 << 7);
pub const BONKED_ORCAS_WALL: Flag = Flag(0x803B8731, 1 << 0);
pub const VISITED_STURGEON_ON_OUTSET_BEGIN: Flag = Flag(0x803B8731, 1 << 1);
pub const VISITED_STURGEON_ON_OUTSET_END: Flag = Flag(0x803B8731, 1 << 3);
pub const TALKED_TO_STURGEON_ON_OUTSET_BEFORE_HELMAROC: Flag = Flag(0x803B8731, 1 << 4);
pub const GOSSIP_STONE_AT_FF1: Flag = Flag(0x803B8731, 1 << 5);
pub const GRAPPLED_VALOOS_TAIL_2: Flag = Flag(0x803B8731, 1 << 6);
pub const USED_GRAPPLING_HOOK: Flag = Flag(0x803B8731, 1 << 7);
pub const TALKED_TO_GRANDMA_AFTER_SEEING_HELMAROC: Flag = Flag(0x803B8732, 1 << 0);
pub const TALKED_TO_GRANDMA_AFTER_GETTING_SWORD: Flag = Flag(0x803B8732, 1 << 1);
pub const TALKED_TO_OLIVIO_IN_FOREST_HAVEN: Flag = Flag(0x803B8732, 1 << 2);
pub const VISITED_ORCA_BEFORE_HELMAROC: Flag = Flag(0x803B8732, 1 << 6);
pub const TALKED_TO_MAKO_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8733, 1 << 0);
pub const TALKED_TO_ZUKO_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8733, 1 << 1);
pub const TALKED_TO_NIKO_AFTER_MINIGAME: Flag = Flag(0x803B8733, 1 << 2);
pub const COMPLETED_PIRATE_SHIP_MINIGAME: Flag = Flag(0x803B8733, 1 << 4);
pub const SAW_PIRATE_SHIP_MINIGAME_INTRO: Flag = Flag(0x803B8733, 1 << 5);
pub const TALKED_TO_GRANDMA_AFTER_ARYLL_GOT_CAPTURED: Flag = Flag(0x803B8733, 1 << 6);
pub const GOT_CATAPULTED_TO_FF1_AND_SPAWN_THERE: Flag = Flag(0x803B8734, 1 << 0);
pub const LONG_TETRA_TEXT_ON_OUTSET: Flag = Flag(0x803B8734, 1 << 1);
pub const TETRA_TOLD_YOU_TO_CLIMB_UP_THE_LADDER: Flag = Flag(0x803B8734, 1 << 2);
pub const COMPLETED_PIRATE_SHIP_MINIGAME_AND_SPAWN_ON_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 3);
pub const TALKED_TO_TETRA_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 4);
pub const TALKED_TO_NUDGE_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 5);
pub const TALKED_TO_SENZA_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 6);
pub const TALKED_TO_GONZO_ON_SAILING_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 7);
pub const TRIGGERED_MAP_FISH: Flag = Flag(0x803B8735, 1 << 0);
pub const SAW_DRAGON_ROOST_ISLAND_INTRO: Flag = Flag(0x803B8735, 1 << 1);
pub const SAIL_INTRODUCTION_TEXT_AND_MAP_UNLOCKED: Flag = Flag(0x803B8735, 1 << 3);
pub const TALKED_TO_GONZO_ON_OUTSET_BEACH: Flag = Flag(0x803B8735, 1 << 4);
pub const TALKED_TO_ALDO_IN_FOREST_HAVEN: Flag = Flag(0x803B8735, 1 << 5); 
pub const TALKED_TO_NIKO_ON_OUTSET_BEACH: Flag = Flag(0x803B8735, 1 << 6);
pub const TALKED_TO_KORL_AT_GREATFISH: Flag = Flag(0x803B8736, 1 << 0);
pub const ENDLESS_NIGHT: Flag = Flag(0x803B8736, 1 << 1);
pub const TALKED_TO_KORL_AFTER_LEAVING_FH: Flag = Flag(0x803B8736, 1 << 3);
pub const TALKED_TO_KORL_ON_DRI: Flag = Flag(0x803B8736, 1 << 4);
pub const WATCHED_FOREST_HAVEN_INTRO_CUTSCENE: Flag = Flag(0x803B8736, 1 << 5);
pub const TALKED_TO_POMPIE_AND_VERA_ON_WINDFALL: Flag = Flag(0x803B8736, 1 << 6);
pub const KORL_DINS_PEARL_TEXT_ALLOWING_YOU_TO_ENTER_HIM: Flag = Flag(0x803B8736, 1 << 7);
pub const TALKED_TO_ABE_WITH_PIRATES_ON_OUTSET: Flag = Flag(0x803B8737, 1 << 0);
pub const TALKED_POSITIVELY_TO_MILAS_FATHER_ON_WINDFALL: Flag = Flag(0x803B8737, 1 << 1);
pub const TALKED_POSITIVELY_TO_MAGGIES_FATHER_ON_WINDFALL: Flag = Flag(0x803B8737, 1 << 2);
pub const TALKED_TO_TOTT_ON_WINDFALL: Flag = Flag(0x803B8737, 1 << 3);
pub const HURRICANE_SPIN_UNLOCKED: Flag = Flag(0x803B8737, 1 << 5);
pub const TALKED_TO_TINGLE_IN_JAIL: Flag = Flag(0x803B8737, 1 << 6);
pub const RESCUED_TINGLE: Flag = Flag(0x803B8737, 1 << 7);
pub const TALKED_TO_ROWN_IN_FOREST_HAVEN: Flag = Flag(0x803B8738, 1 << 1);
pub const TALKED_TO_IRCH_IN_FOREST_HAVEN: Flag = Flag(0x803B8738, 1 << 2);
pub const TALKED_TO_DRONA_IN_FOREST_HAVEN: Flag = Flag(0x803B8738, 1 << 3);
pub const TALKED_TO_LINDER_IN_FOREST_HAVEN: Flag = Flag(0x803B8738, 1 << 4);
pub const TALKED_TO_OAKIN_IN_FOREST_HAVEN: Flag = Flag(0x803B8738, 1 << 5);
pub const TALKED_TO_HOLLO_IN_STORE_IN_FOREST_HAVEN: Flag = Flag(0x803B8739, 1 << 3);
pub const TALKED_TO_ELMA_OUTSIDE_FOREST_HAVEN: Flag = Flag(0x803B8739, 1 << 6);
pub const SHOW_ORCA_KNIGHTS_CREST: Flag = Flag(0x803B8739, 1 << 7);
pub const WATCHED_LIGHT_BRIDGE_IN_TOTG_APPEAR: Flag = Flag(0x803B873A, 1 << 0);
pub const GOT_KOMALIS_LETTER: Flag = Flag(0x803B873A, 1 << 1);
pub const PLAYED_SPLOOSH_KABOOM_WELL: Flag = Flag(0x803B873A, 1 << 2);
pub const PIRATES_ON_OUTSET: Flag = Flag(0x803B873A, 1 << 5);
pub const TALKED_TO_DEKU_TREE_AFTER_LEAF_CUTSCENE: Flag = Flag(0x803B873A, 1 << 6);
pub const TETRA_GOSSIP_STONE_TEXT_AFTER_BOMBS: Flag = Flag(0x803B873B, 1 << 1);
pub const TALKED_TO_KOMALI_AFTER_SHOWING_THE_LETTER: Flag = Flag(0x803B873B, 1 << 2);
pub const TALKED_TO_KOMALI_WITHOUT_THE_LETTER: Flag = Flag(0x803B873B, 1 << 3);
pub const WATCHED_LIGHT_BRIDGE_IN_TOTG_DISAPPEAR: Flag = Flag(0x803B873B, 1 << 6);
pub const KORL_UNLOCKED_AND_SPAWN_ON_WINDFALL: Flag = Flag(0x803B873B, 1 << 7);
pub const WATCHED_FIRE_AND_ICE_ARROWS_CUTSCENE: Flag = Flag(0x803B873C, 1 << 0);
pub const GOT_GRAPPLING_HOOK_FROM_MEDLI: Flag = Flag(0x803B873D, 1 << 0);
pub const GOT_BOTTLE_FROM_MEDLI: Flag = Flag(0x803B873D, 1 << 1);
pub const TALKED_TO_MEDLI_IN_THE_POND: Flag = Flag(0x803B873D, 1 << 2);
pub const TALKED_TO_DOC_BANDAM_ON_WINDFALL: Flag = Flag(0x803B873D, 1 << 5);
pub const RESCUED_MEDLI_IN_DRC: Flag = Flag(0x803B873D, 1 << 6);
pub const TALKED_TO_JUN_ROBERTO_ON_WINDFALL: Flag = Flag(0x803B873E, 1 << 0);
pub const TALKED_TO_JIN_ON_WINDFALL: Flag = Flag(0x803B873E, 1 << 1);
pub const TALKED_TO_JAN_ON_WINDFALL: Flag = Flag(0x803B873E, 1 << 2);
pub const TALKED_TO_LENZO_ON_WINDFALL_WITHOUT_CAMERA: Flag = Flag(0x803B873E, 1 << 3);
pub const TALKED_TO_IVAN_ON_WINDFALL: Flag = Flag(0x803B873E, 1 << 4);
pub const MEDLI_EXPLAINED_GRAPPLING_HOOK: Flag = Flag(0x803B873E, 1 << 7);
pub const AGREED_TO_HELP_MRS_MARIE: Flag = Flag(0x803B873F, 1 << 7);
pub const TALKED_TO_QUILL_ON_OUTSET_BEACH: Flag = Flag(0x803B8740, 1 << 0);
pub const WATCHED_MEDLI_ON_DRI_INTRODUCTION_CUTSCENE: Flag = Flag(0x803B8740, 1 << 1);
pub const TALKED_TO_NICO_AFTER_GOSIP_TEXT_AFTER_BOMBS: Flag = Flag(0x803B8741, 1 << 1);
pub const SHOWED_MEDLI_THE_WIND_WAKER_ON_DRI: Flag = Flag(0x803B8741, 1 << 2);
pub const TALKED_TO_ALL_THE_KIDS_ON_WINDFALL: Flag = Flag(0x803B8742, 1 << 1);
pub const HAS_MAKAR_ON_BOAT_1: Flag = Flag(0x803B8742, 1 << 2);
pub const HAS_MEDLI_ON_BOAT_1: Flag = Flag(0x803B8742, 1 << 3);
pub const HAS_MAKAR_ON_BOAT_2_AND_IS_GRABBABLE: Flag = Flag(0x803B8742, 1 << 4);
pub const HAS_MEDLI_ON_BOAT_2_AND_CAN_CARRY_YOU: Flag = Flag(0x803B8742, 1 << 5);
pub const TALKED_TO_LENZO_ON_WINDFALL_WITH_CAMERA: Flag = Flag(0x803B8743, 1 << 0);
pub const WATCHED_DEKU_TREE_CUTSCENE: Flag = Flag(0x803B8744, 1 << 0);
pub const TALKED_TO_RITO_LEADER_AFTER_DRC: Flag = Flag(0x803B8744, 1 << 1);
pub const SAW_KOMALI_IN_HIS_ROOM: Flag = Flag(0x803B8744, 1 << 4);
pub const TALKED_TO_MAKAR_POST_KALLE_DEMOS: Flag = Flag(0x803B8745, 1 << 2);
pub const TALKED_TO_KORL_AFTER_HYRULE_2_AND_FF3_ACTIVE: Flag = Flag(0x803B8744, 1 << 5);
pub const ENTERED_THE_PASSWORD_ON_PIRATE_SHIP: Flag = Flag(0x803B8745, 1 << 4);
pub const IS_ALLOWED_TO_ENTER_KORL: Flag = Flag(0x803B8745, 1 << 7);
pub const SAID_YES_TO_MAILBAG_GAME_FIRST_TIME: Flag = Flag(0x803B8746, 1 << 0);
pub const TALKED_TO_NICO_BEFORE_ROPE_GAME_2: Flag = Flag(0x803B8746, 1 << 2);
pub const LEFT_DRAGON_ROOST_ISLAND_QUADRANT: Flag = Flag(0x803B8746, 1 << 7); // Potentially time being able to pass?
pub const TALKED_TO_GOSSACK_ON_WINDFALL: Flag = Flag(0x803B8747, 1 << 2);
pub const WATCHED_FARORES_PEARL_CUTSCENE: Flag = Flag(0x803B8748, 1 << 5);
pub const LEAVING_FOREST_HAVEN_WITH_PEARL: Flag = Flag(0x803B8748, 1 << 6);
pub const TALKED_TO_HOLLO_AFTER_FORBIDDEN_WOODS: Flag = Flag(0x803B8748, 1 << 7);
pub const TALKED_TO_MINENCO_ON_WINDFALL: Flag = Flag(0x803B8749, 1 << 4);
pub const TALKED_TO_MISSY_ON_WINDFALL: Flag = Flag(0x803B8749, 1 << 5);
pub const TALKED_TO_MRS_MARIE_ON_WINDFALL: Flag = Flag(0x803B874A, 1 << 0);
pub const TALKED_TO_GARRICKSON_ON_WINDFALL: Flag = Flag(0x803B874A, 1 << 3);
pub const TALKED_TO_GILLIAN_ON_WINDFALL: Flag = Flag(0x803B874A, 1 << 5);
pub const TOWER_OF_THE_GODS_RAISED: Flag = Flag(0x803B874A, 1 << 6);
pub const QUILL_TELLS_US_JABUN_IS_HIDING_AT_OUTSET: Flag = Flag(0x803B874A, 1 << 7);
pub const TALKED_TO_KORL_AFTER_GETTING_BOMBS: Flag = Flag(0x803B874B, 1 << 1);
pub const WATCHED_WINDFALL_PIRATE_CUTSCENE: Flag = Flag(0x803B874B, 1 << 2);
pub const SAW_QUILL_CUTSCENE_ON_DRI: Flag = Flag(0x803B874B, 1 << 6);
pub const TALKED_TO_LINDA_ON_WINDFALL: Flag = Flag(0x803B874D, 1 << 0);
pub const LISTENED_TO_PIRATES_IN_BOMB_SHOP: Flag = Flag(0x803B874D, 1 << 4);
pub const TALKED_TO_RITO_OUTSIDE_MEDLI_ROOM: Flag = Flag(0x803B874D, 1 << 5); //Check Name
pub const TALKED_TO_ANTON_ON_WINDFALL: Flag = Flag(0x803B874E, 1 << 3);
pub const TALKED_TO_GUMMY_ON_WINDFALL: Flag = Flag(0x803B874F, 1 << 1);
pub const WATCHED_DEPARTURE_CUTSCENE_AND_SPAWN_ON_PIRATE_SHIP: Flag = Flag(0x803B8750, 1 << 0);
pub const TALKED_TO_ZUNARI_ON_WINDFALL: Flag = Flag(0x803B8750, 1 << 5);
pub const TALKED_TO_DAMPA_ON_WINDFALL: Flag = Flag(0x803B8750, 1 << 6);
pub const TALKED_TO_SAM_ON_WINDFALL: Flag = Flag(0x803B8751, 1 << 0);
pub const GOT_DELIVERY_BAG: Flag = Flag(0x803B8751, 1 << 1);
pub const PLAYED_SPLOOSH_KABOOM: Flag = Flag(0x803B8751, 1 << 6);
pub const WATCHED_FIND_SISTER_IN_FF1_CUTSCENE: Flag = Flag(0x803B8751, 1 << 7);
pub const PICKED_UP_STATUE_IN_TOTG_FLOOR_1: Flag = Flag(0x803B8752, 1 << 1);
pub const GOT_NOTE_TO_MOM: Flag = Flag(0x803B8753, 1 << 0);
pub const SAID_YES_TO_BAITO_FIRST_TIME: Flag = Flag(0x803B8753, 1 << 1); //That you were "the one" that was good at sorting mail
pub const TALKED_TO_BAITO_AT_MINIGAME: Flag = Flag(0x803B8753, 1 << 2);
pub const LEARNED_WINDS_REQUIEM: Flag = Flag(0x803B8753, 1 << 3); // Doesn't trigger if you do Zephos Skip
pub const SOME_WEIRD_FIRE_AND_ICE_ARROWS_CUTSCENE_FLAG: Flag = Flag(0x803B8753, 1 << 6);
pub const MADE_MILAS_FATHER_ANGRY: Flag = Flag(0x803B8753, 1 << 7);
pub const TALKED_TO_JOANNA_ON_WINDFALL: Flag = Flag(0x803B8754, 1 << 7);
pub const GOT_LEAF: Flag = Flag(0x803B8755, 1 << 1);
pub const TALKED_TO_MESA_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B8755, 1 << 2);
pub const DENIED_LEAVING_OUTSET: Flag = Flag(0x803B8755, 1 << 3);
pub const MAKAR_IN_WIND_TEMPLE: Flag = Flag(0x803B8755, 1 << 4);
pub const MEDLI_IN_EARTH_TEMPLE: Flag = Flag(0x803B8755, 1 << 5);
pub const ENTERED_KORL_AFTER_GREATFISH: Flag = Flag(0x803B8756, 1 << 0);
pub const ENTERED_KORL_AFTER_FORBIDDEN_WOODS: Flag = Flag(0x803B8756, 1 << 1); // Maybe visit any quadrant?
pub const ENTER_KORL_FOR_THE_FIRST_TIME_AND_SPAWN_ANYWHERE: Flag = Flag(0x803B8756, 1 << 3);
pub const CUT_ALL_ROPES_HOLDING_PLATFORM_IN_DRC: Flag = Flag(0x803B8756, 1 << 4);
pub const GRANDMA_IS_CURED: Flag = Flag(0x803B8756, 1 << 5);
pub const HAS_HEROS_CLOTHES: Flag = Flag(0x803B8756, 1 << 7);
pub const TALKED_TO_POTOVA_ON_WINDFALL: Flag = Flag(0x803B8757, 1 << 3);
pub const DESTROY_VINE_ON_DOOR_IN_FORBIDDEN_WOODS: Flag = Flag(0x803B8757, 1 << 5);
pub const TALKED_TO_KORL_AFTER_FH_CUTSCENE: Flag = Flag(0x803B8757, 1 << 7);
pub const MIGHTY_DARKNUTS_DEFEATED: Flag = Flag(0x803B8758, 1 << 0);
pub const BARRIER_DOWN: Flag = Flag(0x803B8758, 1 << 1);
pub const TALKED_TO_ROSE_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B8758, 1 << 5);
pub const TALKED_TO_ROSE_ON_OUTSET_1: Flag = Flag(0x803B8758, 1 << 7);
pub const ANIMATION_SET_2: Flag = Flag(0x803B8759, 1 << 0);
pub const TETRA_TO_ZELDA_CUTSCENE: Flag = Flag(0x803B8759, 1 << 1);
pub const MASTER_SWORD_CUTSCENE: Flag = Flag(0x803B8759, 1 << 2);
pub const HYRULE_3_WARP_CUTSCENE: Flag = Flag(0x803B8759, 1 << 3);
pub const RINGING_BELL_AND_HYRULE_1_CUTSCENE: Flag = Flag(0x803B8759, 1 << 4);
pub const WIND_GODS_ARIA_CUTSCENE: Flag = Flag(0x803B8759, 1 << 5);
pub const EARTH_GODS_LYRIC_CUTSCENE: Flag = Flag(0x803B8759, 1 << 6);
pub const WATCHED_MEETING_KORL_CUTSCENE: Flag = Flag(0x803B875A, 1 << 0);
pub const MAKAR_IN_WIND_TEMPLE_ENTRANCE: Flag = Flag(0x803B875A, 1 << 1);
pub const MEDLI_IN_EARTH_TEMPLE_ENTRANCE: Flag = Flag(0x803B875A, 1 << 2);
pub const PEARL_TOWER_CUTSCENE: Flag = Flag(0x803B875A, 1 << 7);
pub const DID_SWORD_FIGHTING_TUTORIAL: Flag = Flag(0x803B875B, 1 << 4);
pub const DID_EARLY_SWORD_FIGHTING_TUTORIAL: Flag = Flag(0x803B875B, 1 << 6);
pub const TALKED_TO_GREAT_FAIRY_ON_OUTSET: Flag = Flag(0x803B875C, 1 << 4);
pub const TALKED_TO_ZILL_ON_OUTSET_WEST: Flag = Flag(0x803B875D, 1 << 0);
pub const TALKED_BAITO_OUTSIDE_DRI_AFTER_DRC: Flag = Flag(0x803B875D, 1 << 1);
pub const TALKED_TO_ABE_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B875D, 1 << 6);
pub const SAW_SHIELD_IS_MISSING: Flag = Flag(0x803B875E, 1 << 1);
pub const WATCHED_DESCENDING_DOWN_TO_HYRULE_2_CUTSCENE: Flag = Flag(0x803B875E, 1 << 7);
pub const WATCHED_TEXT_AFTER_FIRE_AND_ICE_ARROWS_CUTSCENE: Flag = Flag(0x803B875F, 1 << 7);
pub const HAS_SEEN_INTRO: Flag = Flag(0x803B8761, 1 << 4);
pub const MIGHTY_DARKNUTS_SPAWNED: Flag = Flag(0x803B8761, 1 << 5);
pub const TALKED_TO_STURGEON_ON_OUTSET_AFTER_HELMAROC: Flag = Flag(0x803B8763, 1 << 2);
pub const TALKED_TO_STURGEON_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B8763, 1 << 3);
pub const COLORS_IN_HYRULE: Flag = Flag(0x803B8764, 1 << 1);
pub const WATCHED_COURTYARD_CUTSCENE: Flag = Flag(0x803B8764, 1 << 2);
pub const ENTERED_HYRULE_1_WARP_AND_LOWERED_ELECTRICAL_BARRIER: Flag = Flag(0x803B8764, 1 << 4);
pub const TALKED_TO_KORL_POST_TOWER_CUTSCENE: Flag = Flag(0x803B8764, 1 << 6);
pub const MOVED_STATUE_IN_HYRULE: Flag = Flag(0x803B8764, 1 << 5);
pub const WATCHED_FF2_GANONDORF_CUTSCENE: Flag = Flag(0x803B8765, 1 << 4);
pub const TALKED_TO_DEKU_TREE_AFTER_FARORES_PEARL_CUTSCENE: Flag = Flag(0x803B8765, 1 << 6);
pub const HYRULE_3_ELECTRICAL_BARRIER_CUTSCENE_1: Flag = Flag(0x803B8765, 1 << 7);
pub const TRIALS_JALHALLA: Flag = Flag(0x803B8765, 1 << 0);
pub const TRIALS_KALLE_DEMOS: Flag = Flag(0x803B8765, 1 << 1);
pub const TRIALS_GOHMA: Flag = Flag(0x803B8765, 1 << 2);
pub const SAW_DRC_BEATEN_CUTSCENE: Flag = Flag(0x803B8765, 1 << 3);
pub const TRIALS_MOLGERA: Flag = Flag(0x803B8766, 1 << 7);
pub const PULLED_MASTER_SWORD_IN_HYRULE_1_SWINGING_CUTSCENE: Flag = Flag(0x803B8766, 1 << 2);
pub const TALKED_TO_SUE_BELLE_WHEN_VISITING_OUTSET_A_SECOND_TIME: Flag = Flag(0x803B8766, 1 << 6);
pub const WATCHED_MEDLI_DRAGON_ROOST_CUTSCENE: Flag = Flag(0x803B8767, 1 << 4);
pub const TALKED_TO_LOCKED_PIRATE_SHIP_DOOR: Flag = Flag(0x803B8767, 1 << 5);
pub const HYRULE_1_ELECTRICAL_BARRIER_DEACTIVATED: Flag = Flag(0x803B8767, 1 << 6);
pub const FF3_TO_HYRULE_WARP_ACTIVE: Flag = Flag(0x803B8769, 1 << 1);
pub const DEFEATED_KALLE_DEMOS: Flag = Flag(0x803B8769, 1 << 6);
pub const DEFEATED_GOHMA: Flag = Flag(0x803B8769, 1 << 7);
pub const ENTER_GRANDMAS_ROOM_DURRING_ENDLESS_NIGHT: Flag = Flag(0x803B876A, 1 << 0);
pub const ARRIVING_AT_OUTSET_AFTER_BOMBS: Flag = Flag(0x803B876A, 1 << 4);
pub const DONT_SHOW_WEAPONS: Flag = Flag(0x803B876B, 1 << 6);
pub const TALKED_TO_KORL_LEAVING_OUTSET_BEFORE_NAYRUS_PEARL: Flag = Flag(0x803B876B, 1 << 7);

impl Flag {
    pub fn activate(self) {
        let Flag(addr, value) = self;
        let ptr = ptr::<u8>(addr);
        unsafe {
            *ptr |= value;
        }
    }

    pub fn deactivate(self) {
        let Flag(addr, value) = self;
        let ptr = ptr::<u8>(addr);
        unsafe {
            *ptr &= 0xFF ^ value;
        }
    }

    pub fn is_active(self) -> bool {
        let Flag(addr, mask) = self;
        let value = read::<u8>(addr);
        value & mask != 0
    }

    pub fn toggle(self) {
        if self.is_active() {
            self.deactivate()
        } else {
            self.activate()
        }
    }
}
