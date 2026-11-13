//! What authors work with: a project folder laid out as a `.luma` is, and tools to pack, check and open them. The rules are
//! in docs/format.md; `check` holds a file to all of them.

use crate::game_store::GameStore;
use crate::known_patches;
use crate::package::{self, Outcome};
use crate::PACKAGE_FORMAT;
use anyhow::{bail, Context, Result};
use nebula_import::cook::overlay::Change;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const VERSION_LETTERS: [&str; 6] = ["E", "P", "J", "K", "W", "C"];
const PLATFORMS: [&str; 5] = ["windows", "android", "linux", "macos", "ios"];
const WHEN_KEYS: [&str; 7] = ["version", "language", "platform", "option", "mods", "not", "any"];
const LANGUAGES: [&str; 14] = ["en", "en-GB", "fr", "fr-CA", "es", "es-MX", "de", "it", "nl", "pt", "ja", "ko", "zh-Hans", "zh-Hant"];

/// What Nebula reads as an id: it names a folder on every device (luma.cpp's valid_id).
pub(crate) fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && !id.starts_with('-')
        && !id.ends_with('-')
        && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !crate::plan::reserved_id(id)
}

pub(crate) fn valid_version(version: &str) -> bool {
    !version.is_empty() && version.split('.').all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// The plugin interface, for a project that has code.
const PLUGIN_HEADER: &str = include_str!("../sdk/nebula_mod_api.h");
const PLUGIN_SOURCE: &str = r#"/* A plugin: the game calls nebula_plugin_start once, before it starts. Everything it can do is in nebula_mod_api.h. */
#include "nebula_mod_api.h"

static const NebulaModApi* game;
static const NebulaPlugin* me;

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    (void)user;
    game->log(me, data->name);
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
}
"#;
const PLUGIN_BUILD_CMD: &str = r#"@echo off
rem Builds the plugin for every device the game runs on: Windows, Linux, Android and macOS, x64 and arm64 each.
rem It needs clang and lld, from https://llvm.org; nothing else.
lumalink build "%~dp0."
"#;
const PLUGIN_BUILD_SH: &str = r#"#!/bin/sh
# Builds the plugin for every device the game runs on: Windows, Linux, Android and macOS, x64 and arm64 each.
# It needs clang and lld, from https://llvm.org (a Mac: brew install llvm lld); nothing else.
set -e
lumalink build "$(dirname "$0")"
"#;

/// Every device a plugin is built for: its name in `binaries`, clang's target, and the library's ending. Android 7
/// (API 24) is the oldest the game runs on.
pub const BUILD_TARGETS: &[(&str, &str, &str)] = &[
    ("windows-x64", "x86_64-pc-windows-msvc", "dll"),
    ("windows-arm64", "aarch64-pc-windows-msvc", "dll"),
    ("linux-x64", "x86_64-linux-gnu", "so"),
    ("linux-arm64", "aarch64-linux-gnu", "so"),
    ("android-x64", "x86_64-linux-android24", "so"),
    ("android-arm64", "aarch64-linux-android24", "so"),
    ("macos-x64", "x86_64-apple-macos10.15", "dylib"),
    ("macos-arm64", "arm64-apple-macos11", "dylib"),
];

/// A library `lumalink build` made: its device and its path in the project.
pub struct BuiltLibrary {
    pub target: &'static str,
    pub file: String,
}

/// The library a project's plugin has for a device: plugins/<id>-<device>.<ending>.
fn library_path(id: &str, target: &str, ending: &str) -> String {
    format!("plugins/{id}-{target}.{ending}")
}

/// clang: the one named, else the one on the PATH, else LLVM's usual places.
pub fn find_clang(named: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = named {
        return Some(path.to_path_buf());
    }
    let exe = if cfg!(windows) { "clang.exe" } else { "clang" };
    let on_path = std::env::var_os("PATH").into_iter().flat_map(|p| std::env::split_paths(&p).collect::<Vec<_>>()).map(|d| d.join(exe));
    let usual = ["C:/Program Files/LLVM/bin/clang.exe", "/opt/homebrew/opt/llvm/bin/clang", "/usr/local/opt/llvm/bin/clang", "/usr/bin/clang"]
        .into_iter()
        .map(PathBuf::from);
    on_path.chain(usual).find(|p| p.is_file())
}

/// lld's linker for a library ending (lld-link, ld.lld or ld64.lld): beside clang, on the PATH, or in LLVM's and
/// Homebrew's usual places. clang is given its path, so an lld Homebrew didn't put on the PATH is used too.
fn find_lld(clang: &Path, ending: &str) -> Option<PathBuf> {
    let name = match ending {
        "dll" => "lld-link",
        "so" => "ld.lld",
        _ => "ld64.lld",
    };
    let exe = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    let beside = clang.parent().map(Path::to_path_buf);
    let on_path = std::env::var_os("PATH").into_iter().flat_map(|p| std::env::split_paths(&p).collect::<Vec<_>>());
    let usual = ["C:/Program Files/LLVM/bin", "/opt/homebrew/opt/lld/bin", "/opt/homebrew/opt/llvm/bin", "/usr/local/opt/lld/bin", "/usr/local/opt/llvm/bin", "/usr/bin"]
        .into_iter()
        .map(PathBuf::from);
    beside.into_iter().chain(on_path).chain(usual).map(|d| d.join(&exe)).find(|p| p.is_file())
}

/// Builds the project's plugin from source/*.c for every device in BUILD_TARGETS, with clang and lld alone: a plugin
/// calls the game through the table it is given and links no library, not even the C runtime. An undefined symbol
/// (a C library call, a compiler helper) stops the build, not the game. luma.json's first plugin then names each
/// library (it is made when the project has none).
pub fn build_plugin(folder: &Path, clang: Option<&Path>) -> Result<Vec<BuiltLibrary>> {
    let mut manifest = read_manifest(folder)?;
    let id = manifest.get("id").and_then(Value::as_str).context("the project has no id")?.to_string();
    let mut sources: Vec<PathBuf> = fs::read_dir(folder.join("source"))
        .with_context(|| format!("{} has no source folder", folder.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("c")))
        .collect();
    sources.sort();
    if sources.is_empty() {
        bail!("source/ has no .c file");
    }
    let clang = find_clang(clang).context("clang wasn't found: install LLVM (clang and lld) from https://llvm.org, or name it with --clang")?;
    let mut linkers = Vec::new();
    for &(_, _, ending) in BUILD_TARGETS {
        let lld = find_lld(&clang, ending)
            .with_context(|| format!("lld wasn't found beside {}: install it (https://llvm.org; a Mac: brew install lld)", clang.display()))?;
        linkers.push(lld);
    }
    fs::create_dir_all(folder.join("plugins"))?;
    let mut built = Vec::new();
    for (&(target, triple, ending), lld) in BUILD_TARGETS.iter().zip(&linkers) {
        let file = library_path(&id, target, ending);
        let mut command = std::process::Command::new(&clang);
        command
            .arg(format!("--target={triple}"))
            .args(["-O2", "-shared", "-nostdlib", "-ffreestanding", "-fno-stack-protector", "-fvisibility=hidden", "-fuse-ld=lld"])
            .arg(format!("--ld-path={}", lld.display()))
            .arg("-I")
            .arg(folder.join("source"));
        match ending {
            "dll" => {
                command.arg("-Wl,/NOENTRY");
            }
            "so" => {
                command.args(["-fPIC", "-Wl,-z,defs"]);
            }
            _ => {}
        }
        command.arg("-o").arg(folder.join(&file)).args(&sources);
        let output = command.output().with_context(|| format!("{} didn't start", clang.display()))?;
        if !output.status.success() {
            bail!("{target}: clang said:\n{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
        built.push(BuiltLibrary { target, file });
    }
    let binaries: serde_json::Map<String, Value> = built.iter().map(|b| (b.target.to_string(), Value::from(b.file.clone()))).collect();
    let plugins = manifest.as_object_mut().context("luma.json isn't an object")?.entry("plugins").or_insert_with(|| json!([]));
    let list = plugins.as_array_mut().context("plugins is a list")?;
    if list.is_empty() {
        list.push(json!({ "api": PLUGIN_API }));
    }
    let first = list[0].as_object_mut().context("luma.json: the first of \"plugins\" isn't a set of fields")?;
    first.insert("binaries".to_string(), Value::Object(binaries));
    fs::write(folder.join("luma.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(built)
}

/// Adds code to a project just started: its source, what builds it, and the manifest's entry.
pub fn add_plugin(folder: &Path) -> Result<()> {
    let mut manifest = read_manifest(folder)?;
    let id = manifest.get("id").and_then(Value::as_str).context("the project has no id")?.to_string();
    fs::create_dir_all(folder.join("source"))?;
    fs::create_dir_all(folder.join("plugins"))?;
    fs::write(folder.join("source/nebula_mod_api.h"), PLUGIN_HEADER)?;
    fs::write(folder.join("source/plugin.c"), PLUGIN_SOURCE)?;
    fs::write(folder.join("build.cmd"), PLUGIN_BUILD_CMD.replace("{id}", &id))?;
    fs::write(folder.join("build.sh"), PLUGIN_BUILD_SH.replace("{id}", &id))?;
    let binaries: serde_json::Map<String, Value> =
        BUILD_TARGETS.iter().map(|&(target, _, ending)| (target.to_string(), Value::from(library_path(&id, target, ending)))).collect();
    manifest["plugins"] = json!([{ "api": PLUGIN_API, "binaries": binaries }]);
    fs::write(folder.join("luma.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(())
}

/// Writes each material's pixel program as Nebula generates it, as `<material>.hlsl` beside the output with the headers it
/// includes, from a model (the game's, or a mod's .glb or .gltf, as Nebula converts it) or an archive holding one: the
/// start of a mod's own program (a layer's `"materials"`), which takes the place of the plain form's. What's returned:
/// the files.
pub fn write_generated_shaders(model: &Path, output: &Path) -> Result<Vec<String>> {
    use nebula_import::cook::model_ir::{read_file, Source};
    use nebula_import::cook::programs::{self, Form};
    use nebula_import::cook::{archive, gltf, Stats};
    let mut bytes = fs::read(model).with_context(|| format!("{} can't be read", model.display()))?;
    let extension = model.extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase();
    let ir = if extension == "glb" || extension == "gltf" {
        // The model as Nebula's conversion makes it, which its programs are written from.
        gltf::to_ir(&bytes, model.parent()).with_context(|| format!("{} can't be converted", model.display()))?.0
    } else {
        if archive::is_compressed(&bytes) {
            bytes = archive::expand(&bytes)?;
        }
        let is_model = |b: &[u8]| b.starts_with(b"J3D2bmd3") || b.starts_with(b"J3D2bdl4");
        if archive::is_archive(&bytes) {
            let members = archive::members(&bytes)?;
            bytes = members.into_iter().map(|m| m.bytes).find(|b| is_model(b)).context("the archive holds no model")?;
        }
        if !is_model(&bytes) {
            bail!("{} isn't a model (.bdl, .bmd, .glb, .gltf, or an archive with one)", model.display());
        }
        read_file(&bytes, Source::of(&bytes), &mut Stats::default())?
    };
    let written = programs::written_or_unknown();
    fs::create_dir_all(output)?;
    for (name, text) in programs::HEADERS {
        fs::write(output.join(name), text)?;
    }
    let mut files = Vec::new();
    for m in &ir.materials {
        let text = programs::generate(m, Form::default(), programs::mesh_skinned(&ir, m), &written.registers(m))
            .with_context(|| format!("material {}", m.name))?;
        let safe: String = m.name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect();
        let file = format!("{safe}.hlsl");
        fs::write(output.join(&file), text.pixel)?;
        files.push(file);
    }
    Ok(files)
}

/// The names of the materials of every model in the layers' files/: .glb and .gltf as they are named there, .bdl and
/// .bmd (alone or in an archive) as the model holds them.
fn mod_material_names(folder: &Path, layers: &[Value]) -> std::collections::BTreeSet<String> {
    use nebula_import::cook::model_ir::{read_file, Source};
    use nebula_import::cook::{archive, Stats};
    let mut names = std::collections::BTreeSet::new();
    let from_model = |bytes: &[u8], names: &mut std::collections::BTreeSet<String>| {
        if bytes.starts_with(b"J3D2bmd3") || bytes.starts_with(b"J3D2bdl4") {
            if let Ok(ir) = read_file(bytes, Source::of(bytes), &mut Stats::default()) {
                names.extend(ir.materials.iter().map(|m| m.name.clone()));
            }
        }
    };
    for layer in layers {
        let Some(path) = layer.get("path").and_then(Value::as_str).filter(|p| inside(p)) else { continue };
        for entry in walkdir::WalkDir::new(folder.join(path).join("files")).into_iter().filter_map(std::result::Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            let file = entry.file_name().to_string_lossy().to_ascii_lowercase();
            let Ok(mut bytes) = fs::read(entry.path()) else { continue };
            if file.ends_with(".glb") || file.ends_with(".gltf") {
                let json = if bytes.starts_with(b"glTF") && bytes.len() >= 20 {
                    let length = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
                    bytes.get(20..20 + length).and_then(|b| serde_json::from_slice::<Value>(b).ok())
                } else {
                    serde_json::from_slice::<Value>(&bytes).ok()
                };
                let materials = json.as_ref().and_then(|j| j.get("materials")).and_then(Value::as_array);
                names.extend(materials.into_iter().flatten().filter_map(|m| m.get("name").and_then(Value::as_str)).map(str::to_string));
                continue;
            }
            if archive::is_compressed(&bytes) {
                match archive::expand(&bytes) {
                    Ok(expanded) => bytes = expanded,
                    Err(_) => continue,
                }
            }
            if archive::is_archive(&bytes) {
                for member in archive::members(&bytes).unwrap_or_default() {
                    from_model(&member.bytes, &mut names);
                }
            } else {
                from_model(&bytes, &mut names);
            }
        }
    }
    names
}

/// Starts a project: the folder, its `luma.json`, and the folders a mod usually fills.
pub fn new_project(folder: &Path, name: &str) -> Result<()> {
    if folder.join("luma.json").exists() {
        bail!("{} already has a luma.json", folder.display());
    }
    for sub in ["layers/base/files/ObjectData", "layers/base/files/StageData", "layers/base/textures"] {
        fs::create_dir_all(folder.join(sub)).with_context(|| format!("{} can't be created", folder.display()))?;
    }
    let manifest = json!({
        "format": PACKAGE_FORMAT,
        "id": crate::plan::mod_id(name),
        "name": name,
        "version": "1.0.0",
        "game": crate::plan::ANY_VERSION,
        "regions": "any",
        "authors": [],
        "layers": [{ "path": "layers/base" }],
    });
    fs::write(folder.join("luma.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(())
}

fn read_manifest(folder: &Path) -> Result<Value> {
    let path = folder.join("luma.json");
    let text = fs::read_to_string(&path).with_context(|| format!("{} can't be read", path.display()))?;
    serde_json::from_str(text.trim_start_matches('\u{feff}')).with_context(|| format!("{} has a mistake", path.display()))
}

fn layer_changes(folder: &Path, layer: &Value) -> Vec<Change> {
    let Some(path) = layer.get("path").and_then(Value::as_str) else {
        return Vec::new();
    };
    let files = folder.join(path).join("files");
    walkdir::WalkDir::new(&files)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let relative = e.path().strip_prefix(&files).ok()?.to_string_lossy().replace('\\', "/");
            // A system's or version control's hidden files (.DS_Store, .gitkeep) are not the mod's.
            if relative.split('/').any(|part| part.starts_with('.')) {
                return None;
            }
            Some(Change { path: relative, source: e.path().to_path_buf(), merge: false })
        })
        .collect()
}

/// What of a project goes in its file: not version control or a system's hidden files, not a file packed from it before,
/// not what building a plugin leaves beside its libraries in plugins/ (import libraries, objects, debug data).
fn belongs_in_file(relative: &Path) -> bool {
    let hidden = relative.components().any(|c| c.as_os_str().to_string_lossy().starts_with('.'));
    let packed = relative.components().count() == 1 && relative.extension().is_some_and(|e| e.eq_ignore_ascii_case("luma") || e.eq_ignore_ascii_case("lumacoop") || e.eq_ignore_ascii_case("writing"));
    let build_leftover = relative.components().next().is_some_and(|c| c.as_os_str().eq_ignore_ascii_case("plugins"))
        && relative.extension().is_some_and(|e| ["lib", "exp", "pdb", "ilk", "obj", "o"].iter().any(|x| e.eq_ignore_ascii_case(x)));
    let script_source = relative.components().next().is_some_and(|c| c.as_os_str().eq_ignore_ascii_case("scripts"))
        && relative.extension().is_some_and(|e| e.eq_ignore_ascii_case("lscript"));
    !hidden && !packed && !build_leftover && !script_source
}

/// A script of the project (scripts/<name>.lscript) compiled for the game, or why it can't be.
pub fn compile_script(folder: &Path, script: &str, toolkit: &crate::lumascript::Toolkit) -> std::result::Result<Vec<u8>, String> {
    let ok_name = !script.is_empty()
        && !script.starts_with('/')
        && !script.ends_with('/')
        && !script.contains("//")
        && script.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'/');
    if !ok_name {
        return Err(format!("{script} is not a script's name (letters, digits, _ and -, in folders split by /)"));
    }
    let file = format!("scripts/{script}.lscript");
    let text = fs::read_to_string(folder.join(&file)).map_err(|_| format!("{file} can't be read"))?;
    crate::lumascript::compile(&text, toolkit).map_err(|p| format!("{file}, line {}: {}", p.line, p.text))
}

/// Whether a manifest is a co-op mod's: its `requires` lists `coop`, a feature only NebulaCoop has.
pub fn is_coop(manifest: &Value) -> bool {
    manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "coop"))
}

/// The packed file's extension: `lumacoop` for a co-op mod (NebulaCoop's own; upstream Nebula refuses it), else `luma`.
pub fn file_extension(manifest: &Value) -> &'static str {
    if is_coop(manifest) { "lumacoop" } else { "luma" }
}

/// Packs a project folder into a `.luma`, or a `.lumacoop` for a co-op mod. A problem `check` calls serious stops it.
pub fn pack(folder: &Path, output: &Path, store: Option<&GameStore>) -> Result<Outcome> {
    let found = check(folder, store)?;
    if let Some(serious) = found.iter().find(|f| f.serious) {
        bail!("{}", serious.text);
    }
    let manifest = read_manifest(folder)?;
    let name = manifest.get("name").map_or_else(String::new, |n| text_of(n).to_string());
    let safe = package::file_stem_for(&name);
    fs::create_dir_all(output).with_context(|| format!("{} cannot be created", output.display()))?;
    let building = output.join(format!(".{safe}.building"));
    if building.exists() {
        fs::remove_dir_all(&building)?;
    }
    fs::create_dir_all(&building)?;
    let result = (|| {
        let mut jobs = Vec::new();
        let layers = manifest.get("layers").and_then(Value::as_array).cloned().unwrap_or_default();
        for (i, layer) in layers.iter().enumerate() {
            jobs.push((format!("layer{i}"), layer_changes(folder, layer)));
        }
        let game_id = store.map_or("RMGE01".to_string(), |s| s.game_id.clone());
        let (programs, failed, converted) = package::check_layers(&jobs, &game_id, store, &building)?;
        if let Some((path, why)) = failed.first() {
            bail!("{path}: {why}");
        }
        let id = manifest.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
        let toolkit = crate::lumascript::Toolkit::bundled();
        for layer in &layers {
            let Some(path) = layer.get("path").and_then(Value::as_str) else {
                continue;
            };
            // The objects', classes' and forms' scripts and the mod-wide ones ("scripts"), each compiled once.
            let mut scripts: Vec<&str> = ["objects", "object_classes", "forms"]
                .iter()
                .filter_map(|key| layer.get(*key).and_then(Value::as_array))
                .flatten()
                .filter_map(|r| r.get("script").and_then(Value::as_str))
                .collect();
            scripts.extend(layer.get("scripts").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str));
            scripts.sort_unstable();
            scripts.dedup();
            for script in scripts {
                let bytes = compile_script(folder, script, &toolkit).map_err(|why| anyhow::anyhow!(why))?;
                let target = building.join(path).join("files").join("LumaScripts").join(&id).join(format!("{script}.lumascript"));
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&target, bytes)?;
            }
        }
        for entry in walkdir::WalkDir::new(folder) {
            let entry = entry.with_context(|| format!("{} can't be read through", folder.display()))?;
            let relative = entry.path().strip_prefix(folder)?;
            if !entry.file_type().is_file() || !belongs_in_file(relative) {
                continue;
            }
            let target = building.join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
        let file = output.join(format!("{safe}.{}", file_extension(&manifest)));
        package::pack_folder(&building, &file)?;
        Ok(Outcome {
            file,
            converted,
            total: jobs.iter().map(|(_, c)| c.len()).sum(),
            programs,
            failed,
            report: String::new(),
        })
    })();
    let _ = fs::remove_dir_all(&building);
    result
}

/// One problem `check` found.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Finding {
    /// Whether the mod can't be published like this.
    pub serious: bool,
    pub text: String,
}

#[derive(Default)]
struct Findings(Vec<Finding>);

impl Findings {
    fn problem(&mut self, text: impl Into<String>) {
        self.0.push(Finding { serious: true, text: text.into() });
    }

    fn note(&mut self, text: impl Into<String>) {
        self.0.push(Finding { serious: false, text: text.into() });
    }
}

fn text_of(value: &Value) -> &str {
    match value {
        Value::Object(map) => map.get("default").and_then(Value::as_str).unwrap_or(""),
        other => other.as_str().unwrap_or(""),
    }
}

/// A piece of text shown to players: a string, or languages with a default.
fn check_text(value: &Value, field: &str, found: &mut Findings) {
    match value {
        Value::String(_) => {}
        Value::Object(map) => {
            if !map.get("default").is_some_and(Value::is_string) {
                found.problem(format!("{field} needs a \"default\""));
            }
            for (language, text) in map {
                if language != "default" && !LANGUAGES.contains(&language.as_str()) {
                    found.problem(format!("{field}: {language} isn't a language this format knows ({})", LANGUAGES.join(", ")));
                }
                if !text.is_string() {
                    found.problem(format!("{field}: {language} has to be text"));
                }
            }
        }
        _ => found.problem(format!("{field} has to be text")),
    }
}

/// The pictures a mod's page shows besides its icon (the game's `kMaxPictures`).
const MAX_PICTURES: usize = 16;
/// The largest side of a picture the game reads.
const MAX_PICTURE_SIDE: u32 = 8192;
/// The largest picture file the game reads.
const MAX_PICTURE_BYTES: u64 = 64 * 1024 * 1024;

/// A PNG's size from its header; None when the bytes aren't a PNG.
fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
    if bytes.len() < 24 || bytes[..8] != SIGNATURE || &bytes[12..16] != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    Some((width, height))
}

/// The icon or a picture as the game reads it: a PNG in the file, at most 8192 pixels a side and 64 MB. The game shows
/// pictures at most 1024 pixels a side: a larger one is shown at a half, quarter... of its size.
fn check_picture(folder: &Path, path: &str, what: &str, found: &mut Findings) {
    let file = folder.join(path);
    let Ok(meta) = fs::metadata(&file) else {
        found.problem(format!("{what} isn't in the file"));
        return;
    };
    if !meta.is_file() {
        found.problem(format!("{what} isn't in the file"));
        return;
    }
    if meta.len() > MAX_PICTURE_BYTES {
        found.problem(format!("{what} is over 64 MB: the game doesn't read it"));
        return;
    }
    let mut head = [0u8; 24];
    let read = fs::File::open(&file).and_then(|mut f| std::io::Read::read_exact(&mut f, &mut head));
    match read.ok().and_then(|_| png_size(&head)) {
        None => found.problem(format!("{what} has to be a PNG")),
        Some((width, height)) if width == 0 || height == 0 || width > MAX_PICTURE_SIDE || height > MAX_PICTURE_SIDE => {
            found.problem(format!("{what} is {width}x{height}: the game reads pictures up to {MAX_PICTURE_SIDE} pixels a side"))
        }
        Some((width, height)) if width > 1024 || height > 1024 => {
            found.note(format!("{what} is {width}x{height}: the game shows it at most 1024 pixels a side"))
        }
        Some(_) => {}
    }
}

/// A path inside the file, by the rule for an archive's names: going up, starting from a drive, a control character or a
/// part of dots and spaces (Windows drops a part's last dots and spaces) is refused, read the same on every system.
fn inside(path: &str) -> bool {
    !path.is_empty() && !path.starts_with('\\') && crate::source::safe_relative(path).is_some()
}

fn string_list<'a>(value: &'a Value, field: &str, found: &mut Findings) -> Vec<&'a str> {
    match value.as_array() {
        Some(list) => list
            .iter()
            .filter_map(|v| {
                let text = v.as_str();
                if text.is_none() {
                    found.problem(format!("{field} is a list of text"));
                }
                text
            })
            .collect(),
        None => {
            found.problem(format!("{field} is a list"));
            Vec::new()
        }
    }
}

/// The discs a mod is for: "any" (region-free), or a list of US, PAL, JP and KR. Nebula refuses a luma.json with
/// anything else, and blocks the mod on other discs, naming the ones it is for.
const REGIONS: [&str; 4] = ["US", "PAL", "JP", "KR"];

fn check_regions(manifest: &Value, found: &mut Findings) {
    let Some(regions) = manifest.get("regions") else {
        return;
    };
    if regions.as_str() == Some("any") {
        return;
    }
    if regions.is_string() {
        found.problem("regions is \"any\" or a list of the discs it is for, like [\"US\", \"PAL\"]");
        return;
    }
    let list = string_list(regions, "regions", found);
    if regions.as_array().is_some_and(Vec::is_empty) {
        found.problem("regions lists no disc: write \"any\" for every disc");
    }
    for region in list {
        if !REGIONS.contains(&region) {
            found.problem(format!("regions: {region} isn't a region ({})", REGIONS.join(", ")));
        }
    }
}

fn check_mod_list(value: &Value, field: &str, version_key: &str, found: &mut Findings) {
    let Some(list) = value.as_array() else {
        found.problem(format!("{field} is a list"));
        return;
    };
    for entry in list {
        if !entry.get("id").and_then(Value::as_str).is_some_and(valid_id) {
            found.problem(format!("{field}: every entry needs an id"));
        }
        if let Some(version) = entry.get(version_key) {
            if !version.as_str().is_some_and(valid_version) {
                found.problem(format!("{field}: {version_key} is numbers with dots, like 1.2.0"));
            }
        }
    }
}

/// The options, with their choices, for checking layers against.
fn check_options(manifest: &Value, found: &mut Findings) -> BTreeMap<String, Vec<String>> {
    let mut options = BTreeMap::new();
    let Some(list) = manifest.get("options") else {
        return options;
    };
    let Some(list) = list.as_array() else {
        found.problem("options is a list");
        return options;
    };
    for option in list {
        let id = option.get("id").and_then(Value::as_str).unwrap_or("");
        if !valid_id(id) {
            found.problem(format!("the option id {id:?} can only have lower case letters, digits and -"));
            continue;
        }
        if options.contains_key(id) {
            found.problem(format!("two options are called {id}"));
        }
        match option.get("name") {
            Some(name) => check_text(name, &format!("option {id}'s name"), found),
            None => found.problem(format!("option {id} needs a name")),
        }
        let mut choices = Vec::new();
        // A number option: a choice for each number of a range, its id the number.
        if let Some(range) = option.get("range").and_then(Value::as_object) {
            let number = |key: &str, missing: f64| range.get(key).and_then(Value::as_f64).unwrap_or(missing);
            let (from, to, step) = (number("from", 0.0), number("to", 0.0), number("step", 1.0));
            if step <= 0.0 || to < from || (to - from) / step > 10000.0 {
                found.problem(format!("option {id}: a range goes from a number up to another, by a step above 0, in at most 10,000 steps"));
            } else {
                let mut at = from;
                while at <= to + step * 0.001 {
                    choices.push(format!("{at}"));
                    at += step;
                }
                match option.get("default").and_then(Value::as_f64) {
                    Some(default) if default >= from && default <= to => {}
                    _ => found.problem(format!("option {id}'s default is a number in its range")),
                }
                options.insert(id.to_string(), choices);
                continue;
            }
        }
        for choice in option.get("choices").and_then(Value::as_array).map_or(&[][..], Vec::as_slice) {
            let choice_id = choice.get("id").and_then(Value::as_str).unwrap_or("");
            if !valid_id(choice_id) {
                found.problem(format!("option {id}: the choice id {choice_id:?} can only have lower case letters, digits and -"));
                continue;
            }
            match choice.get("name") {
                Some(name) => check_text(name, &format!("option {id}'s choice {choice_id}"), found),
                None => found.problem(format!("option {id}: choice {choice_id} needs a name")),
            }
            choices.push(choice_id.to_string());
        }
        if choices.is_empty() {
            found.problem(format!("option {id} needs choices"));
        }
        match option.get("default").and_then(Value::as_str) {
            Some(default) if choices.iter().any(|c| c == default) => {}
            Some(default) => found.problem(format!("option {id}'s default {default} isn't one of its choices")),
            None => found.problem(format!("option {id} needs a default")),
        }
        options.insert(id.to_string(), choices);
    }
    options
}

fn check_when(when: &Value, name: &str, options: &BTreeMap<String, Vec<String>>, found: &mut Findings) {
    let Some(map) = when.as_object() else {
        found.problem(format!("{name}: when is a set of conditions"));
        return;
    };
    for key in map.keys() {
        if !WHEN_KEYS.contains(&key.as_str()) {
            found.problem(format!("{name}: when has {key}, which this format doesn't know"));
        }
    }
    if let Some(versions) = map.get("version") {
        for version in string_list(versions, &format!("{name}'s version"), found) {
            if !VERSION_LETTERS.contains(&version) {
                found.problem(format!("{name}: {version} isn't a version ({})", VERSION_LETTERS.join(", ")));
            }
        }
    }
    if let Some(languages) = map.get("language") {
        for language in string_list(languages, &format!("{name}'s language"), found) {
            if !crate::layout::LANGUAGE_FOLDERS.contains(&language) {
                found.problem(format!("{name}: {language} isn't one of the game's language folders"));
            }
        }
    }
    if let Some(platforms) = map.get("platform") {
        for platform in string_list(platforms, &format!("{name}'s platform"), found) {
            if !PLATFORMS.contains(&platform) {
                found.problem(format!("{name}: {platform} isn't a device ({})", PLATFORMS.join(", ")));
            }
        }
    }
    if let Some(mods) = map.get("mods") {
        for id in string_list(mods, &format!("{name}'s mods"), found) {
            if !valid_id(id) {
                found.problem(format!("{name}: {id} isn't a mod's id"));
            }
        }
    }
    for key in ["not", "any"] {
        if let Some(inner) = map.get(key) {
            let list = inner.as_array().map_or_else(|| std::slice::from_ref(inner), Vec::as_slice);
            for one in list {
                check_when(one, name, options, found);
            }
        }
    }
    if let Some(wanted) = map.get("option") {
        let Some(wanted) = wanted.as_object() else {
            found.problem(format!("{name}: option names options and their choices"));
            return;
        };
        for (option, choices) in wanted {
            let Some(known) = options.get(option) else {
                found.problem(format!("{name}: there's no option called {option}"));
                continue;
            };
            for choice in string_list(choices, &format!("{name}'s option {option}"), found) {
                if !known.iter().any(|c| c == choice) {
                    found.problem(format!("{name}: option {option} has no choice {choice}"));
                }
            }
        }
    }
}

/// What a file in a layer's `files/` can be: a kind of the game's that Nebula cooks, a modern file it converts (models,
/// pictures, tables as JSON, music), or one the game reads as it is.
fn game_reads(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    nebula_import::cook::cooks(&lower)
        || nebula_import::cook::modern_audio::is_stream_source(&lower)
        || [".json", ".gltf", ".glb", ".png", ".aw", ".bin", ".kcl"].iter().any(|e| lower.ends_with(e))
}

fn check_layers(folder: &Path, manifest: &Value, options: &BTreeMap<String, Vec<String>>, store: Option<&GameStore>, found: &mut Findings) {
    let Some(layers) = manifest.get("layers").and_then(Value::as_array) else {
        found.problem("it has no layers");
        return;
    };
    let says_tunables = manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "tunables"));
    // A mod can be only code, or only a galaxy's row over files another mod brings.
    let mut has_anything = manifest.get("plugins").and_then(Value::as_array).is_some_and(|p| !p.is_empty())
        || manifest.get("text").and_then(Value::as_object).is_some_and(|t| !t.is_empty())
        || ["music", "event_flags", "comet_schedule"].iter().any(|k| manifest.get(*k).and_then(Value::as_array).is_some_and(|l| !l.is_empty()));
    has_anything |= check_story(manifest, "the mod", found);
    let says_custom_shaders = manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "custom-shaders"));
    // The materials of the models the mod brings: a hand-written shader applies to those alone.
    let own_materials = if layers.iter().any(|l| l.get("materials").and_then(Value::as_object).is_some_and(|m| !m.is_empty())) {
        Some(mod_material_names(folder, layers))
    } else {
        None
    };
    let says_player_animations = manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "player-animations"));
    let says_characters = manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "characters"));
    for (i, layer) in layers.iter().enumerate() {
        if layer.get("player_animations").and_then(Value::as_array).is_some_and(|r| !r.is_empty()) && !says_player_animations {
            found.problem("a layer adds player_animations, so requires has to list player-animations");
        }
        if layer.get("characters").and_then(Value::as_array).is_some_and(|r| !r.is_empty()) && !says_characters {
            found.problem("a layer adds characters, so requires has to list characters");
        }
        if let Some(materials) = layer.get("materials") {
            let Some(materials) = materials.as_object() else {
                found.problem("materials is a set of material names and their shader files");
                continue;
            };
            if !materials.is_empty() && !says_custom_shaders {
                found.problem("a layer names shaders for materials, so requires has to list custom-shaders");
            }
            for (name, file) in materials {
                match file.as_str().filter(|f| inside(f)) {
                    Some(path) if folder.join(path).is_file() => has_anything = true,
                    Some(path) => found.problem(format!("material {name}: {path} isn't in the file")),
                    None => found.problem(format!("material {name}: its shader is a path in the file")),
                }
                if let Some(own) = own_materials.as_ref().filter(|own| !own.contains(name)) {
                    let some: Vec<&str> = own.iter().take(12).map(String::as_str).collect();
                    found.problem(if some.is_empty() {
                        format!("material {name}: the mod brings no model; a shader applies to the materials of the models in a layer's files/")
                    } else {
                        format!("material {name}: no model the mod brings has it (its materials: {})", some.join(", "))
                    });
                }
            }
        }
        let name = layer.get("path").and_then(Value::as_str).map_or_else(|| format!("layer {}", i + 1), str::to_string);
        if let Some(path) = layer.get("path").and_then(Value::as_str) {
            if !inside(path) {
                found.problem(format!("{path} leaves the file"));
                continue;
            }
            let folder = folder.join(path);
            if !folder.is_dir() {
                found.problem(format!("{path} isn't in the file"));
                continue;
            }
            has_anything |= check_merge(&folder.join("merge"), store, found);
            for entry in walkdir::WalkDir::new(folder.join("files")).into_iter().filter_map(std::result::Result::ok) {
                if !entry.file_type().is_file() {
                    continue;
                }
                has_anything = true;
                if !game_reads(&entry.file_name().to_string_lossy()) {
                    found.note(format!("{}: the game may not read this kind of file", entry.path().strip_prefix(&folder).unwrap_or(entry.path()).display()));
                }
            }
            has_anything |= walkdir::WalkDir::new(folder.join("textures")).into_iter().filter_map(std::result::Result::ok).any(|e| e.file_type().is_file());
            has_anything |= walkdir::WalkDir::new(folder.join("movies")).into_iter().filter_map(std::result::Result::ok).any(|e| e.file_type().is_file());
        }
        // Switches gave way to the game's tunables: each is named with what took its place.
        if let Some(switches) = layer.get("switches") {
            let named: Vec<String> = switches.as_object().into_iter().flatten().map(|(switch, _)| match known_patches::tunable_for_switch(switch) {
                Some((key, true)) => format!("{switch} is \"{key}\", the opposite sense"),
                Some((key, false)) => format!("{switch} is \"{key}\""),
                None => format!("{switch} has no tunable"),
            }).collect();
            found.problem(format!("{name}: \"switches\" are now \"tunables\" (\"requires\": [\"tunables\"]): {}", named.join("; ")));
        }
        if let Some(tunables) = layer.get("tunables") {
            let Some(tunables) = tunables.as_object() else {
                found.problem(format!("{name}: tunables names the game's numbers (\"<file>.<name>\") and their values"));
                continue;
            };
            if !says_tunables {
                found.problem(format!("{name}: a mod with tunables says \"requires\": [\"tunables\"]"));
            }
            for (key, value) in tunables {
                has_anything = true;
                if !key.contains('.') || key.starts_with('.') || key.ends_with('.') {
                    found.problem(format!("{name}: {key} is not \"<file stem>.<name>\" (\"GameRules.cPlayerHealth\")"));
                }
                check_tunable(&name, key, value, found);
                let one = |v: &Value| v.is_number() || v.is_boolean();
                if let Some(option) = value.get("option").and_then(Value::as_str) {
                    if !options.contains_key(option) {
                        found.problem(format!("{name}: {key} takes its number from {option}, and there's no such option"));
                    }
                } else if let Some(list) = value.as_array() {
                    if !list.iter().all(|v| v.is_null() || one(v)) {
                        found.problem(format!("{name}: {key}'s list holds numbers, true or false, or null to keep a place"));
                    }
                } else if !one(value) {
                    found.problem(format!("{name}: {key} needs a number, true or false, a list, or {{ \"option\": its id }}"));
                }
            }
        }
        has_anything |= check_rows(folder, layer, &name, store, found);
        has_anything |= check_story(layer, &name, found);
        // Rows a mod can have can also be in a layer, so that an option turns them on.
        check_galaxies(folder, layer, found);
        check_music(folder, layer, found);
        check_progression(layer, found);
        has_anything |= ["music", "event_flags", "comet_schedule", "galaxies"].iter().any(|k| layer.get(*k).and_then(Value::as_array).is_some_and(|l| !l.is_empty()))
            || layer.get("text").and_then(Value::as_object).is_some_and(|t| !t.is_empty());
        if let Some(when) = layer.get("when") {
            check_when(when, &name, options, found);
        }
    }
    if !has_anything {
        found.problem("it holds nothing the game can use");
    }
}

/// The journey: where a file starts, what the hub is.
fn check_story(value: &Value, name: &str, found: &mut Findings) -> bool {
    let Some(story) = value.get("story") else {
        return false;
    };
    let Some(story) = story.as_object() else {
        found.problem(format!("{name}: story is a set of keys"));
        return false;
    };
    for (key, value) in story {
        match key.as_str() {
            "first_stage" | "hub" if value.is_string() => {}
            "first_mission" if value.is_u64() => {}
            "story_scenes" if value.is_boolean() || value.as_str().is_some_and(|v| v == "on" || v == "off") => {}
            "first_stage" | "hub" | "first_mission" | "story_scenes" => found.problem(format!("{name}: story's {key} has the wrong kind of value")),
            other => found.problem(format!("{name}: story has {other}, which this format doesn't know")),
        }
    }
    !story.is_empty()
}

/// A layer's rows that aren't files. Whether it has any.
/// A character's id (luma.cpp character_id_ok): 1 to 31 of a-z, 0-9, '_' and '-'.
fn character_id_ok(id: &str) -> bool {
    !id.is_empty() && id.len() <= 31 && id.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-')
}

fn check_rows(folder: &Path, layer: &Value, name: &str, store: Option<&GameStore>, found: &mut Findings) -> bool {
    let mut any = false;
    if let Some(rows) = layer.get("object_changes").and_then(Value::as_array) {
        any |= !rows.is_empty();
        for row in rows {
            if row.get("object").and_then(Value::as_str).unwrap_or_default().is_empty() {
                found.problem(format!("{name}: a row of object_changes names no object"));
            }
            match row.get("in").and_then(Value::as_str) {
                None | Some("placed" | "display_models" | "treasure_boxes") => {}
                Some(other) => found.problem(format!("{name}: object_changes can't be in {other}")),
            }
        }
    }
    if let Some(rows) = layer.get("comet_time_limits").and_then(Value::as_array) {
        any |= !rows.is_empty();
        for row in rows {
            let good = row.get("galaxy").and_then(Value::as_str).is_some_and(|g| !g.is_empty())
                && row.get("mission").and_then(Value::as_u64).is_some_and(|m| m > 0)
                && row.get("seconds").and_then(Value::as_u64).is_some_and(|s| s > 0);
            if !good {
                found.problem(format!("{name}: a row of comet_time_limits needs a galaxy, a mission and seconds"));
            }
        }
    }
    if let Some(rows) = layer.get("stages").and_then(Value::as_array) {
        any |= !rows.is_empty();
        for row in rows {
            let stage = row.get("stage").and_then(Value::as_str).unwrap_or_default();
            if stage.is_empty() {
                found.problem(format!("{name}: a row of stages names no stage"));
            }
            let acts = row.get("acts_as").and_then(Value::as_array).is_some_and(|a| !a.is_empty());
            let own = row.get("own_rules").and_then(Value::as_bool).unwrap_or(true);
            if !acts && own {
                found.note(format!("{name}: the row for {stage} changes nothing: give it acts_as, or own_rules false"));
            }
        }
    }
    if let Some(constants) = layer.get("player_constants") {
        let Some(people) = constants.as_object() else {
            found.problem(format!("{name}: player_constants is mario, luigi or both, each with its numbers"));
            return any;
        };
        for (who, numbers) in people {
            // Another key is a character's id (a layer's "characters", this mod's or another's).
            if !matches!(who.as_str(), "mario" | "luigi" | "both") && !character_id_ok(who) {
                found.problem(format!("{name}: player_constants: {who} is none of mario, luigi, both and a character's id"));
                continue;
            }
            let Some(numbers) = numbers.as_object() else {
                found.problem(format!("{name}: player_constants: {who} is a set of names and numbers"));
                continue;
            };
            for (number, value) in numbers {
                any = true;
                let Some(field) = crate::player_constants::field(number) else {
                    found.problem(format!("{name}: the player has no number {number}. `lumalink list player` names them"));
                    continue;
                };
                let good = value.is_number() || value.get("option").is_some_and(Value::is_string) || value.as_array().is_some_and(|l| l.len() <= field.count as usize && l.iter().all(|i| i.is_number() || i.is_null()));
                if !good {
                    found.problem(format!("{name}: {number} is a number, or a list of up to {} numbers (null keeps the game's)", field.count));
                }
            }
        }
    }
    if let Some(rows) = layer.get("objects").and_then(Value::as_array) {
        any |= !rows.is_empty();
        let toolkit = crate::lumascript::Toolkit::bundled();
        for row in rows {
            let object = row.get("name").and_then(Value::as_str).unwrap_or_default();
            let script = row.get("script").and_then(Value::as_str).unwrap_or_default();
            if object.is_empty() || script.is_empty() {
                found.problem(format!("{name}: a row of objects needs name and script"));
                continue;
            }
            if let Some(kind) = row.get("kind").and_then(Value::as_str) {
                if !["enemy", "map", "npc", "ride", "decoration"].contains(&kind) {
                    found.problem(format!("{name}: {object}'s kind is enemy, map, npc, ride or decoration, not {kind}"));
                }
            }
            match compile_script(folder, script, &toolkit) {
                Ok(_) => {}
                Err(why) => found.problem(format!("{name}: {object}: {why}")),
            }
        }
    }
    if let Some(scripts) = layer.get("scripts") {
        // Mod-wide scripts: one instance each for the whole run (luma.cpp "scripts").
        match scripts.as_array() {
            None => found.problem(format!("{name}: scripts is a list of script names")),
            Some(rows) => {
                any |= !rows.is_empty();
                let toolkit = crate::lumascript::Toolkit::bundled();
                for row in rows {
                    match row.as_str() {
                        None => found.problem(format!("{name}: scripts is a list of script names, not {row}")),
                        Some(script) => {
                            if let Err(why) = compile_script(folder, script, &toolkit) {
                                found.problem(format!("{name}: {why}"));
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(rows) = layer.get("characters") {
        // Playable characters (luma.cpp "characters", NebulaCoop's NebulaCharacters): a model on Mario or Luigi.
        match rows.as_array() {
            None => found.problem(format!("{name}: characters is a list of rows")),
            Some(rows) => {
                any |= !rows.is_empty();
                for row in rows {
                    let id = row.get("id").and_then(Value::as_str).unwrap_or_default();
                    if !character_id_ok(id) || id == "mario" || id == "luigi" {
                        found.problem(format!(
                            "{name}: a character's id is 1 to 31 of a-z, 0-9, _ and - (not mario or luigi), not \"{id}\""
                        ));
                        continue;
                    }
                    let like = row.get("like").map(|v| v.as_str().unwrap_or("?")).unwrap_or("mario");
                    if like != "mario" && like != "luigi" {
                        found.problem(format!("{name}: {id} is like mario or luigi, not {like}"));
                    }
                    if row.get("name").is_some_and(|n| !n.as_str().is_some_and(|n| n.len() <= 31)) {
                        found.problem(format!("{name}: {id}'s name is text of at most 31 bytes"));
                    }
                    let archive = match row.get("archive") {
                        None => {
                            let mut chars = id.chars();
                            chars.next().map(|c| c.to_ascii_uppercase().to_string() + chars.as_str()).unwrap_or_default()
                        }
                        Some(a) => a.as_str().unwrap_or_default().to_string(),
                    };
                    if archive.is_empty() || archive.len() > 31 || !archive.chars().all(|c| c > ' ' && c <= '~' && c != '/' && c != '\\' && c != '.') {
                        found.problem(format!("{name}: {id}'s archive is a name in ObjectData without .arc, not \"{archive}\""));
                        continue;
                    }
                    let layer_path = layer.get("path").and_then(Value::as_str).unwrap_or_default();
                    let objects = folder.join(layer_path).join("files").join("ObjectData");
                    let shipped = std::fs::read_dir(&objects).ok().is_some_and(|entries| {
                        entries.flatten().any(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(&format!("{archive}.arc")))
                    });
                    if !layer_path.is_empty() && !shipped {
                        found.note(format!("{name}: {id}'s model {archive}.arc isn't in this layer's files/ObjectData"));
                    }
                }
            }
        }
    }
    if let Some(rows) = layer.get("forms") {
        // Power-ups from data (luma.cpp "forms"): the game's forms by name, whole numbers, a script.
        const FORMS: [&str; 9] = ["normal", "invincible", "fire", "ice", "bee", "spring", "boo", "flying", "tornado"];
        match rows.as_array() {
            None => found.problem(format!("{name}: forms is a list of rows")),
            Some(rows) => {
                any |= !rows.is_empty();
                let toolkit = crate::lumascript::Toolkit::bundled();
                for row in rows {
                    let form = row.get("name").and_then(Value::as_str).unwrap_or_default();
                    if form.is_empty() {
                        found.problem(format!("{name}: a row of forms needs a name"));
                        continue;
                    }
                    for key in ["acts_like", "looks_like"] {
                        if let Some(value) = row.get(key) {
                            if !value.as_str().is_some_and(|v| FORMS.contains(&v)) {
                                found.problem(format!("{name}: {form}'s {key} is one of {}", FORMS.join(", ")));
                            }
                        }
                    }
                    for (key, most) in [("color_frame", 255u64), ("frames", 65535)] {
                        if let Some(value) = row.get(key) {
                            if !value.as_u64().is_some_and(|v| v <= most) {
                                found.problem(format!("{name}: {form}'s {key} is a whole number from 0 to {most}"));
                            }
                        }
                    }
                    if let Some(script) = row.get("script").and_then(Value::as_str) {
                        if let Err(why) = compile_script(folder, script, &toolkit) {
                            found.problem(format!("{name}: {form}: {why}"));
                        }
                    }
                }
            }
        }
    }
    if let Some(rows) = layer.get("object_classes").and_then(Value::as_array) {
        any |= !rows.is_empty();
        let toolkit = crate::lumascript::Toolkit::bundled();
        for row in rows {
            let object = row.get("name").and_then(Value::as_str).unwrap_or_default();
            let like = row.get("like").and_then(Value::as_str).unwrap_or_default();
            let script = row.get("script").and_then(Value::as_str).unwrap_or_default();
            // A class script runs on every actor placed by the name (a game class, or one made like it here).
            if !script.is_empty() {
                if let Err(why) = compile_script(folder, script, &toolkit) {
                    found.problem(format!("{name}: {object}: {why}"));
                }
                if !object.is_empty() && like.is_empty() && !crate::object_table::OBJECT_ROWS.iter().any(|(known, _, _)| known.eq_ignore_ascii_case(object)) {
                    found.note(format!("{name}: {object} isn't an object of the game's; its script runs only where another mod or a stage places it"));
                }
            }
            if object.is_empty() || (like.is_empty() && script.is_empty()) {
                found.problem(format!("{name}: a row of object_classes needs name, and like or script"));
            } else if like.is_empty() {
            } else if !crate::object_table::OBJECT_ROWS.iter().any(|(known, _, _)| known.eq_ignore_ascii_case(like)) {
                found.note(format!("{name}: {like}, which {object} is made like, isn't an object of the game's. A plugin may add it"));
            } else if let Some(store) = store {
                let own_path = format!("ObjectData/{object}.arc");
                let own = layer.get("path").and_then(Value::as_str).map(|path| folder.join(path).join("files").join(&own_path));
                match own.filter(|own| own.is_dir()) {
                    Some(own) => {
                        if let Ok(games) = nebula_import::cook::store::read_folder(&store.path, &format!("ObjectData/{like}.arc")) {
                            compare_with_like(&own, &games, name, object, like, found);
                        }
                    }
                    None if !store.has(&own_path) => {
                        found.problem(format!("{name}: {object} has no {own_path}, in the layer or the game: Nebula leaves it out"));
                    }
                    None => {}
                }
            }
        }
    }
    // The player's animations: a name, a bck of the player's (ObjectData/MarioAnime.arc: the layer's, or the game's), a
    // rate over 0 up to 100 and whole blend frames up to 600, as Nebula reads them (luma.cpp).
    if let Some(rows) = layer.get("player_animations") {
        let Some(rows) = rows.as_array() else {
            found.problem(format!("{name}: player_animations is a list of rows"));
            return any;
        };
        any |= !rows.is_empty();
        let arc = layer.get("path").and_then(Value::as_str).map(|path| folder.join(path).join("files").join("ObjectData").join("MarioAnime.arc"));
        let has_modern = arc.as_ref().is_some_and(|arc| {
            walkdir::WalkDir::new(arc).into_iter().filter_map(std::result::Result::ok).any(|e| {
                let file = e.file_name().to_string_lossy().to_ascii_lowercase();
                file.ends_with(".glb") || file.ends_with(".gltf")
            })
        });
        for row in rows {
            let animation = row.get("name").and_then(Value::as_str).unwrap_or_default();
            let file = row.get("file").and_then(Value::as_str).unwrap_or_default();
            if animation.is_empty() || file.is_empty() {
                found.problem(format!("{name}: a row of player_animations needs name and file"));
                continue;
            }
            if let Some(rate) = row.get("rate") {
                if rate.as_f64().is_none_or(|r| r <= 0.0 || r > 100.0) {
                    found.problem(format!("{name}: the player animation {animation}: rate is a number over 0, up to 100"));
                }
            }
            if let Some(blend) = row.get("blend_frames") {
                if blend.as_u64().is_none_or(|b| b > 600) {
                    found.problem(format!("{name}: the player animation {animation}: blend_frames is a whole number up to 600"));
                }
            }
            let in_layer = arc.as_ref().is_some_and(|arc| arc.join(format!("{file}.bck")).is_file() || arc.join(file).is_file());
            let games = format!("objectdata/marioanime.arc/{}.bck", file.to_ascii_lowercase());
            match store {
                _ if in_layer || has_modern => {}
                Some(store) if store.has(&games) => {}
                Some(_) => found.problem(format!(
                    "{name}: the player animation {animation}: {file} is neither in the layer's ObjectData/MarioAnime.arc nor the game's"
                )),
                None => found.note(format!(
                    "{name}: the player animation {animation}: {file} isn't in the layer's ObjectData/MarioAnime.arc, so it has to be the game's (check with --game)"
                )),
            }
        }
    }
    if let Some(rows) = layer.get("sound_effects").and_then(Value::as_array) {
        any |= !rows.is_empty();
        for row in rows {
            let sound = row.get("name").and_then(Value::as_str).unwrap_or_default();
            let file = row.get("file").and_then(Value::as_str).unwrap_or_default();
            if sound.is_empty() || file.is_empty() {
                found.problem(format!("{name}: a row of sound_effects needs name and file"));
                continue;
            }
            if let Some(path) = layer.get("path").and_then(Value::as_str) {
                let there = folder.join(path).join("files").join(file);
                if !there.is_file() {
                    found.problem(format!("{name}: the sound {sound}: {file} isn't in the layer's files"));
                }
            }
            // "<name>@luigi": the sound while Luigi plays.
            if let Some(base) = sound.strip_suffix("@luigi") {
                if base.is_empty() || base.contains('@') {
                    found.problem(format!("{name}: the sound {sound}: @luigi follows a sound's name"));
                }
            } else if sound.contains('@') {
                found.problem(format!("{name}: the sound {sound}: a name has no @, but for @luigi at its end"));
            }
        }
    }
    any
}

/// An object made like the game's `like`, with an archive of its own: what the game's model and archive have that its
/// own lack. The game's code for `like` moves joints and plays animations by name: a joint its model lacks moves its
/// root, an animation it lacks plays nothing (Nebula carries on). Notes, not problems: a model may need fewer.
/// `own`: the layer's folder of the object's archive; `games`: the files of the game's archive of `like`.
fn compare_with_like(own: &Path, games: &[(String, Vec<u8>)], name: &str, object: &str, like: &str, found: &mut Findings) {
    let stem = |file: &str| file.rsplit('/').next().unwrap_or(file).split('.').next().unwrap_or_default().to_ascii_lowercase();
    let ending = |file: &str| file.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).unwrap_or_default();
    const ANIMATIONS: [&str; 8] = ["bck", "bca", "btk", "brk", "bpk", "btp", "bva", "bxk"];
    // The game's: its model's joints (the one named as the object, else the first) and its animations.
    let model = games
        .iter()
        .filter(|(file, _)| matches!(ending(file).as_str(), "bdl" | "bmd"))
        .max_by_key(|(file, _)| stem(file) == like.to_ascii_lowercase());
    let game_joints = model.and_then(|(_, bytes)| nebula_import::cook::model_write::names(bytes).ok()).map(|n| n.joints).unwrap_or_default();
    let mut game_animations: Vec<String> =
        games.iter().filter(|(file, _)| ANIMATIONS.contains(&ending(file).as_str())).map(|(file, _)| stem(file)).collect();
    game_animations.sort();
    game_animations.dedup();
    // Its own: what its glTF models hold, and its animation files.
    let mut own_joints: Vec<String> = Vec::new();
    let mut own_animations: Vec<String> = Vec::new();
    for entry in walkdir::WalkDir::new(own).into_iter().filter_map(std::result::Result::ok).filter(|e| e.file_type().is_file()) {
        let file = entry.file_name().to_string_lossy().to_string();
        let lower = file.to_ascii_lowercase();
        // A cutscene camera (Name.canm.glb) is no model.
        if (lower.ends_with(".glb") || lower.ends_with(".gltf")) && !lower.contains(".canm.") {
            let Ok(bytes) = fs::read(entry.path()) else { continue };
            match nebula_import::cook::gltf::to_ir(&bytes, entry.path().parent()) {
                Ok((ir, clips)) => {
                    own_joints.extend(ir.joints.iter().map(|j| j.name.clone()));
                    own_animations.extend(clips.iter().map(|(clip, _)| clip.to_ascii_lowercase()));
                }
                Err(error) => found.problem(format!("{name}: {file}: {error:#}")),
            }
        } else if ANIMATIONS.contains(&ending(&file).as_str()) {
            own_animations.push(stem(&file));
        }
    }
    let missing_joints: Vec<&str> = game_joints.iter().filter(|j| !own_joints.iter().any(|o| o == *j)).map(String::as_str).collect();
    if !own_joints.is_empty() && !missing_joints.is_empty() {
        found.note(format!("{name}: {object}'s model has no joint {} of {like}'s: what {like}'s code moves there moves its root", missing_joints.join(", ")));
    }
    let missing: Vec<&str> = game_animations.iter().filter(|a| !own_animations.contains(a)).map(String::as_str).collect();
    if !missing.is_empty() {
        found.note(format!(
            "{name}: {object} has {} of {like}'s {} animations; where {like}'s code plays one of the {} others ({}), {object} goes on as it was",
            game_animations.len() - missing.len(),
            game_animations.len(),
            missing.len(),
            missing.join(" ")
        ));
    }
}

/// A tunable's type, by its key ("GameRules.cPlayerHealth", or a list's place "X.sSteps[2]"): its kind ("f32", "bool") and,
/// for a list, its length. None: the game has no such tunable.
pub fn tunable_kind(key: &str) -> Option<(&'static str, Option<usize>)> {
    let (name, place) = match key.strip_suffix(']').and_then(|k| k.rsplit_once('[')) {
        Some((name, place)) => (name, Some(place.parse::<usize>().ok()?)),
        None => (key, None),
    };
    let found = crate::tunable_names::TUNABLES.binary_search_by(|(k, _)| k.as_bytes().cmp(name.as_bytes())).ok()?;
    let kind = crate::tunable_names::TUNABLES[found].1;
    let (base, length) = match kind.strip_suffix(']').and_then(|k| k.split_once('[')) {
        Some((base, length)) => (base, length.parse::<usize>().ok()),
        None => (kind, None),
    };
    match (place, length) {
        (Some(place), Some(length)) if place < length => Some((base, None)),
        (Some(_), _) => None,
        (None, length) => Some((base, length)),
    }
}

/// The game's tunable nearest a key it lacks, for "did you mean": the same file's name that differs least.
fn nearest_tunable(key: &str) -> Option<&'static str> {
    let distance = |a: &str, b: &str| {
        let b: Vec<char> = b.chars().collect();
        let mut row: Vec<usize> = (0..=b.len()).collect();
        for (i, ca) in a.chars().enumerate() {
            let mut last = row[0];
            row[0] = i + 1;
            for (j, cb) in b.iter().enumerate() {
                let next = (last + usize::from(ca != *cb)).min(row[j] + 1).min(row[j + 1] + 1);
                last = row[j + 1];
                row[j + 1] = next;
            }
        }
        row[b.len()]
    };
    let lower = key.to_ascii_lowercase();
    crate::tunable_names::TUNABLES
        .iter()
        .map(|(k, _)| (distance(&lower, &k.to_ascii_lowercase()), *k))
        .filter(|(d, k)| *d <= 3.max(k.len() / 6))
        .min()
        .map(|(_, k)| k)
}

/// A layer's tunable against the game's list: a key it lacks, a list's place past its end, a value its type can't hold.
fn check_tunable(layer: &str, key: &str, value: &Value, found: &mut Findings) {
    let Some((kind, length)) = tunable_kind(key) else {
        let near = nearest_tunable(key).map_or(String::new(), |k| format!(" (did you mean {k}?)"));
        found.problem(format!("{layer}: the game has no tunable {key}{near}. The whole list: Nebula's docs/modding/TUNABLES.md"));
        return;
    };
    let fits = |v: &Value| -> Option<String> {
        if v.is_null() || v.get("option").is_some() {
            return None;
        }
        let number = match v {
            Value::Bool(b) => f64::from(u8::from(*b)),
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            _ => return None,
        };
        let range = match kind {
            "bool" => Some((0.0, 1.0)),
            "u8" => Some((0.0, 255.0)),
            "s8" => Some((-128.0, 127.0)),
            "u16" => Some((0.0, 65535.0)),
            "s16" => Some((-32768.0, 32767.0)),
            "u32" => Some((0.0, 4294967295.0)),
            "s32" => Some((-2147483648.0, 2147483647.0)),
            _ => None,
        };
        match range {
            Some((low, high)) if number.fract() != 0.0 || number < low || number > high => {
                Some(format!("{key} is {kind}: a whole number from {low} to {high}"))
            }
            _ => None,
        }
    };
    match (value.as_array(), length) {
        (Some(list), Some(length)) if list.len() > length => {
            found.problem(format!("{layer}: {key} is a list of {length}; this one has {}", list.len()));
        }
        (Some(_), None) => found.problem(format!("{layer}: {key} is one number, not a list")),
        (None, Some(length)) if value.get("option").is_none() => {
            found.problem(format!("{layer}: {key} is a list of {length}: a list, or one place of it ({key}[0])"));
        }
        _ => {}
    }
    let values: Vec<&Value> = value.as_array().map_or_else(|| vec![value], |l| l.iter().collect());
    if let Some(problem) = values.into_iter().find_map(fits) {
        found.problem(format!("{layer}: {problem}"));
    }
}

/// A table merge tried on the game's own table, as Nebula applies it: a column the table lacks is a problem, a remove or
/// change that matches no row is a note (Nebula applies the rest and names it in its log). Not tried without the game,
/// or when the mod brings that table in its own files/.
fn check_table_merge(layer: &Path, relative: &str, shown: &str, bytes: &[u8], store: Option<&GameStore>, found: &mut Findings) {
    let Some(store) = store.filter(|s| !s.path.as_os_str().is_empty()) else { return };
    let table = &relative[..relative.len() - ".json".len()];
    if layer.join("files").join(table).is_file() {
        return;
    }
    let Ok(game) = nebula_import::cook::store::read_file(&store.path, table) else {
        found.problem(format!("merge/{shown}: your game has no table {table}; a merge adds to one of the game's"));
        return;
    };
    let tried = nebula_import::cook::table::read_rows(bytes).and_then(|rows| nebula_import::cook::table::merge(&game, bytes, &rows));
    match tried {
        Ok((_, unmatched)) => {
            for words in unmatched {
                found.note(format!("merge/{shown}: {words} in your game"));
            }
        }
        Err(error) => found.problem(format!("merge/{shown}: {error:#}")),
    }
}

/// What a layer adds to files many mods share. Whether it found anything.
fn check_merge(folder: &Path, store: Option<&GameStore>, found: &mut Findings) -> bool {
    let mut any = false;
    for entry in walkdir::WalkDir::new(folder).into_iter().filter_map(std::result::Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        any = true;
        // As the game names places, with '/' on every device.
        let shown = entry.path().strip_prefix(folder).map_or_else(|_| entry.path().display().to_string(), |p| p.display().to_string()).replace('\\', "/");
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        let Ok(bytes) = fs::read(entry.path()) else {
            found.problem(format!("merge/{shown} can't be read"));
            continue;
        };
        let relative = shown.clone();
        if let Some(language) = messages_language(&relative) {
            // Lines and talk flows: checked as the cook checks them, the game's lines standing in.
            if let Err(error) = nebula_import::cook::message_merge::check_shape(&bytes, &language) {
                found.problem(format!("merge/{shown}: {error:#}"));
            }
        } else if name.ends_with(".jpc.json") {
            // Particles written as JSON: checked as the cook reads them, any name standing for one of the game's.
            let names = entry.path().with_file_name("ParticleNames.bcsv.json");
            if !names.is_file() {
                found.problem(format!("merge/{shown}: new particles need their names: ParticleNames.bcsv.json beside them"));
            }
            match nebula_import::cook::particle_json::check(&bytes, entry.path().parent().unwrap_or(Path::new("."))) {
                Ok(count) => {
                    let named = fs::read(&names).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok()).and_then(|v| v.get("rows").and_then(Value::as_array).map(Vec::len));
                    if let Some(named) = named.filter(|&n| n != count) {
                        found.problem(format!("merge/{shown}: {count} particles, and ParticleNames.bcsv.json names {named}: one name each, in order"));
                    }
                }
                Err(error) => found.problem(format!("merge/{shown}: {error:#}")),
            }
        } else if [".png", ".jpg", ".jpeg"].iter().any(|ending| name.ends_with(ending)) {
            let parent = entry.path().parent().unwrap_or(Path::new("."));
            let named = fs::read_dir(parent).into_iter().flatten().filter_map(std::result::Result::ok).any(|e| {
                e.file_name().to_string_lossy().to_ascii_lowercase().ends_with(".jpc.json")
                    && fs::read_to_string(e.path()).is_ok_and(|text| text.contains(&*entry.file_name().to_string_lossy()))
            });
            if !named {
                found.problem(format!("merge/{shown}: a picture in merge/ is a texture of particles written as JSON beside it (Particles.jpc.json), and none names it"));
            }
        } else if name.ends_with(".json") {
            let parsed = serde_json::from_slice::<Value>(&bytes).ok();
            if parsed.is_some() {
                check_table_merge(folder.parent().unwrap_or(folder), &relative, &shown, &bytes, store, found);
            }
            if let Some(root) = &parsed {
                for row in root.get("remove").and_then(Value::as_array).map_or(&[][..], Vec::as_slice) {
                    if !row.is_object() {
                        found.problem(format!("merge/{shown}: each row of remove is a set of columns"));
                    }
                }
                for change in root.get("change").and_then(Value::as_array).map_or(&[][..], Vec::as_slice) {
                    if !change.get("where").is_some_and(Value::is_object) || !change.get("set").is_some_and(Value::is_object) {
                        found.problem(format!("merge/{shown}: each change has where and set, each a set of columns"));
                    }
                }
                if root.get("rows").is_none() && (root.get("remove").is_some() || root.get("change").is_some()) {
                    any = true;
                    continue;
                }
            }
            let rows = parsed.and_then(|v| v.get("rows").and_then(Value::as_array).cloned());
            match rows {
                Some(rows) if !rows.is_empty() && rows.iter().all(Value::is_object) => {}
                _ => found.problem(format!("merge/{shown}: rows is a list of rows, each a set of columns")),
            }
        } else if name.ends_with(".jpc") {
            if !bytes.starts_with(b"JPAC2-10") {
                found.problem(format!("merge/{shown} isn't a particle file"));
            }
            if !entry.path().with_file_name("ParticleNames.bcsv.json").is_file() {
                found.problem(format!("merge/{shown}: new particles need their names: ParticleNames.bcsv.json beside them"));
            }
        } else {
            found.problem(format!(
                "merge/{shown}: only tables (.bcsv.json), messages (MessageData/messages.json, <Language>/MessageData/Message.arc/message.bmg.json) and particles (.jpc, or Particles.jpc.json with its pictures) can be added to"
            ));
        }
    }
    any
}

/// The language a messages merge is checked in: its folder's, or UsEnglish for every language's (where a new line
/// needs a default). None: not a messages merge.
fn messages_language(path: &str) -> Option<String> {
    let lower = path.to_ascii_lowercase();
    if lower == "messagedata/messages.json" {
        return Some("UsEnglish".to_string());
    }
    let parts: Vec<&str> = path.split('/').collect();
    (parts.len() == 4 && lower.ends_with("/messagedata/message.arc/message.bmg.json")).then(|| parts[0].to_string())
}

/// Galaxies beside the game's, and the hungry Lumas that open them.
fn check_progression(manifest: &Value, found: &mut Findings) {
    if let Some(flags) = manifest.get("event_flags") {
        match flags.as_array() {
            Some(flags) => {
                for flag in flags {
                    let name = flag.get("name").and_then(Value::as_str).filter(|n| !n.is_empty());
                    let Some(name) = name else {
                        found.problem("event_flags: every row names the flag it sets");
                        continue;
                    };
                    for key in ["kind", "a", "b"] {
                        if flag.get(key).is_some_and(|v| v.as_u64().is_none_or(|n| n > 255)) {
                            found.problem(format!("event_flags {name}: {key} is from 0 to 255"));
                        }
                    }
                }
            }
            None => found.problem("event_flags is a list"),
        }
    }
    if let Some(schedule) = manifest.get("comet_schedule") {
        match schedule.as_array() {
            Some(schedule) => {
                for dome in schedule {
                    if !dome.get("dome").and_then(Value::as_u64).is_some_and(|d| (1..=16).contains(&d)) {
                        found.problem("comet_schedule: dome is from 1 to 16");
                    }
                    let comets = dome.get("comets").and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
                    if comets.is_empty() || comets.iter().any(|c| c.get("galaxy").and_then(Value::as_str).is_none_or(str::is_empty)) {
                        found.problem("comet_schedule: comets is a list of galaxies");
                    }
                }
            }
            None => found.problem("comet_schedule is a list"),
        }
    }
}

fn check_music(folder: &Path, manifest: &Value, found: &mut Findings) {
    let Some(music) = manifest.get("music") else { return };
    let Some(music) = music.as_array() else {
        found.problem("music is a list");
        return;
    };
    let mut seen = std::collections::BTreeSet::new();
    for (i, track) in music.iter().enumerate() {
        let Some(name) = track.get("name").and_then(Value::as_str).filter(|n| !n.is_empty() && n.is_ascii()) else {
            found.problem(format!("music {}: name is letters and digits of your choosing", i + 1));
            continue;
        };
        if ["SE_", "BGM_", "MBGM_", "STM_", "ME_"].iter().any(|p| name.starts_with(p)) {
            found.problem(format!("music {name}: names starting SE_, BGM_, MBGM_, STM_ or ME_ are the game's"));
        }
        if !seen.insert(name.to_string()) {
            found.problem(format!("music {name}: two tracks have this name"));
        }
        let Some(file) = track.get("file").and_then(Value::as_str).filter(|f| !f.is_empty()) else {
            found.problem(format!("music {name}: file is its place in the game: AudioRes/Stream/{name}.wav"));
            continue;
        };
        let lower = file.replace('\\', "/").to_ascii_lowercase();
        if ![".wav", ".ogg", ".ast"].iter().any(|e| lower.ends_with(e)) {
            found.problem(format!("music {name}: file is a .wav, .ogg or .ast"));
        }
        // In any layer, with any letter case: the disc's own names are matched the same way.
        let wanted = format!("files/{lower}");
        let has_file = walkdir::WalkDir::new(folder.join("layers")).into_iter().filter_map(std::result::Result::ok).any(|e| {
            e.path().to_string_lossy().replace('\\', "/").to_ascii_lowercase().ends_with(&wanted)
        });
        if !has_file {
            found.problem(format!("music {name}: no layer has files/{file}"));
        }
        let Some(stages) = track.get("stages") else { continue };
        let Some(stages) = stages.as_array() else {
            found.problem(format!("music {name}: stages is a list"));
            continue;
        };
        for row in stages {
            if row.get("stage").and_then(Value::as_str).is_none_or(str::is_empty) {
                found.problem(format!("music {name}: each row of stages names a stage"));
            }
            match row.get("scenario").map(Value::as_u64) {
                None | Some(Some(0..=255)) => {}
                _ => found.problem(format!("music {name}: scenario is a mission's number, 0 for all")),
            }
            match row.get("change_area").map(Value::as_u64) {
                None | Some(Some(0..=255)) => {}
                _ => found.problem(format!("music {name}: change_area is a ChangeBgmCube's Obj_arg1")),
            }
            if row.get("scenario").is_some() && row.get("change_area").is_some() {
                found.problem(format!("music {name}: a row is for a mission or for an area, not both"));
            }
        }
    }
}

fn check_galaxies(folder: &Path, manifest: &Value, found: &mut Findings) {
    let Some(galaxies) = manifest.get("galaxies") else { return };
    let Some(galaxies) = galaxies.as_array() else {
        found.problem("galaxies is a list");
        return;
    };
    let mut slots = BTreeMap::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        let Some(name) = galaxy.get("name").and_then(Value::as_str).filter(|n| !n.is_empty() && n.is_ascii() && !n.contains(['/', '\\'])) else {
            found.problem(format!("galaxy {}: name is the galaxy's folder name", i + 1));
            continue;
        };
        // In any layer, with any letter case: the disc's own names are matched the same way.
        let scenario = format!("files/StageData/{name}/{name}Scenario.arc").to_ascii_lowercase();
        let has_files = walkdir::WalkDir::new(folder.join("layers")).into_iter().filter_map(std::result::Result::ok).any(|e| {
            e.path().to_string_lossy().replace('\\', "/").to_ascii_lowercase().ends_with(&scenario)
        });
        if !has_files {
            found.problem(format!("galaxy {name}: no layer has files/StageData/{name}/{name}Scenario.arc"));
        }
        if manifest.get("save").and_then(|s| s.get("mode")).and_then(Value::as_str).is_none_or(|m| m == "shared") {
            found.problem(format!("galaxy {name}: a mod that adds a galaxy keeps its own saves: set save.mode to own or copy"));
        }
        match galaxy.get("stars_to_open").map(Value::as_u64) {
            None | Some(Some(0..=255)) => {}
            _ => found.problem(format!("galaxy {name}: stars_to_open is from 0 to 255")),
        }
        match galaxy.get("dome").map(Value::as_u64) {
            None | Some(Some(0..=16)) => {}
            _ => found.problem(format!("galaxy {name}: dome is from 0 to 16")),
        }
        if let Some(conditions) = galaxy.get("open_conditions") {
            let names = string_list(conditions, &format!("galaxy {name}'s open_conditions"), found);
            if names.len() > 3 {
                found.problem(format!("galaxy {name}: at most 3 open_conditions"));
            }
        }
        if let Some(place) = galaxy.get("map_position") {
            if !place.as_array().is_some_and(|p| p.len() == 2 && p.iter().all(Value::is_number)) {
                found.problem(format!("galaxy {name}: map_position is [right, up] from the middle of the map's page"));
            }
        }
        if galaxy.get("map_pane").is_some_and(|p| !p.is_string()) {
            found.problem(format!("galaxy {name}: map_pane is a pane's name in the map's layout"));
        }
        if let Some(luma) = galaxy.get("hungry_luma") {
            match luma.get("slot").and_then(Value::as_u64) {
                Some(slot @ 0..=200) => {
                    if let Some(other) = slots.insert(slot, name.to_string()) {
                        found.problem(format!("galaxy {name}: hungry Luma slot {slot} is already {other}'s"));
                    }
                }
                _ => found.problem(format!("galaxy {name}: hungry_luma.slot is from 0 to 200")),
            }
            match luma.get("star_bits").and_then(Value::as_u64) {
                Some(1..=32767) => {}
                _ => found.problem(format!("galaxy {name}: hungry_luma.star_bits is from 1 to 32767")),
            }
        }
    }
}

/// The device a plugin's library is for, as `binaries` names it, with the machine its header has to say.
const PLUGIN_TARGETS: &[(&str, BinaryKind, u32)] = &[
    ("windows-x64", BinaryKind::Pe, 0x8664),
    ("windows-arm64", BinaryKind::Pe, 0xAA64),
    ("linux-x64", BinaryKind::Elf, 62),
    ("linux-arm64", BinaryKind::Elf, 183),
    ("android-x64", BinaryKind::Elf, 62),
    ("android-arm64", BinaryKind::Elf, 183),
    ("macos-x64", BinaryKind::MachO, 0x0100_0007),
    ("macos-arm64", BinaryKind::MachO, 0x0100_000C),
];
/// The plugin interface this LumaLink knows: `NEBULA_MOD_API_VERSION`.
const PLUGIN_API: u64 = 2;

#[derive(Clone, Copy, PartialEq)]
enum BinaryKind {
    Pe,
    Elf,
    MachO,
}

/// What a native library's header says it is: its kind and machine. A universal Mac library holds several: each is listed.
fn binary_machines(bytes: &[u8]) -> Vec<(BinaryKind, u32)> {
    let u16le = |at: usize| bytes.get(at..at + 2).map(|b| u32::from(u16::from_le_bytes([b[0], b[1]])));
    let u32le = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let u32be = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]));
    if bytes.starts_with(b"MZ") {
        let Some(pe) = u32le(0x3C) else { return Vec::new() };
        let pe = pe as usize;
        if bytes.get(pe..pe + 4) == Some(b"PE\0\0") {
            return u16le(pe + 4).map(|m| (BinaryKind::Pe, m)).into_iter().collect();
        }
        return Vec::new();
    }
    if bytes.starts_with(b"\x7fELF") {
        return u16le(18).map(|m| (BinaryKind::Elf, m)).into_iter().collect();
    }
    if bytes.starts_with(&[0xCF, 0xFA, 0xED, 0xFE]) {
        return u32le(4).map(|m| (BinaryKind::MachO, m)).into_iter().collect();
    }
    if bytes.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]) {
        let count = u32be(4).unwrap_or(0).min(16) as usize;
        return (0..count).filter_map(|i| u32be(8 + i * 20)).map(|m| (BinaryKind::MachO, m)).collect();
    }
    Vec::new()
}

/// The achievements a mod adds (the game's `kMaxAchievements`).
const MAX_ACHIEVEMENTS: usize = 64;
/// The events an achievement's "when" can name: NebulaModEvent's, in lower case.
const ACHIEVEMENT_EVENTS: [&str; 29] = [
    "frame", "stage_frame", "stage_start", "player_damage", "player_miss", "power_up", "coin", "star_bit", "star_get",
    "cutscene_start", "cutscene_end", "actor_appear", "actor_kill", "player_spin", "player_jump", "player_land", "pause",
    "stage_end", "game_save", "power_star_appear", "talk_line", "talk_choice", "cutscene_part", "camera_change",
    "sound_start", "effect_start", "screen", "actor_message", "player_action",
];

/// A mod's own achievements: each with an id and a name, unlocked by an event ("when") or by the mod's code.
fn check_achievements(manifest: &Value, found: &mut Findings) {
    let Some(list) = manifest.get("achievements") else {
        return;
    };
    let Some(list) = list.as_array() else {
        found.problem("achievements is a list");
        return;
    };
    if list.len() > MAX_ACHIEVEMENTS {
        found.problem(format!("achievements lists {}: the game keeps the first {MAX_ACHIEVEMENTS}", list.len()));
    }
    let has_code = manifest.get("plugins").and_then(Value::as_array).is_some_and(|p| !p.is_empty());
    let mut seen = std::collections::BTreeSet::new();
    for (index, entry) in list.iter().enumerate() {
        let id = entry.get("id").and_then(Value::as_str).unwrap_or("");
        let name = if id.is_empty() { format!("achievement {}", index + 1) } else { format!("the achievement {id}") };
        if !entry.is_object() {
            found.problem(format!("{name}: an achievement is a set of fields"));
            continue;
        }
        if !valid_id(id) {
            found.problem(format!("{name}: its id has to be lower case letters, digits and -, at most 64"));
        } else if !seen.insert(id) {
            found.problem(format!("two achievements are called {id}"));
        }
        match entry.get("name") {
            Some(text) if !text_of(text).trim().is_empty() => check_text(text, &format!("{name}: name"), found),
            _ => found.problem(format!("{name} has no name")),
        }
        if let Some(text) = entry.get("description") {
            check_text(text, &format!("{name}: description"), found);
        }
        if let Some(hidden) = entry.get("hidden") {
            if !hidden.is_boolean() {
                found.problem(format!("{name}: hidden is true or false"));
            }
        }
        match entry.get("when") {
            Some(when) => {
                let Some(fields) = when.as_object() else {
                    found.problem(format!("{name}: when is a set of fields (event, and maybe name and value)"));
                    continue;
                };
                match fields.get("event").and_then(Value::as_str) {
                    Some(event) if ACHIEVEMENT_EVENTS.contains(&event) => {}
                    Some(event) => found.problem(format!("{name}: {event} isn't an event ({})", ACHIEVEMENT_EVENTS.join(", "))),
                    None => found.problem(format!("{name}: when needs an event")),
                }
                if fields.get("name").is_some_and(|n| n.as_str().is_none_or(str::is_empty)) {
                    found.problem(format!("{name}: when's name is text"));
                }
                if fields.get("value").is_some_and(|v| v.as_i64().is_none_or(|v| i32::try_from(v).is_err())) {
                    found.problem(format!("{name}: when's value is a whole number"));
                }
                for key in fields.keys() {
                    if !["event", "name", "value"].contains(&key.as_str()) {
                        found.problem(format!("{name}: when has event, name and value, not {key}"));
                    }
                }
            }
            None if !has_code => found.problem(format!("{name}: nothing unlocks it (no when, and the mod has no code to call unlock_achievement)")),
            None => {}
        }
    }
}

/// A mod's code: a library for each device, each truly for that device.
fn check_plugins(folder: &Path, manifest: &Value, options: &BTreeMap<String, Vec<String>>, found: &mut Findings) {
    let Some(plugins) = manifest.get("plugins") else { return };
    let Some(plugins) = plugins.as_array() else {
        found.problem("plugins is a list");
        return;
    };
    for (i, plugin) in plugins.iter().enumerate() {
        let name = format!("plugin {}", i + 1);
        match plugin.get("api").and_then(Value::as_u64) {
            Some(PLUGIN_API) => {}
            Some(api) if api < PLUGIN_API => found.problem(format!(
                "{name}: api {api} is an older interface; rebuild it with nebula_mod_api.h {PLUGIN_API} (setting() is now get_tunable())"
            )),
            Some(api) => found.problem(format!("{name}: api {api} is newer than this LumaLink knows ({PLUGIN_API})")),
            None => found.problem(format!("{name}: api is the number of the interface it was built for ({PLUGIN_API})")),
        }
        if let Some(when) = plugin.get("when") {
            check_when(when, &name, options, found);
        }
        let Some(binaries) = plugin.get("binaries").and_then(Value::as_object).filter(|b| !b.is_empty()) else {
            found.problem(format!("{name}: binaries names a library for each device"));
            continue;
        };
        for (target, file) in binaries {
            let Some((_, kind, machine)) = PLUGIN_TARGETS.iter().find(|(t, _, _)| t == target) else {
                let known: Vec<&str> = PLUGIN_TARGETS.iter().map(|(t, _, _)| *t).collect();
                found.problem(format!("{name}: {target} isn't a device ({})", known.join(", ")));
                continue;
            };
            let Some(file) = file.as_str().filter(|f| inside(f)) else {
                found.problem(format!("{name}: {target} is a path in the file"));
                continue;
            };
            let Ok(bytes) = fs::read(folder.join(file)) else {
                found.problem(format!("{name}: {file} isn't in the file"));
                continue;
            };
            if !binary_machines(&bytes).contains(&(*kind, *machine)) {
                found.problem(format!("{name}: {file} isn't a library for {target}"));
            }
        }
        // A .luma is for every device the game runs on: one without a library for some can't be played there.
        let missing: Vec<&str> = BUILD_TARGETS.iter().map(|(t, _, _)| *t).filter(|t| !binaries.contains_key(*t)).collect();
        if !missing.is_empty() {
            found.problem(format!("{name} has no library for: {}. lumalink build makes every one from its source.", missing.join(", ")));
        }
    }
    if !plugins.is_empty() {
        found.note("Runs its own code. Players are asked before it first runs.");
    }
}

fn check_shader_file(path: &Path, found: &mut Findings) {
    let Ok(bytes) = fs::read(path) else {
        found.problem(format!("{} can't be read", path.display()));
        return;
    };
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    if bytes.len() < 56 || &bytes[..8] != b"NBSHADER" {
        found.problem(format!("{name} is damaged"));
        return;
    }
    let count = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    if 56 + count * 24 > bytes.len() {
        found.problem(format!("{name} is cut short"));
    }
}

fn check_folder(folder: &Path, store: Option<&GameStore>, found: &mut Findings) {
    let manifest = match read_manifest(folder) {
        Ok(manifest) => manifest,
        Err(error) => {
            found.problem(if folder.join("luma.json").exists() { format!("{error:#}") } else { "this has no luma.json".to_string() });
            return;
        }
    };
    if !manifest.is_object() {
        found.problem("luma.json has a mistake: it's not a set of fields");
        return;
    }
    let format = manifest.get("format").and_then(Value::as_u64).unwrap_or(0);
    if format == 0 {
        found.problem("luma.json doesn't say its format");
    } else if format > u64::from(PACKAGE_FORMAT) {
        found.problem(format!("made by a newer tool (format {format}); update LumaLink to check it"));
        return;
    }
    let id = manifest.get("id").and_then(Value::as_str).unwrap_or("");
    if !valid_id(id) {
        found.problem(format!("the id {id:?} has to be lower case letters, digits and -, at most 64, not starting or ending with -, and not a name Windows keeps for itself (con, nul, com1)"));
    }
    match manifest.get("name") {
        Some(name) if !text_of(name).trim().is_empty() => check_text(name, "name", found),
        _ => found.problem("it has no name"),
    }
    match manifest.get("version").and_then(Value::as_str) {
        Some(version) if valid_version(version) => {}
        _ => found.problem("its version has to be numbers with dots, like 1.2.0"),
    }
    match manifest.get("game").and_then(Value::as_str) {
        Some("RMG") => {}
        Some("SB4" | "SB5") => found.problem("this says it's for Super Mario Galaxy 2"),
        Some(other) => found.problem(format!("game has to be RMG, not {other}")),
        None => found.problem("game has to be RMG"),
    }
    if let Some(versions) = manifest.get("versions") {
        for version in string_list(versions, "versions", found) {
            if !VERSION_LETTERS.contains(&version) {
                found.problem(format!("versions: {version} isn't a version ({})", VERSION_LETTERS.join(", ")));
            }
        }
    }
    check_regions(&manifest, found);
    if let Some(nebula) = manifest.get("nebula") {
        if !nebula.as_str().is_some_and(valid_version) {
            found.problem("nebula has to be numbers with dots, like 2.1.0");
        }
    }
    if let Some(requires) = manifest.get("requires") {
        for feature in string_list(requires, "requires", found) {
            if feature != "custom-shaders" && feature != "tunables" && feature != "player-animations" && feature != "coop" && feature != "characters" {
                found.problem(format!("requires {feature}, a feature this version of LumaLink doesn't know"));
            }
        }
    }
    if let Some(needs) = manifest.get("needs") {
        check_mod_list(needs, "needs", "version", found);
    }
    if let Some(conflicts) = manifest.get("conflicts") {
        check_mod_list(conflicts, "conflicts", "below", found);
    }
    for key in ["load_after", "load_before"] {
        if let Some(list) = manifest.get(key) {
            for other in string_list(list, key, found) {
                if !valid_id(other) {
                    found.problem(format!("{key}: {other:?} isn't an id"));
                }
            }
        }
    }
    if let Some(description) = manifest.get("description") {
        check_text(description, "description", found);
    }
    match manifest.get("authors") {
        Some(authors) if authors.as_array().is_some_and(|a| !a.is_empty()) => {
            let _ = string_list(authors, "authors", found);
        }
        Some(authors) if !authors.is_array() => found.problem("authors is a list of names"),
        _ => found.note("it doesn't name its authors"),
    }
    if let Some(icon) = manifest.get("icon") {
        match icon.as_str() {
            Some(path) if inside(path) => check_picture(folder, path, "the icon", found),
            _ => found.problem("icon is a path in the file"),
        }
    }
    if let Some(pictures) = manifest.get("pictures") {
        let list = string_list(pictures, "pictures", found);
        if list.len() > MAX_PICTURES {
            found.problem(format!("pictures lists {} pictures: the game shows the first {MAX_PICTURES}", list.len()));
        }
        for path in list {
            if inside(path) {
                check_picture(folder, path, &format!("the picture {path}"), found);
            } else {
                found.problem(format!("pictures: {path:?} isn't a path in the file"));
            }
        }
    }
    if let Some(homepage) = manifest.get("homepage") {
        if !homepage.as_str().is_some_and(|h| h.starts_with("https://") || h.starts_with("http://")) {
            found.problem("homepage is a web address starting with https://");
        }
    }
    if let Some(license) = manifest.get("license") {
        if license.as_str().is_none_or(|l| l.trim().is_empty()) {
            found.problem("license is text: a name such as CC-BY-4.0, or the license's own words");
        }
    }
    if let Some(save) = manifest.get("save") {
        if let Some(path) = save.get("path") {
            match path.as_str() {
                Some(path) if inside(path) && folder.join(path).is_dir() => {}
                _ => found.problem("save.path has to be a folder in the file"),
            }
        }
        match save.get("mode").and_then(Value::as_str) {
            Some("shared" | "own" | "copy") | None => {}
            Some(other) => found.problem(format!("save.mode is shared, own or copy, not {other}")),
        }
        if let Some(tag) = save.get("tag") {
            if !tag.as_str().is_some_and(valid_id) {
                found.problem("save.tag can only have lower case letters, digits and -");
            }
        }
        if save.get("player_can_change").is_some_and(|v| !v.is_boolean()) {
            found.problem("save.player_can_change is true or false");
        }
    }
    let options = check_options(&manifest, found);
    check_plugins(folder, &manifest, &options, found);
    check_achievements(&manifest, found);
    check_galaxies(folder, &manifest, found);
    check_music(folder, &manifest, found);
    check_progression(&manifest, found);
    if let Some(text) = manifest.get("text") {
        match text.as_object() {
            Some(messages) => {
                for (id, words) in messages {
                    check_text(words, &format!("text {id}"), found);
                }
            }
            None => found.problem("text is a set of message ids and their words"),
        }
    }
    check_layers(folder, &manifest, &options, store, found);
    for name in ["dxil.shaders", "spirv.shaders"] {
        let path = folder.join("shaders").join(name);
        if path.exists() {
            check_shader_file(&path, found);
        }
    }
}

/// Everything wrong with a project folder or `.luma`, before anyone else finds it.
pub fn check(input: &Path, store: Option<&GameStore>) -> Result<Vec<Finding>> {
    let mut found = Findings::default();
    if input.is_dir() {
        check_folder(input, store, &mut found);
        return Ok(found.0);
    }
    // A folder of its own, with a name nobody can guess and put a link at first.
    let work = tempfile::Builder::new().prefix("lumalink-check-").tempdir().context("a working folder can't be made")?;
    match first_entry_is_manifest(input).and_then(|first| {
        if !first {
            found.problem("luma.json has to be the file's first entry");
        }
        unpack(input, work.path())
    }) {
        Ok(()) => check_folder(work.path(), store, &mut found),
        Err(error) => found.problem(format!("{error:#}")),
    }
    Ok(found.0)
}

fn first_entry_is_manifest(input: &Path) -> Result<bool> {
    let file = fs::File::open(input).with_context(|| format!("{} can't be read", input.display()))?;
    let mut zip = zip::ZipArchive::new(file).with_context(|| format!("{} is damaged, or isn't a .luma", input.display()))?;
    if zip.is_empty() {
        bail!("{} is empty", input.display());
    }
    let first = zip.by_index(0)?.name() == "luma.json";
    Ok(first)
}

/// Opens a `.luma` into a folder.
pub fn unpack(input: &Path, output: &Path) -> Result<()> {
    fs::create_dir_all(output).with_context(|| format!("{} can't be created", output.display()))?;
    crate::source::unzip(input, output).with_context(|| format!("{} is damaged, or isn't a .luma", input.display()))
}

/// One file converted on its own, as the game will: `path` is where it goes in the game, such as `ObjectData/Kuribo.arc`.
/// Returns what the conversion produced, for an author testing one model, texture or track.
pub fn cook_one(source: &Path, path: &str, game_id: &str, store: Option<&Path>, output: &Path) -> Result<String> {
    use nebula_import::cook::overlay::write_overlay;
    use nebula_import::cook::Stats;
    let changes = [Change { path: path.trim_start_matches('/').to_string(), source: PathBuf::from(source), merge: false }];
    let mut stats = Stats::default();
    let failed = write_overlay(game_id, &changes, store, output, &mut stats)?;
    if let Some((_, why)) = failed.first() {
        bail!("{why}");
    }
    Ok(stats.summary())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manifest_path_stays_inside_as_the_game_reads_it() {
        for outside in ["../b", "a/../../b", "a/.. /b", "..\u{0}/b", "a/.../b", "C:b", "/b", "\\b", "a\\..\\..\\b", ""] {
            assert!(!inside(outside), "{outside:?}");
        }
        for path in ["icon.png", "pictures/1.png", "./a/b.png", "a.b/c"] {
            assert!(inside(path), "{path:?}");
        }
    }

    #[test]
    fn only_what_belongs_goes_in_the_file() {
        for good in ["luma.json", "layers/base/files/a.arc", "source/plugin.c", "plugins/a.dll", "layers/base/files/old.luma",
                     "layers/base/files/a.lib"] {
            assert!(belongs_in_file(Path::new(good)), "{good}");
        }
        for bad in [".git/config", ".DS_Store", "layers/.hidden/a", "My Mod.luma", "My Mod.lumacoop", "My Mod.luma.writing", "plugins/a-windows-x64.lib",
                    "plugins/a.pdb", "plugins/a.exp", "plugins/obj/a.o"] {
            assert!(!belongs_in_file(Path::new(bad)), "{bad}");
        }
    }

    #[test]
    fn music_is_checked() {
        let dir = tempfile::tempdir().unwrap();
        let files = dir.path().join("layers/base/files/AudioRes/Stream");
        fs::create_dir_all(&files).unwrap();
        fs::write(files.join("Mine.wav"), b"x").unwrap();
        let problems = |music: Value| {
            let mut found = Findings::default();
            check_music(dir.path(), &json!({ "music": music }), &mut found);
            found.0.len()
        };
        assert_eq!(problems(json!([{ "name": "Mine", "file": "AudioRes/Stream/Mine.wav", "stages": [{ "stage": "S" }, { "stage": "S", "scenario": 2 }, { "stage": "S", "change_area": 1 }] }])), 0);
        assert_eq!(problems(json!([{ "name": "BGM_MINE", "file": "AudioRes/Stream/Mine.wav" }])), 1);
        assert_eq!(problems(json!([{ "name": "Mine", "file": "AudioRes/Stream/Gone.wav" }])), 1);
        assert_eq!(problems(json!([{ "name": "Mine", "file": "AudioRes/Stream/Mine.txt" }])), 2);
        assert_eq!(problems(json!([{ "name": "Mine", "file": "AudioRes/Stream/Mine.wav" }, { "name": "Mine", "file": "AudioRes/Stream/Mine.wav" }])), 1);
        assert_eq!(problems(json!([{ "name": "Mine", "file": "AudioRes/Stream/Mine.wav", "stages": [{ "stage": "S", "scenario": 1, "change_area": 1 }] }])), 1);
        assert_eq!(problems(json!("Mine")), 1);
    }

    /// Every example in examples/ checks with no problem, but for its libraries before `lumalink build` makes them
    /// (they are not kept in the repository).
    #[test]
    fn every_example_checks() {
        let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let mut count = 0;
        for entry in fs::read_dir(&examples).unwrap() {
            let folder = entry.unwrap().path();
            if !folder.join("luma.json").is_file() {
                continue;
            }
            count += 1;
            let found = check(&folder, None).unwrap();
            let left: Vec<&str> = problems(&found)
                .into_iter()
                .filter(|p| (!p.contains("isn't in the file") || !p.contains("plugins/")) && !p.contains("has no library for"))
                .collect();
            assert!(left.is_empty(), "{}: {left:?}", folder.display());
        }
        assert!(count >= 15, "{count} examples");
    }

    /// NebulaCoop's built-in mods (builtin/, copied into nebula-coop's runtime/src/builtin_mods.inc) check with no
    /// problem, and Character Movesets sets Luigi's numbers alone, among them NebulaCoop's own.
    #[test]
    fn every_builtin_mod_checks() {
        let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../builtin");
        let mut count = 0;
        for entry in fs::read_dir(&builtin).unwrap() {
            let folder = entry.unwrap().path();
            if !folder.join("luma.json").is_file() {
                continue;
            }
            count += 1;
            let found = check(&folder, None).unwrap();
            assert!(problems(&found).is_empty(), "{}: {:?}", folder.display(), problems(&found));
            assert!(is_coop(&read_manifest(&folder).unwrap()), "{}: a built-in mod is a co-op mod", folder.display());
        }
        assert!(count >= 1);
        let movesets = read_manifest(&builtin.join("character-movesets")).unwrap();
        let numbers = &movesets["layers"][0]["player_constants"];
        assert_eq!(numbers.as_object().unwrap().keys().collect::<Vec<_>>(), vec!["luigi"]);
        assert_eq!(numbers["luigi"]["mBrakeSlide"], json!(1));
    }

    #[test]
    fn a_shader_names_a_material_of_the_mods_models() {
        let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let lantern = examples.join("glowing-lantern");
        assert!(problems(&check(&lantern, None).unwrap()).is_empty());
        let dir = tempfile::tempdir().unwrap();
        for entry in walkdir::WalkDir::new(&lantern).into_iter().filter_map(std::result::Result::ok) {
            let to = dir.path().join(entry.path().strip_prefix(&lantern).unwrap());
            if entry.file_type().is_dir() {
                fs::create_dir_all(&to).unwrap();
            } else {
                fs::copy(entry.path(), &to).unwrap();
            }
        }
        let mut manifest = read_manifest(dir.path()).unwrap();
        manifest["layers"][0]["materials"] = json!({ "Lantern_Papr": "layers/base/shaders/paper.hlsl" });
        fs::write(dir.path().join("luma.json"), manifest.to_string()).unwrap();
        let found = problems(&check(dir.path(), None).unwrap()).join("\n");
        assert!(found.contains("material Lantern_Papr: no model the mod brings has it") && found.contains("Lantern_Paper"), "{found}");
        // lumalink shader starts from a mod's own .glb.
        let out = tempfile::tempdir().unwrap();
        let model = lantern.join("layers/base/files/ObjectData/GlowLantern.arc/GlowLantern.bdl.glb");
        let written = write_generated_shaders(&model, out.path()).unwrap();
        assert!(written.contains(&"Lantern_Paper.hlsl".to_string()), "{written:?}");
        assert!(out.path().join("program.hlsli").is_file());
    }

    #[test]
    fn regions_are_checked() {
        let problems = |regions: Value| {
            let mut found = Findings::default();
            check_regions(&json!({ "regions": regions }), &mut found);
            found.0.iter().filter(|f| f.serious).count()
        };
        assert_eq!(problems(json!("any")), 0);
        assert_eq!(problems(json!(["US", "PAL"])), 0);
        assert_eq!(problems(json!(["KR"])), 0);
        assert_eq!(problems(json!("US")), 1);
        assert_eq!(problems(json!([])), 1);
        assert_eq!(problems(json!(["EU"])), 1);
        assert_eq!(problems(json!([1])), 1);
        let mut found = Findings::default();
        check_regions(&json!({}), &mut found);
        assert!(found.0.is_empty());
    }

    #[test]
    fn class_scripts_are_checked() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("scripts")).unwrap();
        fs::write(dir.path().join("scripts/angry.lscript"), "on control { return true }\non touch(other, sensor) { }\n").unwrap();
        fs::write(dir.path().join("scripts/broken.lscript"), "on nothing_like_this { }\n").unwrap();
        let problems = |row: Value| {
            let mut found = Findings::default();
            check_rows(dir.path(), &json!({ "path": "layers/base", "object_classes": [row] }), "base", None, &mut found);
            found.0.iter().filter(|f| f.serious).count()
        };
        assert_eq!(problems(json!({ "name": "Kuribo", "script": "angry" })), 0);
        assert_eq!(problems(json!({ "name": "Bramblet", "like": "Kuribo", "script": "angry" })), 0);
        assert_eq!(problems(json!({ "name": "Bramblet", "like": "Kuribo" })), 0);
        assert_eq!(problems(json!({ "name": "Kuribo" })), 1, "neither like nor script");
        assert_eq!(problems(json!({ "name": "Kuribo", "script": "missing" })), 1);
        assert_eq!(problems(json!({ "name": "Kuribo", "script": "broken" })), 1);
    }

    #[test]
    fn forms_are_checked() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("scripts")).unwrap();
        fs::write(dir.path().join("scripts/gold.lscript"), "on start { }\non frame { }\non end { }\n").unwrap();
        let problems = |row: Value| {
            let mut found = Findings::default();
            check_rows(dir.path(), &json!({ "path": "layers/base", "forms": [row] }), "base", None, &mut found);
            found.0.iter().filter(|f| f.serious).count()
        };
        assert_eq!(problems(json!({ "name": "gold", "acts_like": "fire", "looks_like": "fire", "color_frame": 2, "frames": 600, "script": "gold" })), 0);
        assert_eq!(problems(json!({ "name": "plain" })), 0);
        assert_eq!(problems(json!({ "acts_like": "fire" })), 1);
        assert_eq!(problems(json!({ "name": "x", "acts_like": "lava" })), 1);
        assert_eq!(problems(json!({ "name": "x", "frames": 1.5 })), 1);
        assert_eq!(problems(json!({ "name": "x", "script": "missing" })), 1);
    }

    #[test]
    fn characters_are_checked() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("layers/base/files/ObjectData")).unwrap();
        fs::write(dir.path().join("layers/base/files/ObjectData/Wario.arc"), b"").unwrap();
        let check = |layer: Value| {
            let mut found = Findings::default();
            check_rows(dir.path(), &layer, "base", None, &mut found);
            (found.0.iter().filter(|f| f.serious).count(), found.0.iter().filter(|f| !f.serious).count())
        };
        let row = |row: Value| check(json!({ "path": "layers/base", "characters": [row] }));
        assert_eq!(row(json!({ "id": "wario", "name": "WARIO" })), (0, 0));
        assert_eq!(row(json!({ "id": "waluigi", "like": "luigi" })), (0, 1), "no Waluigi.arc: a note");
        assert_eq!(row(json!({ "id": "Wario" })).0, 1);
        assert_eq!(row(json!({ "id": "mario" })).0, 1);
        assert_eq!(row(json!({ "id": "w", "like": "wario" })).0, 1);
        assert_eq!(row(json!({ "id": "w", "archive": "../Wario" })).0, 1);
        assert_eq!(check(json!({ "path": "layers/base", "characters": { "id": "w" } })).0, 1);
        assert_eq!(check(json!({ "path": "layers/base", "player_constants": { "wario": { "mJumpHeight": 20 } } })).0, 0);
        assert_eq!(check(json!({ "path": "layers/base", "player_constants": { "Wario Bros": { "mJumpHeight": 20 } } })).0, 1);
    }

    #[test]
    fn achievements_are_checked() {
        let problems = |manifest: Value| {
            let mut found = Findings::default();
            check_achievements(&manifest, &mut found);
            found.0.iter().filter(|f| f.serious).count()
        };
        let one = |entry: Value| problems(json!({ "achievements": [entry] }));
        assert_eq!(one(json!({ "id": "coins", "name": "Coins", "when": { "event": "coin" } })), 0);
        assert_eq!(one(json!({ "id": "boss", "name": { "default": "Boss", "ja": "ボス" }, "description": "Beat it", "hidden": true,
                                "when": { "event": "actor_kill", "name": "Kameck", "value": 1 } })), 0);
        assert_eq!(problems(json!({ "achievements": [{ "id": "a", "name": "A" }], "plugins": [{ "file": "x" }] })), 0);
        assert_eq!(one(json!({ "id": "a", "name": "A" })), 1, "nothing unlocks it");
        assert_eq!(one(json!({ "id": "A b", "name": "A", "when": { "event": "coin" } })), 1);
        assert_eq!(one(json!({ "id": "a", "when": { "event": "coin" } })), 1);
        assert_eq!(one(json!({ "id": "a", "name": "A", "when": { "event": "coins" } })), 1);
        assert_eq!(one(json!({ "id": "a", "name": "A", "when": { "event": "coin", "value": 1.5 } })), 1);
        assert_eq!(one(json!({ "id": "a", "name": "A", "when": { "event": "coin", "count": 3 } })), 1);
        assert_eq!(one(json!({ "id": "a", "name": "A", "hidden": "yes", "when": { "event": "coin" } })), 1);
        assert_eq!(one(json!({ "id": "a", "name": "A", "when": "coin" })), 1);
        let twice = json!({ "id": "a", "name": "A", "when": { "event": "coin" } });
        assert_eq!(problems(json!({ "achievements": [twice.clone(), twice.clone()] })), 1);
        assert_eq!(problems(json!({ "achievements": vec![twice; 65] })), 65, "too many, and 64 repeats");
        assert_eq!(problems(json!({ "achievements": "a" })), 1);
    }

    fn write(root: &Path, relative: &str, text: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn problems(found: &[Finding]) -> Vec<&str> {
        found.iter().filter(|f| f.serious).map(|f| f.text.as_str()).collect()
    }

    #[test]
    fn a_project_starts_packs_checks_and_opens() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Test Mod").unwrap();
        assert!(new_project(&project, "Again").is_err());
        let empty = check(&project, None).unwrap();
        assert!(problems(&empty).iter().any(|t| t.contains("nothing the game can use")), "{empty:?}");
        write(&project, "layers/base/files/StageData/note.bin", "hello");
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
        assert!(found.iter().any(|f| f.text.contains("authors")));

        let out = dir.path().join("out");
        let outcome = pack(&project, &out, None).unwrap();
        assert!(outcome.file.ends_with("Test Mod.luma"));
        assert!(problems(&check(&outcome.file, None).unwrap()).is_empty());

        let opened = dir.path().join("opened");
        unpack(&outcome.file, &opened).unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&fs::read(opened.join("luma.json")).unwrap()).unwrap();
        assert_eq!(manifest["id"], "test-mod");
        assert_eq!(manifest["version"], "1.0.0");
        assert_eq!(manifest["game"], "RMG");
        assert!(opened.join("layers/base/files/StageData/note.bin").is_file());
    }

    #[test]
    fn a_coop_project_packs_as_lumacoop() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Coop Mod").unwrap();
        write(&project, "layers/base/files/StageData/note.bin", "hello");
        let mut manifest: serde_json::Value = serde_json::from_slice(&fs::read(project.join("luma.json")).unwrap()).unwrap();
        manifest["requires"] = serde_json::json!(["coop"]);
        fs::write(project.join("luma.json"), serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");

        let out = dir.path().join("out");
        let outcome = pack(&project, &out, None).unwrap();
        assert!(outcome.file.ends_with("Coop Mod.lumacoop"), "{}", outcome.file.display());
        assert!(!out.join("Coop Mod.luma").exists());
        assert!(problems(&check(&outcome.file, None).unwrap()).is_empty());
        let opened = dir.path().join("opened");
        unpack(&outcome.file, &opened).unwrap();
        let back: serde_json::Value = serde_json::from_slice(&fs::read(opened.join("luma.json")).unwrap()).unwrap();
        assert!(is_coop(&back));
        // Packed again from inside the project, the earlier file isn't taken in.
        fs::copy(&outcome.file, project.join("Coop Mod.lumacoop")).unwrap();
        let again = pack(&project, &out, None).unwrap();
        let inner = dir.path().join("inner");
        unpack(&again.file, &inner).unwrap();
        assert!(!inner.join("Coop Mod.lumacoop").exists());
    }

    fn checked(manifest: &str) -> Vec<String> {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "luma.json", manifest);
        write(dir.path(), "layers/base/files/StageData/a.arc", "x");
        check(dir.path(), None).unwrap().into_iter().filter(|f| f.serious).map(|f| f.text).collect()
    }

    const GOOD: &str = r#"{"format":1,"id":"m","name":"M","version":"1.0","game":"RMG","requires":["tunables"],"authors":["A"],
        "options":[{"id":"o","name":{"default":"O","ja":"オ"},"default":"a","choices":[{"id":"a","name":"A"},{"id":"b","name":"B"}]}],
        "layers":[{"path":"layers/base","tunables":{"GameRules.cDodoryuShellOnly":true},"when":{"version":["E","P"],"platform":["android"],"option":{"o":["b"]},"language":["UsEnglish"]}}]}"#;

    #[test]
    fn check_passes_a_good_file_and_names_each_mistake() {
        assert!(checked(GOOD).is_empty(), "{:?}", checked(GOOD));
        for (from, to, says) in [
            (r#""format":1"#, r#""format":2"#, "newer tool"),
            (r#""id":"m""#, r#""id":"Bad Id""#, "the id"),
            (r#""version":"1.0""#, r#""version":"one""#, "its version"),
            (r#""game":"RMG""#, r#""game":"SB4""#, "Super Mario Galaxy 2"),
            (r#""game":"RMG""#, r#""game":"X""#, "game has to be RMG"),
            (r#""default":"a""#, r#""default":"z""#, "isn't one of its choices"),
            (r#""ja":"オ""#, r#""xx":"オ""#, "isn't a language"),
            (r#""GameRules.cDodoryuShellOnly""#, r#""cDodoryuShellOnly""#, "is not \"<file stem>.<name>\""),
            (r#"true},"when""#, r#""yes"},"when""#, "needs a number, true or false"),
            (r#""tunables":{"#, r#""switches":{"no_low_health_warning":1},"tunables":{"#, "no_low_health_warning is \"MarioEffect.cLowHealthWarning\", the opposite sense"),
            (r#""requires":["tunables"],"#, "", "says \"requires\": [\"tunables\"]"),
            (r#""version":["E","P"]"#, r#""version":["Q"]"#, "isn't a version"),
            (r#""platform":["android"]"#, r#""platform":["toaster"]"#, "isn't a device"),
            (r#""o":["b"]"#, r#""o":["z"]"#, "has no choice z"),
            (r#""o":["b"]"#, r#""z":["b"]"#, "no option called z"),
            (r#""language":["UsEnglish"]"#, r#""language":["Klingon"]"#, "language folders"),
            (r#""language":["UsEnglish"]"#, r#""sky":["blue"]"#, "doesn't know"),
            (r#""path":"layers/base""#, r#""path":"layers/none""#, "isn't in the file"),
            (r#""path":"layers/base""#, r#""path":"../up""#, "leaves the file"),
            (r#""authors":["A"]"#, r#""authors":["A"],"save":{"mode":"mine"}"#, "save.mode"),
            (r#""authors":["A"]"#, r#""authors":["A"],"save":{"tag":"Bad Tag"}"#, "save.tag"),
            (r#""authors":["A"]"#, r#""authors":["A"],"save":{"path":"none"}"#, "save.path"),
            (r#""authors":["A"]"#, r#""authors":["A"],"icon":"none.png""#, "the icon isn't in the file"),
            (r#""authors":["A"]"#, r#""authors":["A"],"icon":"../up.png""#, "icon is a path in the file"),
            (r#""authors":["A"]"#, r#""authors":["A"],"pictures":["none.png"]"#, "the picture none.png isn't in the file"),
            (r#""authors":["A"]"#, r#""authors":["A"],"pictures":["../x.png"]"#, "isn't a path in the file"),
            (r#""authors":["A"]"#, r#""authors":["A"],"pictures":"x.png""#, "pictures is a list"),
            (r#""authors":["A"]"#, r#""authors":["A"],"homepage":"example.com""#, "homepage is a web address"),
            (r#""authors":["A"]"#, r#""authors":["A"],"license":"""#, "license is text"),
        ] {
            assert!(GOOD.contains(from), "{from}");
            let bad = checked(&GOOD.replace(from, to));
            assert!(bad.iter().any(|t| t.contains(says)), "{from} -> {to}: {bad:?}");
        }
    }

    fn png(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n', 0, 0, 0, 13];
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&[8, 6, 0, 0, 0]);
        bytes
    }

    #[test]
    fn check_reads_the_icon_and_pictures() {
        let dir = tempfile::tempdir().unwrap();
        let with = |extra: &str| {
            write(dir.path(), "luma.json", &GOOD.replace(r#""authors":["A"]"#, &format!(r#""authors":["A"],{extra}"#)));
            write(dir.path(), "layers/base/files/StageData/a.arc", "x");
            check(dir.path(), None).unwrap()
        };
        fs::write(dir.path().join("icon.png"), png(256, 256)).unwrap();
        fs::write(dir.path().join("big.png"), png(4096, 2048)).unwrap();
        fs::write(dir.path().join("huge.png"), png(9000, 10)).unwrap();
        fs::write(dir.path().join("fake.png"), b"not a picture at all, just words").unwrap();
        let good = with(r#""icon":"icon.png","pictures":["icon.png","big.png"],"homepage":"https://x.example","license":"MIT""#);
        assert!(good.iter().all(|f| !f.serious), "{good:?}");
        assert!(good.iter().any(|f| f.text.contains("big.png is 4096x2048")), "{good:?}");
        let huge = with(r#""pictures":["huge.png"]"#);
        assert!(huge.iter().any(|f| f.serious && f.text.contains("9000x10")), "{huge:?}");
        let fake = with(r#""icon":"fake.png""#);
        assert!(fake.iter().any(|f| f.serious && f.text.contains("the icon has to be a PNG")), "{fake:?}");
        let many: Vec<String> = (0..17).map(|_| "\"icon.png\"".to_string()).collect();
        let lots = with(&format!(r#""pictures":[{}]"#, many.join(",")));
        assert!(lots.iter().any(|f| f.serious && f.text.contains("the first 16")), "{lots:?}");
    }

    fn pe(machine: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; 0x100];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[0x3C] = 0x80;
        bytes[0x80..0x84].copy_from_slice(b"PE\0\0");
        bytes[0x84..0x86].copy_from_slice(&machine.to_le_bytes());
        bytes
    }

    #[test]
    fn check_reads_a_plugins_libraries() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Code").unwrap();
        write(&project, "layers/base/files/StageData/note.bin", "hello");
        let with = |plugins: &str| {
            let mut manifest: Value = serde_json::from_slice(&fs::read(project.join("luma.json")).unwrap()).unwrap();
            manifest["plugins"] = serde_json::from_str(plugins).unwrap();
            fs::write(project.join("luma.json"), manifest.to_string()).unwrap();
            check(&project, None).unwrap()
        };
        fs::create_dir_all(project.join("plugins")).unwrap();
        fs::write(project.join("plugins/a.dll"), pe(0x8664)).unwrap();
        fs::write(project.join("plugins/arm.dll"), pe(0xAA64)).unwrap();
        fs::write(project.join("plugins/not.dll"), "text").unwrap();
        let all = every_library(&project);

        let good = with(&format!(r#"[{{ "api": 2, "binaries": {all} }}]"#));
        assert!(problems(&good).is_empty(), "{good:?}");
        assert!(good.iter().any(|f| f.text.contains("its own code")));
        let windows_only = with(r#"[{ "api": 2, "binaries": { "windows-x64": "plugins/a.dll" } }]"#);
        assert!(problems(&windows_only).iter().any(|t| t.contains("no library for") && t.contains("android-arm64")), "{windows_only:?}");

        for (plugins, says) in [
            (r#"[{ "binaries": { "windows-x64": "plugins/a.dll" } }]"#, "api is"),
            (r#"[{ "api": 3, "binaries": { "windows-x64": "plugins/a.dll" } }]"#, "newer"),
            (r#"[{ "api": 1, "binaries": { "windows-x64": "plugins/a.dll" } }]"#, "setting() is now get_tunable()"),
            (r#"[{ "api": 2 }]"#, "binaries names"),
            (r#"[{ "api": 2, "binaries": { "amiga": "plugins/a.dll" } }]"#, "isn't a device"),
            (r#"[{ "api": 2, "binaries": { "windows-x64": "../a.dll" } }]"#, "path in the file"),
            (r#"[{ "api": 2, "binaries": { "windows-x64": "plugins/gone.dll" } }]"#, "isn't in the file"),
            (r#"[{ "api": 2, "binaries": { "windows-x64": "plugins/not.dll" } }]"#, "isn't a library"),
            (r#"[{ "api": 2, "binaries": { "windows-x64": "plugins/arm.dll" } }]"#, "isn't a library"),
            (r#"[{ "api": 2, "binaries": { "linux-x64": "plugins/a.dll" } }]"#, "isn't a library"),
        ] {
            let found = with(plugins);
            assert!(problems(&found).iter().any(|t| t.contains(says)), "{plugins}: {found:?}");
        }
    }

    #[test]
    fn check_reads_a_mods_galaxies() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "New Galaxy").unwrap();
        write(&project, "layers/base/files/StageData/MyGalaxy/MyGalaxyScenario.arc", "x");
        let with = |galaxies: &str, save: &str| {
            let mut manifest: Value = serde_json::from_slice(&fs::read(project.join("luma.json")).unwrap()).unwrap();
            manifest["galaxies"] = serde_json::from_str(galaxies).unwrap();
            manifest["save"] = serde_json::from_str(save).unwrap();
            fs::write(project.join("luma.json"), manifest.to_string()).unwrap();
            check(&project, None).unwrap()
        };
        let own = r#"{ "mode": "own" }"#;
        let good = with(r#"[{ "name": "MyGalaxy", "dome": 1, "hungry_luma": { "slot": 7, "star_bits": 400 } }]"#, own);
        assert!(problems(&good).is_empty(), "{good:?}");
        // A dome past the game's six is one the mod builds.
        let past_six = with(r#"[{ "name": "MyGalaxy", "dome": 9 }]"#, own);
        assert!(problems(&past_six).is_empty(), "{past_six:?}");
        for (galaxies, save, says) in [
            (r#"[{ "dome": 1 }]"#, own, "name is"),
            (r#"[{ "name": "Other" }]"#, own, "no layer has"),
            (r#"[{ "name": "MyGalaxy" }]"#, r#"{ "mode": "shared" }"#, "own saves"),
            (r#"[{ "name": "MyGalaxy", "dome": 17 }]"#, own, "dome"),
            (r#"[{ "name": "MyGalaxy", "stars_to_open": 999 }]"#, own, "stars_to_open"),
            (r#"[{ "name": "MyGalaxy", "hungry_luma": { "slot": 999, "star_bits": 1 } }]"#, own, "slot"),
            (r#"[{ "name": "MyGalaxy", "hungry_luma": { "slot": 7, "star_bits": 0 } }]"#, own, "star_bits"),
        ] {
            let found = with(galaxies, save);
            assert!(problems(&found).iter().any(|t| t.contains(says)), "{galaxies}: {found:?}");
        }
    }

    #[test]
    fn check_reads_a_layers_merge_folder() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Adds").unwrap();
        let merge = "layers/base/merge/ParticleData/Effect.arc/";
        write(&project, &format!("{merge}AutoEffectList.bcsv.json"), r#"{ "rows": [ { "GroupName": "Mine" } ] }"#);
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
        write(&project, &format!("{merge}Particles.jpc"), "JPAC2-10 and the rest");
        let found = check(&project, None).unwrap();
        assert!(problems(&found).iter().any(|t| t.contains("need their names")), "{found:?}");
        write(&project, &format!("{merge}ParticleNames.bcsv.json"), r#"{ "rows": [ { "name": "MyFire" } ] }"#);
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
        write(&project, &format!("{merge}Particles.jpc"), "not one");
        assert!(problems(&check(&project, None).unwrap()).iter().any(|t| t.contains("isn't a particle file")));
        write(&project, &format!("{merge}Particles.jpc"), "JPAC2-10");
        write(&project, &format!("{merge}Other.bin"), "x");
        assert!(problems(&check(&project, None).unwrap()).iter().any(|t| t.contains("only tables")));
        fs::remove_file(project.join(format!("{merge}Other.bin"))).unwrap();
        write(&project, &format!("{merge}Bad.bcsv.json"), r#"{ "rows": [] }"#);
        assert!(problems(&check(&project, None).unwrap()).iter().any(|t| t.contains("rows is a list")));
    }

    #[test]
    fn tunables_are_checked_against_the_game() {
        assert_eq!(tunable_kind("Kuribo.exeFlatDown.1"), Some(("s32", None)));
        assert_eq!(tunable_kind("GameRules.cDodoryuShellOnly"), Some(("bool", None)));
        assert_eq!(tunable_kind("GameRules.cPlayerHealthh"), None);
        assert!(crate::tunable_names::TUNABLES.windows(2).all(|w| w[0].0.as_bytes() < w[1].0.as_bytes()), "sorted, each once");
        // A list: whole, or one place of it.
        let list = crate::tunable_names::TUNABLES.iter().find(|(_, k)| k.ends_with("[4]")).unwrap().0;
        assert_eq!(tunable_kind(&format!("{list}[3]")).map(|k| k.1), Some(None));
        assert_eq!(tunable_kind(&format!("{list}[4]")), None);
        assert_eq!(tunable_kind(list).and_then(|k| k.1), Some(4));
        let check = |key: &str, value: Value| {
            let mut found = Findings::default();
            check_tunable("layer 1", key, &value, &mut found);
            found.0.into_iter().map(|f| f.text).collect::<Vec<_>>()
        };
        assert!(check("Kuribo.exeFlatDown.1", json!(600)).is_empty());
        assert!(check("Kuribo.exeFlatDown.1", json!(1.5))[0].contains("whole number"));
        assert!(check("GameRules.cDodoryuShellOnly", json!(true)).is_empty());
        assert!(check("GameRules.cPlayerHealthh", json!(2))[0].contains("did you mean GameRules.cPlayerHealth?"));
        assert!(check("Kuribo.cWalkSpeed", json!(2))[0].contains("no tunable Kuribo.cWalkSpeed"));
        assert!(check(list, json!([1, null, 2])).is_empty());
        assert!(check(list, json!([1, 2, 3, 4, 5]))[0].contains("list of 4"));
        assert!(check(list, json!(1))[0].contains("list of 4"));
        assert!(check("GameRules.cPlayerHealth", json!({ "option": "health" })).is_empty());
        // Every tunable LumaLink writes when it converts a console mod is one the game has.
        for key in crate::known_patches::tunable_keys() {
            assert!(tunable_kind(key).is_some(), "{key}");
        }
    }

    #[test]
    fn check_tries_a_table_merge_on_the_game() {
        use std::io::{Seek, Write};
        let dir = tempfile::tempdir().unwrap();
        let pak_path = dir.path().join("game.pak");
        let table = nebula_import::cook::table::TableIr::from_json(br#"{"rows":[{"name":"Kuribo","scale":1.5}]}"#).unwrap().to_ntbl();
        let mut pak = fs::File::create(&pak_path).unwrap();
        pak.write_all(&[0u8; 0x100]).unwrap();
        let position = pak.stream_position().unwrap();
        pak.write_all(&table).unwrap();
        let end = pak.stream_position().unwrap();
        let placed = [nebula_import::cook::store::Placed { path: "ObjectData/Things/Things.bcsv".into(), position, size: table.len() as u64, folder: false }];
        nebula_import::cook::store::write_index(&mut pak, end, &placed, b"RMGE01\0\0").unwrap();
        drop(pak);
        let store = GameStore::open(&pak_path).unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Changes").unwrap();
        let merge = "layers/base/merge/ObjectData/Things.arc/Things.bcsv.json";
        let notes = |found: &[Finding]| found.iter().filter(|f| !f.serious).map(|f| f.text.clone()).collect::<Vec<_>>();
        write(&project, merge, r#"{ "change": [ { "where": { "name": "Kuribo" }, "set": { "scale": 2.5 } } ] }"#);
        let found = check(&project, Some(&store)).unwrap();
        assert!(problems(&found).is_empty() && !notes(&found).iter().any(|t| t.contains("matched no row")), "{found:?}");
        write(&project, merge, r#"{ "change": [ { "where": { "name": "Kuribo" }, "set": { "scael": 2.5 } } ] }"#);
        let found = check(&project, Some(&store)).unwrap();
        assert!(problems(&found).iter().any(|t| t.contains("no column scael")), "{found:?}");
        // Without the game it can't be tried.
        assert!(problems(&check(&project, None).unwrap()).is_empty());
        write(&project, merge, r#"{ "remove": [ { "name": "Nobody" } ] }"#);
        let found = check(&project, Some(&store)).unwrap();
        assert!(problems(&found).is_empty() && notes(&found).iter().any(|t| t.contains("matched no row in your game")), "{found:?}");
        fs::remove_file(project.join(merge)).unwrap();
        write(&project, "layers/base/merge/ObjectData/Nothing.arc/Nothing.bcsv.json", r#"{ "rows": [ { "name": "New" } ] }"#);
        let found = check(&project, Some(&store)).unwrap();
        assert!(problems(&found).iter().any(|t| t.contains("your game has no table")), "{found:?}");
    }

    #[test]
    fn check_reads_player_animations() {
        let folder = std::env::temp_dir().join(format!("lumalink-anim-{}", std::process::id()));
        let arc = folder.join("layers/base/files/ObjectData/MarioAnime.arc");
        fs::create_dir_all(&arc).unwrap();
        fs::write(arc.join("GoldSpin.bck"), b"J3D1bck1").unwrap();
        let mut found = Findings::default();
        let layer = serde_json::json!({"path": "layers/base", "player_animations": [
            {"name": "GoldSpin", "file": "GoldSpin", "rate": 0.5, "blend_frames": 4},
            {"name": "Jump", "file": "Jump2"},
            {"name": "Bad", "file": "GoldSpin", "rate": 0, "blend_frames": 2.5},
            {"name": "NoFile"}]});
        assert!(check_rows(&folder, &layer, "base", None, &mut found));
        let said = format!("{:?}", found.0);
        assert!(said.contains("Jump2 isn't in the layer's"), "{said}");
        assert!(!said.contains("GoldSpin isn't"), "{said}");
        assert!(said.contains("rate is a number over 0"), "{said}");
        assert!(said.contains("blend_frames is a whole number"), "{said}");
        assert!(said.contains("needs name and file"), "{said}");
        // With the game: its own bck is found, another is a problem.
        let store = GameStore::from_paths("RMGE01", &["ObjectData/MarioAnime.arc/Jump2.bck"]);
        let layer = serde_json::json!({"path": "layers/base", "player_animations": [{"name": "Jump", "file": "Jump2"}, {"name": "X", "file": "Nope"}]});
        let mut found = Findings::default();
        check_rows(&folder, &layer, "base", Some(&store), &mut found);
        let said = format!("{:?}", found.0);
        assert!(!said.contains("Jump2"), "{said}");
        assert!(said.contains("Nope is neither"), "{said}");
        fs::remove_dir_all(&folder).ok();
    }

    /// A skinned glTF binary with these joints (a chain) and these animations (each moving the first joint).
    fn skinned_glb(joints: &[&str], clips: &[&str]) -> Vec<u8> {
        let mut nodes: Vec<Value> = joints
            .iter()
            .enumerate()
            .map(|(i, j)| if i + 1 < joints.len() { json!({"name": j, "children": [i + 1]}) } else { json!({"name": j}) })
            .collect();
        nodes.push(json!({"name": "mesh", "mesh": 0, "skin": 0}));
        let floats = |values: &[f32]| values.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>();
        let mut bin = floats(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]); // 0: positions, 36
        bin.extend([0u8, 0, 1, 0, 2, 0]); // 36: indices, 6
        bin.extend([0u8; 24]); // 42: joints, 3 x 4 u16
        bin.extend(floats(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])); // 66: weights, 48
        bin.extend(floats(&[0.0, 1.0])); // 114: times, 8
        bin.extend(floats(&[0.0; 6])); // 122: translations, 24
        let identity = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for _ in joints {
            bin.extend(floats(&identity)); // 146: inverse binds
        }
        let animations: Vec<Value> = clips
            .iter()
            .map(|c| json!({"name": c, "samplers": [{"input": 5, "output": 6}], "channels": [{"sampler": 0, "target": {"node": 0, "path": "translation"}}]}))
            .collect();
        let mut document = json!({
            "asset": {"version": "2.0"}, "scene": 0, "scenes": [{"nodes": [0, joints.len()]}], "nodes": nodes,
            "meshes": [{"primitives": [{"attributes": {"POSITION": 0, "JOINTS_0": 2, "WEIGHTS_0": 3}, "indices": 1}]}],
            "skins": [{"joints": (0..joints.len()).collect::<Vec<_>>(), "inverseBindMatrices": 4}],
            "accessors": [
                {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0, 0, 0], "max": [1, 1, 0]},
                {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"},
                {"bufferView": 2, "componentType": 5123, "count": 3, "type": "VEC4"},
                {"bufferView": 3, "componentType": 5126, "count": 3, "type": "VEC4"},
                {"bufferView": 6, "componentType": 5126, "count": joints.len(), "type": "MAT4"},
                {"bufferView": 4, "componentType": 5126, "count": 2, "type": "SCALAR", "min": [0.0], "max": [1.0]},
                {"bufferView": 5, "componentType": 5126, "count": 2, "type": "VEC3"}],
            "bufferViews": [
                {"buffer": 0, "byteOffset": 0, "byteLength": 36}, {"buffer": 0, "byteOffset": 36, "byteLength": 6},
                {"buffer": 0, "byteOffset": 42, "byteLength": 24}, {"buffer": 0, "byteOffset": 66, "byteLength": 48},
                {"buffer": 0, "byteOffset": 114, "byteLength": 8}, {"buffer": 0, "byteOffset": 122, "byteLength": 24},
                {"buffer": 0, "byteOffset": 146, "byteLength": 64 * joints.len()}],
            "buffers": [{"byteLength": bin.len()}],
        });
        if !animations.is_empty() {
            document["animations"] = Value::Array(animations);
        }
        let mut text = serde_json::to_vec(&document).unwrap();
        while text.len() % 4 != 0 {
            text.push(b' ');
        }
        while bin.len() % 4 != 0 {
            bin.push(0);
        }
        let mut out = b"glTF".to_vec();
        out.extend(2u32.to_le_bytes());
        out.extend(u32::try_from(12 + 8 + text.len() + 8 + bin.len()).unwrap().to_le_bytes());
        out.extend(u32::try_from(text.len()).unwrap().to_le_bytes());
        out.extend(b"JSON");
        out.extend(&text);
        out.extend(u32::try_from(bin.len()).unwrap().to_le_bytes());
        out.extend(b"BIN\0");
        out.extend(&bin);
        out
    }

    #[test]
    fn an_object_made_like_the_games_is_compared_with_it() {
        // The game's: a model with joints JointRoot, body, head, and the animations wait, talk, spin.
        let (model, _) = nebula_import::cook::gltf::to_files(&skinned_glb(&["JointRoot", "body", "head"], &[]), None, "Kinopio").unwrap();
        let games = vec![
            ("kinopio.bdl".to_string(), model),
            ("wait.bck".to_string(), Vec::new()),
            ("talk.bck".to_string(), Vec::new()),
            ("spin.bck".to_string(), Vec::new()),
        ];
        let dir = tempfile::tempdir().unwrap();
        let own = dir.path().join("Lumie.arc");
        fs::create_dir_all(&own).unwrap();
        fs::write(own.join("Lumie.bdl.glb"), skinned_glb(&["JointRoot", "body"], &["Wait", "Talk"])).unwrap();
        let mut found = Findings::default();
        compare_with_like(&own, &games, "base", "Lumie", "Kinopio", &mut found);
        let said = format!("{:?}", found.0);
        assert!(said.contains("has no joint head of Kinopio's"), "{said}");
        assert!(said.contains("2 of Kinopio's 3 animations") && said.contains("(spin)"), "{said}");
        // Everything there: nothing said.
        fs::write(own.join("Lumie.bdl.glb"), skinned_glb(&["JointRoot", "body", "head"], &["Wait", "Talk", "Spin"])).unwrap();
        let mut found = Findings::default();
        compare_with_like(&own, &games, "base", "Lumie", "Kinopio", &mut found);
        assert!(found.0.is_empty(), "{:?}", found.0);
    }

    /// The reference (docs/reference/plugin-api.md, written by tools/plugin_api_doc.py) explains every call of the header
    /// `lumalink new` gives plugins: a call added to the game's header and not to the reference fails here.
    #[test]
    fn the_plugin_reference_has_every_call() {
        let reference = include_str!("../../../docs/reference/plugin-api.md");
        let api = PLUGIN_HEADER.split("typedef struct NebulaModApi {").nth(1).and_then(|rest| rest.split("} NebulaModApi;").next()).unwrap();
        let mut calls = 0;
        for line in api.lines() {
            let Some(name) = line.split_once("(*").and_then(|(_, rest)| rest.split_once(")(")).map(|(name, _)| name) else { continue };
            if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                calls += 1;
                assert!(reference.contains(&format!("### `{name}`")), "{name} is not in docs/reference/plugin-api.md: run tools/plugin_api_doc.py");
            }
        }
        assert!(calls > 150, "{calls} calls read from the header");
        let tutorial = include_str!("../../../docs/tutorials/10-first-plugin.md");
        assert!(tutorial.contains(&format!("lists all {calls} calls")), "docs/tutorials/10-first-plugin.md counts the calls: {calls}");
    }

    #[test]
    fn hidden_files_are_not_a_layers() {
        let dir = tempfile::tempdir().unwrap();
        let files = dir.path().join("layers/base/files");
        fs::create_dir_all(files.join(".git")).unwrap();
        fs::write(files.join(".gitkeep"), b"").unwrap();
        fs::write(files.join(".DS_Store"), b"x").unwrap();
        fs::write(files.join(".git/HEAD"), b"x").unwrap();
        fs::create_dir_all(files.join("ObjectData")).unwrap();
        fs::write(files.join("ObjectData/.DS_Store"), b"x").unwrap();
        let changes = layer_changes(dir.path(), &serde_json::json!({"path": "layers/base"}));
        assert!(changes.is_empty(), "{:?}", changes.iter().map(|c| &c.path).collect::<Vec<_>>());
    }

    #[test]
    fn check_reads_a_layers_messages_merge() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Talks").unwrap();
        let all = "layers/base/merge/MessageData/messages.json";
        write(&project, all, r#"{ "lines": { "MyLine": "Hi [player]!" }, "flows": { "AstroGalaxy_Kinopio012": "MyLine" } }"#);
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
        write(&project, all, r#"{ "flows": { "AstroGalaxy_Kinopio012": { "ask": "maybe" } } }"#);
        assert!(problems(&check(&project, None).unwrap()).iter().any(|t| t.contains("not one of")));
        fs::remove_file(project.join(all)).unwrap();
        // One language's: a new line needs that language's words.
        let french = "layers/base/merge/UsFrench/MessageData/Message.arc/message.bmg.json";
        write(&project, french, r#"{ "lines": { "MyLine": { "default": "Salut" } } }"#);
        assert!(problems(&check(&project, None).unwrap()).is_empty());
        write(&project, french, r#"{ "lines": { "MyLine": { "de": "Hallo", "like": "X" } }, "flows": { "X": { "ask": "select", "choices": { "yes": "a" } } } }"#);
        assert!(problems(&check(&project, None).unwrap()).iter().any(|t| t.contains("choices need no")));
    }

    #[test]
    fn a_project_can_start_with_code() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Has Code").unwrap();
        add_plugin(&project).unwrap();
        assert!(project.join("source/nebula_mod_api.h").is_file());
        assert!(fs::read_to_string(project.join("build.cmd")).unwrap().contains("lumalink build"));
        let found = check(&project, None).unwrap();
        assert!(problems(&found).iter().any(|t| t.contains("plugins/has-code-windows-x64.dll isn't in the file")), "{found:?}");
        for &(target, _, ending) in BUILD_TARGETS {
            fs::write(project.join(library_path("has-code", target, ending)), library_for(target)).unwrap();
        }
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
    }

    /// A library header for a device, as check reads it.
    fn library_for(target: &str) -> Vec<u8> {
        let (_, kind, machine) = PLUGIN_TARGETS.iter().find(|(t, _, _)| *t == target).unwrap();
        match kind {
            BinaryKind::Pe => pe(*machine as u16),
            BinaryKind::Elf => {
                let mut bytes = vec![0u8; 64];
                bytes[..4].copy_from_slice(b"\x7fELF");
                bytes[18..20].copy_from_slice(&(*machine as u16).to_le_bytes());
                bytes
            }
            BinaryKind::MachO => {
                let mut bytes = vec![0u8; 64];
                bytes[..4].copy_from_slice(&[0xCF, 0xFA, 0xED, 0xFE]);
                bytes[4..8].copy_from_slice(&machine.to_le_bytes());
                bytes
            }
        }
    }

    /// A library of each device under plugins/, and the binaries naming them.
    fn every_library(project: &Path) -> String {
        let mut names = Vec::new();
        for &(target, _, ending) in BUILD_TARGETS {
            let file = library_path("x", target, ending);
            fs::write(project.join(&file), library_for(target)).unwrap();
            names.push(format!(r#""{target}": "{file}""#));
        }
        format!("{{ {} }}", names.join(", "))
    }

    #[test]
    fn build_makes_every_devices_library() {
        // Only where clang and lld are installed: the build is theirs.
        let Some(clang) = find_clang(None) else { return };
        if ["dll", "so", "dylib"].iter().any(|ending| find_lld(&clang, ending).is_none()) {
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Built").unwrap();
        add_plugin(&project).unwrap();
        let built = build_plugin(&project, None).unwrap();
        assert_eq!(built.len(), BUILD_TARGETS.len());
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
        // A call the C library would answer is refused when built, not when played.
        fs::write(project.join("source/plugin.c"), "#include <stddef.h>\nvoid* malloc(size_t);\n__attribute__((visibility(\"default\"))) void* f(void) { return malloc(4); }\n").unwrap();
        assert!(build_plugin(&project, None).is_err());
    }

    #[test]
    fn check_reads_a_layers_shaders() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Shader").unwrap();
        write(&project, "layers/base/shaders/water.hlsl", "// a shader");
        // A model of the mod's with that material.
        write(&project, "layers/base/files/ObjectData/Pool.arc/Pool.bdl.gltf", r#"{ "asset": { "version": "2.0" }, "materials": [{ "name": "Water" }] }"#);
        let with = |requires: &str, materials: &str| {
            let mut manifest: Value = serde_json::from_slice(&fs::read(project.join("luma.json")).unwrap()).unwrap();
            manifest["requires"] = serde_json::from_str(requires).unwrap();
            manifest["layers"][0]["materials"] = serde_json::from_str(materials).unwrap();
            fs::write(project.join("luma.json"), manifest.to_string()).unwrap();
            check(&project, None).unwrap()
        };
        let good = with(r#"["custom-shaders"]"#, r#"{ "Water": "layers/base/shaders/water.hlsl" }"#);
        assert!(problems(&good).is_empty(), "{good:?}");
        assert!(problems(&with("[]", r#"{ "Water": "layers/base/shaders/water.hlsl" }"#)).iter().any(|t| t.contains("custom-shaders")));
        assert!(problems(&with(r#"["custom-shaders"]"#, r#"{ "Water": "gone.hlsl" }"#)).iter().any(|t| t.contains("isn't in the file")));
        assert!(problems(&with(r#"["custom-shaders"]"#, r#"{ "Water": "../gone.hlsl" }"#)).iter().any(|t| t.contains("path in the file")));
    }

    #[test]
    fn check_says_what_a_bad_file_is() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        fs::create_dir_all(&project).unwrap();
        assert!(check(&project, None).unwrap()[0].text.contains("no luma.json"));
        fs::write(project.join("luma.json"), "{ nope").unwrap();
        assert!(check(&project, None).unwrap()[0].text.contains("mistake"));
        let junk = dir.path().join("junk.luma");
        fs::write(&junk, b"not a zip").unwrap();
        assert!(check(&junk, None).unwrap()[0].serious);
    }

    #[test]
    fn check_finds_a_file_in_the_wrong_order() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("m.luma");
        let mut zip = zip::ZipWriter::new(fs::File::create(&path).unwrap());
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("layers/base/files/StageData/a.arc", options).unwrap();
        std::io::Write::write_all(&mut zip, b"x").unwrap();
        zip.start_file("luma.json", options).unwrap();
        std::io::Write::write_all(&mut zip, GOOD.as_bytes()).unwrap();
        zip.finish().unwrap();
        let found = check(&path, None).unwrap();
        assert!(found.iter().any(|f| f.text.contains("first entry")), "{found:?}");
    }
}
