# Contributing to LumaLink

## Reporting a mod that converts badly

Open an issue with:

- the mod's name and where it can be downloaded
- the `report.txt` from the converted file, or the output of `lumalink inspect`
- what you expected, and what happened in Nebula

Please don't attach game files or other people's mods.

## Adding a known code patch

A code patch has a native version when Nebula has a switch that makes the same change.

1. Note the mod's region, the patch's address and its bytes.
2. Add the switch to Nebula (`NEBULA_PATCH_*` in `nebula_game_api.h`) and the code it controls.
3. Add a row to `crates/lumalink-core/src/known_patches.rs`.

A patch to a table in the Wii program can often become data with no new switch at all: see how `known_patches.rs` reads the comet schedule and the table of what opens what, and add the table's place and the names its rows point at.

## Code

```console
cargo test
cargo clippy --all-targets
cargo fmt --check
```

- Everything LumaLink can't use must be named in the report with the reason. Nothing is dropped silently.
- A path from a mod or archive must never reach outside its folder.
- Tests build their own small files. Don't commit game files or mods.
- New user-facing wording should say what happened and what to do next.

By contributing, you agree that your work is licensed under GPL-3.0-only.
