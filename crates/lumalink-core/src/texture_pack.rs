//! Dolphin-style texture packs: `tex1_<width>x<height>[_m]_<hash>[_tlut hash]_<format>[_mip<n>][_arb].(png|dds)` files in a
//! game-id folder, loose, or in a resource pack with a `manifest.json`.

use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PackInfo {
    pub folder: PathBuf,
    pub textures: usize,
    pub mips: usize,
    pub wildcards: usize,
    pub png: usize,
    pub dds: usize,
    /// Texture files whose names this game's packs do not use.
    pub other_games: BTreeMap<String, usize>,
    pub title: String,
    pub authors: String,
    pub description: String,
    pub version: String,
}

/// The first part of sixteen hexadecimal digits of a pack file's name, as Nebula matches textures by it.
pub fn hash_of_name(stem: &str) -> Option<u64> {
    let rest = stem.to_ascii_lowercase();
    let rest = rest.strip_prefix("tex1_")?;
    rest.split('_')
        .find(|part| part.len() == 16 && part.bytes().all(|b| b.is_ascii_hexdigit()))
        .and_then(|part| u64::from_str_radix(part, 16).ok())
}

/// A game id a folder's name means: RMGE01, RMG, or another game's.
fn game_folder(name: &str) -> Option<&str> {
    ((name.len() == 3 || name.len() == 6) && name.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()) && name.bytes().next().is_some_and(|b| b.is_ascii_uppercase())).then_some(name)
}

fn manifest_field(text: &str, field: &str) -> String {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
        return String::new();
    };
    match value.get(field) {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(a)) => a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "),
        _ => String::new(),
    }
}

/// What a folder holds of a texture pack, if anything.
pub fn scan(root: &Path) -> Option<PackInfo> {
    let mut info = PackInfo { folder: root.to_path_buf(), ..Default::default() };
    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if name == "manifest.json" {
            let text = crate::riivolution::decode(&std::fs::read(entry.path()).unwrap_or_default());
            info.title = manifest_field(&text, "name");
            info.authors = manifest_field(&text, "authors");
            info.description = manifest_field(&text, "description");
            info.version = manifest_field(&text, "version");
            continue;
        }
        if !name.starts_with("tex1_") || !(name.ends_with(".png") || name.ends_with(".dds")) {
            continue;
        }
        let stem = &name[..name.len() - 4];
        let of_game = entry
            .path()
            .ancestors()
            .skip(1)
            .take_while(|a| *a != root)
            .filter_map(|a| a.file_name()?.to_str())
            .find_map(game_folder)
            .map(str::to_string);
        if let Some(game) = of_game {
            if !game.starts_with("RMG") {
                *info.other_games.entry(game).or_default() += 1;
                continue;
            }
        }
        if name.ends_with(".png") {
            info.png += 1;
        } else {
            info.dds += 1;
        }
        if stem.contains("_mip") {
            info.mips += 1;
            continue;
        }
        if stem.contains('$') {
            info.wildcards += 1;
        }
        info.textures += 1;
    }
    // Anything at all under another game's folder says who the pack is for, whatever its files are named.
    for entry in walkdir::WalkDir::new(root).max_depth(3).into_iter().filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().to_string();
        if entry.file_type().is_dir() && crate::layout::is_other_game_folder(&name) {
            let files = walkdir::WalkDir::new(entry.path())
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some_and(|x| x.eq_ignore_ascii_case("png") || x.eq_ignore_ascii_case("dds"))
                })
                .count();
            if files != 0 {
                info.other_games.insert(name, files);
            }
        }
    }
    (info.textures > 0 || !info.other_games.is_empty()).then_some(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_name_gives_its_hash() {
        assert_eq!(hash_of_name("tex1_64x64_m_0123456789abcdef_14"), Some(0x0123456789abcdef));
        assert_eq!(hash_of_name("TEX1_32x32_FEDCBA9876543210_ABCDEF0123456789_9"), Some(0xfedcba9876543210));
        assert_eq!(hash_of_name("tex1_64x64_14"), None);
        assert_eq!(hash_of_name("other"), None);
    }

    #[test]
    fn a_pack_is_counted_and_another_games_kept_apart() {
        let dir = tempfile::tempdir().unwrap();
        let ours = dir.path().join("Load/Textures/RMGE01");
        let theirs = dir.path().join("Load/Textures/SB4E01");
        fs::create_dir_all(&ours).unwrap();
        fs::create_dir_all(&theirs).unwrap();
        fs::write(ours.join("tex1_8x8_0123456789abcdef_14.png"), b"").unwrap();
        fs::write(ours.join("tex1_8x8_0123456789abcdee_14_mip1.png"), b"").unwrap();
        fs::write(ours.join("tex1_8x8_$_14.dds"), b"").unwrap();
        fs::write(theirs.join("tex1_8x8_0123456789abcdef_14.png"), b"").unwrap();
        fs::write(dir.path().join("manifest.json"), br#"{"name":"HD","authors":["a","b"],"version":"1.0"}"#).unwrap();
        let info = scan(dir.path()).unwrap();
        assert_eq!(info.textures, 2);
        assert_eq!(info.mips, 1);
        assert_eq!(info.wildcards, 1);
        assert_eq!(info.png, 2);
        assert_eq!(info.dds, 1);
        assert_eq!(info.other_games["SB4E01"], 1);
        assert_eq!(info.title, "HD");
        assert_eq!(info.authors, "a, b");
    }

    #[test]
    fn a_folder_without_textures_is_no_pack() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("readme.txt"), b"").unwrap();
        assert!(scan(dir.path()).is_none());
    }
}
