//! The game's messages as a mod writes them in `text` rows: words with the game's message tags in brackets, the same
//! words Nebula reads (native/include/nebula_text_tags.h):
//!
//! `[wait 30]` `[page]` `[center]` `[sound SE_NAME]` `[icon 7]` `[small]` `[normal]` `[large]` `[player]` `[color 1]`, and
//! `[tag G T u...]` for any other tag (group, tag number, its 16-bit parameter units). `[[` is one `[`.
//!
//! [`decode`] gives a message's text in those words; [`encode`] is Nebula's reading of them, so a message read out and
//! put back is the game's own, unit for unit.

use anyhow::{Context, Result};
use nebula_import::cook::message::Nmsg;
use nebula_import::cook::table::{Cell, TableIr};

const TAG: u16 = 0x1A;
const TAG_UNITS_MAX: usize = 124;

/// One message's units (to its 0, without it) as mod text.
pub fn decode(units: &[u16]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < units.len() {
        let unit = units[i];
        if unit == TAG && i + 2 < units.len() {
            let length = usize::from(units[i + 1] & 0xFF);
            let group = units[i + 1] >> 8;
            if length >= 6 && length % 2 == 0 && i + length / 2 <= units.len() {
                let id = units[i + 2];
                let params = &units[i + 3..i + length / 2];
                out.push_str(&tag_words(group, id, params));
                i += length / 2;
                continue;
            }
        }
        // A character, or a surrogate pair.
        let end = if (0xD800..0xDC00).contains(&unit) && units.get(i + 1).is_some_and(|u| (0xDC00..0xE000).contains(u)) { i + 2 } else { i + 1 };
        let text = String::from_utf16_lossy(&units[i..end]);
        if text == "[" {
            out.push_str("[[");
        } else {
            out.push_str(&text);
        }
        i = end;
    }
    out
}

fn tag_words(group: u16, id: u16, params: &[u16]) -> String {
    let is_sound_name = |units: &[u16]| {
        !units.is_empty() && units.len() <= TAG_UNITS_MAX && units.iter().all(|u| (0x21..0x7F).contains(u) && *u != u16::from(b'[') && *u != u16::from(b']'))
    };
    match (group, id, params) {
        (1, 0, [frames]) => format!("[wait {frames}]"),
        (1, 1, []) => "[page]".to_string(),
        (1, 3, []) => "[center]".to_string(),
        (2, 0, name) if is_sound_name(name) => format!("[sound {}]", String::from_utf16_lossy(name)),
        (3, picture, []) => format!("[icon {picture}]"),
        (4, 0, []) => "[small]".to_string(),
        (4, 1, []) => "[normal]".to_string(),
        (4, 2, []) => "[large]".to_string(),
        (5, 0, [0]) => "[player]".to_string(),
        (255, 0, [colour]) if colour & 0xFF == 0 => format!("[color {}]", colour >> 8),
        _ => {
            let mut words = format!("[tag {group} {id}");
            for unit in params {
                words.push_str(&format!(" {unit}"));
            }
            words.push(']');
            words
        }
    }
}

fn number(word: &str) -> Option<u16> {
    let value = match word.strip_prefix("0x").or_else(|| word.strip_prefix("0X")) {
        Some(hex) if !hex.is_empty() => u32::from_str_radix(hex, 16).ok()?,
        _ if !word.is_empty() && word.bytes().all(|b| b.is_ascii_digit()) => word.parse::<u32>().ok()?,
        _ => return None,
    };
    u16::try_from(value).ok()
}

/// The tag a bracket's words name: (group, tag, parameter units); None where they name none.
fn tag_of(words: &[&str]) -> Option<(u16, u16, Vec<u16>)> {
    let name = words.first()?.to_ascii_lowercase();
    let args = &words[1..];
    Some(match (name.as_str(), args) {
        ("wait", [frames]) => (1, 0, vec![number(frames)?]),
        ("page", []) => (1, 1, vec![]),
        ("center", []) => (1, 3, vec![]),
        ("sound", [name]) if name.len() <= TAG_UNITS_MAX && name.is_ascii() => (2, 0, name.bytes().map(u16::from).collect()),
        ("icon", [picture]) => (3, number(picture)?, vec![]),
        ("small", []) => (4, 0, vec![]),
        ("normal", []) => (4, 1, vec![]),
        ("large", []) => (4, 2, vec![]),
        ("player", []) => (5, 0, vec![0]),
        ("color", [colour]) => {
            let colour = number(colour)?;
            if colour > 0xFF {
                return None;
            }
            (255, 0, vec![colour << 8])
        }
        ("tag", [group, id, units @ ..]) => {
            let group = number(group)?;
            if group > 0xFF || units.len() > TAG_UNITS_MAX - 1 {
                return None;
            }
            (group, number(id)?, units.iter().map(|u| number(u)).collect::<Option<Vec<_>>>()?)
        }
        _ => return None,
    })
}

/// Mod text as the game's units, as Nebula reads it (nebula_text_tags.h encode): without the 0.
pub fn encode(text: &str) -> Vec<u16> {
    let mut out = Vec::with_capacity(text.len());
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        if c == '[' {
            if rest[1..].starts_with('[') {
                out.push(u16::from(b'['));
                rest = &rest[2..];
                continue;
            }
            let inner_end = rest[1..].find(['[', ']']).map(|at| at + 1);
            if let Some(end) = inner_end.filter(|at| rest.as_bytes()[*at] == b']') {
                let words: Vec<&str> = rest[1..end].split([' ', '\t']).filter(|w| !w.is_empty()).collect();
                if words.len() <= 2 + TAG_UNITS_MAX {
                    if let Some((group, id, units)) = tag_of(&words) {
                        out.push(TAG);
                        out.push((6 + 2 * units.len() as u16) | (group << 8));
                        out.push(id);
                        out.extend(units);
                        rest = &rest[end + 1..];
                        continue;
                    }
                }
            }
        }
        let mut buffer = [0u16; 2];
        out.extend_from_slice(c.encode_utf16(&mut buffer));
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// A language's messages, cooked (`<Language>/MessageData/Message`: message.bmg and messageid.tbl): (id, text) in the id
/// table's order.
pub fn read(message_file: &[u8], id_table: &[u8]) -> Result<Vec<(String, Vec<u16>)>> {
    let messages = Nmsg::from_nmsg(message_file).context("the message file")?;
    let ids = TableIr::from_ntbl(id_table).context("the message id table")?;
    let hash = |name: &str| name.bytes().fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(u32::from(b)));
    let column = |name: &str| ids.columns.iter().position(|c| c.hash == hash(name)).with_context(|| format!("the id table has no {name}"));
    let (id_at, index_at) = (column("MessageId")?, column("Index")?);
    let mut out = Vec::with_capacity(ids.rows.len());
    for row in &ids.rows {
        let (Cell::Text(id), Cell::Bits(index)) = (&row[id_at], &row[index_at]) else {
            continue;
        };
        let Some(message) = messages.messages.get(*index as usize) else {
            continue;
        };
        let start = message.text as usize;
        let end = messages.text.get(start..).and_then(|t| t.iter().position(|u| *u == 0)).map_or(messages.text.len(), |e| start + e);
        out.push((String::from_utf8_lossy(id).into_owned(), messages.text.get(start..end).unwrap_or(&[]).to_vec()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_text_reads_as_nebula_reads_it() {
        assert_eq!(encode("A[wait 30]B"), [u16::from(b'A'), 0x1A, 8 | (1 << 8), 0, 30, u16::from(b'B')]);
        assert_eq!(encode("[color 1]x[color 0]"), [0x1A, 8 | (255 << 8), 0, 0x100, u16::from(b'x'), 0x1A, 8 | (255 << 8), 0, 0]);
        assert_eq!(encode("[player]"), [0x1A, 8 | (5 << 8), 0, 0]);
        assert_eq!(encode("[icon 7][LARGE]"), [0x1A, 6 | (3 << 8), 7, 0x1A, 6 | (4 << 8), 2]);
        assert_eq!(encode("[sound SE_A]"), [0x1A, 14 | (2 << 8), 0, 83, 69, 95, 65]);
        assert_eq!(encode("[tag 6 5 0 0x1 2]"), [0x1A, 12 | (6 << 8), 5, 0, 1, 2]);
        let plain = "[a] [nope 1] [wait] [wait x] [color 256]";
        assert_eq!(encode(&format!("[{plain}")), plain.encode_utf16().collect::<Vec<_>>());
        assert_eq!(encode("[wait 1"), "[wait 1".encode_utf16().collect::<Vec<_>>());
        assert_eq!(encode("é😀"), [0xE9, 0xD83D, 0xDE00]);
    }

    #[test]
    fn a_message_read_out_goes_back_the_same() {
        let units = encode("Hi [player]! [[sic] [color 3]Star[color 0][wait 20][page][icon 12][tag 6 5 0 1 0 0][sound SE_SV_TICO]");
        let text = decode(&units);
        assert_eq!(text, "Hi [player]! [[sic] [color 3]Star[color 0][wait 20][page][icon 12][tag 6 5 0 1 0 0][sound SE_SV_TICO]");
        assert_eq!(encode(&text), units);
    }
}
