//! The LumaLink window: a grid of tiles, one per mod. A tile's circle says whether it will be converted; pressing the tile
//! opens the mod's page. Along the bottom, the game these are for and two round buttons.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sky;
mod words;

use eframe::egui::{self, Align, Color32, FontData, FontDefinitions, FontFamily, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use lumalink_core::game_store::GameStore;
use lumalink_core::layout::Layout;
use lumalink_core::plan::{self, Level, Plan, PlayerOption};
use lumalink_core::{package, source};
use sky::{AMBER, BLUE_INK, GREEN, INK, RED, SOFT};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use words::{counted, word, Word};

const COLUMNS: usize = 4;
const ROWS: usize = 3;
const PER_PAGE: usize = COLUMNS * ROWS;
/// The space from the window's edges to the grid.
const SIDE: f32 = 44.0;
const TOP: f32 = 20.0;
const GAP: f32 = 12.0;

#[derive(Clone, PartialEq)]
enum State {
    Reading,
    Ready,
    CantConvert(String),
    Converting,
    Done { file: PathBuf, summary: String },
    Failed(String),
    /// Converting would leave these files out: nothing is written until the person says to build it without them.
    Losses(String),
}

impl State {
    fn word(&self) -> (&'static str, Color32) {
        match self {
            State::Reading => (word(Word::Reading), SOFT),
            State::Ready => (word(Word::Ready), BLUE_INK),
            State::CantConvert(_) => (word(Word::CantConvert), RED),
            State::Converting => (word(Word::Converting), SOFT),
            State::Done { .. } => (word(Word::Done), GREEN),
            State::Failed(_) => (word(Word::DidntWork), RED),
            State::Losses(_) => (word(Word::LeavesFilesOut), AMBER),
        }
    }
}

struct ModRow {
    /// Never reused, so that answers from other threads find their mod after others have gone.
    id: u64,
    input: PathBuf,
    name: String,
    state: State,
    /// Whether it goes in with the next Convert.
    ticked: bool,
    /// One line on what it holds.
    about: String,
    /// What did not reach the game, for people to read.
    notes: Vec<(Level, String)>,
    /// What players will choose in the game.
    options: Vec<PlayerOption>,
    /// Which planet its tile shows.
    hue: u32,
    /// When it finished, for its burst of stars.
    done_at: Option<f64>,
}

enum Event {
    Read { id: u64, about: String, notes: Vec<(Level, String)>, options: Vec<PlayerOption>, blocked: Option<String> },
    Converted { id: u64, result: Result<(PathBuf, String), Refused> },
}

struct App {
    mods: Vec<ModRow>,
    next_id: u64,
    game: Option<PathBuf>,
    game_label: String,
    game_problem: Option<String>,
    output: PathBuf,
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    page: usize,
    /// The mod whose page is open.
    open: Option<u64>,
    /// Where to save a picture of the window once its mods are read, then close (for this project's own checks).
    screenshot: Option<PathBuf>,
    screenshot_asked: bool,
    screenshot_convert: bool,
}

/// Where Nebula's installer keeps the game on this computer, if it's there.
fn find_game() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
    [base.join("Programs/Nebula/content/game.pak"), base.join("Nebula/content/game.pak")].into_iter().find(|p| p.is_file())
}

fn describe_game(id: &str) -> String {
    let region = match id.chars().nth(3) {
        Some('E') => word(Word::Usa),
        Some('P') => word(Word::Europe),
        Some('J') => word(Word::Japan),
        Some('K') => word(Word::Korea),
        _ => id,
    };
    format!("Super Mario Galaxy ({region})")
}

fn about(layout: &Layout, plan: &Plan) -> String {
    let what = if !layout.patches.is_empty() {
        word(Word::ARiivolutionMod)
    } else if layout.files_root.is_some() || !layout.loose.is_empty() {
        word(Word::AMod)
    } else {
        word(Word::ATexturePack)
    };
    let mut parts = Vec::new();
    let files = plan.file_count();
    if files != 0 {
        parts.push(counted(Word::Files, files));
    }
    if let Some(pack) = layout.pack.as_ref().filter(|p| p.textures != 0) {
        parts.push(counted(Word::Textures, pack.textures));
    }
    if !plan.known_patches.is_empty() {
        parts.push(counted(Word::GameplayChanges, plan.known_patches.len()));
    }
    if parts.is_empty() {
        what.to_string()
    } else {
        format!("{what}: {}", parts.join(", "))
    }
}

/// What a dropped file is, when it isn't something to convert.
fn not_a_mod(path: &Path) -> Option<State> {
    let ending = path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    match ending.as_str() {
        "luma" => Some(State::Done {
            file: path.to_path_buf(),
            summary: word(Word::AlreadyFinished).to_string(),
        }),
        "iso" | "rvz" | "wbfs" | "wia" | "gcz" | "ciso" | "nkit" => {
            Some(State::CantConvert(word(Word::WholeGame).to_string()))
        }
        _ => None,
    }
}

type ReadMod = (Layout, Plan, Option<GameStore>, tempfile::TempDir);

/// Why a conversion wrote nothing: files it would leave out, for the person to accept, or an error.
enum Refused {
    Losses(String),
    Error(String),
}

impl From<String> for Refused {
    fn from(error: String) -> Self {
        Refused::Error(error)
    }
}

/// A mod that breaks the tool in a way nobody foresaw is that mod's tile saying so, not a tile waiting for ever.
fn guarded<T, E: From<String>>(job: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(job)).unwrap_or_else(|_| Err(E::from(word(Word::SomethingWrong).to_string())))
}

fn read_mod(input: &Path, game: Option<&Path>) -> Result<ReadMod, String> {
    let store = game.map(GameStore::open).transpose().map_err(|e| format!("{e:#}"))?;
    let id = store.as_ref().map_or("RMGE01".to_string(), |s| s.game_id.clone());
    let work = tempfile::tempdir().map_err(|e| e.to_string())?;
    if let Some(game) = game {
        source::base_from_store(game);
    }
    let root = source::open(input, work.path()).map_err(|e| format!("{e:#}"))?;
    let layout = Layout::of(&root);
    let plan = plan::build(&layout, &id, store.as_ref()).map_err(|e| format!("{e:#}"))?;
    Ok((layout, plan, store, work))
}

fn show_file(file: &Path) {
    // Explorer reads its own command line: the quotes go after the comma, not around the whole argument.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = std::process::Command::new("explorer").raw_arg(format!("/select,\"{}\"", file.display())).spawn();
    }
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg("-R").arg(file).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(file.parent().unwrap_or(file)).spawn();
}

fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("bold".into()))
}

fn regular(size: f32) -> FontId {
    FontId::proportional(size)
}

/// A capsule button at a place; true when it was pressed.
fn capsule_at(ui: &mut egui::Ui, area: Rect, id: &str, label: &str, font: &FontId, enabled: bool) -> bool {
    let response = ui.interact(area, ui.id().with(id), if enabled { Sense::click() } else { Sense::hover() });
    sky::capsule(ui.painter(), area, label, font, response.hovered() && enabled, enabled);
    response.clicked() && enabled
}

impl App {
    fn new() -> Self {
        let (sender, receiver) = channel();
        let mut app = Self {
            mods: Vec::new(),
            next_id: 1,
            game: None,
            game_label: String::new(),
            game_problem: None,
            output: std::env::current_dir().unwrap_or_default(),
            sender,
            receiver,
            page: 0,
            open: None,
            screenshot: std::env::var_os("LUMALINK_SCREENSHOT").map(PathBuf::from),
            screenshot_asked: false,
            screenshot_convert: std::env::var_os("LUMALINK_SCREENSHOT_CONVERT").is_some(),
        };
        if let Some(path) = std::env::var_os("LUMALINK_GAME").map(PathBuf::from).or_else(find_game) {
            app.set_game(path);
        }
        // Mods named when it starts, as "Open with LumaLink" does.
        for path in std::env::args_os().skip(1).map(PathBuf::from) {
            app.add_path(path);
        }
        if let Some(index) = std::env::var("LUMALINK_SCREENSHOT_PAGE").ok().and_then(|v| v.parse::<usize>().ok()) {
            app.open = app.mods.get(index).map(|m| m.id);
        }
        if std::env::var_os("LUMALINK_SCREENSHOT_UNTICK").is_some() {
            if let Some(row) = app.mods.first_mut() {
                row.ticked = false;
            }
        }
        app
    }

    fn set_game(&mut self, path: PathBuf) {
        match GameStore::open(&path) {
            Ok(store) => {
                self.game_label = describe_game(&store.game_id);
                self.game_problem = None;
                self.game = Some(path);
                for i in 0..self.mods.len() {
                    if self.mods[i].input.is_dir() || source::is_archive(&self.mods[i].input) || source::is_disc(&self.mods[i].input) {
                        self.read(i);
                    }
                }
            }
            Err(_) => self.game_problem = Some(word(Word::NotTheGame).to_string()),
        }
    }

    /// A mod, or a folder of mods: everything in it comes in together.
    fn add_path(&mut self, path: PathBuf) {
        if path.is_dir() {
            let layout = Layout::of(&path);
            if layout.patches.is_empty() && layout.files_root.is_none() && layout.pack.is_none() {
                let mut inside: Vec<PathBuf> = std::fs::read_dir(&path)
                    .map(|entries| entries.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir() || source::is_archive(p)).collect())
                    .unwrap_or_default();
                inside.sort();
                if !inside.is_empty() {
                    for child in inside {
                        self.add(child);
                    }
                    return;
                }
            }
            self.add(path);
        } else if source::is_archive(&path) || source::is_disc(&path) {
            self.add(path);
        } else {
            let state = not_a_mod(&path).unwrap_or_else(|| State::CantConvert(word(Word::NotAKind).to_string()));
            self.add(path.clone());
            if let Some(row) = self.mods.iter_mut().find(|m| m.input == path) {
                row.ticked = false;
                row.state = state;
            }
        }
    }

    fn add(&mut self, input: PathBuf) {
        if self.mods.iter().any(|m| m.input == input) {
            return;
        }
        let input_is_a_mod = input.is_dir() || source::is_archive(&input) || source::is_disc(&input);
        let name = plan::nice_name(&source::mod_name(&input));
        let hue = name.bytes().fold(7u32, |h, b| h.wrapping_mul(31).wrapping_add(u32::from(b))) >> 3;
        let id = self.next_id;
        self.next_id += 1;
        self.mods.push(ModRow {
            id,
            name,
            input,
            state: State::Reading,
            ticked: true,
            about: String::new(),
            notes: Vec::new(),
            options: Vec::new(),
            hue,
            done_at: None,
        });
        if input_is_a_mod {
            self.read(self.mods.len() - 1);
        }
    }

    fn pick_mods(&mut self) {
        if let Some(files) = rfd::FileDialog::new().add_filter(word(Word::ModsFilter), &["zip", "7z", "rar", "iso", "wbfs", "rvz", "luma"]).pick_files() {
            for file in files {
                self.add_path(file);
            }
        }
    }

    fn read(&mut self, index: usize) {
        let row = &mut self.mods[index];
        row.state = State::Reading;
        let (id, input, game, sender) = (row.id, row.input.clone(), self.game.clone(), self.sender.clone());
        std::thread::spawn(move || {
            let event = match guarded(|| read_mod(&input, game.as_deref())) {
                Ok((layout, plan, _, _)) => Event::Read {
                    id,
                    about: about(&layout, &plan),
                    notes: plan.notes.iter().map(|n| (n.level, n.text.clone())).collect(),
                    options: plan.options.clone(),
                    blocked: plan.notes.iter().find(|n| n.level == Level::Blocked).map(|n| n.text.clone()),
                },
                Err(error) => Event::Read { id, about: String::new(), notes: Vec::new(), options: Vec::new(), blocked: Some(error) },
            };
            let _ = sender.send(event);
        });
    }

    fn convert(&mut self, which: Vec<u64>) {
        let (game, output, sender) = (self.game.clone(), self.output.clone(), self.sender.clone());
        let mut jobs = Vec::new();
        for row in self.mods.iter_mut().filter(|m| which.contains(&m.id)) {
            // A mod whose losses were shown and converted again is built without those files.
            let accept_losses = matches!(row.state, State::Losses(_));
            jobs.push((row.id, row.input.clone(), row.name.clone(), accept_losses));
            row.state = State::Converting;
        }
        // One at a time: a conversion's shader compile already uses every core.
        std::thread::spawn(move || {
            for (id, input, name, accept_losses) in jobs {
                let result = guarded(|| {
                    let (layout, plan, store, _work) = read_mod(&input, game.as_deref()).map_err(Refused::Error)?;
                    package::write(&name, &layout, &plan, store.as_ref(), &output, accept_losses)
                        .map(|outcome| (outcome.file, outcome.total, outcome.converted))
                        .map_err(|e| match e.downcast_ref::<package::Losses>() {
                            Some(losses) => Refused::Losses(losses.to_string()),
                            None => Refused::Error(format!("{e:#}")),
                        })
                });
                let result = result.map(|(file, total, converted)| {
                    let summary = if converted == total {
                        word(Word::OpenInNebula).to_string()
                    } else {
                        counted(Word::FilesLeftOut, total - converted)
                    };
                    (file, summary)
                });
                let _ = sender.send(Event::Converted { id, result });
            }
        });
    }

    fn take_events(&mut self, now: f64) {
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                Event::Read { id, about, notes, options, blocked } => {
                    if let Some(row) = self.mods.iter_mut().find(|m| m.id == id) {
                        row.about = about;
                        row.notes = notes;
                        row.options = options;
                        row.state = blocked.map_or(State::Ready, State::CantConvert);
                    }
                }
                Event::Converted { id, result } => {
                    if let Some(row) = self.mods.iter_mut().find(|m| m.id == id) {
                        row.state = match result {
                            Ok((file, summary)) => {
                                row.done_at = Some(now);
                                row.ticked = false;
                                State::Done { file, summary }
                            }
                            Err(Refused::Losses(list)) => State::Losses(list),
                            Err(Refused::Error(error)) => State::Failed(error),
                        };
                    }
                }
            }
        }
    }

    /// The grid: every mod a tile, the next free place the one that takes more.
    fn grid(&mut self, ui: &mut egui::Ui, area: Rect, dropping: bool, now: f64) {
        let pages = (self.mods.len() / PER_PAGE) + 1;
        self.page = self.page.min(pages - 1);
        let tile = Vec2::new(
            (area.width() - SIDE * 2.0 - GAP * (COLUMNS - 1) as f32) / COLUMNS as f32,
            (area.height() - GAP * (ROWS - 1) as f32) / ROWS as f32,
        );
        let first = self.page * PER_PAGE;
        let mut opened = None;
        let mut pick = false;
        for slot in 0..PER_PAGE {
            let at = area.min + Vec2::new(SIDE + (slot % COLUMNS) as f32 * (tile.x + GAP), (slot / COLUMNS) as f32 * (tile.y + GAP));
            let place = Rect::from_min_size(at, tile);
            let response = ui.interact(place, ui.id().with(("tile", first + slot)), Sense::click());
            let painter = ui.painter().clone();
            if let Some(row) = self.mods.get_mut(first + slot) {
                let dim = matches!(row.state, State::CantConvert(_) | State::Failed(_));
                let sky_area = sky::mod_tile(&painter, place, row.hue, dim, now);
                // The name and its state, each centred in the white below the sky.
                let name_font = bold(14.0);
                let word_font = regular(12.5);
                let lines = sky::two_lines(&painter, &row.name, &name_font, tile.x - 20.0);
                let below_sky = sky_area.bottom();
                let state_y = place.bottom() - 15.0;
                let name_middle = (below_sky + state_y - 7.0) / 2.0;
                let line_step = 17.0;
                let mut y = name_middle - line_step * (lines.len() as f32 - 1.0) / 2.0;
                for line in &lines {
                    sky::text(&painter, Pos2::new(place.center().x, y), Align::Center, line, &name_font, INK);
                    y += line_step;
                }
                let (state_word, color) = row.state.word();
                sky::text(&painter, Pos2::new(place.center().x, state_y), Align::Center, state_word, &word_font, color);
                if matches!(row.state, State::Reading | State::Converting) {
                    sky::orbiting_star(&painter, sky_area.center(), sky_area.height() * 0.62, now);
                }
                if let Some(done_at) = row.done_at {
                    if now - done_at < sky::BURST_SECONDS {
                        sky::burst(&painter, sky_area.center(), now - done_at);
                    }
                }
                if response.hovered() {
                    sky::tile_outline(&painter, place);
                }
                // Its circle: the tile's top corner, for a mod that can be converted.
                let mut circle_pressed = false;
                if row.state == State::Ready || row.state == State::Converting {
                    let centre = Pos2::new(sky_area.right() - 17.0, sky_area.top() + 17.0);
                    let hit = ui.interact(Rect::from_center_size(centre, Vec2::splat(34.0)), ui.id().with(("tick", row.id)), Sense::click());
                    sky::tick(&painter, centre, row.ticked, hit.hovered());
                    if hit.clicked() && row.state == State::Ready {
                        row.ticked = !row.ticked;
                        circle_pressed = true;
                    }
                }
                if response.clicked() && !circle_pressed {
                    opened = Some(row.id);
                }
            } else {
                let next = first + slot == self.mods.len();
                sky::empty_tile(&painter, place, response.hovered() || (dropping && next));
                if next {
                    sky::text(&painter, place.center() - Vec2::new(0.0, 11.0), Align::Center, word(Word::DropMods), &bold(15.0), INK);
                    sky::text(&painter, place.center() + Vec2::new(0.0, 11.0), Align::Center, word(Word::ClickToChoose), &regular(12.5), SOFT);
                }
                if response.clicked() {
                    pick = true;
                }
            }
        }
        // Arrows at the sides when the mods fill more than a page.
        let middle = area.center().y;
        if self.page > 0 {
            let at = Pos2::new(area.left() + SIDE / 2.0, middle);
            let response = ui.interact(Rect::from_center_size(at, Vec2::splat(40.0)), ui.id().with("back"), Sense::click());
            sky::page_arrow(ui.painter(), at, true, response.hovered());
            if response.clicked() {
                self.page -= 1;
            }
        }
        if self.page + 1 < pages {
            let at = Pos2::new(area.right() - SIDE / 2.0, middle);
            let response = ui.interact(Rect::from_center_size(at, Vec2::splat(40.0)), ui.id().with("next"), Sense::click());
            sky::page_arrow(ui.painter(), at, false, response.hovered());
            if response.clicked() {
                self.page += 1;
            }
        }
        if pick {
            self.pick_mods();
        }
        if opened.is_some() {
            self.open = opened;
        }
    }

    /// A mod's own page: what it is, its options, and its buttons at the foot.
    fn mod_page(&mut self, ui: &mut egui::Ui, area: Rect, index: usize) {
        let row = &self.mods[index];
        let (state_word, color) = row.state.word();
        let inner = Rect::from_min_max(area.min + Vec2::new(SIDE, 6.0), area.max - Vec2::new(SIDE, 78.0));
        ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                ui.label(RichText::new(&row.name).font(bold(25.0)).color(INK));
                ui.label(RichText::new(state_word).font(bold(14.5)).color(color));
                let line = match &row.state {
                    State::CantConvert(why) | State::Failed(why) => why.clone(),
                    State::Losses(list) => format!("{list}\n\n{}", word(Word::BuildWithout)),
                    State::Done { summary, .. } => summary.clone(),
                    _ => row.about.clone(),
                };
                if !line.is_empty() {
                    ui.add_space(4.0);
                    ui.label(RichText::new(line).size(15.0).color(INK));
                }
                ui.add_space(8.0);
                if !row.options.is_empty() {
                    ui.label(RichText::new(word(Word::PlayersChoose)).size(15.0).color(INK));
                    for option in &row.options {
                        let names: Vec<&str> = option.choices.iter().map(|c| c.name.as_str()).collect();
                        ui.label(RichText::new(format!("{}: {}", option.name, names.join(", "))).size(14.5).color(SOFT));
                    }
                }
                ui.add_space(6.0);
                for (level, note) in &row.notes {
                    if *level == Level::Blocked {
                        continue;
                    }
                    ui.label(RichText::new(note).size(14.5).color(if *level == Level::Info { SOFT } else { INK }));
                }
            });
        });
        // Back, Remove and the page's main button, evenly spaced and centred.
        let size = Vec2::new(170.0, 50.0);
        let gap = 18.0;
        let y = area.bottom() - size.y - 10.0;
        let left = area.center().x - size.x * 1.5 - gap;
        let place = |i: usize| Rect::from_min_size(Pos2::new(left + i as f32 * (size.x + gap), y), size);
        let font = bold(16.5);
        if capsule_at(ui, place(0), "page-back", word(Word::Back), &font, true) {
            self.open = None;
        }
        let id = self.mods[index].id;
        let state = self.mods[index].state.clone();
        if capsule_at(ui, place(1), "page-remove", word(Word::Remove), &font, state != State::Converting) {
            self.mods.remove(index);
            self.open = None;
            return;
        }
        match state {
            State::Done { file, .. } => {
                if capsule_at(ui, place(2), "page-show", word(Word::ShowFile), &font, true) {
                    show_file(&file);
                }
            }
            State::Ready | State::Losses(_) => {
                if capsule_at(ui, place(2), "page-convert", word(Word::Convert), &font, true) {
                    self.convert(vec![id]);
                }
            }
            _ => {
                let _ = capsule_at(ui, place(2), "page-convert", word(Word::Convert), &font, false);
            }
        }
    }

    fn bar(&mut self, ui: &mut egui::Ui, screen: Rect) {
        let painter = ui.painter().clone();
        sky::bottom_bar(&painter, screen);
        let radius = sky::ROUND_RADIUS;
        let middle_y = screen.bottom() - radius - 14.0;
        let middle_x = screen.center().x;
        let label_font = regular(13.0);

        // The middle: the game these mods go with, with its button under it.
        let (line, color, font) = match (&self.game_problem, self.game.is_some()) {
            (Some(problem), _) => (problem.clone(), RED, regular(13.5)),
            (None, true) => (self.game_label.clone(), INK, bold(19.0)),
            (None, false) => (word(Word::ChooseYourGame).to_string(), INK, bold(19.0)),
        };
        let room = screen.width() - (radius * 2.0 + 150.0) * 2.0;
        let line = sky::fitted(&painter, &line, &font, room.max(120.0));
        sky::text(&painter, Pos2::new(middle_x, middle_y - 19.0), Align::Center, &line, &font, color);
        let change = Rect::from_center_size(Pos2::new(middle_x, middle_y + 17.0), Vec2::new(150.0, 32.0));
        if capsule_at(ui, change, "game", if self.game.is_some() { word(Word::ChangeGame) } else { word(Word::Choose) }, &bold(13.5), true) {
            if let Some(path) = rfd::FileDialog::new().add_filter(word(Word::GameFilter), &["pak"]).pick_file() {
                self.set_game(path);
            }
        }

        // Left: where they are saved.
        let left = Pos2::new(screen.left() + radius + 24.0, middle_y);
        let hit = ui.interact(Rect::from_center_size(left, Vec2::splat(radius * 2.0)), ui.id().with("save"), Sense::click());
        sky::round_button(&painter, left, radius, word(Word::SaveTo), &bold(14.5), hit.hovered(), true);
        let folder = self.output.file_name().map_or_else(|| self.output.display().to_string(), |n| n.to_string_lossy().to_string());
        let folder = sky::fitted(&painter, &folder, &label_font, 120.0);
        sky::text(&painter, left + Vec2::new(radius + 12.0, 0.0), Align::Min, &folder, &label_font, SOFT);
        let hit = hit.on_hover_text(self.output.display().to_string());
        if hit.clicked() {
            if let Some(chosen) = rfd::FileDialog::new().pick_folder() {
                self.output = chosen;
            }
        }

        // Right: everything ticked.
        let right = Pos2::new(screen.right() - radius - 24.0, middle_y);
        let ticked: Vec<u64> = self.mods.iter().filter(|m| m.state == State::Ready && m.ticked).map(|m| m.id).collect();
        let enabled = !ticked.is_empty();
        let hit = ui.interact(Rect::from_center_size(right, Vec2::splat(radius * 2.0)), ui.id().with("convert"), if enabled { Sense::click() } else { Sense::hover() });
        sky::round_button(&painter, right, radius, word(Word::Convert), &bold(14.5), hit.hovered() && enabled, enabled);
        if enabled {
            let count = match ticked.len() {
                1 => word(Word::OneMod).to_string(),
                n => counted(Word::Mods, n),
            };
            sky::text(&painter, right - Vec2::new(radius + 12.0, 0.0), Align::Max, &count, &bold(13.0), BLUE_INK);
        }
        if hit.clicked() && enabled {
            self.convert(ticked);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        self.take_events(now);
        if let Some(path) = self.screenshot.clone() {
            let image = ctx.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });
            if let Some(image) = image {
                let [w, h] = image.size;
                let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
                out.extend(image.pixels.iter().flat_map(|c| [c.r(), c.g(), c.b()]));
                let _ = std::fs::write(&path, out);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else if self.screenshot_convert && now > 1.0 && !self.mods.iter().any(|m| m.state == State::Reading) {
                self.screenshot_convert = false;
                let ticked: Vec<u64> = self.mods.iter().filter(|m| m.state == State::Ready && m.ticked).map(|m| m.id).collect();
                self.convert(ticked);
            } else if !self.screenshot_asked && !self.screenshot_convert && now > 1.0 && !self.mods.iter().any(|m| m.state == State::Reading || m.state == State::Converting) {
                self.screenshot_asked = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            }
        }
        let dropped: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect());
        for path in dropped {
            self.add_path(path);
        }
        // The tiles' stars twinkle; no more than this is needed.
        ctx.request_repaint_after(std::time::Duration::from_millis(50));

        let screen = ctx.screen_rect();
        sky::backdrop(&ctx.layer_painter(egui::LayerId::background()), screen);
        let dropping = ctx.input(|i| !i.raw.hovered_files.is_empty());
        egui::CentralPanel::default().frame(egui::Frame::new()).show(ctx, |ui| {
            let area = Rect::from_min_max(screen.min + Vec2::new(0.0, TOP), screen.max - Vec2::new(0.0, sky::BAR_HEIGHT + 44.0));
            let open = self.open.and_then(|id| self.mods.iter().position(|m| m.id == id));
            match open {
                Some(index) => self.mod_page(ui, area, index),
                None => {
                    self.open = None;
                    self.grid(ui, area, dropping, now);
                }
            }
            self.bar(ui, screen);
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.95, 0.96, 0.97, 1.0]
    }
}

fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("rounded".to_string(), std::sync::Arc::new(FontData::from_static(include_bytes!("../assets/MPLUSRounded1c-Medium.ttf"))));
    fonts.font_data.insert("rounded-bold".to_string(), std::sync::Arc::new(FontData::from_static(include_bytes!("../assets/MPLUSRounded1c-ExtraBold.ttf"))));
    fonts.families.insert(FontFamily::Name("bold".into()), vec!["rounded-bold".to_string(), "rounded".to_string()]);
    if let Some(body) = fonts.families.get_mut(&FontFamily::Proportional) {
        body.insert(0, "rounded".to_string());
    }
    // Korean and Chinese aren't in the rounded font: the system's stand behind it, each weight behind its own family.
    for (family, weight, bold) in [(FontFamily::Name("bold".into()), "system-bold", true), (FontFamily::Proportional, "system", false)] {
        for (i, (path, face)) in words::system_fonts(bold).into_iter().enumerate() {
            let Ok(bytes) = std::fs::read(&path) else { continue };
            let name = format!("{weight}{i}");
            let mut data = FontData::from_owned(bytes);
            data.index = face;
            fonts.font_data.insert(name.clone(), std::sync::Arc::new(data));
            if let Some(list) = fonts.families.get_mut(&family) {
                list.push(name);
            }
        }
    }
    if words::system_font_first() {
        // Every character of a line from one font, Latin letters too: the rounded font holds some Chinese characters.
        for family in [FontFamily::Proportional, FontFamily::Name("bold".into())] {
            if let Some(list) = fonts.families.get_mut(&family) {
                list.sort_by_key(|name| !name.starts_with("system"));
            }
        }
    }
    fonts
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default().with_title("LumaLink").with_inner_size([860.0, 640.0]).with_min_inner_size([700.0, 520.0]).with_drag_and_drop(true);
    if std::env::var_os("LUMALINK_SCREENSHOT").is_some() {
        // Out of the way and without taking the keyboard.
        viewport = viewport.with_position([-4000.0, -4000.0]).with_active(false).with_decorations(false);
    }
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "LumaLink",
        options,
        Box::new(|creation| {
            creation.egui_ctx.set_fonts(fonts());
            creation.egui_ctx.set_visuals(egui::Visuals::light());
            let mut style = (*creation.egui_ctx.style()).clone();
            style.visuals.panel_fill = Color32::TRANSPARENT;
            style.visuals.override_text_color = Some(INK);
            style.visuals.window_fill = Color32::WHITE;
            style.visuals.window_corner_radius = egui::CornerRadius::same(14);
            style.visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
            style.visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, sky::BLUE_EDGE);
            style.visuals.widgets.hovered.weak_bg_fill = Color32::WHITE;
            style.visuals.widgets.hovered.bg_stroke = Stroke::new(2.5, sky::BLUE);
            style.visuals.widgets.open.bg_stroke = Stroke::new(2.5, sky::BLUE);
            for widget in [&mut style.visuals.widgets.inactive, &mut style.visuals.widgets.hovered, &mut style.visuals.widgets.active, &mut style.visuals.widgets.open] {
                widget.corner_radius = egui::CornerRadius::same(15);
            }
            style.spacing.button_padding = Vec2::new(14.0, 7.0);
            creation.egui_ctx.set_style(style);
            Ok(Box::new(App::new()))
        }),
    )
}
