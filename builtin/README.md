# Built-in mods

NebulaCoop's own mods, as sm64coopdx has its built-in mods. Each folder is a mod project like those in
[examples](../examples), checked by `cargo test` (`every_builtin_mod_checks`) and packed with `lumalink pack` like any
other. NebulaCoop carries a copy of each one's `luma.json` (its `runtime/src/builtin_mods.inc`) and writes it into the
player's mods folder at every start, off until they turn it on. The "nebula-coop match" check compares the two: change
a mod here first, then copy its `luma.json` there.

| Mod | What it does |
|---|---|
| [character-movesets](character-movesets) | sm64coopdx's Character Movesets: Luigi gets Super Mario Galaxy's own moves back (a higher, floatier jump, less grip and a long sliding brake, a stronger stomp bounce, his Boo form's handling, a swim spin that costs oxygen). Without it he moves as Mario does. |

A built-in mod is a co-op mod (`"requires": ["coop"]`, packed as `.lumacoop`).
