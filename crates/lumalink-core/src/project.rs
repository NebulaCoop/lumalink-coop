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
use std::path::{Component, Path, PathBuf};

const VERSION_LETTERS: [&str; 6] = ["E", "P", "J", "K", "W", "C"];
const PLATFORMS: [&str; 5] = ["windows", "android", "linux", "macos", "ios"];
const WHEN_KEYS: [&str; 4] = ["version", "language", "platform", "option"];
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
rem Builds plugins\{id}.dll with clang, from https://llvm.org. It needs nothing else.
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -o plugins\{id}.dll source\plugin.c
"#;
const PLUGIN_BUILD_SH: &str = r#"#!/bin/sh
# Builds this device's library with clang. Run it on each kind of computer the mod is for.
set -e
case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/{id}.so source/plugin.c ;;
    Linux-aarch64) cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/{id}-arm64.so source/plugin.c ;;
    Darwin-x86_64) cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/{id}-x64.dylib source/plugin.c ;;
    Darwin-arm64) cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/{id}.dylib source/plugin.c ;;
    *) echo "Run build.cmd on Windows." ;;
esac
"#;

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
    manifest["plugins"] = json!([{
        "api": PLUGIN_API,
        "binaries": { "windows-x64": format!("plugins/{id}.dll") },
    }]);
    fs::write(folder.join("luma.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(())
}

/// Writes each material's generated shader beside the output, as `<material>.hlsl`, from a model or an archive holding
/// one. What's returned: the files.
pub fn write_generated_shaders(model: &Path, output: &Path) -> Result<Vec<String>> {
    use nebula_import::cook::{archive, material};
    let mut bytes = fs::read(model).with_context(|| format!("{} can't be read", model.display()))?;
    if archive::is_compressed(&bytes) {
        bytes = archive::expand(&bytes)?;
    }
    let is_model = |b: &[u8]| b.starts_with(b"J3D2bmd3") || b.starts_with(b"J3D2bdl4");
    if archive::is_archive(&bytes) {
        let members = archive::members(&bytes)?;
        bytes = members.into_iter().map(|m| m.bytes).find(|b| is_model(b)).context("the archive holds no model")?;
    }
    if !is_model(&bytes) {
        bail!("{} isn't a model", model.display());
    }
    fs::create_dir_all(output)?;
    let mut written = Vec::new();
    for (name, source) in material::generated_shaders(&bytes)? {
        let safe: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect();
        let file = format!("{safe}.hlsl");
        fs::write(output.join(&file), source)?;
        written.push(file);
    }
    Ok(written)
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
        "id": crate::plan::slug(name),
        "name": name,
        "version": "1.0.0",
        "game": crate::plan::ANY_VERSION,
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
            Some(Change { path: relative, source: e.path().to_path_buf(), merge: false })
        })
        .collect()
}

/// What of a project goes in its file: not version control or a system's hidden files, not a file packed from it before.
fn belongs_in_file(relative: &Path) -> bool {
    let hidden = relative.components().any(|c| c.as_os_str().to_string_lossy().starts_with('.'));
    let packed = relative.components().count() == 1 && relative.extension().is_some_and(|e| e.eq_ignore_ascii_case("luma") || e.eq_ignore_ascii_case("writing"));
    !hidden && !packed
}

/// Packs a project folder into a `.luma`. A problem `check` calls serious stops it.
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
        let (shaders, failed, converted) = package::check_and_build_shaders(&jobs, &game_id, store, &building)?;
        if let Some((path, why)) = failed.first() {
            bail!("{path}: {why}");
        }
        for entry in walkdir::WalkDir::new(folder) {
            let entry = entry.with_context(|| format!("{} can't be read through", folder.display()))?;
            let relative = entry.path().strip_prefix(folder)?;
            let is_built = relative.components().next().is_some_and(|c| c.as_os_str() == "shaders");
            if !entry.file_type().is_file() || (is_built && building.join("shaders").is_dir()) || !belongs_in_file(relative) {
                continue;
            }
            let target = building.join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
        let file = output.join(format!("{safe}.luma"));
        package::pack_folder(&building, &file)?;
        Ok(Outcome {
            file,
            converted,
            total: jobs.iter().map(|(_, c)| c.len()).sum(),
            materials: shaders.materials,
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

/// A path inside the file: going up or starting from a drive is refused.
fn inside(path: &str) -> bool {
    !path.is_empty() && Path::new(path).components().all(|c| matches!(c, Component::Normal(_) | Component::CurDir)) && !path.contains(':')
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
            if !crate::layout::LANGUAGE_FOLDERS.iter().any(|l| *l == language) {
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

/// What a file in a layer's `files/` can be.
const GAME_FILE_ENDINGS: [&str; 31] = ["json", "gltf", "glb", "bin", "png", "wav", "ogg", 
    "arc", "szs", "bcsv", "tbl", "pa", "bcam", "banmt", "bmd", "bdl", "brk", "btk", "bck", "bpk", "btp", "bva", "blk", "brstm", "ast", "aw", "thp", "png", "bin", "bti", "brfnt",
];

fn check_layers(folder: &Path, manifest: &Value, options: &BTreeMap<String, Vec<String>>, found: &mut Findings) {
    let Some(layers) = manifest.get("layers").and_then(Value::as_array) else {
        found.problem("it has no layers");
        return;
    };
    let names: Vec<&str> = known_patches::switch_names().collect();
    // A mod can be only code, or only a galaxy's row over files another mod brings.
    let mut has_anything = manifest.get("plugins").and_then(Value::as_array).is_some_and(|p| !p.is_empty())
        || manifest.get("text").and_then(Value::as_object).is_some_and(|t| !t.is_empty())
        || ["music", "event_flags", "comet_schedule"].iter().any(|k| manifest.get(*k).and_then(Value::as_array).is_some_and(|l| !l.is_empty()));
    let says_custom_shaders = manifest.get("requires").and_then(Value::as_array).is_some_and(|r| r.iter().any(|f| f == "custom-shaders"));
    for (i, layer) in layers.iter().enumerate() {
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
            has_anything |= check_merge(&folder.join("merge"), found);
            for entry in walkdir::WalkDir::new(folder.join("files")).into_iter().filter_map(std::result::Result::ok) {
                if !entry.file_type().is_file() {
                    continue;
                }
                has_anything = true;
                let ending = entry.path().extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
                if !GAME_FILE_ENDINGS.contains(&ending.as_str()) {
                    found.note(format!("{}: the game may not read this kind of file", entry.path().strip_prefix(&folder).unwrap_or(entry.path()).display()));
                }
            }
            has_anything |= walkdir::WalkDir::new(folder.join("textures")).into_iter().filter_map(std::result::Result::ok).any(|e| e.file_type().is_file());
            has_anything |= walkdir::WalkDir::new(folder.join("movies")).into_iter().filter_map(std::result::Result::ok).any(|e| e.file_type().is_file());
        }
        if let Some(switches) = layer.get("switches") {
            let Some(switches) = switches.as_object() else {
                found.problem(format!("{name}: switches names switches and their values"));
                continue;
            };
            for (switch, value) in switches {
                has_anything = true;
                if !names.contains(&switch.as_str()) {
                    found.problem(format!("{name}: there's no switch called {switch}"));
                }
                if !value.is_u64() {
                    found.problem(format!("{name}: {switch} needs a whole number"));
                }
            }
        }
        if let Some(when) = layer.get("when") {
            check_when(when, &name, options, found);
        }
    }
    if !has_anything {
        found.problem("it holds nothing the game can use");
    }
}

/// What a layer adds to files many mods share. Whether it found anything.
fn check_merge(folder: &Path, found: &mut Findings) -> bool {
    let mut any = false;
    for entry in walkdir::WalkDir::new(folder).into_iter().filter_map(std::result::Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        any = true;
        let shown = entry.path().strip_prefix(folder).map_or_else(|_| entry.path().display().to_string(), |p| p.display().to_string());
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        let Ok(bytes) = fs::read(entry.path()) else {
            found.problem(format!("merge/{shown} can't be read"));
            continue;
        };
        if name.ends_with(".json") {
            let rows = serde_json::from_slice::<Value>(&bytes).ok().and_then(|v| v.get("rows").and_then(Value::as_array).cloned());
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
            found.problem(format!("merge/{shown}: only tables (.bcsv.json) and particles (.jpc) can be added to"));
        }
    }
    any
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
                        if flag.get(key).is_some_and(|v| !v.as_u64().is_some_and(|n| n <= 255)) {
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
                    if !dome.get("dome").and_then(Value::as_u64).is_some_and(|d| (1..=6).contains(&d)) {
                        found.problem("comet_schedule: dome is from 1 to 6");
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
            None | Some(Some(0..=6)) => {}
            _ => found.problem(format!("galaxy {name}: dome is from 0 to 6")),
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
const PLUGIN_API: u64 = 1;

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
            Some(api) if (1..=PLUGIN_API).contains(&api) => {}
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
        let missing: Vec<&str> = PLUGIN_TARGETS.iter().map(|(t, _, _)| *t).filter(|t| !binaries.contains_key(*t)).collect();
        if !missing.is_empty() {
            found.note(format!("{name} can't run on: {}. Players there are told the mod has no version for their device.", missing.join(", ")));
        }
    }
    if !plugins.is_empty() {
        found.note("This mod runs its own code, with everything the game can do on the player's device. Players are told before it first runs.");
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

fn check_folder(folder: &Path, found: &mut Findings) {
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
    if let Some(nebula) = manifest.get("nebula") {
        if !nebula.as_str().is_some_and(valid_version) {
            found.problem("nebula has to be numbers with dots, like 2.1.0");
        }
    }
    if let Some(requires) = manifest.get("requires") {
        for feature in string_list(requires, "requires", found) {
            if feature != "custom-shaders" {
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
            Some(path) if inside(path) => {
                if !folder.join(path).is_file() {
                    found.problem(format!("the icon {path} isn't in the file"));
                } else if !Path::new(path).extension().is_some_and(|e| e.eq_ignore_ascii_case("png")) {
                    found.problem("the icon has to be a PNG");
                }
            }
            _ => found.problem("icon is a path in the file"),
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
    check_layers(folder, &manifest, &options, found);
    for name in ["dxil.shaders", "spirv.shaders"] {
        let path = folder.join("shaders").join(name);
        if path.exists() {
            check_shader_file(&path, found);
        }
    }
}

/// Everything wrong with a project folder or `.luma`, before anyone else finds it.
pub fn check(input: &Path, _store: Option<&GameStore>) -> Result<Vec<Finding>> {
    let mut found = Findings::default();
    if input.is_dir() {
        check_folder(input, &mut found);
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
        Ok(()) => check_folder(work.path(), &mut found),
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
    fn only_what_belongs_goes_in_the_file() {
        for good in ["luma.json", "layers/base/files/a.arc", "source/plugin.c", "plugins/a.dll", "layers/base/files/old.luma"] {
            assert!(belongs_in_file(Path::new(good)), "{good}");
        }
        for bad in [".git/config", ".DS_Store", "layers/.hidden/a", "My Mod.luma", "My Mod.luma.writing"] {
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

    fn checked(manifest: &str) -> Vec<String> {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "luma.json", manifest);
        write(dir.path(), "layers/base/files/StageData/a.arc", "x");
        check(dir.path(), None).unwrap().into_iter().filter(|f| f.serious).map(|f| f.text).collect()
    }

    const GOOD: &str = r#"{"format":1,"id":"m","name":"M","version":"1.0","game":"RMG","authors":["A"],
        "options":[{"id":"o","name":{"default":"O","ja":"オ"},"default":"a","choices":[{"id":"a","name":"A"},{"id":"b","name":"B"}]}],
        "layers":[{"path":"layers/base","switches":{"dodoryu_shell_only":1},"when":{"version":["E","P"],"platform":["android"],"option":{"o":["b"]},"language":["UsEnglish"]}}]}"#;

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
            (r#""dodoryu_shell_only""#, r#""nope""#, "no switch called"),
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
        ] {
            assert!(GOOD.contains(from), "{from}");
            let bad = checked(&GOOD.replace(from, to));
            assert!(bad.iter().any(|t| t.contains(says)), "{from} -> {to}: {bad:?}");
        }
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

        let good = with(r#"[{ "api": 1, "binaries": { "windows-x64": "plugins/a.dll" } }]"#);
        assert!(problems(&good).is_empty(), "{good:?}");
        assert!(good.iter().any(|f| f.text.contains("its own code")));
        assert!(good.iter().any(|f| f.text.contains("linux-x64")));

        for (plugins, says) in [
            (r#"[{ "binaries": { "windows-x64": "plugins/a.dll" } }]"#, "api is"),
            (r#"[{ "api": 2, "binaries": { "windows-x64": "plugins/a.dll" } }]"#, "newer"),
            (r#"[{ "api": 1 }]"#, "binaries names"),
            (r#"[{ "api": 1, "binaries": { "amiga": "plugins/a.dll" } }]"#, "isn't a device"),
            (r#"[{ "api": 1, "binaries": { "windows-x64": "../a.dll" } }]"#, "path in the file"),
            (r#"[{ "api": 1, "binaries": { "windows-x64": "plugins/gone.dll" } }]"#, "isn't in the file"),
            (r#"[{ "api": 1, "binaries": { "windows-x64": "plugins/not.dll" } }]"#, "isn't a library"),
            (r#"[{ "api": 1, "binaries": { "windows-x64": "plugins/arm.dll" } }]"#, "isn't a library"),
            (r#"[{ "api": 1, "binaries": { "linux-x64": "plugins/a.dll" } }]"#, "isn't a library"),
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
        for (galaxies, save, says) in [
            (r#"[{ "dome": 1 }]"#, own, "name is"),
            (r#"[{ "name": "Other" }]"#, own, "no layer has"),
            (r#"[{ "name": "MyGalaxy" }]"#, r#"{ "mode": "shared" }"#, "own saves"),
            (r#"[{ "name": "MyGalaxy", "dome": 9 }]"#, own, "dome"),
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
    fn a_project_can_start_with_code() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Has Code").unwrap();
        add_plugin(&project).unwrap();
        assert!(project.join("source/nebula_mod_api.h").is_file());
        assert!(fs::read_to_string(project.join("build.cmd")).unwrap().contains("plugins\\has-code.dll"));
        let found = check(&project, None).unwrap();
        assert!(problems(&found).iter().any(|t| t.contains("plugins/has-code.dll isn't in the file")), "{found:?}");
        fs::write(project.join("plugins/has-code.dll"), pe(0x8664)).unwrap();
        let found = check(&project, None).unwrap();
        assert!(problems(&found).is_empty(), "{found:?}");
    }

    #[test]
    fn check_reads_a_layers_shaders() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("proj");
        new_project(&project, "Shader").unwrap();
        write(&project, "layers/base/shaders/water.hlsl", "// a shader");
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
