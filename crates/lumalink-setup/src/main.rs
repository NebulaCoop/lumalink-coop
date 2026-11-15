//! LumaLink Setup: installs LumaLink for the signed-in user, updates it, repairs it and removes it (install.rs), the
//! same way on Windows, Linux and macOS.
//!
//! With no option it opens its window: the folder, PATH, the menu entry, the desktop shortcut and the update check,
//! then Install (Update, Repair or Replace when a LumaLink is there), with Uninstall beside. From the command line:
//!
//! ```text
//! LumaLink-Setup --silent [--folder <folder>] [--no-path] [--no-menu] [--desktop-shortcut] [--no-update-check]
//! LumaLink-Setup --uninstall [--silent]
//! LumaLink-Setup --version
//! ```
//!
//! `--silent` installs with no window, with the last install's choices and the options given (exit code 0: done,
//! 1: failed, 2: a wrong command line). `--no-start-menu` is `--no-menu`'s Windows name. LumaLink's updates run the
//! new Setup with `--update --wait <process> [--open]`: it waits for that LumaLink to end, installs with the last
//! choices showing only its progress (with no display, as `--silent`), and opens the LumaLink window again when asked.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod install;

use eframe::egui::{self, pos2, vec2, Align, Color32, Id, Key, Rect, Vec2};
use install::{Choices, Found, Progress};
use lumalink_core::update::VERSION;
use lumalink_core::{say, text};
use lumalink_gui::chrome::{self, End, Mark};
use lumalink_gui::sky::{self, BLUE_INK, INK, RED, SOFT};
use lumalink_gui::{look, words};
use std::cmp::Ordering as Order;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::sync::Arc;

/// The version this Setup installs, written into its bytes as LumaLink's updates read it (update::setup_version):
/// checked against the release's version once the signature has passed. `--version` prints it from here, which also
/// keeps the linker from dropping it.
static VERSION_MARK: &str = concat!("LUMALINK-SETUP-VERSION=", env!("CARGO_PKG_VERSION"), ";");

/// The window's size in points: fixed, every page laid out in it.
const SIZE: Vec2 = vec2(640.0, 560.0);

/// What the command line asked for.
#[derive(Default)]
struct Args {
    silent: bool,
    update: bool,
    uninstall: bool,
    version: bool,
    open: bool,
    /// This is the copy an uninstall runs from the temporary folder (Windows); it removes itself at the end.
    copy: bool,
    wait: Option<u32>,
    folder: Option<PathBuf>,
    no_path: bool,
    no_menu: bool,
    desktop: bool,
    no_update_check: bool,
}

impl Args {
    fn read(list: &[String]) -> Result<Args, String> {
        let mut args = Args::default();
        let mut it = list.iter();
        // The menu entry's option by its name on this system.
        let menu = if cfg!(windows) { "--no-start-menu" } else { "--no-menu" };
        let bad = |arg: &str| say!("LumaLink.Setup.BadArgument", arg = arg, menu = menu);
        while let Some(arg) = it.next() {
            match arg.to_ascii_lowercase().as_str() {
                "--silent" => args.silent = true,
                // The usual installer switches on Windows.
                "/s" | "/silent" | "/quiet" if cfg!(windows) => args.silent = true,
                "--update" => args.update = true,
                "--uninstall" => args.uninstall = true,
                "--version" => args.version = true,
                "--open" => args.open = true,
                "--from-copy" => args.copy = true,
                "--no-path" => args.no_path = true,
                "--no-menu" | "--no-start-menu" => args.no_menu = true,
                "--desktop-shortcut" => args.desktop = true,
                "--no-update-check" => args.no_update_check = true,
                "--wait" => args.wait = Some(it.next().and_then(|p| p.parse().ok()).ok_or_else(|| bad(arg))?),
                "--folder" => {
                    let folder = PathBuf::from(it.next().ok_or_else(|| bad(arg))?);
                    if !folder.is_absolute() {
                        return Err(if cfg!(windows) {
                            say!("LumaLink.Setup.FolderNotFull", folder = folder.display())
                        } else {
                            say!("LumaLink.Setup.FolderNotFullSlash", folder = folder.display())
                        });
                    }
                    args.folder = Some(folder);
                }
                _ => return Err(bad(arg)),
            }
        }
        Ok(args)
    }

    /// The choices for a silent install: the last ones, then what the command line says.
    fn choices(&self) -> Choices {
        let mut choices = Choices::last();
        if let Some(folder) = &self.folder {
            choices.folder = folder.clone();
        }
        choices.path &= !self.no_path;
        choices.menu &= !self.no_menu;
        choices.desktop |= self.desktop;
        choices.check_for_updates &= !self.no_update_check;
        choices
    }
}

/// Output for a run from a terminal: on Windows the Setup has no console of its own.
fn attach_console() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
    }
}

/// Waits (at most a minute) for the process `pid` to end: the LumaLink an update replaces.
fn wait_for(pid: u32) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{OpenProcess, WaitForSingleObject};
        const SYNCHRONIZE: u32 = 0x0010_0000;
        unsafe {
            let process = OpenProcess(SYNCHRONIZE, 0, pid);
            if !process.is_null() {
                WaitForSingleObject(process, 60_000);
                CloseHandle(process);
            }
        }
    }
    #[cfg(unix)]
    {
        let Ok(pid) = libc::pid_t::try_from(pid) else { return };
        for _ in 0..600 {
            // Signal 0 only asks whether the process is there.
            if unsafe { libc::kill(pid, 0) } != 0 {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

/// The copies of the Setup that earlier uninstalls ran from the temporary folder. Windows keeps a running program's
/// file, so each stays after its uninstall; the next Setup removes them (as Windows' clean-up of the temporary folder
/// does), and no other program is started to do it.
#[cfg(windows)]
fn remove_earlier_copies() {
    let me = std::env::current_exe().unwrap_or_default();
    let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(UNINSTALL_COPY) && name.ends_with(".exe") && !same_path(&entry.path(), &me) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// The start of the name of the Setup's copy an uninstall runs from (`<temp>\LumaLink-Uninstall-<process>.exe`).
#[cfg(windows)]
const UNINSTALL_COPY: &str = "LumaLink-Uninstall-";

#[cfg(windows)]
fn same_path(a: &std::path::Path, b: &std::path::Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

/// Whether a window can open: on Linux only with a display (X11 or Wayland); a terminal over SSH has none.
fn has_display() -> bool {
    if cfg!(any(windows, target_os = "macos")) {
        return true;
    }
    ["DISPLAY", "WAYLAND_DISPLAY"].iter().any(|v| std::env::var_os(v).is_some_and(|d| !d.is_empty()))
}

fn main() -> ExitCode {
    text::set_language(words::language());
    let list: Vec<String> = std::env::args().skip(1).collect();
    let mut args = match Args::read(&list) {
        Ok(args) => args,
        Err(why) => {
            attach_console();
            eprintln!("{why}");
            return ExitCode::from(2);
        }
    };
    if args.version {
        attach_console();
        let marked = std::hint::black_box(VERSION_MARK);
        println!("LumaLink Setup {}", marked.trim_start_matches("LUMALINK-SETUP-VERSION=").trim_end_matches(';'));
        return ExitCode::SUCCESS;
    }
    if !args.silent && !has_display() {
        // An update started from a terminal installs there; anything else says how.
        if args.update {
            args.silent = true;
        } else {
            eprintln!("{}", say!("LumaLink.Setup.NoDisplay"));
            return ExitCode::from(2);
        }
    }
    #[cfg(windows)]
    remove_earlier_copies();
    if let Some(pid) = args.wait {
        wait_for(pid);
    }
    if args.uninstall { uninstall(&args) } else { install(&args) }
}

/// A silent run's end: its error on the terminal and exit code 1.
fn finished(result: anyhow::Result<()>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::from(1)
        }
    }
}

fn install(args: &Args) -> ExitCode {
    if args.silent {
        attach_console();
        let choices = args.choices();
        let result = install::install(&choices, &Meter::default());
        if result.is_ok() {
            println!("{}", say!("LumaLink.Setup.Done", version = VERSION));
            println!("{}", say!("LumaLink.Setup.DoneFolder", folder = choices.folder.display()));
        }
        return finished(result);
    }
    let mut app = App::new(args.choices());
    if args.update && install::installed().is_some() {
        app.passive = true;
        app.open_after = args.open;
        app.start_install();
    }
    window(app)
}

fn uninstall(args: &Args) -> ExitCode {
    let found = install::installed();
    // On Windows, from the install's folder: a copy in the temporary folder does it, which can remove this one.
    #[cfg(windows)]
    if let (Some(found), false) = (&found, args.copy) {
        let me = std::env::current_exe().unwrap_or_default();
        if me.starts_with(&found.folder) {
            let copy = std::env::temp_dir().join(format!("{UNINSTALL_COPY}{}.exe", std::process::id()));
            if std::fs::copy(&me, &copy).is_ok() {
                // The copy waits for this one to end: its file is the install's too.
                let mut command = std::process::Command::new(&copy);
                command.arg("--uninstall").arg("--from-copy").arg("--wait").arg(std::process::id().to_string());
                if args.silent {
                    command.arg("--silent");
                }
                if command.spawn().is_ok() {
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
    if args.silent {
        attach_console();
        let Some(found) = found else {
            println!("{}", say!("LumaLink.Setup.NotInstalled"));
            return ExitCode::SUCCESS;
        };
        let result = install::uninstall(&found);
        if result.is_ok() {
            println!("{}", say!("LumaLink.Setup.RemovedTitle"));
        }
        return finished(result);
    }
    let mut app = App::new(Choices::last());
    app.page = match found {
        Some(found) => Page::ConfirmRemove(found),
        None => Page::Failed { why: say!("LumaLink.Setup.NotInstalled"), retry: Retry::None },
    };
    window(app)
}

fn window(app: App) -> ExitCode {
    let viewport = egui::ViewportBuilder::default()
        .with_title(say!("LumaLink.Setup.Title"))
        .with_app_id(format!("{}.setup", look::APP_ID))
        .with_inner_size(SIZE)
        .with_resizable(false)
        .with_maximize_button(false)
        .with_icon(look::icon());
    let options = eframe::NativeOptions { viewport, centered: true, ..Default::default() };
    let result = eframe::run_native(
        "LumaLink Setup",
        options,
        Box::new(|creation| {
            look::setup(&creation.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    if result.is_err() { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

/// An install's progress, shared with the window.
#[derive(Default)]
struct Meter {
    done: AtomicU64,
    whole: AtomicU64,
}

impl Progress for Meter {
    fn step(&self, done: u64, whole: u64) {
        self.done.store(done, Ordering::Relaxed);
        self.whole.store(whole, Ordering::Relaxed);
    }
}

/// What to offer after a failure.
enum Retry {
    None,
    Install,
    Remove(Found),
}

enum Page {
    /// The choices and Install (Update, Repair, Replace), Uninstall, Cancel.
    Choose,
    ConfirmRemove(Found),
    Working { title: String, meter: Arc<Meter>, done: Receiver<Result<(), String>>, removing: bool },
    Installed,
    Removed,
    Failed { why: String, retry: Retry },
}

/// The window's measures: margins, the header, the bar along the bottom and the space between them.
struct Measure {
    full: Rect,
    /// The page's own space: inside the margins, between the header and the bar.
    content: Rect,
    margin: f32,
    header: f32,
    bar_top: f32,
    button: f32,
}

impl Measure {
    fn of(full: Rect) -> Measure {
        let (margin, header, bar, gap, button) = (28.0, 88.0, 76.0, 22.0, 46.0);
        let bar_top = (full.bottom() - bar).round();
        let content = Rect::from_min_max(pos2(full.left() + margin, full.top() + header + gap), pos2(full.right() - margin, bar_top - gap));
        Measure { full, content, margin, header, bar_top, button }
    }

    /// The middle of the bar, where its buttons stand.
    fn bar_middle(&self) -> f32 {
        ((self.bar_top + self.full.bottom()) / 2.0).round()
    }
}

/// A button along the bottom: its words and whether it is the page's main one (and red: it removes).
struct Button {
    label: String,
    primary: bool,
    danger: bool,
    enabled: bool,
}

impl Button {
    fn plain(label: String) -> Button {
        Button { label, primary: false, danger: false, enabled: true }
    }

    fn main(label: String) -> Button {
        Button { label, primary: true, danger: false, enabled: true }
    }
}

struct App {
    page: Page,
    choices: Choices,
    existing: Option<Found>,
    /// An update: only the progress, and the window closes when it is done.
    passive: bool,
    /// Open the LumaLink window when the update is done.
    open_after: bool,
    #[cfg(debug_assertions)]
    shot: Option<shot::Shot>,
}

impl App {
    fn new(choices: Choices) -> App {
        App {
            page: Page::Choose,
            choices,
            existing: install::installed(),
            passive: false,
            open_after: false,
            #[cfg(debug_assertions)]
            shot: shot::asked(),
        }
    }

    /// The version installed against this one: Less, an update; Equal, a repair; Greater, a replacement with an
    /// older version.
    fn against(&self) -> Option<Order> {
        self.existing.as_ref().map(|f| lumalink_core::update::compare_versions(&f.version, VERSION))
    }

    fn start_install(&mut self) {
        let title = match self.against() {
            Some(Order::Less) => say!("LumaLink.Setup.Updating", version = VERSION),
            _ => say!("LumaLink.Setup.Installing", version = VERSION),
        };
        let meter = Arc::new(Meter::default());
        let (sender, done) = channel();
        let (choices, m) = (self.choices.clone(), meter.clone());
        std::thread::spawn(move || {
            let _ = sender.send(install::install(&choices, m.as_ref()).map_err(|e| format!("{e:#}")));
        });
        self.page = Page::Working { title, meter, done, removing: false };
    }

    fn start_remove(&mut self, found: Found) {
        let (sender, done) = channel();
        std::thread::spawn(move || {
            let _ = sender.send(install::uninstall(&found).map_err(|e| format!("{e:#}")));
        });
        self.page = Page::Working { title: say!("LumaLink.Setup.Removing"), meter: Arc::new(Meter::default()), done, removing: true };
    }

    fn open_lumalink(&self) {
        let _ = std::process::Command::new(self.choices.folder.join(install::GUI)).current_dir(&self.choices.folder).spawn();
    }

    /// The header: LumaLink's badge, its name and what it is for.
    fn header(&self, ui: &egui::Ui, m: &Measure) {
        let painter = ui.painter();
        chrome::header(painter, Rect::from_min_size(m.full.min, vec2(m.full.width(), m.header)));
        let badge = 48.0;
        let middle = (m.header / 2.0).round();
        chrome::badge(painter, pos2(m.margin + badge / 2.0, middle), badge);
        let left = m.margin + badge + 16.0;
        let width = m.full.right() - m.margin - left;
        let name = chrome::fitted(painter, "LumaLink", chrome::bold(24.0), INK, width, 1, Align::Min);
        let tagline = chrome::fitted(painter, &say!("LumaLink.Setup.Tagline"), chrome::body(15.0), SOFT, width, 1, Align::Min);
        let block = name.size().y + 2.0 + tagline.size().y;
        let top = (middle - block / 2.0).round();
        let name_height = name.size().y;
        painter.galley(pos2(left, top), name, INK);
        painter.galley(pos2(left, top + name_height + 2.0), tagline, SOFT);
    }

    /// The bar along the bottom with `buttons` at its right, the last the rightmost; `left` at its left. Returns which
    /// was pressed (left's number is `buttons.len()`).
    fn bar(&self, ui: &mut egui::Ui, m: &Measure, buttons: &[Button], left: Option<Button>, notice: Option<&str>) -> Option<usize> {
        chrome::bar(ui.painter(), m.full, m.bar_top, 0.0, &[]);
        let y = m.bar_middle() - m.button / 2.0;
        let mut x = m.full.right() - m.margin;
        let mut pressed = None;
        for (i, b) in buttons.iter().enumerate().rev() {
            let width = chrome::big_button_width(ui.painter(), &b.label, m.button, 120.0);
            x -= width;
            let rect = Rect::from_min_size(pos2(x, y), vec2(width, m.button));
            let id = Id::new(("bar", i, &b.label));
            let response = if b.primary { chrome::primary_button(ui, rect, id, &b.label, b.enabled, b.danger) } else { chrome::big_button(ui, rect, id, &b.label, b.enabled) };
            if response.clicked() {
                pressed = Some(i);
            }
            x -= 12.0;
        }
        let left_edge = m.full.left() + m.margin;
        if let Some(b) = left {
            let width = chrome::big_button_width(ui.painter(), &b.label, m.button, 120.0);
            let rect = Rect::from_min_size(pos2(left_edge, y), vec2(width, m.button));
            if chrome::big_button(ui, rect, Id::new(("bar-left", &b.label)), &b.label, b.enabled).clicked() {
                pressed = Some(buttons.len());
            }
        } else if let Some(notice) = notice {
            // A notice in the space the buttons leave, in the middle of the bar's height.
            let width = (x - 12.0 - left_edge).max(80.0);
            let galley = chrome::fitted(ui.painter(), notice, chrome::body(13.0), RED, width, 3, Align::Min);
            let top = m.bar_middle() - galley.size().y / 2.0;
            ui.painter().galley(pos2(left_edge, top), galley, RED);
        }
        pressed
    }

    /// A page's title, with a round mark before it when there is one; returns its bottom.
    fn title(&self, ui: &egui::Ui, m: &Measure, top: f32, title: &str, color: Color32, mark: Option<Mark>) -> f32 {
        let painter = ui.painter();
        let r = 15.0;
        let left = m.content.left() + if mark.is_some() { 2.0 * r + 14.0 } else { 0.0 };
        let galley = chrome::fitted(painter, title, chrome::bold(21.0), color, m.content.right() - left, 2, Align::Min);
        let height = galley.size().y.max(if mark.is_some() { 2.0 * r } else { 0.0 });
        let middle = top + height / 2.0;
        if let Some(mark) = mark {
            chrome::mark(painter, pos2(m.content.left() + r + 1.5, middle), r, mark);
        }
        painter.galley(pos2(left, (middle - galley.size().y / 2.0).round()), galley, color);
        top + height
    }

    /// Words wrapped to the page's width at `top`; returns their bottom.
    fn words(&self, ui: &egui::Ui, m: &Measure, top: f32, words: &str, size: f32, color: Color32) -> f32 {
        let galley = ui.painter().layout(words.to_string(), chrome::body(size), color, m.content.width());
        let bottom = top + galley.size().y;
        ui.painter().galley(pos2(m.content.left(), top), galley, color);
        bottom
    }

    fn choose(&mut self, ui: &mut egui::Ui, m: &Measure) {
        let (title, action) = match (&self.existing, self.against()) {
            (Some(found), Some(Order::Less)) => (say!("LumaLink.Setup.UpdateTitle", old = found.version, version = VERSION), say!("LumaLink.Setup.UpdateButton")),
            (Some(_), Some(Order::Equal)) => (say!("LumaLink.Setup.RepairTitle", version = VERSION), say!("LumaLink.Setup.RepairButton")),
            (Some(found), Some(Order::Greater)) => (say!("LumaLink.Setup.ReplaceTitle", old = found.version, version = VERSION), say!("LumaLink.Setup.ReplaceButton")),
            _ => (say!("LumaLink.Setup.InstallTitle", version = VERSION), say!("LumaLink.Setup.InstallButton")),
        };
        let menu = if cfg!(windows) {
            say!("LumaLink.Setup.StartMenu")
        } else if cfg!(target_os = "macos") {
            say!("LumaLink.Setup.Applications")
        } else {
            say!("LumaLink.Setup.AppMenu")
        };
        // The title and the five rows, as one block in the middle of the page's space.
        let (row, tall, gap, under_title) = (48.0, 68.0, 10.0, 16.0);
        let rows_height = 4.0 * row + tall + 4.0 * gap;
        let title_height = ui.painter().layout_no_wrap(title.clone(), chrome::bold(21.0), INK).size().y;
        let block = title_height + under_title + rows_height;
        let top = (m.content.top() + ((m.content.height() - block) / 2.0).max(0.0)).round();
        let below_title = self.title(ui, m, top, &title, INK, None) + under_title;
        let width = m.content.width();
        let mut at = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(pos2(m.content.left(), below_title), vec2(width, rows_height))));
        at.spacing_mut().item_spacing = Vec2::ZERO;
        let ui_rows = &mut at;
        // An installed LumaLink stays where it is: Uninstall, then install elsewhere.
        let movable = self.existing.is_none();
        let folder = self.choices.folder.display().to_string();
        let color = if movable { BLUE_INK } else { SOFT };
        let folder_row = chrome::row(ui_rows, vec2(width, row), Id::new("folder"), &say!("LumaLink.Setup.Folder"), None, End::Value(folder, color), movable);
        // Its tip says what a click does.
        let folder_row = if movable { folder_row.on_hover_text(say!("LumaLink.Setup.Change")) } else { folder_row };
        if folder_row.clicked() {
            if let Some(picked) = rfd::FileDialog::new().set_title(say!("LumaLink.Setup.Folder")).set_directory(&self.choices.folder).pick_folder() {
                self.choices.folder = folder_for(&picked);
            }
        }
        ui_rows.add_space(gap);
        let tip = say!("LumaLink.Setup.PathTip");
        if chrome::row(ui_rows, vec2(width, tall), Id::new("path"), &say!("LumaLink.Setup.Path"), Some(&tip), End::Switch(self.choices.path), true).clicked() {
            self.choices.path = !self.choices.path;
        }
        ui_rows.add_space(gap);
        if chrome::row(ui_rows, vec2(width, row), Id::new("menu"), &menu, None, End::Switch(self.choices.menu), true).clicked() {
            self.choices.menu = !self.choices.menu;
        }
        ui_rows.add_space(gap);
        if chrome::row(ui_rows, vec2(width, row), Id::new("desktop"), &say!("LumaLink.Setup.Desktop"), None, End::Switch(self.choices.desktop), true).clicked() {
            self.choices.desktop = !self.choices.desktop;
        }
        ui_rows.add_space(gap);
        if chrome::row(ui_rows, vec2(width, row), Id::new("updates"), &say!("LumaLink.Setup.CheckForUpdates"), None, End::Switch(self.choices.check_for_updates), true).clicked() {
            self.choices.check_for_updates = !self.choices.check_for_updates;
        }
        let has_files = install::has_files();
        let empty = say!("LumaLink.Setup.Empty", setup = lumalink_core::update::package_name(VERSION));
        let left = self.existing.as_ref().map(|_| Button::plain(say!("LumaLink.Setup.Uninstall")));
        let buttons = [Button::plain(say!("LumaLink.Setup.Cancel")), Button { enabled: has_files, ..Button::main(action) }];
        let enter = has_files && ui.input(|i| i.key_pressed(Key::Enter)) && ui.memory(|mem| mem.focused().is_none());
        match self.bar(ui, m, &buttons, left, (!has_files).then_some(empty.as_str())) {
            Some(0) => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
            Some(1) => self.start_install(),
            Some(_) => {
                if let Some(found) = self.existing.clone() {
                    self.page = Page::ConfirmRemove(found);
                }
            }
            None if enter => self.start_install(),
            None => {}
        }
    }

    fn working(&mut self, ui: &mut egui::Ui, m: &Measure) {
        let Page::Working { title, meter, done, removing } = &self.page else { return };
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));
        // The title, the bar and the percentage, as one block in the middle.
        let (bar, under) = (16.0, 18.0);
        let painter = ui.painter();
        let title_galley = chrome::fitted(painter, title, chrome::bold(21.0), INK, m.content.width(), 2, Align::Min);
        let percent_height = painter.layout_no_wrap("0%".into(), chrome::body(14.0), SOFT).size().y;
        let block = title_galley.size().y + under + bar + 10.0 + percent_height;
        let top = (m.content.center().y - block / 2.0).round();
        let title_height = title_galley.size().y;
        painter.galley(pos2(m.content.left(), top), title_galley, INK);
        let bar_rect = Rect::from_min_size(pos2(m.content.left(), top + title_height + under), vec2(m.content.width(), bar));
        if *removing {
            let time = ui.input(|i| i.time);
            chrome::spinner(painter, pos2(m.content.left() + bar / 2.0 + 2.0, bar_rect.center().y), bar / 2.0, sky::BLUE, time);
        } else {
            let whole = meter.whole.load(Ordering::Relaxed);
            let fraction = if whole > 0 { meter.done.load(Ordering::Relaxed) as f32 / whole as f32 } else { 0.0 };
            let shown = ui.ctx().animate_value_with_time(Id::new("progress"), fraction, 0.2);
            chrome::progress(painter, bar_rect, shown);
            let percent = painter.layout_no_wrap(format!("{}%", (fraction * 100.0).floor() as u32), chrome::body(14.0), SOFT);
            painter.galley(pos2(bar_rect.right() - percent.size().x, bar_rect.bottom() + 10.0), percent, SOFT);
        }
        chrome::bar(ui.painter(), m.full, m.bar_top, 0.0, &[]);
        let next = match done.try_recv() {
            Ok(Ok(())) if *removing => Some(Page::Removed),
            Ok(Ok(())) => {
                if self.passive {
                    if self.open_after {
                        self.open_lumalink();
                    }
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
                self.existing = install::installed();
                Some(Page::Installed)
            }
            Ok(Err(why)) => Some(Page::Failed { why, retry: if *removing { self.existing.clone().map_or(Retry::None, Retry::Remove) } else { Retry::Install } }),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Page::Failed { why: say!("LumaLink.Setup.Stopped"), retry: Retry::None }),
        };
        if let Some(next) = next {
            self.page = next;
        }
    }

    fn installed(&mut self, ui: &mut egui::Ui, m: &Measure) {
        let painter = ui.painter();
        // What is shown, measured first so the block stands in the middle.
        let code = "lumalink --help";
        let code_galley = painter.layout_no_wrap(code.into(), egui::FontId::monospace(15.0), BLUE_INK);
        let pill = vec2(code_galley.size().x + 28.0, code_galley.size().y + 14.0);
        let say_path = say!("LumaLink.Setup.DonePath");
        let folder = say!("LumaLink.Setup.DoneFolder", folder = self.choices.folder.display());
        let lines = |text: &str, size: f32| painter.layout(text.to_string(), chrome::body(size), INK, m.content.width()).size().y;
        let title = say!("LumaLink.Setup.Done", version = VERSION);
        let title_height = painter.layout(title.clone(), chrome::bold(21.0), INK, m.content.width() - 44.0).size().y.max(30.0);
        let path_block = if self.choices.path { 18.0 + lines(&say_path, 15.0) + 8.0 + pill.y } else { 0.0 };
        let block = title_height + path_block + 18.0 + lines(&folder, 14.0);
        let top = (m.content.center().y - block / 2.0).round();
        let mut y = self.title(ui, m, top, &title, sky::GREEN, Some(Mark::Good));
        if self.choices.path {
            y = self.words(ui, m, y + 18.0, &say_path, 15.0, INK) + 8.0;
            let rect = Rect::from_min_size(pos2(m.content.left(), y), pill);
            let shape = chrome::rounded(rect, 10.0);
            chrome::fill(ui.painter(), &shape, chrome::SLOT, chrome::SLOT);
            chrome::outline(ui.painter(), &shape, egui::Stroke::new(1.0, chrome::SLOT_EDGE));
            ui.painter().galley(rect.center() - code_galley.size() / 2.0, code_galley, BLUE_INK);
            y = rect.bottom();
        }
        self.words(ui, m, y + 18.0, &folder, 14.0, SOFT);
        let buttons = [Button::plain(say!("LumaLink.Setup.Close")), Button::main(say!("LumaLink.Setup.Open"))];
        match self.bar(ui, m, &buttons, None, None) {
            Some(0) => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
            Some(_) => {
                self.open_lumalink();
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            None => {}
        }
    }

    /// A page of a title with its mark, words under it, and buttons: the confirmation, the end of an uninstall, a
    /// failure. The title is green when it is good news, red when it is bad. Returns the button pressed.
    fn message(&mut self, ui: &mut egui::Ui, m: &Measure, title: &str, mark: Mark, words: &str, buttons: &[Button]) -> Option<usize> {
        let color = match mark {
            Mark::Good => sky::GREEN,
            Mark::Warn => INK,
            Mark::Bad => RED,
        };
        let painter = ui.painter();
        let title_height = painter.layout(title.to_string(), chrome::bold(21.0), INK, m.content.width() - 44.0).size().y.max(30.0);
        let words_height = if words.is_empty() { 0.0 } else { 14.0 + painter.layout(words.to_string(), chrome::body(15.0), INK, m.content.width()).size().y };
        // Long words (a failure's) start under the title at the top and scroll; short ones stand in the middle.
        let block = title_height + words_height;
        let top = if block <= m.content.height() { (m.content.center().y - block / 2.0).round() } else { m.content.top() };
        let below = self.title(ui, m, top, title, color, Some(mark));
        if !words.is_empty() {
            let area = Rect::from_min_max(pos2(m.content.left(), below + 14.0), m.content.right_bottom());
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(area));
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(&mut child, |ui| {
                ui.add(egui::Label::new(egui::RichText::new(words).font(chrome::body(15.0)).color(INK)).wrap());
            });
        }
        self.bar(ui, m, buttons, None, None)
    }
}

/// The folder a picked folder means: itself when empty or a LumaLink's, else a LumaLink folder in it (so LumaLink's
/// files never mix with others, and Uninstall removes a folder of its own).
fn folder_for(picked: &Path) -> PathBuf {
    let empty = std::fs::read_dir(picked).map_or(true, |mut d| d.next().is_none());
    if empty || picked.join(install::CLI).is_file() {
        picked.to_path_buf()
    } else {
        picked.join("LumaLink")
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let full = ctx.screen_rect();
        sky::backdrop(&ctx.layer_painter(egui::LayerId::background()), full);
        let m = Measure::of(full);
        let working = matches!(self.page, Page::Working { .. });
        if !working && ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::CentralPanel::default().frame(egui::Frame::new()).show(ctx, |ui| {
            self.header(ui, &m);
            match &self.page {
                Page::Choose => self.choose(ui, &m),
                Page::Working { .. } => self.working(ui, &m),
                Page::Installed => self.installed(ui, &m),
                Page::ConfirmRemove(found) => {
                    let found = found.clone();
                    let title = say!("LumaLink.Setup.RemoveTitle", version = found.version);
                    let what = say!("LumaLink.Setup.RemoveWhat", folder = found.folder.display());
                    let buttons = [Button::plain(say!("LumaLink.Setup.Cancel")), Button { danger: true, ..Button::main(say!("LumaLink.Setup.Uninstall")) }];
                    match self.message(ui, &m, &title, Mark::Warn, &what, &buttons) {
                        Some(0) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                        Some(_) => self.start_remove(found),
                        None => {}
                    }
                }
                Page::Removed => {
                    if self.message(ui, &m, &say!("LumaLink.Setup.RemovedTitle"), Mark::Good, "", &[Button::main(say!("LumaLink.Setup.Close"))]).is_some() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                Page::Failed { why, retry } => {
                    let why = why.clone();
                    let again = match retry {
                        Retry::None => None,
                        Retry::Install => Some(None),
                        Retry::Remove(found) => Some(Some(found.clone())),
                    };
                    let buttons = if again.is_some() {
                        vec![Button::plain(say!("LumaLink.Setup.Close")), Button::main(say!("LumaLink.Setup.TryAgain"))]
                    } else {
                        vec![Button::main(say!("LumaLink.Setup.Close"))]
                    };
                    match self.message(ui, &m, &say!("LumaLink.Setup.FailedTitle"), Mark::Bad, &why, &buttons) {
                        Some(0) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                        Some(_) => match again {
                            Some(Some(found)) => self.start_remove(found),
                            Some(None) => self.start_install(),
                            None => {}
                        },
                        None => {}
                    }
                }
            }
        });
        #[cfg(debug_assertions)]
        if let Some(mut shot) = self.shot.take() {
            shot.frame(self, ctx);
            self.shot = Some(shot);
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.95, 0.96, 0.97, 1.0]
    }
}

#[cfg(debug_assertions)]
mod shot {
    //! Debug builds only: the window takes a picture of a page, for checking its layout. LUMALINK_SHOT is the .bmp to
    //! write; LUMALINK_SHOT_PAGE the page (`choose`, `confirm`, `working`, `removing`, `installed`, `removed`,
    //! `failed`), shown with made-up values and nothing installed or removed. The window closes once it is written.
    use super::*;

    pub struct Shot {
        file: PathBuf,
        frames: u32,
        asked: bool,
        /// Keeps a made-up job's channel open.
        _hold: Option<std::sync::mpsc::Sender<Result<(), String>>>,
    }

    pub fn asked() -> Option<Shot> {
        Some(Shot { file: std::env::var_os("LUMALINK_SHOT")?.into(), frames: 0, asked: false, _hold: None })
    }

    impl Shot {
        pub fn frame(&mut self, app: &mut App, ctx: &egui::Context) {
            self.frames += 1;
            if self.frames == 1 {
                let found = Found { folder: app.choices.folder.clone(), version: "1.0.0".into() };
                let (sender, done) = channel();
                let meter = Arc::new(Meter::default());
                meter.step(62, 100);
                app.page = match std::env::var("LUMALINK_SHOT_PAGE").as_deref() {
                    Ok("confirm") => Page::ConfirmRemove(found),
                    Ok("working") => Page::Working { title: say!("LumaLink.Setup.Installing", version = VERSION), meter, done, removing: false },
                    Ok("removing") => Page::Working { title: say!("LumaLink.Setup.Removing"), meter, done, removing: true },
                    Ok("installed") => Page::Installed,
                    Ok("removed") => Page::Removed,
                    Ok("failed") => Page::Failed { why: say!("LumaLink.Setup.CannotWrite", folder = app.choices.folder.display(), why = "Access is denied. (os error 5)"), retry: Retry::Install },
                    _ => Page::Choose,
                };
                self._hold = Some(sender);
            }
            if self.frames == 40 && !self.asked {
                self.asked = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            let image = ctx.input(|i| {
                i.raw.events.iter().find_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });
            if let Some(image) = image {
                if let Err(e) = lumalink_gui::picture::write_bmp(&self.file, &image) {
                    eprintln!("{e}");
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(args: &[&str]) -> Result<Args, String> {
        Args::read(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn the_command_line_is_read() {
        let folder = if cfg!(windows) { r"D:\Tools\LumaLink" } else { "/opt/tools/LumaLink" };
        let args = read(&["--silent", "--folder", folder, "--no-path", "--desktop-shortcut"]).unwrap();
        assert!(args.silent && args.no_path && args.desktop && !args.no_menu);
        assert_eq!(args.folder.as_deref(), Some(Path::new(folder)));
        assert!(read(&["--no-menu"]).unwrap().no_menu && read(&["--no-start-menu"]).unwrap().no_menu, "both names");
        let update = read(&["--update", "--wait", "1234", "--open"]).unwrap();
        assert!(update.update && update.open && update.wait == Some(1234));
        assert_eq!(read(&["/S"]).is_ok_and(|a| a.silent), cfg!(windows), "the usual installer switch, on Windows");
        assert!(read(&["--folder", "relative"]).is_err());
        assert!(read(&["--wait"]).is_err());
        let wrong = read(&["--frobnicate"]).err().unwrap();
        assert!(wrong.contains("--frobnicate") && wrong.contains("--silent"), "{wrong}");
    }

    #[test]
    fn a_picked_folder_with_other_files_gets_a_lumalink_folder() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(folder_for(dir.path()), dir.path());
        std::fs::write(dir.path().join("other.txt"), "x").unwrap();
        assert_eq!(folder_for(dir.path()), dir.path().join("LumaLink"));
        std::fs::write(dir.path().join(install::CLI), "x").unwrap();
        assert_eq!(folder_for(dir.path()), dir.path());
    }
}
