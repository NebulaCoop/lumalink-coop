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

/// Opens a mod: an archive unpacks under `work`, a folder is used in place and never written to.
pub fn open(input: &Path, work: &Path) -> Result<PathBuf> {
    if input.is_dir() {
        return Ok(input.to_path_buf());
    }
    if !input.is_file() {
        bail!("{} does not exist", input.display());
    }
    if !is_archive(input) {
        bail!("{}: a mod is a folder or a .zip, .7z or .rar archive", input.display());
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
