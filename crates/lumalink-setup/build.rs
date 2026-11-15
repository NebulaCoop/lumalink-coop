//! Packs the files the Setup installs into one archive it carries (`OUT_DIR/payload.zip`, each file Zstandard level
//! 19, with its Unix permissions): the folder `LUMALINK_SETUP_PAYLOAD` names, laid out as installed
//! (installer/build-setup.ps1 and installer/build-setup.sh make it). Links in it are left out: the Setup makes lld's
//! other names itself. Without it the archive is empty: a Setup for the tests, which refuses to install.
//!
//! LumaLink's icon (lumalink-gui's icon.rs draws it) as PNGs of 128, 256 and 512 for Linux's menu and macOS's bundle.
//! On Windows the program gets a manifest that runs it as the user who started it (no administrator; Windows'
//! installer detection would otherwise ask for one for a program named "Setup"), and the icon.

include!("../lumalink-gui/icon.rs");

use std::io::Write;
use std::path::{Path, PathBuf};

fn files(folder: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(folder).unwrap_or_else(|e| panic!("{}: {e}", folder.display())).flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        let kind = entry.file_type().unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        if kind.is_dir() {
            files(&path, out);
        } else if kind.is_file() {
            out.push(path);
        }
    }
}

/// A file's Unix permissions: its own where the build runs on Unix; on Windows the programs (`.exe`, and clang and lld
/// under `toolchain/bin`) run.
fn mode(file: &Path) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(file).map_or(0o644, |m| if m.permissions().mode() & 0o111 != 0 { 0o755 } else { 0o644 })
    }
    #[cfg(not(unix))]
    {
        let program = file.extension().is_some_and(|e| e == "exe") || file.parent().is_some_and(|p| p.ends_with("toolchain/bin"));
        if program { 0o755 } else { 0o644 }
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=LUMALINK_SETUP_PAYLOAD");
    println!("cargo:rerun-if-changed=../lumalink-gui/icon.rs");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("payload.zip");
    for size in [128, 256, 512] {
        std::fs::write(out.with_file_name(format!("icon-{size}.png")), icon::png(size)).unwrap();
    }
    let windows = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    let exe = if windows { ".exe" } else { "" };
    let mut zip = zip::ZipWriter::new(std::fs::File::create(&out).unwrap());
    if let Some(folder) = std::env::var_os("LUMALINK_SETUP_PAYLOAD").map(PathBuf::from).filter(|p| !p.as_os_str().is_empty()) {
        println!("cargo:rerun-if-changed={}", folder.display());
        if !folder.join(format!("lumalink{exe}")).is_file() || !folder.join(format!("lumalink-gui{exe}")).is_file() {
            panic!("LUMALINK_SETUP_PAYLOAD {} has no lumalink{exe} and lumalink-gui{exe}", folder.display());
        }
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Zstd)
            .compression_level(Some(19))
            .large_file(false);
        let mut list = Vec::new();
        files(&folder, &mut list);
        for file in list {
            let name = file.strip_prefix(&folder).unwrap().to_string_lossy().replace('\\', "/");
            zip.start_file(name, options.unix_permissions(mode(&file))).unwrap();
            zip.write_all(&std::fs::read(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()))).unwrap();
        }
    }
    zip.finish().unwrap();
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:level='asInvoker'");
        let res = out.with_file_name("icon.res");
        let mut bytes = icon::res(1);
        icon::add_details(&mut bytes, "LumaLink Setup", "LumaLink-Setup.exe", env!("CARGO_PKG_VERSION"));
        std::fs::write(&res, bytes).unwrap();
        println!("cargo:rustc-link-arg-bins={}", res.display());
    }
}
