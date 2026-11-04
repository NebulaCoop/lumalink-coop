//! A mod as published: a folder, or a `.zip`, `.7z` or `.rar` archive.

use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

/// How much a mod may unpack to (a guard against a broken or hostile archive).
const MAX_UNPACKED: u64 = 64 * 1024 * 1024 * 1024;

/// Names no folder on Windows can hold, with or without an ending.
fn is_device_name(part: &str) -> bool {
    let stem = part.split('.').next().unwrap_or("").to_ascii_lowercase();
    matches!(stem.as_str(), "con" | "prn" | "aux" | "nul")
        || (stem.len() == 4 && (stem.starts_with("com") || stem.starts_with("lpt")) && stem.as_bytes()[3].is_ascii_digit())
}

/// A path inside an archive as a safe relative path: nothing that leaves the destination, reaches a device or a file's
/// hidden stream, on any system the archive may be opened on.
pub(crate) fn safe_relative(name: &str) -> Option<PathBuf> {
    let name = name.replace('\\', "/");
    if name.starts_with('/') {
        return None;
    }
    let path = Path::new(&name);
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            Component::Normal(p) => {
                let text = p.to_string_lossy();
                if text.contains(':') || text.chars().any(char::is_control) || text.chars().all(|c| c == '.') || is_device_name(&text) {
                    return None;
                }
                out.push(p);
            }
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!out.as_os_str().is_empty()).then_some(out)
}

/// Writes one entry, and no more of it than is left of what the archive may unpack to, whatever it says of itself.
fn write_limited(from: &mut dyn Read, target: &Path, left: &mut u64) -> Result<()> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).with_context(|| format!("{} cannot be created", parent.display()))?;
    }
    let mut to = std::io::BufWriter::new(fs::File::create(target).with_context(|| format!("{} cannot be created", target.display()))?);
    let copied = std::io::copy(&mut from.take(*left + 1), &mut to).with_context(|| format!("{} cannot be unpacked", target.display()))?;
    to.flush().with_context(|| format!("{} cannot be written", target.display()))?;
    if copied > *left {
        bail!("the archive unpacks to more than {MAX_UNPACKED} bytes");
    }
    *left -= copied;
    Ok(())
}

/// Unpacks a zip. Links in it are left out: they would be a way out of the folder, and no mod needs one.
pub(crate) fn unzip(archive: &Path, into: &Path) -> Result<()> {
    let file = fs::File::open(archive).with_context(|| format!("{} cannot be read", archive.display()))?;
    let mut zip = zip::ZipArchive::new(file).with_context(|| format!("{} is not a readable zip", archive.display()))?;
    let mut left = MAX_UNPACKED;
    for i in 0..zip.len() {
        // LZMA first: the library hands this method's four-byte zip header to the decoder as if it were LZMA data.
        let lzma = zip.by_index_raw(i)?.compression() == zip::CompressionMethod::Lzma;
        if lzma {
            let mut entry = zip.by_index_raw(i)?;
            let name = entry.name().to_string();
            let Some(relative) = safe_relative(&name) else {
                bail!("{name}: a path in the archive leaves its folder");
            };
            let target = into.join(relative);
            let (size, crc) = (entry.size(), entry.crc32());
            if size > left {
                bail!("the archive unpacks to more than {MAX_UNPACKED} bytes");
            }
            let mut packed = Vec::new();
            entry.read_to_end(&mut packed).with_context(|| format!("{name} cannot be read"))?;
            let plain = unzip_lzma(&packed, size).with_context(|| format!("{name} cannot be unpacked"))?;
            if crc32fast::hash(&plain) != crc {
                bail!("{name} is damaged in the archive");
            }
            write_limited(&mut plain.as_slice(), &target, &mut left)?;
            continue;
        }
        let mut entry = zip.by_index(i)?;
        let name = entry.name().to_string();
        let Some(relative) = safe_relative(&name) else {
            bail!("{name}: a path in the archive leaves its folder");
        };
        let target = into.join(relative);
        if entry.is_symlink() {
            continue;
        }
        if entry.is_dir() {
            fs::create_dir_all(&target).with_context(|| format!("{} cannot be created", target.display()))?;
            continue;
        }
        write_limited(&mut entry, &target, &mut left)?;
    }
    Ok(())
}

/// A zip entry's LZMA: version (2 bytes), size of the properties (2), the properties, then the stream. The stream may or
/// may not end in a marker, so the size comes from the zip's own record.
fn unzip_lzma(packed: &[u8], size: u64) -> Result<Vec<u8>> {
    let Some(head) = packed.get(2..4) else {
        bail!("too short");
    };
    if u16::from_le_bytes([head[0], head[1]]) != 5 || packed.len() < 9 {
        bail!("not the LZMA a zip holds");
    }
    let options = lzma_rs::decompress::Options {
        unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(size)),
        memlimit: None,
        allow_incomplete: false,
    };
    let mut plain = Vec::with_capacity(size.min(1 << 30) as usize);
    let mut stream = &packed[4..];
    lzma_rs::lzma_decompress_with_options(&mut stream, &mut plain, &options).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    Ok(plain)
}

fn un7z(archive: &Path, into: &Path) -> Result<()> {
    let mut left = MAX_UNPACKED;
    sevenz_rust2::decompress_file_with_extract_fn(archive, into, |entry, reader, _| {
        // The library joins the archive's names as they are: each goes through the same check as a zip's.
        let Some(relative) = safe_relative(entry.name()) else {
            return Err(sevenz_rust2::Error::other(format!("{}: a path in the archive leaves its folder", entry.name())));
        };
        let target = into.join(relative);
        if entry.is_directory() {
            fs::create_dir_all(&target)?;
        } else {
            write_limited(reader, &target, &mut left).map_err(|e| sevenz_rust2::Error::other(format!("{e:#}")))?;
        }
        Ok(true)
    })
    .with_context(|| format!("{} is not a readable 7z", archive.display()))
}

/// The commands that can read a `.rar`, in the order to try: the system's own first, never a program of that name that
/// happens to come first on the path. Windows 10 and later and macOS have bsdtar as `tar`; Linux has it as `bsdtar`.
fn rar_readers() -> Vec<PathBuf> {
    if cfg!(windows) {
        let root = std::env::var_os("SystemRoot").map_or_else(|| PathBuf::from("C:\\Windows"), PathBuf::from);
        vec![root.join("System32").join("tar.exe")]
    } else if cfg!(target_os = "macos") {
        vec![PathBuf::from("/usr/bin/tar"), PathBuf::from("/usr/bin/bsdtar")]
    } else {
        vec![PathBuf::from("/usr/bin/bsdtar"), PathBuf::from("/bin/bsdtar")]
    }
}

/// A `.rar` through the system's bsdtar, which keeps paths inside the folder and won't write through a link. What it
/// says it holds is checked first, and what it wrote after.
fn unrar(archive: &Path, into: &Path) -> Result<()> {
    let mut last = String::from("reading a .rar needs bsdtar (libarchive), which isn't on this system");
    for reader in rar_readers() {
        let Ok(listing) = std::process::Command::new(&reader).arg("-tvf").arg(archive).output() else { continue };
        if !listing.status.success() {
            last = String::from_utf8_lossy(&listing.stderr).trim().to_string();
            continue;
        }
        let stated: u64 = String::from_utf8_lossy(&listing.stdout)
            .lines()
            .filter_map(|line| line.split_whitespace().nth(4).and_then(|size| size.parse::<u64>().ok()))
            .fold(0, u64::saturating_add);
        if stated > MAX_UNPACKED {
            bail!("the archive unpacks to more than {MAX_UNPACKED} bytes");
        }
        let output = std::process::Command::new(&reader).arg("-xf").arg(archive).arg("-C").arg(into).output().with_context(|| format!("{} cannot be started", reader.display()))?;
        if !output.status.success() {
            bail!("{} cannot be unpacked: {}", archive.display(), String::from_utf8_lossy(&output.stderr).trim());
        }
        let root = fs::canonicalize(into)?;
        let mut total = 0u64;
        for entry in walkdir::WalkDir::new(into) {
            let entry = entry?;
            if !fs::canonicalize(entry.path())?.starts_with(&root) {
                bail!("{}: a path in the archive leaves its folder", entry.path().display());
            }
            if entry.file_type().is_file() {
                total = total.saturating_add(entry.metadata()?.len());
            }
        }
        if total > MAX_UNPACKED {
            bail!("the archive unpacks to more than {MAX_UNPACKED} bytes");
        }
        return Ok(());
    }
    bail!("{} cannot be unpacked: {last}", archive.display())
}

/// A mod's name from its file or folder name.
pub fn mod_name(input: &Path) -> String {
    input
        .file_stem()
        .or_else(|| input.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "mod".to_string())
}

/// Whether a file name is an archive this tool unpacks.
pub fn is_archive(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref(),
        Some("zip" | "7z" | "rar")
    )
}

/// A disc image of the game.
pub fn is_disc(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref(),
        Some("iso" | "wbfs" | "rvz" | "wia" | "ciso" | "gcz" | "nfs")
    )
}

/// A folder holding a whole game, as an emulator extracts one.
fn is_extracted_disc(path: &Path) -> bool {
    path.join("sys").join("main.dol").is_file() && path.join("files").is_dir()
        || path.join("DATA").join("sys").join("main.dol").is_file() && path.join("DATA").join("files").is_dir()
}

/// The game as a disc or a folder: to compare a mod that's a whole changed game with.
static BASE_GAME: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// The disc files Setup keeps beside the imported game, as the base when none was named.
pub fn base_from_store(store: &Path) {
    let inputs = store.parent().map(|p| p.join("game-inputs"));
    if let Some(inputs) = inputs.filter(|p| p.join("sys").join("main.dol").is_file()) {
        let mut base = BASE_GAME.lock().unwrap_or_else(|e| e.into_inner());
        if base.is_none() {
            *base = Some(inputs);
        }
    }
}

/// What `--base` names, when it's a whole game.
pub fn set_base_game(path: Option<PathBuf>) {
    *BASE_GAME.lock().unwrap_or_else(|e| e.into_inner()) = path;
}

/// A mod that's a whole changed game: only what differs from the game's own, in the folders of the disc, with its program beside them.
fn changes_from_game(input: &Path, work: &Path) -> Result<PathBuf> {
    use nebula_import::cook::store;
    let Some(base) = BASE_GAME.lock().unwrap_or_else(|e| e.into_inner()).clone() else {
        bail!("{}: this is a whole game. To take only what the mod changes, name the game's own disc with --base", input.display());
    };
    let to_store = |from: &Path, name: &str| -> Result<PathBuf> {
        let pak = work.join(name);
        let mut partition = nebula_import::Partition::open(from).with_context(|| format!("{} can't be read as this game", from.display()))?;
        partition.write_pak(&pak, &mut |_, _| true)?;
        Ok(pak)
    };
    let changed = to_store(input, "changed.pak")?;
    let original = to_store(&base, "original.pak")?;
    let into = work.join("changes");
    fs::create_dir_all(&into)?;
    let mut count = 0usize;
    for folder in crate::layout::DISC_FOLDERS {
        let mut hashes = std::collections::BTreeMap::new();
        let _ = store::visit_folder(&original, folder, &mut |path, bytes| {
            hashes.insert(path.to_ascii_lowercase(), (bytes.len(), nebula_import::cook::texture::xxh64(&bytes)));
        });
        let _ = store::visit_folder(&changed, folder, &mut |path, bytes| {
            let same = hashes.get(&path.to_ascii_lowercase()).is_some_and(|(size, hash)| *size == bytes.len() && *hash == nebula_import::cook::texture::xxh64(&bytes));
            if !same {
                let target = into.join(folder).join(path);
                if let Some(parent) = target.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                if fs::write(target, &bytes).is_ok() {
                    count += 1;
                }
            }
        });
    }
    // The program, for what it changes.
    for name in ["sys/main.dol", "main.dol"] {
        if let (Ok(new), Ok(old)) = (store::read_file(&changed, name), store::read_file(&original, name)) {
            if new != old {
                fs::write(into.join("main.dol"), new)?;
                let kept = work.join("original.dol");
                fs::write(&kept, old)?;
                crate::plan::set_base_program(Some(kept));
                count += 1;
            }
            break;
        }
    }
    fs::remove_file(changed).ok();
    fs::remove_file(original).ok();
    if count == 0 {
        bail!("{}: it's the same as the game's own disc", input.display());
    }
    Ok(into)
}

/// Opens a mod: an archive unpacks under `work`, a folder is used in place and never written to.
pub fn open(input: &Path, work: &Path) -> Result<PathBuf> {
    if input.is_dir() {
        if is_extracted_disc(input) {
            return changes_from_game(input, work);
        }
        return Ok(input.to_path_buf());
    }
    if !input.is_file() {
        bail!("{} does not exist", input.display());
    }
    if is_disc(input) {
        return changes_from_game(input, work);
    }
    if matches!(input.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref(), Some("xdelta" | "vcdiff" | "bps" | "ips" | "ppf")) {
        bail!("{}: a patch for a disc image. Apply it to your disc with its own tool, then give LumaLink the result and your original with --base", input.display());
    }
    if !is_archive(input) {
        bail!("{}: a mod is a folder, a .zip, .7z or .rar archive, or a changed disc image", input.display());
    }
    // The file's name, when it's a name: `...zip` is a real file, and its stem is the folder above.
    let into = work.join(safe_relative(&mod_name(input)).filter(|p| p.components().count() == 1).unwrap_or_else(|| PathBuf::from("mod")));
    if into.exists() {
        fs::remove_dir_all(&into).with_context(|| format!("{} cannot be cleared", into.display()))?;
    }
    fs::create_dir_all(&into).with_context(|| format!("{} cannot be created", into.display()))?;
    match input.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("zip") => unzip(input, &into)?,
        Some("7z") => un7z(input, &into)?,
        _ => unrar(input, &into)?,
    }
    Ok(into)
}

#[cfg(test)]
mod tests {
    /// A zip with one LZMA entry, written as 7-Zip writes one: zip header, properties, stream with an end marker.
    #[test]
    fn unpacks_lzma_entries() {
        let text = b"<wiidisc version=\"1\">the same words, the same words, the same words</wiidisc>".to_vec();
        let mut lzma = Vec::new();
        lzma_rs::lzma_compress(&mut text.as_slice(), &mut lzma).unwrap();
        // lzma_compress writes properties (5), size (8), stream.
        let mut data = vec![0x15, 0x07, 5, 0];
        data.extend_from_slice(&lzma[..5]);
        data.extend_from_slice(&lzma[13..]);
        let name = b"a/b.xml";
        let crc = crc32fast::hash(&text);
        let mut record = Vec::new();
        record.extend_from_slice(&63u16.to_le_bytes());
        record.extend_from_slice(&2u16.to_le_bytes());
        record.extend_from_slice(&14u16.to_le_bytes());
        record.extend_from_slice(&[0, 0, 0x21, 0]);
        record.extend_from_slice(&crc.to_le_bytes());
        record.extend_from_slice(&(data.len() as u32).to_le_bytes());
        record.extend_from_slice(&(text.len() as u32).to_le_bytes());
        record.extend_from_slice(&(name.len() as u16).to_le_bytes());
        record.extend_from_slice(&0u16.to_le_bytes());
        let mut zip = b"PK".to_vec();
        zip.extend_from_slice(&record);
        zip.extend_from_slice(name);
        zip.extend_from_slice(&data);
        let central = zip.len() as u32;
        zip.extend_from_slice(b"PK");
        zip.extend_from_slice(&63u16.to_le_bytes());
        zip.extend_from_slice(&record);
        zip.extend_from_slice(&[0u8; 10]);
        zip.extend_from_slice(&0u32.to_le_bytes());
        zip.extend_from_slice(name);
        let size = zip.len() as u32 - central;
        zip.extend_from_slice(b"PK");
        zip.extend_from_slice(&[0, 0, 0, 0, 1, 0, 1, 0]);
        zip.extend_from_slice(&size.to_le_bytes());
        zip.extend_from_slice(&central.to_le_bytes());
        zip.extend_from_slice(&[0, 0]);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.zip");
        std::fs::write(&path, &zip).unwrap();
        let out = dir.path().join("out");
        super::unzip(&path, &out).unwrap();
        assert_eq!(std::fs::read(out.join("a/b.xml")).unwrap(), text);
    }

    use super::*;

    #[test]
    fn a_path_cannot_leave_the_folder() {
        assert_eq!(safe_relative("a/b.arc"), Some(PathBuf::from("a/b.arc")));
        assert_eq!(safe_relative("a\\b.arc"), Some(PathBuf::from("a/b.arc")));
        assert_eq!(safe_relative("./a"), Some(PathBuf::from("a")));
        assert_eq!(safe_relative("../a"), None);
        assert_eq!(safe_relative("a/../../b"), None);
        assert_eq!(safe_relative("/a"), None);
        assert_eq!(safe_relative("C:\\a"), None);
        assert_eq!(safe_relative("\\a"), None);
        assert_eq!(safe_relative("a:stream"), None);
        assert_eq!(safe_relative("a/CON"), None);
        assert_eq!(safe_relative("aux.txt"), None);
        assert_eq!(safe_relative("com7.bin"), None);
        assert_eq!(safe_relative("a/.../b"), None);
        assert_eq!(safe_relative("a/b\u{1}c"), None);
        assert_eq!(safe_relative("console.arc"), Some(PathBuf::from("console.arc")));
    }

    #[test]
    fn a_zip_unpacks_and_a_bad_one_stops() {
        let dir = tempfile::tempdir().unwrap();
        let archive = dir.path().join("My Mod.zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
        zip.start_file("ObjectData/a.arc", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"hello").unwrap();
        zip.finish().unwrap();
        let opened = open(&archive, dir.path()).unwrap();
        assert_eq!(fs::read(opened.join("ObjectData/a.arc")).unwrap(), b"hello");
        assert_eq!(mod_name(&archive), "My Mod");

        let bad = dir.path().join("bad.zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&bad).unwrap());
        zip.start_file("../evil.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"x").unwrap();
        zip.finish().unwrap();
        assert!(open(&bad, dir.path()).is_err());
        assert!(!dir.path().join("evil.txt").exists());
    }

    #[test]
    fn a_file_named_after_the_folder_above_stays_inside() {
        let dir = tempfile::tempdir().unwrap();
        let work = dir.path().join("work");
        fs::create_dir_all(&work).unwrap();
        let keep = dir.path().join("keep.txt");
        fs::write(&keep, b"x").unwrap();
        let archive = dir.path().join("...zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
        zip.start_file("a.bin", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"hello").unwrap();
        zip.finish().unwrap();
        let opened = open(&archive, &work).unwrap();
        assert!(opened.starts_with(&work));
        assert!(keep.is_file());
    }

    #[test]
    fn what_an_entry_really_holds_is_what_counts() {
        let dir = tempfile::tempdir().unwrap();
        let mut left = 4u64;
        assert!(write_limited(&mut &b"12345"[..], &dir.path().join("big"), &mut left).is_err());
        let mut left = 5u64;
        write_limited(&mut &b"12345"[..], &dir.path().join("fits"), &mut left).unwrap();
        assert_eq!(left, 0);
    }

    #[test]
    fn a_link_in_a_zip_is_left_out() {
        let dir = tempfile::tempdir().unwrap();
        let archive = dir.path().join("link.zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
        zip.add_symlink("out", "../..", zip::write::SimpleFileOptions::default()).unwrap();
        zip.start_file("in.bin", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"x").unwrap();
        zip.finish().unwrap();
        let into = dir.path().join("into");
        unzip(&archive, &into).unwrap();
        assert!(into.join("in.bin").is_file());
        assert!(fs::symlink_metadata(into.join("out")).is_err());
    }

    #[test]
    fn a_folder_is_used_in_place_and_others_refused() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(open(dir.path(), dir.path()).unwrap(), dir.path());
        let text = dir.path().join("a.txt");
        fs::write(&text, b"x").unwrap();
        assert!(open(&text, dir.path()).is_err());
        assert!(open(&dir.path().join("none"), dir.path()).is_err());
    }
}
