//! Looking inside the game and inside files: what a modder needs to know the names of.

use anyhow::{bail, Context, Result};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// A column's hash, as the game's tables have it.
fn column_hash(name: &str) -> u32 {
    name.bytes().fold(0u32, |hash, byte| hash.wrapping_mul(31).wrapping_add(u32::from(byte)))
}

/// The names of columns: tables keep only their hashes. A hash not among these comes out as `0x` and its hash, which reads back the same.
fn column_names() -> BTreeMap<u32, &'static str> {
    crate::column_names::NAMES.iter().map(|n| (column_hash(n), *n)).collect()
}

/// A table of the game, disc or cooked, as the JSON a mod writes.
pub fn table_to_json(bytes: &[u8]) -> Result<Value> {
    if bytes.len() < 16 {
        bail!("too short to be a table");
    }
    let little = {
        let fields = u32::from_le_bytes(bytes[4..8].try_into()?) as usize;
        let data = u32::from_le_bytes(bytes[8..12].try_into()?) as usize;
        data == 16 + fields * 12
    };
    let word = |at: usize| -> Result<u32> {
        let b: [u8; 4] = bytes.get(at..at + 4).context("past the table's end")?.try_into()?;
        Ok(if little { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
    };
    let half = |at: usize| -> Result<u16> {
        let b: [u8; 2] = bytes.get(at..at + 2).context("past the table's end")?.try_into()?;
        Ok(if little { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
    };
    let (rows, fields, data, size) = (word(0)? as usize, word(4)? as usize, word(8)? as usize, word(12)? as usize);
    if data != 16 + fields * 12 {
        bail!("this isn't a table");
    }
    let names = column_names();
    struct Column {
        name: String,
        mask: u32,
        offset: usize,
        shift: u32,
        kind: u8,
    }
    let mut columns = Vec::with_capacity(fields);
    for f in 0..fields {
        let at = 16 + 12 * f;
        let hash = word(at)?;
        columns.push(Column {
            name: names.get(&hash).map_or_else(|| format!("0x{hash:08x}"), |n| (*n).to_string()),
            mask: word(at + 4)?,
            offset: half(at + 8)? as usize,
            shift: u32::from(*bytes.get(at + 10).context("past the table's end")?),
            kind: *bytes.get(at + 11).context("past the table's end")?,
        });
    }
    let pool = data + rows * size;
    let mut out = Vec::with_capacity(rows);
    for r in 0..rows {
        let base = data + r * size;
        let mut row = Map::new();
        for column in &columns {
            let at = base + column.offset;
            let value = match column.kind {
                6 => {
                    let start = pool + word(at)? as usize;
                    let end = bytes[start..].iter().position(|b| *b == 0).map_or(bytes.len(), |e| start + e);
                    Value::String(text_of(&bytes[start..end]))
                }
                2 => {
                    let raw = word(at)?;
                    json!(f64::from(f32::from_bits(raw)))
                }
                0 | 3 => {
                    let raw = (word(at)? & column.mask) >> column.shift;
                    if column.mask == u32::MAX {
                        json!(raw as i32)
                    } else {
                        json!(raw)
                    }
                }
                4 => json!(i64::from(half(at)?) & i64::from(column.mask)),
                5 => json!(i64::from(*bytes.get(at).context("past the table's end")?)),
                other => bail!("a column of kind {other}"),
            };
            row.insert(column.name.clone(), value);
        }
        out.push(Value::Object(row));
    }
    let kinds: Map<String, Value> = columns
        .iter()
        .filter(|c| c.kind == 2)
        .map(|c| (c.name.clone(), Value::String("float".to_string())))
        .collect();
    Ok(if kinds.is_empty() { json!({ "rows": out }) } else { json!({ "columns": kinds, "rows": out }) })
}

/// A table's words: as the game's own tables keep Japanese.
fn text_of(bytes: &[u8]) -> String {
    encoding_rs::SHIFT_JIS.decode(bytes).0.into_owned()
}

/// Every file of an imported game under a folder, by its place.
pub fn list_files(store: &Path, folder: &str) -> Result<Vec<(String, u64)>> {
    use nebula_import::cook::store as game_store;
    let found = game_store::read_folder(store, folder.trim_matches('/'))?;
    Ok(found.into_iter().map(|(path, bytes)| (path, bytes.len() as u64)).collect())
}

/// One file or a whole folder of an imported game into `output`: tables as JSON too, so they can be read and changed.
pub fn extract(store: &Path, path: &str, output: &Path) -> Result<usize> {
    use nebula_import::cook::store as game_store;
    let path = path.trim_matches('/');
    let items = match game_store::read_folder(store, path) {
        Ok(items) if !items.is_empty() => items.into_iter().map(|(p, b)| (format!("{path}/{p}"), b)).collect(),
        _ => vec![(path.to_string(), game_store::read_file(store, path)?)],
    };
    let mut written = 0;
    for (place, bytes) in items {
        let target = output.join(&place);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| format!("{} cannot be created", parent.display()))?;
        }
        if let Ok(table) = table_to_json(&bytes) {
            let mut json = target.as_os_str().to_owned();
            json.push(".json");
            fs::write(&json, serde_json::to_vec_pretty(&table)?)?;
        }
        fs::write(&target, &bytes)?;
        written += 1;
    }
    Ok(written)
}

/// A file a game's mod could use: what's in it, in one line.
pub fn describe_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("{} cannot be read", path.display()))?;
    let bytes = if nebula_import::cook::archive::is_compressed(&bytes) { nebula_import::cook::archive::expand(&bytes)? } else { bytes };
    if nebula_import::cook::archive::is_archive(&bytes) {
        let members = nebula_import::cook::archive::members(&bytes)?;
        return Ok(format!("an archive of {} files", members.len()));
    }
    if let Ok(table) = table_to_json(&bytes) {
        return Ok(format!("a table of {} rows", table["rows"].as_array().map_or(0, Vec::len)));
    }
    Ok(format!("{} bytes", bytes.len()))
}

/// An archive's files into a folder, named as the mod's own folder form names them.
pub fn unarchive(path: &Path, output: &Path) -> Result<usize> {
    let bytes = fs::read(path).with_context(|| format!("{} cannot be read", path.display()))?;
    let bytes = if nebula_import::cook::archive::is_compressed(&bytes) { nebula_import::cook::archive::expand(&bytes)? } else { bytes };
    let members = nebula_import::cook::archive::members(&bytes)?;
    let mut written = 0;
    for member in &members {
        let target = output.join(&member.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Ok(table) = table_to_json(&member.bytes) {
            let mut json = target.as_os_str().to_owned();
            json.push(".json");
            fs::write(&json, serde_json::to_vec_pretty(&table)?)?;
        }
        fs::write(&target, &member.bytes)?;
        written += 1;
    }
    Ok(written)
}

/// Each game setting, what it does and what the game has without it.
pub const SETTINGS: &[(&str, &str, &str)] = &[
    ("player_health", "3", "The health a player starts with, and the most before a Life Up"),
    ("player_health_fixed", "off", "Daredevil comets don't change the player's health"),
    ("hit_takes_all", "off", "A hit takes all the player's health"),
    ("life_up_health", "6", "The most health a Life Up gives. 0: none"),
    ("no_low_health_warning", "off", "No warning sound at low health"),
    ("no_dark_comet_rules", "off", "A Daredevil comet is a comet like any other"),
    ("max_lives", "99", "The most lives"),
    ("starting_lives", "4", "The lives a file starts and loads with"),
    ("max_star_bits", "9999", "The most star bits kept in the save"),
    ("max_carried_star_bits", "999", "The most star bits carried in a stage"),
    ("max_coins", "999", "The most coins in a stage"),
    ("coins_per_one_up", "50", "Coins for a 1-up. 0: none"),
    ("star_bits_per_one_up", "50", "Star bits for a 1-up. 0: none"),
    ("purple_coins_for_star", "100", "Purple coins that bring the star"),
    ("water_air", "8", "The air the player has under water"),
    ("miss_star_pieces", "0", "Star bits added after a miss, negative to take. -999: all lost"),
    ("flip_panel_group", "32", "How many flip panels a group holds"),
    ("flip_panel_observer_group", "32", "The same for their observers"),
    ("water_bazooka_shot_flag", "off", "The water bazooka's shots' flag"),
    ("kamek_barrier_offset_y", "0", "Kamek's barrier's height, as float bits"),
    ("skeletal_fish_baby_no_bubble", "off", "No air bubble from a hit skeletal fish"),
    ("dodoryu_shell_only", "off", "Dodoryu is hurt by shells alone"),
    ("no_hidden_star_table", "off", "Hidden stars without their table"),
    ("no_sudden_death_dodoryu", "off", "No sudden-death Dodoryu"),
    ("begoman_fire_proof", "off", "Begomans survive fire ground"),
    ("messenger_always_ex", "off", "The 1-up messenger gives its larger supply"),
    ("messenger_ex_one_ups", "20", "How many 1-ups that is"),
];

/// The stage names the game's rules go by, and what playing as each brings.
pub const STAGE_RULES: &[(&str, &str)] = &[
    ("AstroGalaxy", "The hub: counters stay up, no 1-up from star bits, Mario walks the Observatory's way, shadows as the hub's, star bits from Lumas count for the dome"),
    ("AstroDome", "A dome's inside: the map opens there, its pause menu"),
    ("LibraryRoom", "The library: a hub location"),
    ("PeachCastleGardenGalaxy", "The prologue's stage: its wipe, its pause menu, its shadows"),
    ("PeachCastleFinalGalaxy", "The ending's castle: its Toads"),
    ("EpilogueDemoStage", "The epilogue: no pause menu, no wipe"),
    ("KoopaBattleVs1Galaxy", "A Bowser arena: the grand star's fall, demo and sequence"),
    ("KoopaBattleVs2Galaxy", "A Bowser arena, and enemies that drop items as this one's do"),
    ("KoopaBattleVs3Galaxy", "The last arena: the last grand star's long sequence"),
    ("OceanRingGalaxy", "Swimming with a limit on how far the player can tilt; the ocean's water sound"),
    ("OceanPhantomCaveGalaxy", "Water bazookas as here; Syati's and the music's rules"),
    ("HeavensDoorGalaxy", "Planets from the start, its flip panels, its Butterflies, its pipes; fades in"),
    ("HeavenlyBeachGalaxy", "Water as here"),
    ("HellProminenceGalaxy", "Mission 5's sideways camera, the Tamakoro ball's rules"),
    ("CannonFleetGalaxy", "Mogu and boss music as here"),
    ("BattleShipGalaxy", "Boss music, and the player's fire as here"),
    ("ReverseKingdomGalaxy", "Boss music as here"),
    ("SandClockGalaxy", "The player's sand rules"),
    ("TriLegLv1Galaxy", "Bowser Jr.'s entrance, the player's walking on the walker"),
    ("CosmosGardenGalaxy", "Mission 4 is the sudden-death Dodoryu"),
    ("FactoryGalaxy", "Onimasu's rules, the opening's rules"),
    ("EggStarGalaxy", "The first opening, mission 2's star bit limit"),
    ("HoneyBeeKingdomGalaxy", "Its opening"),
    ("IceVolcanoGalaxy", "Music, wipe"),
    ("SkullSharkGalaxy", "Water, its steam"),
    ("TearDropGalaxy", "Water sphere as here"),
    ("TeresaMario2DGalaxy", "Fans as here"),
    ("FishTunnelGalaxy", "Toad as here"),
    ("SurfingLv1Galaxy", "The surfing race's restart"),
    ("SurfingLv2Galaxy", "The same, harder"),
];
