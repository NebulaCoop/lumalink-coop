//! NebulaCoop's built-in mods (builtin/): each folder is committed with its package beside it (builtin/<id>.lumacoop),
//! which nebula-coop embeds from its `lumalink` submodule. A package is stale when its folder changed and it was not
//! packed again: this fails then, naming the file. Building the code needs LLVM 22.1.6, so the test doesn't pack;
//! builtin/README.md says how to.
//!
//! Checked: the package's luma.json and every file of the folder outside code/ are the folder's, byte for byte (line
//! ends aside), the package has nothing else outside code/, and it has the library of every platform luma.json lists.

use lumalink_core::build;
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

fn builtin() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../builtin")
}

fn lf(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    for (i, &b) in bytes.iter().enumerate() {
        if !(b == b'\r' && bytes.get(i + 1) == Some(&b'\n')) {
            out.push(b);
        }
    }
    out
}

fn folder_files(root: &Path, at: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(at).unwrap() {
        let path = entry.unwrap().path();
        let rel = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
        if rel == "code" || rel.starts_with('.') || rel.contains("/.") {
            continue;
        }
        if path.is_dir() {
            folder_files(root, &path, out);
        } else {
            out.insert(rel, lf(&std::fs::read(&path).unwrap()));
        }
    }
}

#[test]
fn every_builtin_mod_is_packed() {
    let mut mods = 0;
    for entry in std::fs::read_dir(builtin()).unwrap() {
        let folder = entry.unwrap().path();
        if !folder.join("luma.json").is_file() {
            continue;
        }
        mods += 1;
        let id = folder.file_name().unwrap().to_string_lossy().to_string();
        let package = builtin().join(format!("{id}.lumacoop"));
        let file = std::fs::File::open(&package).unwrap_or_else(|e| panic!("{}: {e} (pack {id}: builtin/README.md)", package.display()));
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let mut packed = BTreeMap::new();
        for i in 0..zip.len() {
            let mut item = zip.by_index(i).unwrap();
            if item.is_dir() {
                continue;
            }
            let mut bytes = Vec::new();
            item.read_to_end(&mut bytes).unwrap();
            packed.insert(item.name().to_string(), bytes);
        }
        let mut wanted = BTreeMap::new();
        folder_files(&folder, &folder, &mut wanted);
        let outside_code: BTreeMap<String, Vec<u8>> =
            packed.iter().filter(|(name, _)| !name.starts_with("code/")).map(|(n, b)| (n.clone(), lf(b))).collect();
        assert_eq!(
            outside_code.keys().collect::<Vec<_>>(),
            wanted.keys().collect::<Vec<_>>(),
            "{id}.lumacoop is stale: its files are not the folder's (pack it again: builtin/README.md)"
        );
        for (name, bytes) in &wanted {
            assert!(outside_code[name] == *bytes, "{id}.lumacoop is stale: {name} differs (pack it again: builtin/README.md)");
        }
        let manifest: serde_json::Value = serde_json::from_slice(&wanted["luma.json"]).unwrap();
        let platforms = manifest.get("code").and_then(|c| c.get("platforms")).and_then(|p| p.as_array()).cloned().unwrap_or_default();
        for platform in platforms {
            let platform = platform.as_str().unwrap();
            let target = build::TARGETS.iter().find(|t| t.platform == platform).unwrap_or_else(|| panic!("{id}: no platform {platform}"));
            let library = format!("code/{platform}/{}", target.library);
            assert!(packed.contains_key(&library), "{id}.lumacoop has no {library}");
        }
    }
    assert!(mods >= 2, "the built-in mods are missing from builtin/");
}
