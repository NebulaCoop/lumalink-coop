//! Debug builds only: the window takes a picture of itself, for checking its layout at a size. LUMALINK_SHOT is the
//! .bmp to write; LUMALINK_SHOT_SIZE the window's size in points (`1024x700`); LUMALINK_SHOT_OPEN the tile to open
//! (its number from 0); LUMALINK_SHOT_VIEW `settings` or `languages`; LUMALINK_SHOT_CHECK runs Check on the open tile first;
//! LUMALINK_SHOT_ADD opens the Add mods choices.
//! The window closes once the picture is written.

use super::*;

pub struct Shot {
    file: PathBuf,
    frames: u32,
    asked: bool,
    /// When the last job ended (the screen's motions settle for a second).
    still: Option<f64>,
}

/// The size asked for, when a picture is.
pub fn size() -> Option<[f32; 2]> {
    let size = std::env::var("LUMALINK_SHOT_SIZE").ok()?;
    let (w, h) = size.split_once('x')?;
    Some([w.parse().ok()?, h.parse().ok()?])
}

pub fn asked() -> Option<Shot> {
    Some(Shot { file: std::env::var_os("LUMALINK_SHOT")?.into(), frames: 0, asked: false, still: None })
}

impl Shot {
    pub fn frame(&mut self, app: &mut App, ctx: &egui::Context) {
        self.frames += 1;
        if self.frames == 2 {
            if let Some(index) = std::env::var("LUMALINK_SHOT_OPEN").ok().and_then(|i| i.parse::<usize>().ok()) {
                app.chosen = app.items.get(index).map(|i| i.id);
            }
            match std::env::var("LUMALINK_SHOT_VIEW").as_deref() {
                Ok("settings") => app.view = screen::View::Settings,
                Ok("languages") => app.view = screen::View::Languages,
                _ => {}
            }
            app.add_menu = std::env::var_os("LUMALINK_SHOT_ADD").is_some();
        }
        if self.frames == 3 && std::env::var_os("LUMALINK_SHOT_CHECK").is_some() {
            if let Some(id) = app.chosen {
                app.start(id, Job::Check);
            }
        }
        let now = ctx.input(|i| i.time);
        self.still = if app.items.iter().any(|i| i.busy) { None } else { Some(self.still.unwrap_or(now)) };
        if self.frames >= 30 && self.still.is_some_and(|t| now - t > 1.0) && !self.asked {
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
