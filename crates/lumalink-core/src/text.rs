//! LumaLink's own lines in the game's 14 languages, compiled from `text/LumaLinkText.tsv` by build.rs. The language
//! is chosen once, before any work ([`set_language`]); every line LumaLink says and every refusal it gives is made in
//! it with [`say!`](crate::say). The validator's findings come in the same 14 from its own table.

use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};

include!(concat!(env!("OUT_DIR"), "/lumalink_text.rs"));

static LANGUAGE: AtomicUsize = AtomicUsize::new(0);

/// The 14 languages, in the order of the text's columns and of [`say_in`]'s numbers.
pub use luma_validate::LANGUAGES;

/// Chooses the language of every later line; false (and nothing changed) when `name` is not one of the 14.
pub fn set_language(name: &str) -> bool {
    match luma_validate::LANGUAGES.iter().position(|l| *l == name) {
        Some(i) => {
            LANGUAGE.store(i, Ordering::Relaxed);
            true
        }
        None => false,
    }
}

/// The language chosen, by its name in the game (UsEnglish ... TwTradChinese).
pub fn language() -> &'static str {
    luma_validate::LANGUAGES[LANGUAGE.load(Ordering::Relaxed)]
}

/// The game's language for a language's name in the game or a system language code (`fr-CA`, `zh-Hant-TW`, `pt_BR`,
/// a POSIX locale such as `de_DE.UTF-8`); UsEnglish for any other.
pub fn language_for(wanted: &str) -> &'static str {
    if let Some(l) = luma_validate::LANGUAGES.iter().find(|l| l.eq_ignore_ascii_case(wanted)) {
        return l;
    }
    // A POSIX locale's encoding and modifier (`.UTF-8`, `@euro`) say nothing of the language.
    let code = wanted.split(['.', '@']).next().unwrap_or("").to_ascii_lowercase().replace('_', "-");
    let mut parts = code.split('-');
    let base = parts.next().unwrap_or("");
    let rest: Vec<&str> = parts.collect();
    let region = |r: &[&str]| rest.iter().any(|p| r.contains(p));
    match base {
        // The Americas' English is the US one; every other English the European one.
        "en" => if rest.is_empty() || region(&["us", "ca", "mx", "ph"]) { "UsEnglish" } else { "EuEnglish" },
        "fr" => if region(&["ca", "us"]) { "UsFrench" } else { "EuFrench" },
        // Spain's Spanish is the European one; every other the Americas' (as the game's own regions split them).
        "es" => if rest.is_empty() || region(&["es"]) { "EuSpanish" } else { "UsSpanish" },
        "de" => "EuGerman",
        "it" => "EuItalian",
        "nl" => "EuDutch",
        "pt" => "UsPortuguese",
        "ja" => "JpJapanese",
        "ko" => "KrKorean",
        "zh" => if region(&["hant", "tw", "hk", "mo"]) { "TwTradChinese" } else { "CnSimpChinese" },
        _ => "UsEnglish",
    }
}

/// Whether `id` has a row (for lines whose id is made from a name, as the command line's help is).
pub fn has(id: &str) -> bool {
    TEXT.iter().any(|(row, _)| *row == id)
}

/// The row `id` in the language at `language`, its `{name}`s unfilled.
pub fn template(language: usize, id: &str) -> Option<&'static str> {
    TEXT.iter().find(|(row, _)| *row == id).map(|(_, texts)| texts[language])
}

/// Every row's id.
pub fn ids() -> impl Iterator<Item = &'static str> {
    TEXT.iter().map(|(id, _)| *id)
}

/// The line `id` in the chosen language with each `{name}` filled. An id without a row, a placeholder left unfilled
/// or a value given that the line does not use is a LumaLink fault: tests fail on it (`text::tests` and every test
/// that reaches the line), and a release build shows the id rather than a half-made line.
pub fn say(id: &str, values: &[(&str, &dyn Display)]) -> String {
    say_in(LANGUAGE.load(Ordering::Relaxed), id, values)
}

/// [`say`] in the language at `language` in `luma_validate::LANGUAGES`.
pub fn say_in(language: usize, id: &str, values: &[(&str, &dyn Display)]) -> String {
    let Some((_, texts)) = TEXT.iter().find(|(row, _)| *row == id) else {
        debug_assert!(false, "{id}: no row in LumaLinkText.tsv");
        return id.to_string();
    };
    let template = texts[language];
    let mut out = String::with_capacity(template.len() + 32);
    let mut used = vec![false; values.len()];
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}') else { break };
        let name = &rest[open + 1..open + close];
        out.push_str(&rest[..open]);
        match values.iter().position(|(n, _)| *n == name) {
            Some(i) => {
                used[i] = true;
                out.push_str(&values[i].1.to_string());
            }
            None => {
                debug_assert!(false, "{id}: {{{name}}} is not given");
                return id.to_string();
            }
        }
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    debug_assert!(used.iter().all(|u| *u), "{id}: a value is given that the line does not use");
    out
}

/// `say!("LumaLink.CannotRead", path = p.display())`: the line in the chosen language (see [`say`]).
#[macro_export]
macro_rules! say {
    ($id:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::text::say($id, &[$((stringify!($name), &$value as &dyn ::std::fmt::Display)),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Every id the code of the four crates (the library, the command line, the window, Setup) says has a row, and every
    /// row is said somewhere (the help's in the lumalink crate's own test).
    #[test]
    fn every_id_has_a_row_and_every_row_is_said() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut said = BTreeSet::new();
        for dir in [root.join("lumalink-core/src"), root.join("lumalink/src"), root.join("lumalink-gui/src"), root.join("lumalink-setup/src")] {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let path = entry.unwrap().path();
                let code = path.extension().is_some_and(|e| e == "rs");
                if !code || path.ends_with("text.rs") {
                    continue;
                }
                let source = std::fs::read_to_string(&path).unwrap();
                let mut rest = source.as_str();
                while let Some(at) = rest.find("\"LumaLink.") {
                    let tail = &rest[at + 1..];
                    let end = tail.find('"').unwrap();
                    // An id ending in '.' is the start of ids made from names (the help's), and one with a '{' is
                    // made by format! (the script's); neither is a row.
                    if !tail[..end].ends_with('.') && !tail[..end].contains('{') {
                        said.insert(tail[..end].to_string());
                    }
                    rest = &tail[end..];
                }
            }
        }
        let rows: BTreeSet<String> = TEXT.iter().map(|(id, _)| id.to_string()).collect();
        let missing: Vec<_> = said.difference(&rows).collect();
        // The help's rows and placeholders are made from the commands', arguments' and values' names; the lumalink
        // crate checks each is used. The script's are made from the compiler's codes and phrases (`script::tests`).
        let unsaid: Vec<_> = rows.difference(&said).filter(|id| !id.starts_with("LumaLink.Help.") && !id.starts_with("LumaLink.Cli.Value.") && !id.starts_with("LumaLink.Script.")).collect();
        assert!(missing.is_empty(), "said without a row: {missing:?}");
        assert!(unsaid.is_empty(), "rows never said: {unsaid:?}");
    }

    #[test]
    fn a_system_language_picks_the_games() {
        for (code, language) in [
            ("en-US", "UsEnglish"),
            ("en-GB", "EuEnglish"),
            ("en_GB.UTF-8", "EuEnglish"),
            ("fr-ca", "UsFrench"),
            ("fr-FR", "EuFrench"),
            ("fr_FR@euro", "EuFrench"),
            ("es-MX", "UsSpanish"),
            ("es-ES", "EuSpanish"),
            ("de_DE.UTF-8", "EuGerman"),
            ("pt_BR", "UsPortuguese"),
            ("ja", "JpJapanese"),
            ("ja_JP.eucJP", "JpJapanese"),
            ("zh-cn", "CnSimpChinese"),
            ("zh-Hans-CN", "CnSimpChinese"),
            ("zh-TW", "TwTradChinese"),
            ("zh_TW.UTF-8", "TwTradChinese"),
            ("zh-hant", "TwTradChinese"),
            ("sv-se", "UsEnglish"),
            ("C", "UsEnglish"),
            ("POSIX", "UsEnglish"),
            ("C.UTF-8", "UsEnglish"),
            ("", "UsEnglish"),
            ("KrKorean", "KrKorean"),
        ] {
            assert_eq!(language_for(code), language, "{code}");
        }
    }

    #[test]
    fn fills_in_every_language() {
        for (i, lang) in luma_validate::LANGUAGES.iter().enumerate() {
            let line = say_in(i, "LumaLink.Summary", &[("errors", &2), ("notes", &1), ("unchecked", &0)]);
            assert!(!line.contains('{') && line.contains('2') && line.contains('1') && line.contains('0'), "{lang}: {line}");
        }
        // The other tests run in parallel and read the chosen language, so this one does not change it.
        assert!(!set_language("Klingon"));
    }
}
