//! Writes a `.luma` (docs/format.md): the mod's files in their layers, and its `luma.json`. The
//! files are checked by converting each with the converter Nebula itself uses, but what goes in the file is the mod's own,
//! so a `.luma` keeps working on every Nebula to come.

use crate::game_store::GameStore;
use crate::known_patches;
use crate::layout::Layout;
use crate::plan::{Layer, Level, Plan, PlannedFile, PlayerOption, When, ANY_VERSION};
use crate::PACKAGE_FORMAT;
use anyhow::{bail, Context, Result};
use nebula_import::cook::overlay::{self, Change};
use nebula_import::cook::programs;
use nebula_import::cook::Stats;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize)]
struct ManifestWhen {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    version: Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    option: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ManifestLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    tunables: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    object_changes: Vec<known_patches::ObjectChangeRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    comet_time_limits: Vec<known_patches::CometLimitRow>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    player_constants: crate::player_constants::Constants,
    #[serde(skip_serializing_if = "Option::is_none")]
    when: Option<ManifestWhen>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ManifestSave {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ManifestGalaxy {
    name: String,
}

#[derive(Clone, Debug, Serialize)]
struct ManifestComets {
    dome: u32,
    comets: Vec<known_patches::CometRow>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct Manifest {
    format: u32,
    id: String,
    name: String,
    version: String,
    game: String,
    /// "tunables" when a layer sets any: a Nebula without them refuses the mod rather than playing it without them.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    requires: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    authors: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<PlayerOption>,
    layers: Vec<ManifestLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    save: Option<ManifestSave>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    galaxies: Vec<ManifestGalaxy>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    event_flags: Vec<known_patches::EventFlagRow>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    comet_schedule: Vec<ManifestComets>,
    tool: String,
}

/// What came of a conversion.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Outcome {
    /// The `.luma` file.
    pub file: PathBuf,
    pub converted: usize,
    pub total: usize,
    /// Pixel programs its models' materials use; Nebula builds those it lacks on first play.
    pub programs: usize,
    /// Files the game couldn't use, with the reason. They are left out.
    pub failed: Vec<(String, String)>,
    pub report: String,
}

fn copy_tree(from: &Path, to: &Path, keep: impl Fn(&Path) -> bool) -> Result<usize> {
    let mut count = 0;
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry.with_context(|| format!("{} cannot be read through", from.display()))?;
        if !entry.file_type().is_file() || !keep(entry.path()) {
            continue;
        }
        let relative = entry.path().strip_prefix(from)?;
        let target = to.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(entry.path(), &target).with_context(|| format!("{} cannot be copied", entry.path().display()))?;
        count += 1;
    }
    Ok(count)
}

/// A finished folder as one file: written beside its place, then moved there, so a failure leaves any old file whole.
pub(crate) fn pack_folder(folder: &Path, file: &Path) -> Result<()> {
    let temporary = file.with_extension("luma.writing");
    let result = pack_into(folder, &temporary).and_then(|()| fs::rename(&temporary, file).with_context(|| format!("{} cannot be replaced", file.display())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn pack_into(folder: &Path, temporary: &Path) -> Result<()> {
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).large_file(true);
    let mut zip = zip::ZipWriter::new(std::io::BufWriter::new(fs::File::create(temporary).with_context(|| format!("{} cannot be created", temporary.display()))?));
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in walkdir::WalkDir::new(folder) {
        let entry = entry.with_context(|| format!("{} cannot be read through", folder.display()))?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    // The manifest first: a reader finds what the file is without going through it.
    files.sort_by_key(|p| (!p.ends_with("luma.json"), p.clone()));
    for path in files {
        let name = path.strip_prefix(folder)?.to_string_lossy().replace('\\', "/");
        zip.start_file(name, options)?;
        std::io::copy(&mut fs::File::open(&path)?, &mut zip)?;
    }
    zip.finish()?.into_inner().map_err(|e| e.into_error())?.sync_all()?;
    Ok(())
}

fn pack_file_wanted(path: &Path, pack_root: &Path) -> bool {
    let name = path.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    if name == "manifest.json" || crate::layout::is_note(path) && !name.ends_with(".png") {
        return false;
    }
    let relative = path.strip_prefix(pack_root).unwrap_or(path);
    // A folder named for another game holds that game's textures.
    !relative.components().any(|part| {
        let part = part.as_os_str().to_string_lossy();
        part.len() == 6 && part.chars().all(|c| c.is_ascii_alphanumeric() && !c.is_ascii_lowercase()) && !part.starts_with(ANY_VERSION)
    })
}

/// What checking the layers found: the pixel programs the materials use, (file, problem) pairs, and files converted.
type LayerCheck = (usize, Vec<(String, String)>, usize);

/// Converts every layer as Nebula will, to find files it can't use and to count the programs its models' materials use.
/// Nebula builds any of those it lacks on the player's device from the texts its own conversion writes (mods.sources): a
/// compiled program fits one build of Nebula only, so a `.luma` carries none.
pub(crate) fn check_layers(layers: &[(String, Vec<Change>)], game_id: &str, store: Option<&GameStore>, building: &Path) -> Result<LayerCheck> {
    // One at a time: the collector below is the converter's, for the whole program.
    static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _turn = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut stats = Stats::default();
    let mut failed = Vec::new();
    let mut converted = 0;
    let scratch = building.join(".checking");
    fs::create_dir_all(&scratch)?;
    *programs::MOD_TEXTS.lock().unwrap_or_else(|e| e.into_inner()) = Some(BTreeMap::new());
    for (name, changes) in layers {
        if changes.is_empty() {
            continue;
        }
        match overlay::write_overlay(game_id, changes, store.map(|s| s.path.as_path()), &scratch.join(format!("{name}.pak")), &mut stats) {
            Ok(bad) => {
                converted += changes.len() - bad.len();
                failed.extend(bad);
            }
            Err(_) => {
                failed.extend(changes.iter().map(|c| (c.path.clone(), "the game can't use this file".to_string())));
            }
        }
    }
    let _ = fs::remove_dir_all(&scratch);
    let texts = programs::MOD_TEXTS.lock().unwrap_or_else(|e| e.into_inner()).take().unwrap_or_default();
    let pixel_programs = texts.keys().filter(|name| name.starts_with('p')).count();
    Ok((pixel_programs, failed, converted))
}

fn changes(files: &BTreeMap<String, PlannedFile>) -> Vec<Change> {
    files.values().map(|f| Change { path: f.disc.trim_start_matches('/').to_string(), source: f.host.clone(), merge: false }).collect()
}

fn tunables_of(switches: &BTreeMap<u32, u32>) -> BTreeMap<String, serde_json::Value> {
    switches.iter().filter_map(|(id, number)| known_patches::tunable(*id, *number).map(|(key, value)| (key.to_string(), value))).collect()
}

fn when_of(when: &When) -> Option<ManifestWhen> {
    if when.version.is_empty() && when.option.is_empty() {
        return None;
    }
    Some(ManifestWhen { version: when.version.iter().map(char::to_string).collect(), option: when.option.clone() })
}

fn copy_layer(files: &BTreeMap<String, PlannedFile>, failed: &[(String, String)], building: &Path, path: &str) -> Result<()> {
    for file in files.values() {
        let relative = file.disc.trim_start_matches('/');
        if failed.iter().any(|(bad, _)| bad == relative) {
            continue;
        }
        let Some(safe) = crate::source::safe_relative(relative) else {
            bail!("{relative}: this isn't a place in the game");
        };
        let target = building.join(path).join("files").join(safe);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&file.host, &target).with_context(|| format!("{} cannot be copied", file.host.display()))?;
    }
    Ok(())
}

fn report_text(name: &str, plan: &Plan, outcome: &Outcome, textures: &crate::texture_pack::Conversion) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "{name}");
    let _ = writeln!(text, "Files: {} of {}", outcome.converted, outcome.total);
    let _ = writeln!(text, "Textures: {}", textures.written);
    if textures.dolphin != 0 {
        let _ = writeln!(text, "  Dolphin files given the game's names: {} ({} textures)", textures.dolphin, textures.textures);
    }
    if textures.taken != 0 {
        let _ = writeln!(text, "  Names an earlier file of the pack gave: {}", textures.taken);
    }
    if !textures.kept.is_empty() {
        if textures.named {
            let _ = writeln!(text, "  Dolphin files matching none of this game's textures (another release or edit), kept: {}", textures.kept.len());
        } else {
            let _ = writeln!(text, "  Dolphin files kept under their own names (no imported game with texture names): {}", textures.kept.len());
        }
        let _ = writeln!(text, "  Nebula converts kept files on each player's game when the mod is installed.");
        for file in &textures.kept {
            let _ = writeln!(text, "    {file}");
        }
    }
    let _ = writeln!(text, "Material programs: {} (Nebula builds any it lacks when the mod is first played)", outcome.programs);
    if !plan.known_patches.is_empty() {
        let _ = writeln!(text, "\nGameplay changes that work in Nebula:");
        for what in &plan.known_patches {
            let _ = writeln!(text, "  {what}");
        }
    }
    if !plan.unknown_code.is_empty() {
        let _ = writeln!(text, "\nCode changes Nebula has no version of yet:");
        for code in &plan.unknown_code {
            let _ = writeln!(text, "  0x{:08X} {} ({} bytes)", code.address, code.kind, code.bytes);
        }
    }
    if !plan.options.is_empty() {
        let _ = writeln!(text, "\nChoices for the player, in the game:");
        for option in &plan.options {
            let names: Vec<&str> = option.choices.iter().map(|c| c.name.as_str()).collect();
            let _ = writeln!(text, "  {}: {}", option.name, names.join(", "));
        }
    }
    if !outcome.failed.is_empty() {
        let _ = writeln!(text, "\nLeft out, because the game can't use them:");
        for (path, why) in &outcome.failed {
            let _ = writeln!(text, "  {path}: {why}");
        }
    }
    for (level, title) in [(Level::Lost, "Didn't make it"), (Level::Info, "Good to know")] {
        let notes: Vec<_> = plan.notes.iter().filter(|n| n.level == level).collect();
        if !notes.is_empty() {
            let _ = writeln!(text, "\n{title}:");
            for note in notes {
                let _ = writeln!(text, "  {}", note.text);
            }
        }
    }
    text
}

/// A mod's name as a file's name, whatever it is: a name no file can have on some systems still makes a file.
pub(crate) fn file_stem_for(name: &str) -> String {
    let safe: String = name.chars().map(|c| if r#"<>:"/\|?*"#.contains(c) || c.is_control() { '_' } else { c }).collect();
    let safe = safe.trim_matches(['.', ' ']).to_string();
    if safe.is_empty() {
        "mod".to_string()
    } else if crate::source::safe_relative(&safe).is_none() {
        format!("mod {safe}")
    } else {
        safe
    }
}

/// Writes the `.luma` for a plan. An existing file of the name is replaced only when the new one is complete.
pub fn write(name: &str, layout: &Layout, plan: &Plan, store: Option<&GameStore>, output: &Path) -> Result<Outcome> {
    if plan.blocked() {
        bail!("{}", plan.notes.iter().find(|n| n.level == Level::Blocked).map_or("the mod cannot be used", |n| n.text.as_str()));
    }
    fs::create_dir_all(output).with_context(|| format!("{} cannot be created", output.display()))?;
    let (name, version) = crate::plan::split_version(&crate::plan::nice_name(name));
    let name = name.as_str();
    let safe = file_stem_for(name);
    let file = output.join(format!("{safe}.luma"));
    let building = output.join(format!(".{safe}.building"));
    if building.exists() {
        fs::remove_dir_all(&building)?;
    }
    fs::create_dir_all(&building)?;
    let result = write_inside(name, version.as_deref(), layout, plan, store, &building, &file);
    let _ = fs::remove_dir_all(&building);
    result
}

fn write_inside(name: &str, version: Option<&str>, layout: &Layout, plan: &Plan, store: Option<&GameStore>, building: &Path, file: &Path) -> Result<Outcome> {
    let mut jobs: Vec<(String, Vec<Change>)> = vec![("base".to_string(), changes(&plan.files))];
    jobs.extend(plan.layers.iter().map(|l: &Layer| (l.name.clone(), changes(&l.files))));
    let (programs, failed, converted) = check_layers(&jobs, &plan.game_id, store, building)?;

    let mut layers = Vec::new();
    copy_layer(&plan.files, &failed, building, "layers/base")?;
    let mut textures = crate::texture_pack::Conversion::default();
    if let Some(pack) = &layout.pack {
        let root = pack.folder.clone();
        let names = store.map(|s| crate::texture_pack::store_names(&s.path)).unwrap_or_default();
        textures = crate::texture_pack::convert_into(&pack.folder, &names, &building.join("layers/base/textures"), move |p| pack_file_wanted(p, &root))?;
        if let Some(manifest) = walkdir::WalkDir::new(&pack.folder)
            .max_depth(3)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case("manifest.json"))
        {
            let _ = fs::copy(manifest.path(), building.join("layers/base/textures").join("manifest.json"));
        }
    }
    let has_base = building.join("layers/base").is_dir();
    if has_base || !plan.native_patches.is_empty() || !plan.object_changes.is_empty() || !plan.comet_limits.is_empty() || !plan.player_constants.is_empty() {
        layers.push(ManifestLayer {
            path: has_base.then(|| "layers/base".to_string()),
            tunables: tunables_of(&plan.native_patches),
            object_changes: plan.object_changes.clone(),
            comet_time_limits: plan.comet_limits.clone(),
            player_constants: plan.player_constants.clone(),
            when: None,
        });
    }
    for layer in &plan.layers {
        let path = format!("layers/{}", layer.name);
        copy_layer(&layer.files, &failed, building, &path)?;
        layers.push(ManifestLayer {
            path: building.join(&path).is_dir().then_some(path),
            tunables: tunables_of(&layer.switches),
            object_changes: layer.object_changes.clone(),
            comet_time_limits: layer.comet_limits.clone(),
            player_constants: layer.player_constants.clone(),
            when: when_of(&layer.when),
        });
    }
    let has_save = match &plan.save_folder {
        Some(save) if save.is_dir() => copy_tree(save, &building.join("save"), |_| true)? != 0,
        _ => false,
    };

    let pack = layout.pack.as_ref();
    let shown_name = pack.map(|p| p.title.clone()).filter(|t| !t.is_empty()).unwrap_or_else(|| name.to_string());
    let authors = pack.map(|p| p.authors.clone()).unwrap_or_default();
    let manifest = Manifest {
        format: PACKAGE_FORMAT,
        id: crate::plan::slug(&shown_name),
        name: shown_name,
        version: pack
            .map(|p| p.version.clone())
            .filter(|v| crate::project::valid_version(v))
            .or_else(|| version.map(str::to_string))
            .unwrap_or_else(|| "1.0.0".to_string()),
        game: ANY_VERSION.to_string(),
        requires: if layers.iter().any(|l| !l.tunables.is_empty()) { vec!["tunables".to_string()] } else { Vec::new() },
        authors: if authors.is_empty() { Vec::new() } else { vec![authors] },
        description: pack.map(|p| p.description.clone()).unwrap_or_default(),
        options: plan.options.clone(),
        layers,
        save: (has_save || plan.save_folder.is_some() || plan.adds_galaxy).then(|| ManifestSave {
            mode: if plan.save_clone { "copy" } else { "own" }.to_string(),
            path: has_save.then(|| "save".to_string()),
        }),
        galaxies: plan.new_galaxies.iter().map(|name| ManifestGalaxy { name: name.clone() }).collect(),
        event_flags: plan.event_flags.clone(),
        comet_schedule: plan.comet_schedule.iter().map(|(dome, comets)| ManifestComets { dome: *dome, comets: comets.clone() }).collect(),
        tool: format!("LumaLink {}", env!("CARGO_PKG_VERSION")),
    };
    fs::write(building.join("luma.json"), serde_json::to_vec_pretty(&manifest)?)?;

    let mut outcome = Outcome {
        file: file.to_path_buf(),
        converted,
        total: jobs.iter().map(|(_, c)| c.len()).sum(),
        programs,
        failed,
        report: String::new(),
    };
    outcome.report = report_text(name, plan, &outcome, &textures);
    fs::write(building.join("report.txt"), &outcome.report)?;
    pack_folder(building, file)?;
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::build;

    fn write_file(root: &Path, relative: &str, text: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn a_mod_becomes_a_file_with_its_own_files_inside() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("mod");
        write_file(&source, "StageData/note.txt.bin", "hello");
        write_file(&source, "Load/Textures/RMGE01/tex1_8x8_0123456789abcdef_14.png", "p");
        write_file(&source, "Load/Textures/SB4E01/tex1_8x8_0123456789abcdef_14.png", "other");
        let layout = Layout::of(&source);
        let plan = build(&layout, "RMGE01", None).unwrap();
        let outcome = write("My: Mod", &layout, &plan, None, &dir.path().join("out")).unwrap();
        assert_eq!(outcome.converted, 1);
        assert!(outcome.file.ends_with("My_ Mod.luma"));
        let unpacked = dir.path().join("unpacked");
        let mut zip = zip::ZipArchive::new(fs::File::open(&outcome.file).unwrap()).unwrap();
        assert_eq!(zip.by_index(0).unwrap().name(), "luma.json");
        zip.extract(&unpacked).unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&fs::read(unpacked.join("luma.json")).unwrap()).unwrap();
        assert_eq!(manifest["format"], PACKAGE_FORMAT);
        assert_eq!(manifest["game"], "RMG");
        assert_eq!(manifest["id"], "my-mod");
        assert_eq!(manifest["layers"][0]["path"], "layers/base");
        assert!(manifest.get("options").is_none());
        assert_eq!(fs::read_to_string(unpacked.join("layers/base/files/StageData/note.txt.bin")).unwrap(), "hello");
        assert!(unpacked.join("report.txt").is_file());
        assert!(unpacked.join("layers/base/textures/Load/Textures/RMGE01").is_dir());
        assert!(!unpacked.join("layers/base/textures/Load/Textures/SB4E01").exists());
        assert!(!dir.path().join("out/.My_ Mod.building").exists());
    }

    #[test]
    fn any_name_makes_a_file() {
        for name in ["CON", "...", "aux.old", "   "] {
            let dir = tempfile::tempdir().unwrap();
            let source = dir.path().join("mod");
            write_file(&source, "StageData/note.txt.bin", "hello");
            let layout = Layout::of(&source);
            let plan = build(&layout, "RMGE01", None).unwrap();
            let outcome = write(name, &layout, &plan, None, &dir.path().join("out")).unwrap();
            assert!(outcome.file.is_file(), "{name:?}");
            assert!(crate::source::safe_relative(&outcome.file.file_name().unwrap().to_string_lossy()).is_some(), "{name:?}");
        }
    }

    #[test]
    fn a_blocked_mod_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("mod");
        write_file(&source, "readme.txt", "x");
        let layout = Layout::of(&source);
        let plan = build(&layout, "RMGE01", None).unwrap();
        assert!(write("m", &layout, &plan, None, &dir.path().join("out")).is_err());
        assert!(!dir.path().join("out/m.luma").exists());
    }
}
