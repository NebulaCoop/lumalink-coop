//! Cheat and code lists (Gecko codes), as `.gct` files and as Dolphin's game settings: each plain write in one is a memory write like any patch's,
//! read for what it changes in the game. Anything else in a list (a condition, a jump, inserted code) can't be read that way and is counted.

/// A write a code list makes, and how many lines of it weren't plain writes.
#[derive(Debug, Default, PartialEq)]
pub struct CodeList {
    pub writes: Vec<(u32, Vec<u8>)>,
    pub other: usize,
}

fn word(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// The pairs of words of a list, in order.
fn read_pairs(pairs: &[(u32, u32)]) -> CodeList {
    let mut list = CodeList::default();
    let mut at = 0;
    while at < pairs.len() {
        let (first, second) = pairs[at];
        at += 1;
        if first == 0xF000_0000 {
            break;
        }
        let kind = first >> 24;
        let base = kind & 0xFE;
        // A base of 0x80000000 only: bit 28 is the pointer form, which needs the running game.
        if first & 0x1000_0000 != 0 && matches!(base & 0xF0, 0x00 | 0x10) {
            list.other += 1;
            continue;
        }
        let address = 0x8000_0000 | (first & 0x01FF_FFFF);
        match base {
            0x00 => {
                let times = (second >> 16) + 1;
                for i in 0..times.min(0x1000) {
                    list.writes.push((address + i, vec![second as u8]));
                }
            }
            0x02 => {
                let times = (second >> 16) + 1;
                for i in 0..times.min(0x1000) {
                    list.writes.push((address + i * 2, (second as u16).to_be_bytes().to_vec()));
                }
            }
            0x04 => list.writes.push((address, second.to_be_bytes().to_vec())),
            0x06 => {
                // A run of bytes, in the pairs that follow.
                let size = second as usize;
                let lines = size.div_ceil(8);
                // No more room than the list has lines for: a damaged size may say four gigabytes.
                let mut bytes = Vec::with_capacity(lines.min(pairs.len().saturating_sub(at)) * 8);
                for pair in pairs.iter().skip(at).take(lines) {
                    bytes.extend_from_slice(&pair.0.to_be_bytes());
                    bytes.extend_from_slice(&pair.1.to_be_bytes());
                }
                at += lines;
                bytes.truncate(size);
                list.writes.push((address, bytes));
            }
            // Inserted code: its lines come after, and it's counted once.
            0xC2 | 0xF2 => {
                at += second as usize;
                list.other += 1;
            }
            _ => list.other += 1,
        }
    }
    list
}

/// A `.gct`: a header, then pairs of words.
pub fn from_gct(bytes: &[u8]) -> CodeList {
    let start = if bytes.len() >= 8 && bytes[..8] == [0x00, 0xD0, 0xC0, 0xDE, 0x00, 0xD0, 0xC0, 0xDE] { 8 } else { 0 };
    let pairs: Vec<(u32, u32)> = bytes[start..].chunks_exact(8).map(|p| (word(&p[..4]), word(&p[4..]))).collect();
    read_pairs(&pairs)
}

/// Where Nebula does a popular code natively.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// A mod in LumaLink's examples folder (`examples/<name>`), built for every device by `lumalink build`.
    Example(&'static str),
    /// One of Nebula's own settings, by the name its settings show.
    Setting(&'static str),
}

/// A popular code Nebula does natively: where, and what that is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KnownCode {
    pub route: Route,
    pub what: &'static str,
}

impl KnownCode {
    /// What a converted mod's report says in place of the code.
    pub fn advice(&self, code: &str) -> String {
        match self.route {
            Route::Example(name) => format!(
                "Code \"{code}\": Nebula does this natively ({}). Use the mod {name} from LumaLink's examples (lumalink build, then lumalink pack) in place of the code.",
                self.what
            ),
            Route::Setting(name) => format!("Code \"{code}\": Nebula does this natively ({}). Set {name} in Nebula's settings in place of the code.", self.what),
        }
    }
}

/// Popular codes by the words their names use in shared lists (all of a row's words, in any case and order).
const KNOWN: &[(&[&str], KnownCode)] = &[
    (&["moon", "jump"], KnownCode { route: Route::Example("gecko-moon-jump"), what: "rise while jump is held" }),
    (&["infinite", "health"], KnownCode { route: Route::Example("gecko-infinite-health"), what: "no hit takes health" }),
    (&["inf", "health"], KnownCode { route: Route::Example("gecko-infinite-health"), what: "no hit takes health" }),
    (&["invincib"], KnownCode { route: Route::Example("gecko-infinite-health"), what: "no hit takes health" }),
    (&["lives"], KnownCode { route: Route::Example("gecko-infinite-lives"), what: "lives stay at 99" }),
    (&["star", "bit"], KnownCode { route: Route::Example("gecko-max-star-bits"), what: "star bits filled, coins added" }),
    (&["coin"], KnownCode { route: Route::Example("gecko-max-star-bits"), what: "star bits filled, coins added" }),
    (&["gravity"], KnownCode { route: Route::Example("gecko-low-gravity"), what: "a scale on gravity's pull (set_gravity_scale)" }),
    (&["rainbow"], KnownCode { route: Route::Example("gecko-rainbow-forever"), what: "the star power that never ends" }),
    (&["power", "up", "never"], KnownCode { route: Route::Example("gecko-power-ups-last"), what: "power-ups that never run out" }),
    (&["infinite", "power"], KnownCode { route: Route::Example("gecko-power-ups-last"), what: "power-ups that never run out" }),
    (&["fire", "mario"], KnownCode { route: Route::Example("gecko-power-up-anywhere"), what: "any power-up on a button" }),
    (&["bee", "mario"], KnownCode { route: Route::Example("gecko-power-up-anywhere"), what: "any power-up on a button" }),
    (&["boo", "mario"], KnownCode { route: Route::Example("gecko-power-up-anywhere"), what: "any power-up on a button" }),
    (&["ice", "mario"], KnownCode { route: Route::Example("gecko-power-up-anywhere"), what: "any power-up on a button" }),
    (&["power", "up", "anywhere"], KnownCode { route: Route::Example("gecko-power-up-anywhere"), what: "any power-up on a button" }),
    (&["timer"], KnownCode { route: Route::Example("gecko-freeze-timer"), what: "countdowns held (pause_timer)" }),
    (&["free", "cam"], KnownCode { route: Route::Example("gecko-free-camera"), what: "a camera the d-pad moves (set_camera)" }),
    (&["fov"], KnownCode { route: Route::Example("gecko-wide-fov"), what: "the camera's angle of view (set_camera)" }),
    (&["hud"], KnownCode { route: Route::Example("gecko-hide-hud"), what: "the HUD hidden (show_hud)" }),
    (&["fall"], KnownCode { route: Route::Example("gecko-no-fall-death"), what: "a fall out of the level puts the player back" }),
    (&["slow", "motion"], KnownCode { route: Route::Example("gecko-slow-motion"), what: "slow motion and frame advance (set_game_speed)" }),
    (&["frame", "advance"], KnownCode { route: Route::Example("gecko-slow-motion"), what: "slow motion and frame advance (set_game_speed)" }),
    (&["speed", "hack"], KnownCode { route: Route::Example("gecko-slow-motion"), what: "slow motion and frame advance (set_game_speed)" }),
    (&["position"], KnownCode { route: Route::Example("gecko-position-display"), what: "the player's place and speed on the screen (draw_text)" }),
    (&["coordinates"], KnownCode { route: Route::Example("gecko-position-display"), what: "the player's place and speed on the screen (draw_text)" }),
    (&["freeze", "enem"], KnownCode { route: Route::Example("gecko-freeze-enemies"), what: "enemies held still (hook_nerve)" }),
    (&["stage", "select"], KnownCode { route: Route::Example("gecko-warp-anywhere"), what: "any galaxy and star (change_stage)" }),
    (&["level", "select"], KnownCode { route: Route::Example("gecko-warp-anywhere"), what: "any galaxy and star (change_stage)" }),
    (&["warp"], KnownCode { route: Route::Example("gecko-warp-anywhere"), what: "any galaxy and star (change_stage)" }),
    (&["mario", "size"], KnownCode { route: Route::Example("gecko-mario-size"), what: "the player's model at another size (set_scale)" }),
    (&["big", "mario"], KnownCode { route: Route::Example("gecko-mario-size"), what: "the player's model at another size (set_scale)" }),
    (&["giant", "mario"], KnownCode { route: Route::Example("gecko-mario-size"), what: "the player's model at another size (set_scale)" }),
    (&["tiny", "mario"], KnownCode { route: Route::Example("gecko-mario-size"), what: "the player's model at another size (set_scale)" }),
    (&["mini", "mario"], KnownCode { route: Route::Example("gecko-mario-size"), what: "the player's model at another size (set_scale)" }),
    (&["spin", "cooldown"], KnownCode { route: Route::Example("gecko-no-spin-cooldown"), what: "a spin ready as soon as the last ends (player number mSpinIntervalTime)" }),
    (&["always", "spin"], KnownCode { route: Route::Example("gecko-no-spin-cooldown"), what: "a spin ready as soon as the last ends (player number mSpinIntervalTime)" }),
    (&["infinite", "spin"], KnownCode { route: Route::Example("gecko-no-spin-cooldown"), what: "a spin ready as soon as the last ends (player number mSpinIntervalTime)" }),
    (&["star", "pointer"], KnownCode { route: Route::Example("gecko-spin-and-pointer"), what: "the spin and the Star Pointer from the start (set_event_flag_on)" }),
    (&["spin", "start"], KnownCode { route: Route::Example("gecko-spin-and-pointer"), what: "the spin and the Star Pointer from the start (set_event_flag_on)" }),
    (&["fast", "text"], KnownCode { route: Route::Setting("Text speed"), what: "text up to a whole page at once" }),
    (&["text", "speed"], KnownCode { route: Route::Setting("Text speed"), what: "text up to a whole page at once" }),
    (&["instant", "text"], KnownCode { route: Route::Setting("Text speed"), what: "text up to a whole page at once" }),
];

/// The native way of a code its name says it is, if it is one of the popular codes.
pub fn known_code(name: &str) -> Option<KnownCode> {
    let lower = name.to_ascii_lowercase();
    KNOWN.iter().find(|(words, _)| words.iter().all(|w| lower.contains(w))).map(|(_, code)| *code)
}

/// One code of Dolphin's game settings: its `$name` (without any `[author]` after it), its pairs, and whether it is on.
#[derive(Debug, PartialEq)]
pub struct NamedCode {
    pub name: String,
    pub pairs: Vec<(u32, u32)>,
    pub enabled: bool,
}

/// Dolphin's game settings, code by code.
pub fn ini_codes(text: &str) -> Vec<NamedCode> {
    let mut section = String::new();
    let mut codes: Vec<NamedCode> = Vec::new();
    let mut enabled: Vec<String> = Vec::new();
    let mut says_enabled = false;
    let name_of = |line: &str| line.split('[').next().unwrap_or(line).trim().to_string();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            section = line.trim_matches(['[', ']']).to_ascii_lowercase();
            if section == "gecko_enabled" {
                says_enabled = true;
            }
            continue;
        }
        if section == "gecko_enabled" {
            if let Some(name) = line.strip_prefix('$') {
                enabled.push(name_of(name));
            }
        } else if section == "gecko" {
            if let Some(name) = line.strip_prefix('$') {
                codes.push(NamedCode { name: name_of(name), pairs: Vec::new(), enabled: true });
            } else if let Some((a, b)) = line.split_once(' ') {
                if let (Ok(a), Ok(b)) = (u32::from_str_radix(a.trim(), 16), u32::from_str_radix(b.trim(), 16)) {
                    if codes.is_empty() {
                        codes.push(NamedCode { name: String::new(), pairs: Vec::new(), enabled: true });
                    }
                    if let Some(last) = codes.last_mut() {
                        last.pairs.push((a, b));
                    }
                }
            }
        }
    }
    if says_enabled {
        for code in &mut codes {
            code.enabled = enabled.contains(&code.name);
        }
    }
    codes
}

/// The writes of a list of pairs.
pub fn from_pairs(pairs: &[(u32, u32)]) -> CodeList {
    read_pairs(pairs)
}

/// Dolphin's game settings: lines of two hex words under `[Gecko]`, and `[Gecko_Enabled]` saying which by their `$name`. A list with no enabled section has all on.
pub fn from_ini(text: &str) -> CodeList {
    let mut list = CodeList::default();
    for code in ini_codes(text).into_iter().filter(|c| c.enabled) {
        let one = read_pairs(&code.pairs);
        list.writes.extend(one.writes);
        list.other += one.other;
    }
    list
}

/// What a changed program writes, against the game's own: for the program's code each changed word, for its data each changed run.
pub fn program_changes(changed: &[u8], original: &[u8]) -> Option<Vec<(u32, Vec<u8>)>> {
    let header = |bytes: &[u8]| -> Option<Vec<(usize, u32, usize)>> {
        if bytes.len() < 0x100 {
            return None;
        }
        let number = |at: usize| u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        Some((0..18).map(|i| (number(i * 4) as usize, number(0x48 + i * 4), number(0x90 + i * 4) as usize)).collect())
    };
    let (new_sections, old_sections) = (header(changed)?, header(original)?);
    let mut out = Vec::new();
    for (section, (new, old)) in new_sections.iter().zip(&old_sections).enumerate() {
        let (new_at, address, size) = *new;
        let (old_at, old_address, old_size) = *old;
        if size == 0 || address != old_address {
            continue;
        }
        let same = size.min(old_size);
        let new_bytes = changed.get(new_at..new_at + same)?;
        let old_bytes = original.get(old_at..old_at + same)?;
        let is_code = section < 7;
        let mut at = 0;
        while at + 4 <= same {
            if new_bytes[at..at + 4] == old_bytes[at..at + 4] {
                at += 4;
                continue;
            }
            let start = at;
            at += 4;
            if !is_code {
                while at + 4 <= same && new_bytes[at..at + 4] != old_bytes[at..at + 4] {
                    at += 4;
                }
            }
            out.push((address.checked_add(u32::try_from(start).ok()?)?, new_bytes[start..at].to_vec()));
        }
        if size > old_size {
            out.push((address.checked_add(u32::try_from(same).ok()?)?, changed.get(new_at + same..new_at + size)?.to_vec()));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    /// A damaged list or program is read as far as it goes, never trusted for its sizes.
    #[test]
    fn damaged_sizes_are_not_trusted() {
        let list = read_pairs(&[(0x0600_1000, 0xFFFF_FFFF), (0x1122_3344, 0x5566_7788)]);
        assert_eq!(list.writes, vec![(0x8000_1000, vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88])]);
        let mut original = vec![0u8; 0x200];
        original[0x48..0x4C].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes());
        original[0..4].copy_from_slice(&0x100u32.to_be_bytes());
        original[0x90..0x94].copy_from_slice(&0x100u32.to_be_bytes());
        let mut changed = original.clone();
        changed[0x1F0] = 1;
        assert_eq!(program_changes(&changed, &original), None);
    }

    use super::*;

    #[test]
    fn a_list_reads_as_writes() {
        let mut gct = vec![0x00, 0xD0, 0xC0, 0xDE, 0x00, 0xD0, 0xC0, 0xDE];
        gct.extend_from_slice(&[0x04, 0x5B, 0x06, 0x70, 0x42, 0x20, 0, 0]); // 32 bits at 805B0670
        gct.extend_from_slice(&[0x02, 0x00, 0x10, 0x00, 0, 0, 0xAB, 0xCD]); // 16 bits
        gct.extend_from_slice(&[0x20, 0x00, 0, 0, 0, 0, 0, 1]); // a condition
        gct.extend_from_slice(&[0xF0, 0, 0, 0, 0, 0, 0, 0]);
        let list = from_gct(&gct);
        assert_eq!(list.writes[0], (0x805B_0670, vec![0x42, 0x20, 0, 0]));
        assert_eq!(list.writes[1], (0x8000_1000, vec![0xAB, 0xCD]));
        assert_eq!(list.other, 1);
    }

    #[test]
    fn dolphin_settings_say_which_are_on() {
        let text = "[Gecko]\n$High jump [someone]\n045B0678 42700000\n$Other\n04000000 00000001\n[Gecko_Enabled]\n$High jump [someone]\n";
        let list = from_ini(text);
        assert_eq!(list.writes, vec![(0x805B_0678, vec![0x42, 0x70, 0, 0])]);
    }

    #[test]
    fn popular_codes_are_known_by_their_names() {
        let route = |name: &str| known_code(name).map(|c| c.route);
        assert_eq!(route("Moon Jump [Ralf]"), Some(Route::Example("gecko-moon-jump")));
        assert_eq!(route("INFINITE HEALTH"), Some(Route::Example("gecko-infinite-health")));
        assert_eq!(route("Low Gravity (hold Z)"), Some(Route::Example("gecko-low-gravity")));
        assert_eq!(route("Stage Select"), Some(Route::Example("gecko-warp-anywhere")));
        assert_eq!(route("Freeze Enemies"), Some(Route::Example("gecko-freeze-enemies")));
        assert_eq!(route("Hide HUD"), Some(Route::Example("gecko-hide-hud")));
        assert_eq!(route("Giant Mario"), Some(Route::Example("gecko-mario-size")));
        assert_eq!(route("No Spin Cooldown"), Some(Route::Example("gecko-no-spin-cooldown")));
        assert_eq!(route("Star Pointer from the start"), Some(Route::Example("gecko-spin-and-pointer")));
        assert_eq!(route("Fast Text"), Some(Route::Setting("Text speed")));
        // Every example a code names is in examples/ and in the converting tutorial's list.
        let examples = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let tutorial = include_str!("../../../docs/tutorials/15-converting.md");
        for (_, code) in KNOWN {
            match code.route {
                Route::Example(name) => {
                    assert!(examples.join(name).join("luma.json").is_file(), "{name} is not in examples/");
                    assert!(tutorial.contains(&format!("`{name}`")), "{name} is not in docs/tutorials/15-converting.md");
                }
                Route::Setting(name) => assert!(tutorial.contains(&format!("{name}, in Nebula's settings")), "{name}"),
            }
        }
        assert_eq!(known_code("Widescreen Fix"), None);
        let codes = ini_codes("[Gecko]\n$Moon Jump [someone]\n04001000 00000001\n$Other\n04001004 00000002\n[Gecko_Enabled]\n$Moon Jump\n");
        assert_eq!(codes.len(), 2);
        assert_eq!((codes[0].name.as_str(), codes[0].enabled, codes[0].pairs.len()), ("Moon Jump", true, 1));
        assert_eq!((codes[1].name.as_str(), codes[1].enabled), ("Other", false));
    }

    #[test]
    fn a_changed_program_reads_as_writes() {
        let mut original = vec![0u8; 0x200];
        original[0x48..0x4C].copy_from_slice(&0x8000_4000u32.to_be_bytes());
        original[0..4].copy_from_slice(&0x100u32.to_be_bytes());
        original[0x90..0x94].copy_from_slice(&0x100u32.to_be_bytes());
        let mut changed = original.clone();
        changed[0x110..0x114].copy_from_slice(&[0x38, 0x60, 0, 1]);
        assert_eq!(program_changes(&changed, &original).unwrap(), vec![(0x8000_4010, vec![0x38, 0x60, 0, 1])]);
    }
}
