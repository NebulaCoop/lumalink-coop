//! Dev mode (docs/modding/lumalink/reload.md 4 and 5): a mod folder played in Nebula with `--mod-dev <id>`. Each
//! change of its source is built, put into the game's mods and reloaded in the running game, and LumaLink is told
//! how it went. A change to anything else (luma.json, content) shows at the game's next start.

use crate::jobs::{BuildLine, Game};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// What a watch saw since the last look.
pub enum Event {
    /// Its source changed: it is being built.
    Building,
    /// The build's lines, and whether it built.
    Built { lines: Vec<BuildLine>, ok: bool },
    /// The built mod did not go into the game's mods: why.
    NotInstalled(String),
    /// The game was asked to reload it (the request's number).
    Asked(u64),
    /// The game's answer.
    Reloaded(Answer),
    /// Files other than its source changed: they show at the next start.
    ContentChanged,
    /// The game ended: the watch is over.
    GameEnded,
}

/// The game's reload.result (reload.md 4).
#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    pub n: Option<u64>,
    pub ok: bool,
    /// Why it did not load again, in the game's words ("" when ok).
    pub why: String,
    pub took_out: Vec<String>,
    pub started: Vec<String>,
    pub frames: u64,
}

impl Answer {
    pub fn parse(text: &str) -> Option<Answer> {
        let v: serde_json::Value = serde_json::from_str(text).ok()?;
        let list = |key: &str| v.get(key).and_then(|l| l.as_array()).map(|l| l.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default();
        Some(Answer {
            n: v.get("n").and_then(|n| n.as_u64()),
            ok: v.get("ok")?.as_bool()?,
            why: v.get("why").and_then(|w| w.as_str()).unwrap_or_default().to_string(),
            took_out: list("took_out"),
            started: list("started"),
            frames: v.get("frames").and_then(|f| f.as_u64()).unwrap_or(0),
        })
    }
}

/// Each file's path, time and size: the source's, then everything else's (the built code, `code/`, and names starting
/// with a dot left out).
type Snapshot = (Vec<(PathBuf, Option<SystemTime>, u64)>, Vec<(PathBuf, Option<SystemTime>, u64)>);

fn snapshot(folder: &Path) -> Snapshot {
    let mut source = vec![];
    let mut other = vec![];
    let mut stack = vec![folder.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let name = entry.file_name();
            if name.to_string_lossy().starts_with('.') || (dir == folder && name == "code") {
                continue;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                stack.push(path);
                continue;
            }
            let row = (path.clone(), meta.modified().ok(), meta.len());
            if path.starts_with(folder.join("source")) { source.push(row) } else { other.push(row) }
        }
    }
    source.sort();
    other.sort();
    (source, other)
}

/// Builds a mod folder: its lines, and whether it built.
pub type Build<'a> = dyn FnMut(&Path) -> Result<(Vec<BuildLine>, bool)> + 'a;
/// Puts a mod folder into the game's mods: None, or why not.
pub type Put<'a> = dyn FnMut(&Path) -> Result<Option<String>> + 'a;

/// A mod folder played in the game in dev mode.
pub struct Watch {
    folder: PathBuf,
    id: String,
    /// <the game's mods>/.reload: the requests and results.
    reload: PathBuf,
    game: Game,
    seen: Snapshot,
    n: u64,
    /// A request written, its answer not yet read.
    waiting: Option<u64>,
    /// The source changed while a request stood: built once the answer is read.
    held: bool,
    ended: bool,
}

impl Watch {
    /// The folder as it is now is the one the game started with.
    pub fn new(folder: &Path, id: &str, mods: &Path, game: Game) -> Watch {
        let reload = mods.join(".reload");
        // An answer left from an earlier session is not this one's.
        let _ = std::fs::remove_file(reload.join(format!("{id}.result")));
        let _ = std::fs::remove_file(reload.join(format!("{id}.request")));
        Watch { folder: folder.to_path_buf(), id: id.to_string(), reload, game, seen: snapshot(folder), n: 0, waiting: None, held: false, ended: false }
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    /// One look (every half second or so): `build` builds the folder (its lines and whether it built), `install` puts it
    /// into the game's mods (None, or why not).
    pub fn poll(&mut self, build: &mut Build, install: &mut Put) -> Result<Vec<Event>> {
        let mut events = vec![];
        if self.ended {
            return Ok(events);
        }
        if self.game.ended() {
            self.ended = true;
            events.push(Event::GameEnded);
            return Ok(events);
        }
        if let Some(n) = self.waiting {
            let result = self.reload.join(format!("{}.result", self.id));
            let Ok(text) = std::fs::read_to_string(&result) else {
                if snapshot(&self.folder).0 != self.seen.0 {
                    self.held = true;
                }
                return Ok(events);
            };
            let _ = std::fs::remove_file(&result);
            match Answer::parse(&text) {
                Some(answer) if answer.n.is_none_or(|got| got == n) => {
                    self.waiting = None;
                    events.push(Event::Reloaded(answer));
                }
                Some(_) => {}  // an earlier request's answer
                None => {
                    self.waiting = None;
                    events.push(Event::Reloaded(Answer { n: Some(n), ok: false, why: text.trim().to_string(), took_out: vec![], started: vec![], frames: 0 }));
                }
            }
            if self.waiting.is_some() {
                return Ok(events);
            }
        }
        let now = snapshot(&self.folder);
        if now.0 != self.seen.0 || self.held {
            self.held = false;
            events.push(Event::Building);
            let (lines, ok) = build(&self.folder)?;
            // What the build wrote (its code, luma.json's platforms) is not a change.
            self.seen = snapshot(&self.folder);
            events.push(Event::Built { lines, ok });
            if !ok {
                return Ok(events);
            }
            if let Some(why) = install(&self.folder)? {
                events.push(Event::NotInstalled(why));
                return Ok(events);
            }
            self.n += 1;
            self.ask(self.n)?;
            self.waiting = Some(self.n);
            events.push(Event::Asked(self.n));
        } else if now.1 != self.seen.1 {
            self.seen = now;
            events.push(Event::ContentChanged);
        }
        Ok(events)
    }

    /// <id>.request written whole, then renamed into place.
    fn ask(&self, n: u64) -> Result<()> {
        std::fs::create_dir_all(&self.reload).with_context(|| self.reload.display().to_string())?;
        let part = self.reload.join(format!("{}.request.part", self.id));
        let request = self.reload.join(format!("{}.request", self.id));
        std::fs::write(&part, format!("{{\"n\": {n}}}\n")).with_context(|| part.display().to_string())?;
        std::fs::rename(&part, &request).with_context(|| request.display().to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_games_answer_is_read() {
        let a = Answer::parse(r#"{"n": 3, "ok": false, "took_out": ["a", "b"], "started": ["a"], "frames": 12, "why": "b: L506", "code": 506, "names": [], "values": []}"#).unwrap();
        assert_eq!(a, Answer { n: Some(3), ok: false, why: "b: L506".into(), took_out: vec!["a".into(), "b".into()], started: vec!["a".into()], frames: 12 });
        assert_eq!(Answer::parse(r#"{"n": null, "ok": true}"#).unwrap().n, None);
        assert!(Answer::parse("not json").is_none());
    }

    #[test]
    fn the_source_and_the_rest_are_told_apart() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path();
        std::fs::create_dir_all(f.join("source")).unwrap();
        std::fs::create_dir_all(f.join("code/windows-x64")).unwrap();
        std::fs::write(f.join("source/main.lum"), "a").unwrap();
        std::fs::write(f.join("luma.json"), "{}").unwrap();
        std::fs::write(f.join("code/windows-x64/mod.dll"), "x").unwrap();
        std::fs::write(f.join(".hidden"), "x").unwrap();
        let (source, other) = snapshot(f);
        assert_eq!(source.len(), 1);
        assert_eq!(other.iter().map(|r| r.0.file_name().unwrap().to_string_lossy().into_owned()).collect::<Vec<_>>(), vec!["luma.json"]);
    }
}
