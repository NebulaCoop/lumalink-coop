//! Decides where every file of a mod goes, and says why for each thing it does not use.

use crate::game_store::GameStore;
use crate::known_patches;
use crate::layout::{self, Layout, PatchFile};
use crate::riivolution::{self, Disc, FilePatch, FolderPatch, MemoryPatch};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// One file the mod sets.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PlannedFile {
    pub disc: String,
    pub host: PathBuf,
}

/// How serious a note is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Level {
    /// Worth knowing; nothing is lost.
    Info,
    /// Part of the mod does not reach the game.
    Lost,
    /// The mod cannot be used.
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Note {
    pub level: Level,
    pub text: String,
}

/// A code patch with no native version.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnknownCode {
    pub address: u32,
    pub bytes: usize,
    pub kind: &'static str,
}

/// When a layer is used. Empty: always.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct When {
    pub version: Vec<char>,
    pub option: BTreeMap<String, Vec<String>>,
}

/// A part of a mod used only sometimes.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Layer {
    pub name: String,
    pub when: When,
    pub files: BTreeMap<String, PlannedFile>,
    pub switches: BTreeMap<u32, u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct OptionChoice {
    pub id: String,
    pub name: String,
}

/// Something the player chooses in the game.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PlayerOption {
    pub id: String,
    pub name: String,
    pub default: String,
    pub choices: Vec<OptionChoice>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Plan {
    pub game_id: String,
    /// What is always used, by the lower-case path.
    pub files: BTreeMap<String, PlannedFile>,
    /// The switches always on.
    pub native_patches: BTreeMap<u32, u32>,
    /// Rows of the game's progression a mod's patches set.
    pub event_flags: Vec<known_patches::EventFlagRow>,
    /// Each dome's comets a mod's patches set.
    pub comet_schedule: BTreeMap<u32, Vec<known_patches::CometRow>>,
    /// What is used only for some versions of the game or some of the player's choices.
    pub layers: Vec<Layer>,
    pub options: Vec<PlayerOption>,
    pub notes: Vec<Note>,
    pub known_patches: Vec<String>,
    pub unknown_code: Vec<UnknownCode>,
    pub save_folder: Option<PathBuf>,
    pub save_clone: bool,
    pub for_another_game: Option<String>,
    /// It adds a galaxy: its saves have to be its own, since a save's shape follows the galaxy list.
    pub adds_galaxy: bool,
    /// The galaxies it adds, by their folder names.
    pub new_galaxies: Vec<String>,
}

impl Plan {
    fn note(&mut self, level: Level, text: impl Into<String>) {
        self.notes.push(Note { level, text: text.into() });
    }

    fn set(&mut self, disc: &str, host: &Path) {
        let disc = if disc.starts_with('/') { disc.to_string() } else { format!("/{disc}") };
        if crate::source::safe_relative(disc.trim_start_matches('/')).is_none() {
            self.note(Level::Lost, format!("{disc}: this isn't a place in the game"));
            return;
        }
        if disc.to_ascii_lowercase().starts_with("/nebuladata/") && !disc.eq_ignore_ascii_case("/NebulaData/GalaxyInfoTable.bcsv") {
            self.note(Level::Lost, format!("{disc}: Nebula's own tables are not set by mods"));
            return;
        }
        self.files.insert(disc.to_ascii_lowercase(), PlannedFile { disc, host: host.to_path_buf() });
    }

    pub fn blocked(&self) -> bool {
        self.notes.iter().any(|n| n.level == Level::Blocked)
    }

    /// How many different files it can set, across everything.
    pub fn file_count(&self) -> usize {
        let mut all: std::collections::BTreeSet<&String> = self.files.keys().collect();
        for layer in &self.layers {
            all.extend(layer.files.keys());
        }
        all.len()
    }
}

/// A relative path in a mod to a file, in any case. `..` may not leave the root.
fn resolve(sd_root: &Path, patch_root: &Path, relative: &str) -> Option<PathBuf> {
    let relative = relative.replace('\\', "/");
    let start = if relative.starts_with('/') { sd_root } else { patch_root };
    let mut result = start.to_path_buf();
    let mut depth = 0usize;
    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                depth = depth.checked_sub(1)?;
                result.pop();
            }
            part => {
                if part.contains(':') {
                    return None;
                }
                depth += 1;
                let exact = result.join(part);
                if exact.exists() {
                    result = exact;
                    continue;
                }
                let found = fs::read_dir(&result).ok().and_then(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(part))
                        .map(|e| e.path())
                });
                result = found.unwrap_or(exact);
            }
        }
    }
    Some(result)
}

fn join(a: &str, b: &str) -> String {
    let a = a.trim_end_matches('/');
    let b = b.trim_start_matches('/');
    match (a.is_empty(), b.is_empty()) {
        (true, _) => format!("/{b}"),
        (_, true) => a.to_string(),
        _ => format!("{a}/{b}"),
    }
}

fn is_language_path(disc: &str) -> Option<&'static str> {
    let first = disc.trim_start_matches('/').split('/').next()?;
    layout::LANGUAGE_FOLDERS.iter().copied().find(|l| l.eq_ignore_ascii_case(first))
}

/// Every version of the game, by the letter in its id.
const VERSION_LETTERS: [char; 6] = ['E', 'P', 'J', 'K', 'W', 'C'];

/// What the game's id in a package says: this game, whichever version.
pub const ANY_VERSION: &str = "RMG";

/// The version a patch file was written for, when it names only one.
fn version_named(target: &riivolution::Target, game_id: &str) -> String {
    if target.accepts(game_id) {
        return game_id.to_string();
    }
    let letter = target.game.as_deref().and_then(|g| g.chars().nth(3)).or_else(|| target.regions.first().and_then(|r| r.chars().next()));
    letter.map_or_else(|| game_id.to_string(), |l| format!("RMG{l}01"))
}

/// A name for people from a file's name: words apart, and capitals if it has none at all.
pub fn nice_name(raw: &str) -> String {
    let spaced = raw.replace(['_', '-'], " ");
    let mut out = String::new();
    let mut last = ' ';
    for c in spaced.chars() {
        if c.is_uppercase() && last.is_lowercase() {
            out.push(' ');
        }
        out.push(c);
        last = c;
    }
    let words: Vec<&str> = out.split_whitespace().collect();
    if out.chars().any(char::is_uppercase) {
        return words.join(" ");
    }
    words
        .iter()
        .map(|word| {
            if word.chars().any(|c| c.is_ascii_digit()) {
                return (*word).to_string();
            }
            let mut letters = word.chars();
            letters.next().map_or_else(String::new, |first| first.to_uppercase().chain(letters).collect())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A name and the version at its end, if it has one: "Kaizo V2.1" is Kaizo, version 2.1.
pub fn split_version(name: &str) -> (String, Option<String>) {
    if let Some((before, last)) = name.rsplit_once(' ') {
        let digits = last.strip_prefix(['v', 'V']).unwrap_or(last);
        let named = last.len() != digits.len() || digits.contains('.');
        if named && !digits.is_empty() && digits.split('.').all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit())) {
            return (before.trim().to_string(), Some(digits.to_string()));
        }
    }
    (name.to_string(), None)
}

/// A name as an id: lower case letters and digits, with `-` between words.
pub fn slug(name: &str) -> String {
    let mut id = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            id.push(c.to_ascii_lowercase());
        } else if !id.ends_with('-') && !id.is_empty() {
            id.push('-');
        }
    }
    let mut id = id.trim_end_matches('-').to_string();
    id.truncate(58);
    let id = id.trim_end_matches('-').to_string();
    if id.is_empty() {
        "mod".to_string()
    } else if reserved_id(&id) {
        format!("{id}-mod")
    } else {
        id
    }
}

/// Names Windows keeps for itself, which no folder can have.
pub(crate) fn reserved_id(id: &str) -> bool {
    matches!(id, "con" | "prn" | "aux" | "nul") || (id.len() == 4 && (id.starts_with("com") || id.starts_with("lpt")) && id.as_bytes()[3].is_ascii_digit())
}

struct Builder<'a> {
    plan: Plan,
    store: Option<&'a GameStore>,
}

impl Builder<'_> {
    fn file(&mut self, sd_root: &Path, patch_root: &Path, f: &FilePatch) {
        let Some(host) = resolve(sd_root, patch_root, &f.external).filter(|h| h.is_file()) else {
            self.plan.note(Level::Lost, format!("{}: not in the mod (wanted for {})", f.external, f.disc));
            return;
        };
        if f.offset != 0 || f.file_offset != 0 || f.length != 0 || !f.resize {
            self.plan.note(Level::Lost, format!("{}: patched in part, by offsets into the disc's file, which have no meaning after conversion", f.disc));
            return;
        }
        if f.disc.eq_ignore_ascii_case("main.dol") || f.disc.eq_ignore_ascii_case("/main.dol") {
            self.plan.note(Level::Lost, "main.dol: the Wii program, which Nebula does not have");
            return;
        }
        if f.disc.starts_with('/') {
            if let Some(store) = self.store {
                if !f.create && !store.has(&f.disc) {
                    self.plan.note(Level::Lost, format!("{}: the game has no such file and the patch does not create it", f.disc));
                    return;
                }
            }
            self.plan.set(&f.disc, &host);
            return;
        }
        // A bare name: every file of that name in the game.
        match self.store {
            Some(store) => {
                let paths = store.named(&f.disc);
                if paths.is_empty() {
                    self.plan.note(Level::Lost, format!("{}: no file of that name in the game", f.disc));
                }
                for path in paths {
                    self.plan.set(&path, &host);
                }
            }
            None => self.plan.note(Level::Lost, format!("{}: named without a folder; choose your game store so it can be placed", f.disc)),
        }
    }

    fn folder(&mut self, sd_root: &Path, patch_root: &Path, folder: &FolderPatch, disc: &str, external: &str) {
        let Some(directory) = resolve(sd_root, patch_root, external).filter(|d| d.is_dir()) else {
            // Patches often list every language's folder and a mod brings only some: nothing is lost.
            if is_language_path(disc).is_none() {
                self.plan.note(Level::Lost, format!("{external}: no such folder in the mod"));
            }
            return;
        };
        let Ok(entries) = fs::read_dir(&directory) else {
            self.plan.note(Level::Lost, format!("{external}: cannot be read"));
            return;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() {
                if folder.recursive {
                    self.folder(sd_root, patch_root, folder, &join(disc, &name), &join(external, &name));
                }
                continue;
            }
            self.file(
                sd_root,
                patch_root,
                &FilePatch {
                    disc: join(disc, &name),
                    external: join(external, &name),
                    resize: folder.resize,
                    create: true,
                    length: folder.length,
                    ..Default::default()
                },
            );
        }
    }

    fn memory(&mut self, sd_root: &Path, patch_root: &Path, patch: &MemoryPatch) {
        // Tables in the Wii program that Nebula keeps as data.
        if !patch.ocarina && !patch.value_file.is_empty() {
            let file = resolve(sd_root, patch_root, &patch.value_file).filter(|f| f.is_file());
            let address = patch.offset | 0x8000_0000;
            if let Some(file) = file {
                if address == known_patches::GALAXY_TABLE {
                    self.plan.set("/NebulaData/GalaxyInfoTable.bcsv", &file);
                    self.plan.known_patches.push("where each galaxy is on the map and what opens it".to_string());
                    return;
                }
                if address == known_patches::EVENT_FLAG_TABLE {
                    if let Some(rows) = fs::read(&file).ok().and_then(|bytes| known_patches::event_flag_table(&bytes)) {
                        self.plan.event_flags = rows;
                        self.plan.known_patches.push("the hungry Lumas and which galaxies open which".to_string());
                        return;
                    }
                }
            }
        }
        if !patch.search && !patch.ocarina && patch.value_file.is_empty() {
            if let Some((dome, rows)) = known_patches::comet_table(patch.offset, &patch.value) {
                self.plan.comet_schedule.insert(dome, rows);
                if !self.plan.known_patches.iter().any(|w| w == "the comets' schedule") {
                    self.plan.known_patches.push("the comets' schedule".to_string());
                }
                return;
            }
        }
        if patch.ocarina || !patch.value_file.is_empty() {
            let kind = if patch.ocarina { "code list" } else { "code file" };
            self.plan.unknown_code.push(UnknownCode { address: patch.offset, bytes: 0, kind });
            return;
        }
        if patch.search {
            self.plan.unknown_code.push(UnknownCode { address: patch.offset, bytes: patch.value.len(), kind: "search" });
            return;
        }
        match known_patches::find(patch.offset, &patch.value) {
            Some(known) => {
                if known.id != known_patches::id::NATIVE_ALREADY {
                    self.plan.native_patches.insert(known.id, known.parameter);
                }
                if !self.plan.known_patches.iter().any(|w| w == known.what) {
                    self.plan.known_patches.push(known.what.to_string());
                }
            }
            None => self.plan.unknown_code.push(UnknownCode { address: patch.offset, bytes: patch.value.len(), kind: "memory" }),
        }
    }

    /// What one choice of one option does, on its own.
    fn fragment(&self, disc: &Disc, section: usize, option: usize, choice: usize, file: &PatchFile, game_id: &str) -> Plan {
        let mut alone = disc.clone();
        for each in alone.sections.iter_mut().flat_map(|s| &mut s.options) {
            each.selected = 0;
        }
        alone.sections[section].options[option].selected = choice + 1;
        let mut part = Builder { plan: Plan::default(), store: self.store };
        for patch in alone.active_patches(game_id) {
            let patch_root = if patch.root.is_empty() {
                file.xml.parent().unwrap_or(&file.sd_root).to_path_buf()
            } else {
                resolve(&file.sd_root, file.xml.parent().unwrap_or(&file.sd_root), &patch.root).unwrap_or_else(|| file.sd_root.clone())
            };
            for f in &patch.files {
                part.file(&file.sd_root, &patch_root, f);
            }
            for f in &patch.folders {
                let disc_path = if f.disc.is_empty() { "/".to_string() } else { f.disc.clone() };
                part.folder(&file.sd_root, &patch_root, f, &disc_path, &f.external);
            }
            for memory in &patch.memory {
                part.memory(&file.sd_root, &patch_root, memory);
            }
            for save in &patch.saves {
                if let Some(folder) = resolve(&file.sd_root, &patch_root, &save.external) {
                    part.plan.save_folder = Some(folder);
                    part.plan.save_clone = save.clone;
                }
            }
        }
        part.plan
    }

    /// What a part says about itself goes in the plan; its files and switches go where the caller puts them.
    fn take_notes(&mut self, part: &Plan) {
        for note in &part.notes {
            if !self.plan.notes.contains(note) {
                self.plan.notes.push(note.clone());
            }
        }
        for what in &part.known_patches {
            if !self.plan.known_patches.contains(what) {
                self.plan.known_patches.push(what.clone());
            }
        }
        self.plan.unknown_code.extend(part.unknown_code.iter().cloned());
        if !part.event_flags.is_empty() {
            self.plan.event_flags.clone_from(&part.event_flags);
        }
        for (dome, rows) in &part.comet_schedule {
            self.plan.comet_schedule.insert(*dome, rows.clone());
        }
        if part.save_folder.is_some() {
            self.plan.save_folder.clone_from(&part.save_folder);
            self.plan.save_clone = part.save_clone;
        }
    }

    fn take_always(&mut self, part: Plan) {
        self.take_notes(&part);
        self.plan.files.extend(part.files);
        self.plan.native_patches.extend(part.native_patches);
    }

    /// A question that only asks which version of the game this is. Nobody needs to answer it: what the versions share is
    /// always used, and what differs goes in layers for those versions. A mod's code patches differ between versions only
    /// in their addresses and Nebula's switches don't depend on the version, so the choice Nebula knows most of says them
    /// all.
    fn region_option(&mut self, disc: &Disc, section: usize, option: usize, file: &PatchFile) {
        let choices = &disc.sections[section].options[option].choices;
        let mut parts: Vec<(char, Plan)> = Vec::new();
        for (i, choice) in choices.iter().enumerate() {
            let letter = riivolution::choice_region(&choice.name).unwrap_or('E');
            let part = self.fragment(disc, section, option, i, file, &format!("RMG{letter}01"));
            parts.push((letter, part));
        }
        let best = parts.iter().max_by_key(|(_, p)| (p.known_patches.len(), std::cmp::Reverse(p.unknown_code.len()))).map(|(_, p)| p.clone());
        if let Some(best) = best {
            self.take_notes(&Plan { known_patches: best.known_patches, unknown_code: best.unknown_code, ..Default::default() });
            self.plan.native_patches.extend(best.native_patches);
        }
        let same = parts.windows(2).all(|w| w[0].1.files == w[1].1.files);
        let letters: Vec<char> = parts.iter().map(|(l, _)| *l).collect();
        for (index, (letter, mut part)) in parts.into_iter().enumerate() {
            part.known_patches.clear();
            part.unknown_code.clear();
            self.take_notes(&part);
            if same {
                self.plan.files.extend(part.files);
                continue;
            }
            if part.files.is_empty() {
                continue;
            }
            // The first choice also stands for any version the mod didn't think of.
            let mut versions = vec![letter];
            if index == 0 {
                versions.extend(VERSION_LETTERS.iter().copied().filter(|l| !letters.contains(l)));
            }
            self.plan.layers.push(Layer {
                name: format!("version-{}", letter.to_ascii_lowercase()),
                when: When { version: versions, ..Default::default() },
                files: part.files,
                ..Default::default()
            });
        }
    }

    /// An option the player chooses in the game: each choice a layer.
    fn player_option(&mut self, disc: &Disc, section: usize, option: usize, file: &PatchFile, game_id: &str) {
        let entry = &disc.sections[section].options[option];
        let name = entry.name.trim_end_matches(':').trim().to_string();
        let mut id = slug(&name);
        if self.plan.options.iter().any(|o| o.id == id) {
            id = format!("{}-{id}", slug(&disc.sections[section].name));
        }
        let mut choices = vec![OptionChoice { id: "off".to_string(), name: "Off".to_string() }];
        let mut default = "off".to_string();
        for (i, choice) in entry.choices.iter().enumerate() {
            let mut choice_id = slug(&choice.name);
            if choice_id == "off" || choices.iter().any(|c| c.id == choice_id) {
                choice_id = format!("choice-{}", i + 1);
            }
            let mut part = self.fragment(disc, section, option, i, file, game_id);
            self.take_notes(&part);
            if entry.selected == i + 1 {
                default.clone_from(&choice_id);
            }
            let when = When { option: BTreeMap::from([(id.clone(), vec![choice_id.clone()])]), ..Default::default() };
            if !part.files.is_empty() || !part.native_patches.is_empty() {
                self.plan.layers.push(Layer {
                    name: format!("{id}-{choice_id}"),
                    when,
                    files: std::mem::take(&mut part.files),
                    switches: std::mem::take(&mut part.native_patches),
                });
            }
            choices.push(OptionChoice { id: choice_id, name: choice.name.clone() });
        }
        self.plan.options.push(PlayerOption { id, name, default, choices });
    }

    fn patch_file(&mut self, file: &PatchFile, game_id: &str) -> Result<bool> {
        let name = file.xml.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let bytes = fs::read(&file.xml)?;
        let mut disc: Disc = match riivolution::parse(&riivolution::decode(&bytes)) {
            Ok(disc) => disc,
            Err(error) => {
                // Other XML beside the patches (a tool's settings) is not a patch.
                self.plan.note(Level::Info, format!("{name}: not used: {error:#}"));
                return Ok(false);
            }
        };
        // A mod written with one version in mind still works on the others: their files are the same.
        let wanted = disc.target.game.clone().unwrap_or_default();
        if !wanted.is_empty() && wanted.get(..3).unwrap_or(&wanted) != ANY_VERSION {
            let described = match wanted.get(..3) {
                Some("SB4") | Some("SB5") => "Super Mario Galaxy 2".to_string(),
                _ => "another game".to_string(),
            };
            self.plan.for_another_game.get_or_insert(described);
            return Ok(false);
        }
        let game_id = version_named(&disc.target, game_id);
        let saved = layout::Layout::saved_choices(file, &game_id);
        disc.choose_defaults(&game_id, &saved);
        let total: usize = disc.sections.iter().map(|s| s.options.len()).sum();
        for section in 0..disc.sections.len() {
            for option in 0..disc.sections[section].options.len() {
                let entry = &disc.sections[section].options[option];
                if entry.asks_for_region() {
                    self.region_option(&disc, section, option, file);
                } else if total == 1 && entry.choices.len() == 1 {
                    // A mod's one switch is the mod itself.
                    let part = self.fragment(&disc, section, option, 0, file, &game_id);
                    self.take_always(part);
                } else {
                    self.player_option(&disc, section, option, file, &game_id);
                }
            }
        }
        Ok(true)
    }

    fn files_root(&mut self, root: &Path, top_level: bool) {
        let Ok(entries) = fs::read_dir(root) else {
            return;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());
        let whole = FolderPatch { resize: true, create: true, recursive: true, ..Default::default() };
        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() {
                if !top_level || layout::is_disc_folder(&name) {
                    self.folder(root, root, &whole, &format!("/{name}"), &format!("/{name}"));
                }
            } else if !layout::is_note(&entry.path()) {
                self.file(root, root, &FilePatch { disc: format!("/{name}"), external: format!("/{name}"), resize: true, create: true, ..Default::default() });
            }
        }
    }
}

/// The plan for a mod on this disc. `store`, when given, is the player's imported game; without it some placements cannot
/// be checked and bare file names cannot be placed.
pub fn build(layout: &Layout, game_id: &str, store: Option<&GameStore>) -> Result<Plan> {
    let mut builder = Builder { plan: Plan { game_id: game_id.to_string(), ..Default::default() }, store };
    let mut used_patch = false;
    for file in &layout.patches {
        used_patch |= builder.patch_file(file, game_id)?;
    }
    if !used_patch && !layout.patches.is_empty() {
        if let Some(wanted) = builder.plan.for_another_game.clone() {
            builder.plan.note(Level::Blocked, format!("This mod is for {wanted}."));
        }
    }
    if !builder.plan.blocked() {
        if let Some(root) = &layout.files_root {
            builder.files_root(root, root == &layout.root);
        }
        for file in &layout.loose {
            let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            match store {
                Some(s) => {
                    let paths = s.named(&name);
                    if paths.is_empty() {
                        builder.plan.note(Level::Lost, format!("{name}: no file of that name in the game"));
                    }
                    for path in paths {
                        builder.plan.set(&path, file);
                    }
                }
                None => builder.plan.note(Level::Lost, format!("{name}: a loose file; choose your game store so it can be placed")),
            }
        }
        if builder.plan.for_another_game.is_none() {
            for code in &layout.wii_code {
                let name = code.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                builder.plan.note(Level::Lost, format!("{name} is compiled Wii code and can't be converted. What it does has to be written again as a Nebula plugin."));
            }
        }
    }
    if let Some(store) = store {
        let adds = |file: &PlannedFile| {
            let lower = file.disc.to_ascii_lowercase();
            lower.starts_with("/stagedata/") && lower.ends_with("scenario.arc") && !store.has(&file.disc)
        };
        let mut new_galaxies = std::collections::BTreeSet::new();
        for file in builder.plan.files.values().chain(builder.plan.layers.iter().flat_map(|l| l.files.values())) {
            if adds(file) {
                // /StageData/<name>/<name>Scenario.arc
                if let Some(name) = file.disc.rsplit('/').next().and_then(|f| f.get(..f.len().saturating_sub("Scenario.arc".len()))) {
                    new_galaxies.insert(name.to_string());
                }
            }
        }
        builder.plan.adds_galaxy = !new_galaxies.is_empty();
        builder.plan.new_galaxies = new_galaxies.into_iter().collect();
        if builder.plan.adds_galaxy {
            builder.plan.note(Level::Info, "It adds a galaxy, so it keeps its own saves.");
        }
    }
    // Options all off and nothing else would be a mod that does nothing when it's turned on.
    if builder.plan.files.is_empty() && builder.plan.native_patches.is_empty() && builder.plan.options.iter().all(|o| o.default == "off") {
        if let Some(first) = builder.plan.options.first_mut() {
            if let Some(choice) = first.choices.get(1) {
                first.default.clone_from(&choice.id);
            }
        }
    }
    let count = builder.plan.unknown_code.len();
    if count != 0 {
        builder.plan.note(Level::Lost, format!("Most of this mod works. {count} code changes need a Nebula update or a plugin."));
    }
    if let Some(pack) = &layout.pack {
        for (game, count) in &pack.other_games {
            if !game_id.starts_with(game.get(..3).unwrap_or(game)) {
                builder.plan.note(Level::Info, format!("{count} texture pack files are for {game} and were left out"));
            }
        }
    }
    if builder.plan.file_count() == 0 && builder.plan.native_patches.is_empty() && layout.pack.as_ref().is_none_or(|p| p.textures == 0) && !builder.plan.blocked() {
        let other = layout.pack.as_ref().and_then(|p| p.other_games.keys().next().cloned());
        match other {
            Some(game) if game.starts_with("SB4") => builder.plan.note(Level::Blocked, "These textures are for Super Mario Galaxy 2."),
            Some(_) => builder.plan.note(Level::Blocked, "These textures are for another game."),
            None => builder.plan.note(Level::Blocked, "This doesn't look like a mod. Nothing in it could be used."),
        }
    }
    Ok(builder.plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_patch_cannot_name_a_place_outside_the_game() {
        let mut plan = Plan::default();
        for bad in ["/../../../evil.bin", "/StageData/../../evil.bin", "\\\\server\\evil", "C:/evil.bin", "/a:stream", "/CON"] {
            plan.set(bad, Path::new("x"));
        }
        assert!(plan.files.is_empty(), "{:?}", plan.files.keys().collect::<Vec<_>>());
        plan.set("/StageData/Good.arc", Path::new("x"));
        assert_eq!(plan.files.len(), 1);
    }

    #[test]
    fn any_name_makes_an_id_the_game_reads() {
        for name in ["Aux", "CON", "com1", "!!!", "", "A Very Long Name That Goes On And On And On And On And On And On And On And On", "Kaizo: Rebalanced!"] {
            assert!(crate::project::valid_id(&slug(name)), "{name:?} -> {}", slug(name));
        }
    }

    fn write(root: &Path, relative: &str, text: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    const XML: &str = r#"<wiidisc version="1"><id game="RMG"/><options><section name="M"><option name="Region">
        <choice name="NTSC"><patch id="p"/></choice><choice name="PAL"><patch id="q"/></choice></option></section></options>
        <patch id="p"><folder disc="/StageData" external="/M/stage" create="true"/>
          <file disc="Kuribo.arc" external="/M/k.arc"/>
          <file disc="/ObjectData/New.arc" external="/M/k.arc"/>
          <file disc="/ObjectData/Part.arc" external="/M/k.arc" offset="16"/>
          <file disc="/ObjectData/Gone.arc" external="/M/none.arc" create="true"/>
          <memory offset="0x8039AA2C" value="60000000"/>
          <memory offset="0x8007679C" value="60000000"/>
          <memory offset="0x80000000" value="00000001"/>
          <memory valuefile="code.gct" ocarina="true"/>
          <savegame external="/save"/>
        </patch><patch id="q"><file disc="/PalOnly" external="/M/k.arc" create="true"/></patch></wiidisc>"#;

    fn store() -> GameStore {
        GameStore::from_paths("RMGE01", &["ObjectData", "ObjectData/Kuribo.arc", "StageData", "UsEnglish"])
    }

    #[test]
    fn a_patch_file_places_everything_or_says_why() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "riivolution/m.xml", XML);
        write(dir.path(), "M/stage/a.arc", "x");
        write(dir.path(), "M/stage/Sub/b.arc", "x");
        write(dir.path(), "M/k.arc", "x");
        write(dir.path(), "save/keep.txt", "x");
        let store = store();
        let plan = build(&Layout::of(dir.path()), "RMGE01", Some(&store)).unwrap();
        // NTSC and PAL set different files: a layer for each, and no question.
        assert!(plan.options.is_empty());
        assert_eq!(plan.layers.len(), 2);
        let ntsc = &plan.layers[0];
        assert!(ntsc.when.version.contains(&'E') && ntsc.when.version.contains(&'J'), "{:?}", ntsc.when);
        let mut files: Vec<_> = ntsc.files.values().map(|f| f.disc.as_str()).collect();
        files.sort();
        assert_eq!(files, ["/ObjectData/Kuribo.arc", "/StageData/Sub/b.arc", "/StageData/a.arc"]);
        assert_eq!(plan.layers[1].when.version, ['P']);
        let text: Vec<_> = plan.notes.iter().map(|n| n.text.as_str()).collect();
        assert!(text.iter().any(|t| t.contains("/ObjectData/New.arc") && t.contains("does not create")), "{text:?}");
        assert!(text.iter().any(|t| t.contains("patched in part")), "{text:?}");
        assert!(text.iter().any(|t| t.contains("/M/none.arc")), "{text:?}");
        assert_eq!(plan.native_patches[&known_patches::id::SKELETAL_FISH_BABY_NO_BUBBLE], 1);
        assert_eq!(plan.known_patches.len(), 2);
        assert_eq!(plan.unknown_code.len(), 2);
        assert!(plan.save_folder.is_some());
    }

    const OPTIONS: &str = r#"<wiidisc version="1"><id game="RMG"/><options><section name="M">
        <option name="Lives:"><choice name="Normal"><patch id="a"/></choice><choice name="One health"><patch id="b"/></choice></option>
        <option name="Music"><choice name="New"><patch id="c"/></choice></option></section></options>
        <patch id="a"><file disc="/StageData/A.arc" external="/M/a.arc" create="true"/></patch>
        <patch id="b"><file disc="/StageData/A.arc" external="/M/b.arc" create="true"/></patch>
        <patch id="c"><file disc="/AudioRes/C.arc" external="/M/a.arc" create="true"/></patch></wiidisc>"#;

    #[test]
    fn options_become_the_players_choices() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "riivolution/m.xml", OPTIONS);
        write(dir.path(), "M/a.arc", "a");
        write(dir.path(), "M/b.arc", "b");
        let plan = build(&Layout::of(dir.path()), "RMGP01", None).unwrap();
        assert_eq!(plan.options.len(), 2);
        let lives = &plan.options[0];
        assert_eq!(lives.id, "lives");
        assert_eq!(lives.name, "Lives");
        // Everything off would be a mod that does nothing.
        assert_eq!(lives.default, "normal");
        assert_eq!(lives.choices.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), ["off", "normal", "one-health"]);
        assert_eq!(plan.layers.len(), 3);
        assert_eq!(plan.layers[1].name, "lives-one-health");
        assert_eq!(plan.layers[1].when.option["lives"], ["one-health"]);
        assert_eq!(plan.file_count(), 2);
    }

    #[test]
    fn versions_with_the_same_files_need_no_layers() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "riivolution/m.xml", r#"<wiidisc version="1"><id game="RMG"/><options><section name="M"><option name="Region">
            <choice name="NTSC"><patch id="a"/></choice><choice name="PAL"><patch id="a"/></choice><choice name="NTSC-J"><patch id="a"/></choice></option></section></options>
            <patch id="a"><file disc="/StageData/A.arc" external="/M/a.arc" create="true"/></patch></wiidisc>"#);
        write(dir.path(), "M/a.arc", "a");
        let plan = build(&Layout::of(dir.path()), "RMGJ01", None).unwrap();
        assert!(plan.layers.is_empty());
        assert!(plan.options.is_empty());
        assert_eq!(plan.files.len(), 1);
    }

    #[test]
    fn a_mod_for_galaxy_2_is_named_as_that() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "riivolution/m.xml", r#"<wiidisc version="1"><id game="SB4"/></wiidisc>"#);
        write(dir.path(), "game/CustomCode/CustomCode_SB4E.bin", "x");
        let plan = build(&Layout::of(dir.path()), "RMGE01", None).unwrap();
        assert!(plan.blocked());
        assert!(plan.notes.iter().any(|n| n.text.contains("Super Mario Galaxy 2")), "{:?}", plan.notes);
    }

    #[test]
    fn names_and_versions_come_from_file_names() {
        assert_eq!(nice_name("KaizoMarioGalaxyRebalanced V2.1"), "Kaizo Mario Galaxy Rebalanced V2.1");
        assert_eq!(nice_name("super_mario_galaxy_the_green_stars_"), "Super Mario Galaxy The Green Stars");
        assert_eq!(nice_name("nmg_v1.6.0"), "Nmg v1.6.0");
        assert_eq!(split_version("Kaizo Mario Galaxy Rebalanced V2.1"), ("Kaizo Mario Galaxy Rebalanced".to_string(), Some("2.1".to_string())));
        assert_eq!(split_version("Nmg v1.6.0"), ("Nmg".to_string(), Some("1.6.0".to_string())));
        assert_eq!(split_version("Galaxy 2"), ("Galaxy 2".to_string(), None));
        assert_eq!(split_version("Green Stars"), ("Green Stars".to_string(), None));
    }

    #[test]
    fn folders_in_any_case_and_one_file_once() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "ObjectData/A.arc", "first");
        write(dir.path(), "objectdata2/a.arc", "x");
        write(dir.path(), "readme.txt", "note");
        write(dir.path(), "preview.png", "note");
        let plan = build(&Layout::of(dir.path()), "RMGE01", None).unwrap();
        assert_eq!(plan.files.len(), 1);
    }

    #[test]
    fn a_mod_that_adds_a_galaxy_is_noticed() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "StageData/MyGalaxy/MyGalaxyScenario.arc", "x");
        let store = store();
        let plan = build(&Layout::of(dir.path()), "RMGE01", Some(&store)).unwrap();
        assert!(plan.adds_galaxy);
        assert!(plan.notes.iter().any(|n| n.text.contains("own saves")), "{:?}", plan.notes);
    }

    #[test]
    fn a_mod_for_one_version_converts_for_every_version() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "riivolution/m.xml", r#"<wiidisc version="1"><id game="RMGP"/><options><section name="M"><option name="On"><choice name="Yes"><patch id="p"/></choice></option></section></options><patch id="p"><folder disc="/StageData" external="/M/stage" create="true"/></patch></wiidisc>"#);
        write(dir.path(), "M/stage/a.arc", "x");
        let plan = build(&Layout::of(dir.path()), "RMGE01", None).unwrap();
        assert!(!plan.blocked(), "{:?}", plan.notes);
        assert_eq!(plan.files.len(), 1);
    }

    #[test]
    fn compiled_code_is_named_and_the_rest_applies() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "ObjectData/A.arc", "x");
        write(dir.path(), "CustomCode/CustomCode_USA.bin", "x");
        let plan = build(&Layout::of(dir.path()), "RMGE01", None).unwrap();
        assert_eq!(plan.files.len(), 1);
        assert!(plan.notes.iter().any(|n| n.level == Level::Lost && n.text.contains("CustomCode_USA.bin")), "{:?}", plan.notes);
    }

    #[test]
    fn a_pack_for_galaxy_2_is_refused_by_name() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "SB4/Neo/LayoutData/Buttons/a.png", "x");
        write(dir.path(), "readme.txt", "x");
        let plan = build(&Layout::of(dir.path()), "RMGE01", None).unwrap();
        assert!(plan.blocked());
        assert!(plan.notes.iter().any(|n| n.text.contains("Super Mario Galaxy 2")), "{:?}", plan.notes);
    }

    #[test]
    fn a_path_cannot_leave_the_mod() {
        let dir = tempfile::tempdir().unwrap();
        assert!(resolve(dir.path(), dir.path(), "../outside").is_none());
        assert!(resolve(dir.path(), dir.path(), "C:/outside").is_none());
    }
}
