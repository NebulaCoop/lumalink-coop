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
                let mut bytes = Vec::with_capacity(lines * 8);
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

/// Dolphin's game settings: lines of two hex words under `[Gecko]`, and `[Gecko_Enabled]` saying which by their `$name`. A list with no enabled section has all on.
pub fn from_ini(text: &str) -> CodeList {
    let mut section = String::new();
    let mut current = String::new();
    let mut codes: Vec<(String, Vec<(u32, u32)>)> = Vec::new();
    let mut enabled: Vec<String> = Vec::new();
    let mut says_enabled = false;
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
                enabled.push(name.to_string());
            }
        } else if section == "gecko" {
            if let Some(name) = line.strip_prefix('$') {
                current = name.to_string();
                codes.push((current.clone(), Vec::new()));
            } else if let Some((a, b)) = line.split_once(' ') {
                if let (Ok(a), Ok(b)) = (u32::from_str_radix(a.trim(), 16), u32::from_str_radix(b.trim(), 16)) {
                    if codes.is_empty() {
                        codes.push((current.clone(), Vec::new()));
                    }
                    if let Some(last) = codes.last_mut() {
                        last.1.push((a, b));
                    }
                }
            }
        }
    }
    let mut list = CodeList::default();
    for (name, pairs) in codes {
        if !says_enabled || enabled.contains(&name) {
            let one = read_pairs(&pairs);
            list.writes.extend(one.writes);
            list.other += one.other;
        }
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
            out.push((address + start as u32, new_bytes[start..at].to_vec()));
        }
        if size > old_size {
            out.push((address + same as u32, changed.get(new_at + same..new_at + size)?.to_vec()));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
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
