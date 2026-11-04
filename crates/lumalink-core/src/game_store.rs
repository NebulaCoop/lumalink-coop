//! The player's imported game (`game.pak`, Nebula's asset store): what paths exist, for a mod that names a file by name
//! alone, and which game it is. Only read; nothing of it goes into a package.

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

const NODE: usize = 40;

/// The paths in a store, from its index.
pub struct GameStore {
    pub path: PathBuf,
    pub game_id: String,
    /// Lower-case paths without a leading '/': files and folders, the insides of archives too.
    paths: BTreeSet<String>,
    /// The paths a lower-case file name has. Archives count as files.
    by_name: BTreeMap<String, Vec<String>>,
}

impl GameStore {
    pub fn open(path: &Path) -> Result<Self> {
        let mut pak = File::open(path).with_context(|| format!("{} cannot be read", path.display()))?;
        let mut header = [0u8; 64];
        pak.read_exact(&mut header).with_context(|| format!("{} is too short to be a game store", path.display()))?;
        if &header[..8] != b"NBSTORE\0" {
            bail!("{} is not a Nebula game store", path.display());
        }
        let word = |at: usize| u64::from_le_bytes(header[at..at + 8].try_into().unwrap());
        let count = u32::from_le_bytes(header[12..16].try_into().unwrap()) as usize;
        let (nodes_at, names_at, names_size) = (word(16), word(32), word(40));
        let game_id = String::from_utf8_lossy(&header[48..54]).to_string();

        let mut nodes = vec![0u8; count.checked_mul(NODE).context("the store's index is too large")?];
        pak.seek(SeekFrom::Start(nodes_at))?;
        pak.read_exact(&mut nodes).context("the store's index is cut short")?;
        let mut names = vec![0u8; names_size as usize];
        pak.seek(SeekFrom::Start(names_at))?;
        pak.read_exact(&mut names).context("the store's names are cut short")?;

        let mut full: Vec<String> = Vec::with_capacity(count);
        let mut paths = BTreeSet::new();
        let mut by_name: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for i in 0..count {
            let node = &nodes[i * NODE..(i + 1) * NODE];
            let name_at = u32::from_le_bytes(node[16..20].try_into().unwrap()) as usize;
            let parent = u32::from_le_bytes(node[20..24].try_into().unwrap()) as usize;
            let tail = names.get(name_at..).unwrap_or(&[]);
            let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
            let name = String::from_utf8_lossy(&tail[..end]).to_string();
            if i == 0 {
                full.push(String::new());
                continue;
            }
            let path = match full.get(parent).map(String::as_str) {
                Some("") | None => name.clone(),
                Some(parent) => format!("{parent}/{name}"),
            };
            paths.insert(path.to_ascii_lowercase());
            by_name.entry(name.to_ascii_lowercase()).or_default().push(path.clone());
            full.push(path);
        }
        Ok(Self { path: path.to_path_buf(), game_id, paths, by_name })
    }

    /// A store for the tests.
    pub fn from_paths(game_id: &str, list: &[&str]) -> Self {
        let mut by_name: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for path in list {
            let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
            by_name.entry(name).or_default().push(path.to_string());
        }
        Self {
            path: PathBuf::new(),
            game_id: game_id.to_string(),
            paths: list.iter().map(|p| p.to_ascii_lowercase()).collect(),
            by_name,
        }
    }

    pub fn has(&self, path: &str) -> bool {
        self.paths.contains(&path.trim_start_matches('/').to_ascii_lowercase())
    }

    /// A top-level folder of the game, in any case.
    pub fn has_folder(&self, name: &str) -> bool {
        self.paths.contains(&name.to_ascii_lowercase())
    }

    /// Paths of a file named this anywhere outside an archive.
    pub fn named(&self, name: &str) -> Vec<String> {
        self.by_name
            .get(&name.to_ascii_lowercase())
            .map(|paths| {
                paths
                    .iter()
                    .filter(|p| !p.to_ascii_lowercase().contains(".arc/"))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_store_answers_by_path_and_name_in_any_case() {
        let store = GameStore::from_paths("RMGE01", &["ObjectData", "ObjectData/Kuribo.arc", "ObjectData/Kuribo.arc/x.bdl"]);
        assert!(store.has("/objectdata/KURIBO.arc"));
        assert!(!store.has("/StageData/a.arc"));
        assert!(store.has_folder("objectdata"));
        assert_eq!(store.named("kuribo.ARC"), ["ObjectData/Kuribo.arc"]);
        assert!(store.named("x.bdl").is_empty());
    }

    #[test]
    fn a_file_that_is_no_store_is_said_so() {
        let dir = std::env::temp_dir().join(format!("lumalink_store_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("not.pak");
        std::fs::write(&file, vec![0u8; 100]).unwrap();
        assert!(GameStore::open(&file).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
