use Addr;
use system::memory::{ptr, read};

#[derive(Copy, Clone)]
pub struct Flag(pub Addr, pub u8);

pub const HAS_SEEN_HELMAROC_ARRIVING_AT_OUTSET: Flag = Flag(0x803B872C, 1 << 0);
pub const FOREST_OF_FAIRIES_BOKOBLINS_SPAWNED: Flag = Flag(0x803B872C, 1 << 2);
pub const TALKED_TO_FARM_GUY_ON_OUTSET: Flag = Flag(0x803B872C, 1 << 4);
pub const RESCUED_TETRA: Flag = Flag(0x803B872D, 1 << 0);
pub const GOT_A_RUPEE_ON_THE_OUTSET_ROCKS: Flag = Flag(0x803B872D, 1 << 2);
pub const TALKED_TO_ORCA_AFTER_HELMAROC_ARRIVED: Flag = Flag(0x803B872D, 1 << 3);
pub const SAW_A_LIGHT_OPERATOR_BOKOBLIN: Flag = Flag(0x803B872E, 1 << 0);
pub const TALKED_TO_SNOT_KID: Flag = Flag(0x803B872E, 1 << 5);
pub const SAW_TETRA_IN_FOREST_OF_FAIRIES: Flag = Flag(0x803B872E, 1 << 7);
pub const KILLED_ONE_FOREST_OF_FAIRIES_BOKOBLIN: Flag = Flag(0x803B872F, 1 << 0);
pub const PIRATE_SHIP_ARRIVING_ON_OUTSET: Flag = Flag(0x803B872F, 1 << 4);
pub const PICKED_UP_FIRST_BARREL_IN_FF1: Flag = Flag(0x803B8730, 1 << 0);
pub const GRABBED_FIRST_ROPE_IN_FF1: Flag = Flag(0x803B8730, 1 << 1);
pub const GOT_THROWN_INTO_JAIL_IN_FF1: Flag = Flag(0x803B8730, 1 << 2);
pub const KILLED_BOTH_FOREST_OF_FAIRIES_BOKOBLINS: Flag = Flag(0x803B8730, 1 << 7);
pub const GOSSIP_STONE_AT_FF1: Flag = Flag(0x803B8731, 1 << 5);
pub const COMPLETED_PIRATE_SHIP_MINIGAME: Flag = Flag(0x803B8733, 1 << 4);
pub const SAW_PIRATE_SHIP_MINIGAME_INTRO: Flag = Flag(0x803B8733, 1 << 5);
pub const GOT_CATAPULTED_TO_FF1_AND_SPAWN_THERE: Flag = Flag(0x803B8734, 1 << 0);
pub const LONG_TETRA_TEXT_ON_OUTSET: Flag = Flag(0x803B8734, 1 << 1);
pub const TETRA_TOLD_YOU_TO_CLIMB_UP_THE_LADDER: Flag = Flag(0x803B8734, 1 << 2);
pub const COMPLETED_PIRATE_SHIP_MINIGAME_AND_SPAWN_ON_PIRATE_SHIP: Flag = Flag(0x803B8734, 1 << 3);
pub const SAIL_INTRODUCTION_TEXT_AND_MAP_UNLOCKED: Flag = Flag(0x803B8735, 1 << 3);
pub const ENDLESS_NIGHT: Flag = Flag(0x803B8736, 1 << 1);
pub const KORL_DINS_PEARL_TEXT_ALLOWING_YOU_TO_ENTER_HIM: Flag = Flag(0x803B8736, 1 << 7);
pub const HURRICANE_SPIN_UNLOCKED: Flag = Flag(0x803B8737, 1 << 5);
pub const PIRATES_ON_OUTSET: Flag = Flag(0x803B873A, 1 << 5);
pub const KORL_UNLOCKED_AND_SPAWN_ON_WINDFALL: Flag = Flag(0x803B873B, 1 << 7);
pub const WATCHED_FIRE_AND_ICE_ARROWS_CUTSCENE: Flag = Flag(0x803B873C, 1 << 0);
pub const MEDLI_ON_DRI_INTRODUCTION_CUTSCENE_WATCHED: Flag = Flag(0x803B8740, 1 << 1);
pub const SHOWED_MEDLI_THE_WIND_WAKER_ON_DRI: Flag = Flag(0x803B8741, 1 << 2);
pub const HAS_MAKAR_ON_BOAT_1: Flag = Flag(0x803B8742, 1 << 2);
pub const HAS_MEDLI_ON_BOAT_1: Flag = Flag(0x803B8742, 1 << 3);
pub const HAS_MAKAR_ON_BOAT_2_AND_IS_GRABABLE: Flag = Flag(0x803B8742, 1 << 4);
pub const HAS_MEDLI_ON_BOAT_2_AND_CAN_CARRY_YOU: Flag = Flag(0x803B8742, 1 << 5);
pub const TALKED_TO_KORL_AFTER_HYRULE_2_AND_FF3_ACTIVE: Flag = Flag(0x803B8744, 1 << 5);
pub const IS_ALLOWED_TO_ENTER_KORL: Flag = Flag(0x803B8745, 1 << 7);
pub const TOWER_OF_THE_GODS_RAISED: Flag = Flag(0x803B874A, 1 << 6);
pub const WATCHED_DEPARTURE_CUTSCENE_AND_SPAWN_ON_PIRATE_SHIP: Flag = Flag(0x803B8750, 1 << 0);
pub const TALKED_TO_SAIL_SHOP_NPC: Flag = Flag(0x803B8750, 1 << 5);
pub const WATCHED_FIND_SISTER_IN_FF1_CUTSCENE: Flag = Flag(0x803B8751, 1 << 7);
pub const SOME_WEIRD_FIRE_AND_ICE_ARROWS_CUTSCENE_FLAG: Flag = Flag(0x803B8753, 1 << 6);
pub const MAKAR_IN_WIND_TEMPLE: Flag = Flag(0x803B8755, 1 << 4);
pub const MEDLI_IN_EARTH_TEMPLE: Flag = Flag(0x803B8755, 1 << 5);
pub const ENTER_KORL_FOR_THE_FIRST_TIME_AND_SPAWN_ANYWHERE: Flag = Flag(0x803B8756, 1 << 3);
pub const HAS_HEROS_CLOTHES: Flag = Flag(0x803B8756, 1 << 7);
pub const MIGHTY_DARKNUTS_DEFEATED: Flag = Flag(0x803B8758, 1 << 0);
pub const BARRIER_DOWN: Flag = Flag(0x803B8758, 1 << 1);
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
pub const GOT_SHIELD_FROM_GRANDMA: Flag = Flag(0x803B875E, 1 << 1);
pub const WATCHED_DESCENDING_DOWN_TO_HYRULE_2_CUTSCENE: Flag = Flag(0x803B875E, 1 << 7);
pub const WATCHED_TEXT_AFTER_FIRE_AND_ICE_ARROWS_CUTSCENE: Flag = Flag(0x803B875F, 1 << 7);
pub const HAS_SEEN_INTRO: Flag = Flag(0x803B8761, 1 << 4);
pub const MIGHTY_DARKNUTS_SPAWNED: Flag = Flag(0x803B8761, 1 << 5);
pub const COLORS_IN_HYRULE: Flag = Flag(0x803B8764, 1 << 1);
pub const WATCHED_COURTYARD_CUTSCENE: Flag = Flag(0x803B8764, 1 << 2);
pub const MOVED_STATUE_IN_HYRULE: Flag = Flag(0x803B8764, 1 << 5);
pub const WATCHED_FF2_GANONDORF_CUTSCENE: Flag = Flag(0x803B8765, 1 << 4);
pub const HYRULE_3_ELECTRICAL_BARRIER_CUTSCENE_1: Flag = Flag(0x803B8765, 1 << 7);
pub const TRIALS_JALHALLA: Flag = Flag(0x803B8765, 1 << 0);
pub const TRIALS_KALLE_DEMOS: Flag = Flag(0x803B8765, 1 << 1);
pub const TRIALS_GOHMA: Flag = Flag(0x803B8765, 1 << 2);
pub const TRIALS_MOLGERA: Flag = Flag(0x803B8766, 1 << 7);
pub const PULLED_MASTER_SWORD_IN_HYRULE_1_SWINGING_CUTSCENE: Flag = Flag(0x803B8766, 1 << 2);
pub const MEDLI_DRAGON_ROOST_CUTSCENE_WATCHED: Flag = Flag(0x803B8767, 1 << 4);
pub const HYRULE_1_ELECTRICAL_BARRIER_DEACTIVATED: Flag = Flag(0x803B8767, 1 << 6);
pub const FF3_TO_HYRULE_WARP_ACTIVE: Flag = Flag(0x803B8769, 1 << 1);
pub const DONT_SHOW_WEAPONS: Flag = Flag(0x803B876B, 1 << 6);

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
