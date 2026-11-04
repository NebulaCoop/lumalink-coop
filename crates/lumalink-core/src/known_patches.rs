//! Code patches with a native version. A Riivolution memory patch changes the Wii program, which Nebula does not have, so
//! it cannot be carried over byte for byte. Those listed here were studied and have a switch in the native game (Nebula's
//! `NEBULA_PATCH_*`): when a mod's bytes match, the package turns the switch on.
//!
//! To add one: note the mod's region, address and bytes, add the native switch to Nebula, and add a row.

use serde::Serialize;

/// The native switches, numbered as `nebula_game_api.h` numbers them. 0: nothing to do, Nebula covers it by itself.
pub mod id {
    pub const NATIVE_ALREADY: u32 = 0;
    pub const FLIP_PANEL_GROUP: u32 = 1;
    pub const FLIP_PANEL_OBSERVER_GROUP: u32 = 2;
    pub const WATER_BAZOOKA_SHOT_FLAG: u32 = 3;
    pub const KAMECK_BARRIER_OFFSET_Y: u32 = 4;
    pub const SKELETAL_FISH_BABY_NO_BUBBLE: u32 = 5;
    pub const DODORYU_SHELL_ONLY: u32 = 6;
    pub const MISS_STAR_PIECES: u32 = 7;
    pub const NO_HIDDEN_STAR_TABLE: u32 = 8;
    pub const NO_SUDDEN_DEATH_DODORYU: u32 = 9;
    pub const BEGOMAN_FIRE_PROOF: u32 = 10;
    pub const MESSENGER_ALWAYS_EX: u32 = 11;
    pub const MESSENGER_EX_ONE_UPS: u32 = 12;
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct KnownPatch {
    pub address: u32,
    pub value: &'static str,
    pub id: u32,
    pub parameter: u32,
    pub what: &'static str,
}

use id::*;

const fn row(address: u32, value: &'static str, id: u32, parameter: u32, what: &'static str) -> KnownPatch {
    KnownPatch { address, value, id, parameter, what }
}

pub const KNOWN: &[KnownPatch] = &[
    row(0x801DD690, "38C00080", FLIP_PANEL_OBSERVER_GROUP, 0x80, "flip panel observer groups of 128"),
    row(0x801DCE48, "38C00080", FLIP_PANEL_GROUP, 0x80, "flip panel groups of 128"),
    row(0x80153D84, "39000001", WATER_BAZOOKA_SHOT_FLAG, 1, "water bazooka shots' flag"),
    row(0x80001800, "C4BB8000C03E00103C608000C0031800EC00082AD01E0010809E008C4E800020", KAMECK_BARRIER_OFFSET_Y, 0xC4BB8000, "Kamek's barrier 1500 lower"),
    row(0x800382B8, "4BFC954D", KAMECK_BARRIER_OFFSET_Y, 0xC4BB8000, "Kamek's barrier 1500 lower"),
    row(0x8007679C, "60000000", SKELETAL_FISH_BABY_NO_BUBBLE, 1, "no air bubble from hit skeletal fish"),
    row(0x80054490, "4836F0A9", DODORYU_SHELL_ONLY, 1, "Dodoryu hurt by shells only"),
    row(0x800564C4, "4836D075", DODORYU_SHELL_ONLY, 1, "Dodoryu hurt by shells only"),
    row(0x803F82D8, "4BC0A058", MISS_STAR_PIECES, 0xFFFFFC19, "every star bit lost after a miss"),
    row(0x80002330, "3860FC19483ADFB980010034483F5FA0", MISS_STAR_PIECES, 0xFFFFFC19, "every star bit lost after a miss"),
    row(0x803F6298, "38600000", NO_HIDDEN_STAR_TABLE, 1, "hidden stars without their table"),
    row(0x803F5D40, "386000004E800020", NO_SUDDEN_DEATH_DODORYU, 1, "no sudden-death Dodoryu"),
    row(0x800D02C0, "38600000", BEGOMAN_FIRE_PROOF, 1, "Begomans survive fire ground"),
    row(0x800D5DD8, "38600000", BEGOMAN_FIRE_PROOF, 1, "Begomans survive fire ground"),
    row(0x800D7D50, "38600000", BEGOMAN_FIRE_PROOF, 1, "Begomans survive fire ground"),
    row(0x80275440, "60000000", MESSENGER_ALWAYS_EX, 1, "the 1-up messenger's larger supply"),
    row(0x80275458, "38800063", MESSENGER_EX_ONE_UPS, 99, "99 1-ups from the messenger"),
    row(0x802750F0, "2C1F0063", MESSENGER_EX_ONE_UPS, 99, "99 1-ups from the messenger"),
    row(0x802750FC, "38800063", MESSENGER_EX_ONE_UPS, 99, "99 1-ups from the messenger"),
    row(0x8039AA2C, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x8039AAF0, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x804A432C, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x8039AA48, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x8039AB0C, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x804A430C, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x8039BF80, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x8039C044, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
    row(0x804A656C, "60000000", NATIVE_ALREADY, 0, "the console's crash debugger (Nebula reports crashes itself)"),
];

/// The native version of a patch's bytes, if there is one. Nebula's switches don't depend on the version of the game; the
/// table has the addresses mods use.
pub fn find(address: u32, value: &[u8]) -> Option<&'static KnownPatch> {
    KNOWN.iter().find(|k| k.address == (address | 0x8000_0000) && crate::riivolution::hex_bytes(k.value) == value)
}

/// A switch's name in `luma.json`.
pub fn switch_name(switch: u32) -> Option<&'static str> {
    use id::*;
    Some(match switch {
        FLIP_PANEL_GROUP => "flip_panel_group",
        FLIP_PANEL_OBSERVER_GROUP => "flip_panel_observer_group",
        WATER_BAZOOKA_SHOT_FLAG => "water_bazooka_shot_flag",
        KAMECK_BARRIER_OFFSET_Y => "kamek_barrier_offset_y",
        SKELETAL_FISH_BABY_NO_BUBBLE => "skeletal_fish_baby_no_bubble",
        DODORYU_SHELL_ONLY => "dodoryu_shell_only",
        MISS_STAR_PIECES => "miss_star_pieces",
        NO_HIDDEN_STAR_TABLE => "no_hidden_star_table",
        NO_SUDDEN_DEATH_DODORYU => "no_sudden_death_dodoryu",
        BEGOMAN_FIRE_PROOF => "begoman_fire_proof",
        MESSENGER_ALWAYS_EX => "messenger_always_ex",
        MESSENGER_EX_ONE_UPS => "messenger_ex_one_ups",
        _ => return None,
    })
}

/// Every switch a `luma.json` may name.
pub fn switch_names() -> impl Iterator<Item = &'static str> {
    (1..=12).filter_map(switch_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_patch_is_known_by_region_address_and_bytes() {
        assert_eq!(find(0x8039AA2C, &[0x60, 0, 0, 0]).unwrap().id, 0);
        assert_eq!(find(0x0007679C, &[0x60, 0, 0, 0]).unwrap().id, SKELETAL_FISH_BABY_NO_BUBBLE);
        assert!(find(0x8007679C, &[0x60, 0, 0, 1]).is_none());
    }

    #[test]
    fn every_row_has_bytes() {
        assert!(KNOWN.iter().all(|k| !crate::riivolution::hex_bytes(k.value).is_empty()));
    }
}

/// Tables a mod can patch in the Wii program, which Nebula keeps as data. Their places, and the names their rows point at.
pub const GALAXY_TABLE: u32 = 0x8053_C800;
pub const EVENT_FLAG_TABLE: u32 = 0x8053_BC94;
const COMET_TABLES: [(u32, u32); 4] = [(0x8053_AF78, 2), (0x8053_B000, 3), (0x8053_B088, 4), (0x8053_B150, 5)];
const NAMES: &[(u32, &str)] = &[
    (0x805C9844, "StarDustGalaxy"),
    (0x805C9853, "BattleShipGalaxy"),
    (0x805C9864, "PhantomGalaxy"),
    (0x805C9872, "HeavenlyBeachGalaxy"),
    (0x805C9886, "SandClockGalaxy"),
    (0x805C9896, "CosmosGardenGalaxy"),
    (0x805C98A9, "IceVolcanoGalaxy"),
    (0x805C98BA, "OceanRingGalaxy"),
    (0x805C98CA, "ReverseKingdomGalaxy"),
    (0x805C98DF, "FactoryGalaxy"),
    (0x805C9D97, "HeavenlyBeachGalaxy"),
    (0x805C9E87, "TearDropGalaxy"),
    (0x805CA01E, "HeavensDoorGalaxy"),
    (0x805CA03E, "BeltConveyerExGalaxy"),
    (0x805CA063, "StarDustGalaxy"),
    (0x805CA083, "CocoonExGalaxy"),
    (0x805CA0A7, "PhantomGalaxy"),
    (0x805CA0B5, "TeresaMario2DGalaxy"),
    (0x805CA102, "SandClockGalaxy"),
    (0x805CA112, "SnowCapsuleGalaxy"),
    (0x805CA14E, "OceanRingGalaxy"),
    (0x805CA15E, "TransformationExGalaxy"),
    (0x805CA1B3, "HellProminenceGalaxy"),
    (0x805CA1C8, "FishTunnelGalaxy"),
    (0x805CA60A, "SpecialStarGrand1"),
    (0x805CA61C, "SpecialStarGrand2"),
    (0x805CA62E, "SpecialStarGrand3"),
    (0x805CA640, "SpecialStarGrand4"),
    (0x805CA652, "SpecialStarGrand5"),
    (0x805CA664, "SpecialStarGrand6"),
    (0x805CA747, "ExOpen1CocoonExGalaxy"),
    (0x805CA75D, "ExOpen1TearDropGalaxy"),
    (0x805CA773, "ExOpen1FishTunnelGalaxy"),
    (0x805CA78B, "ExOpenCommonBase"),
    (0x805CA79C, "ExOpenSubTransformationExGalaxy"),
    (0x805CA7BC, "ExOpenSubTeresaMario2DGalaxy"),
    (0x805CA7D9, "ExOpen1TransformationExGalaxy"),
    (0x805CA7F7, "ExOpen1TeresaMario2DGalaxy"),
    (0x805CA812, "ExOpen1SnowCapsuleGalaxy"),
    (0x805CA82B, "ExOpen1BeltConveyerExGalaxy"),
    (0x805CA847, "AppearBeltConveyerExGalaxy"),
    (0x805CA862, "AppearCocoonExGalaxy"),
    (0x805CA877, "AppearTearDropGalaxy"),
    (0x805CA88C, "AppearFishTunnelGalaxy"),
    (0x805CA8A3, "AppearTransformationExGalaxy"),
    (0x805CA8C0, "AppearTeresaMario2DGalaxy"),
    (0x805CA8DA, "AppearSnowCapsuleGalaxy"),
    (0x805CA8F2, "StarPieceBeltConveyerExGalaxy"),
    (0x805CA910, "StarPieceCocoonExGalaxy"),
    (0x805CA928, "StarPieceTearDropGalaxy"),
    (0x805CA940, "StarPieceFishTunnelGalaxy"),
    (0x805CA95A, "StarPieceTransformationExGalaxy"),
    (0x805CA97A, "StarPieceTeresaMario2DGalaxy"),
    (0x805CA997, "StarPieceSnowCapsuleGalaxy"),
];

fn name_at(pointer: u32) -> Option<String> {
    NAMES.iter().find(|(at, _)| *at == pointer).map(|(_, name)| (*name).to_string())
}

fn word(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// A row of the game's progression.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventFlagRow {
    pub name: String,
    pub kind: u8,
    pub a: u8,
    pub b: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<String>,
}

/// One comet of a dome's schedule.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CometRow {
    pub galaxy: String,
    pub coins: u32,
}

/// The dome a patched comet table belongs to, and its rows: a name and a kind, until a row of nothing.
pub fn comet_table(address: u32, value: &[u8]) -> Option<(u32, Vec<CometRow>)> {
    let dome = COMET_TABLES.iter().find(|(at, _)| *at == (address | 0x8000_0000))?.1;
    let mut rows = Vec::new();
    for at in (0..value.len()).step_by(8) {
        let pointer = word(value, at)?;
        if pointer == 0 {
            return Some((dome, rows));
        }
        rows.push(CometRow { galaxy: name_at(pointer)?, coins: word(value, at + 4)? });
    }
    None
}

/// The game's table of what opens what, patched whole: twenty bytes a row.
pub fn event_flag_table(bytes: &[u8]) -> Option<Vec<EventFlagRow>> {
    if bytes.is_empty() || bytes.len() % 20 != 0 {
        return None;
    }
    let optional = |pointer: u32| if pointer == 0 { Some(None) } else { name_at(pointer).map(Some) };
    bytes
        .chunks_exact(20)
        .map(|row| {
            Some(EventFlagRow {
                name: name_at(word(row, 0)?)?,
                kind: row[4],
                a: row[6],
                b: row[7],
                first: optional(word(row, 12)?)?,
                second: optional(word(row, 16)?)?,
            })
        })
        .collect()
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn a_comet_table_reads_by_name_and_a_strange_one_does_not() {
        let mut value = Vec::new();
        value.extend_from_slice(&0x805C_9853u32.to_be_bytes());
        value.extend_from_slice(&1u32.to_be_bytes());
        value.extend_from_slice(&[0; 8]);
        let (dome, rows) = comet_table(0x8053_AF78, &value).unwrap();
        assert_eq!(dome, 2);
        assert_eq!(rows, vec![CometRow { galaxy: "BattleShipGalaxy".to_string(), coins: 1 }]);
        assert!(comet_table(0x8053_0000, &value).is_none());
        value[0] = 0x12;
        assert!(comet_table(0x8053_AF78, &value).is_none());
    }

    #[test]
    fn a_flag_row_keeps_its_kind_and_numbers() {
        let mut row = Vec::new();
        row.extend_from_slice(&0x805C_A79Cu32.to_be_bytes());
        row.extend_from_slice(&[3, 1, 2, 40]);
        row.extend_from_slice(&[0; 4]);
        row.extend_from_slice(&0x805C_9853u32.to_be_bytes());
        row.extend_from_slice(&[0; 4]);
        let rows = event_flag_table(&row).unwrap();
        assert_eq!(rows[0].kind, 3);
        assert_eq!((rows[0].a, rows[0].b), (2, 40));
        assert_eq!(rows[0].first.as_deref(), Some("BattleShipGalaxy"));
        assert_eq!(rows[0].second, None);
        assert!(event_flag_table(&row[..19]).is_none());
    }
}
