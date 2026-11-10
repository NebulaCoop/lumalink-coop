# LumaLink

LumaLink turns Super Mario Galaxy mods and texture packs into `.luma` files that [Nebula](https://github.com/keepvaibin/nebula), the native version of the game, runs directly.

One mod makes one file, for every version of the game and every device. There's a window and a command line.

## Quick start

Open LumaLink, drop a mod on it, and press Convert. Or:

```console
lumalink convert "Some Mod.zip" --game "path/to/game.pak"
```

This writes `Some Mod.luma` and a report. In Nebula: Settings, Mods, Import.

`game.pak` is the game you imported into Nebula. LumaLink reads it to place loose files; none of it goes into the `.luma`.

## What you can give it

| You have | LumaLink handles it as |
|---|---|
| A `.zip`, `.7z` or `.rar`, or a plain folder | Unpacks it and finds what's inside, however its folders are arranged |
| A Riivolution mod (`riivolution/*.xml`) | Reads its patches and options. Options become choices players make in the game |
| Files laid out like the disc (`StageData`, `ObjectData`, ...) | Places each file where it goes |
| Loose files such as `Kuribo.arc` | Places each one where the game has a file of that name |
| A texture pack (`tex1_...png` or `.dds`, with or without `manifest.json`) | Keeps the textures, including mipmaps and wildcard names |

## What converts

| Content | Result |
|---|---|
| Levels, layouts, text and models | Converted automatically |
| Music and sound, including 4-bit ADPCM streams | Converted automatically |
| Texture packs | Carried over. Nebula sorts them into categories players can switch on and off |
| Models with materials Nebula doesn't have | Nebula builds their programs on the player's device, once, with a progress screen |
| A galaxy the mod adds | Listed in the file, and given its own saves |
| Known code patches and Gecko codes (`.gct`, Dolphin `.ini`) | Turned into the tunable or sample plugin that does the same |
| Other code patches, `CustomCode` and `.dol` files | **Not converted.** Named in the report; the rest of the mod applies |
| Mods for Super Mario Galaxy 2 | Refused |

### Why Wii code can't be converted

Code patches are PowerPC instructions for exact addresses in the Wii program, which Nebula doesn't contain. Their effect has to be rewritten as a tunable or a plugin. [`known_patches.rs`](crates/lumalink-core/src/known_patches.rs) and [`gecko.rs`](crates/lumalink-core/src/gecko.rs) list the ones done.

## Commands

```console
lumalink inspect <mod> [--game game.pak] [--json]     # say what a mod is, write nothing
lumalink convert <mod> [-o folder] [--game game.pak]  # convert one mod
lumalink batch <folder> [-o folder] [--game game.pak] # convert every mod in a folder
lumalink new <folder> [--name "My Mod"] [--plugin]    # start a new mod, with code if you ask
lumalink pack <folder> [-o folder] [--game game.pak]  # build a project into a .luma
lumalink check <folder or .luma> [--game game.pak]    # list every problem before you publish
lumalink unpack <file.luma> -o <folder>               # open a .luma to see how it's made
lumalink cook <file> --as <game path> -o <file>       # convert one file to test it
```

`inspect`, `batch` and `check` exit with code 2 if a mod can't be used.

## Making your own mod

```console
lumalink new "My Mod"
# put files where they go in the game under layers/base/files/, and textures under layers/base/textures/
lumalink check "My Mod"
lumalink pack "My Mod"
```

| To do this | Read |
|---|---|
| Make a mod, from the first file to a galaxy of your own | [docs/guide.md](docs/guide.md), with projects to copy in [examples](examples) |
| Everything a `.luma` can hold: options, regions, saves, new galaxies, hungry Lumas, text, shaders | [docs/format.md](docs/format.md) |
| Change what the game does, with your own code | [docs/plugins.md](docs/plugins.md) |
| Everything, in one page | [docs/REFERENCE.md](docs/REFERENCE.md) |
| Every page and the game's lists as one book | [docs/book.md](docs/book.md) |

`lumalink-core` is the library behind LumaLink, if you'd rather use it.

## Building

You need [Rust](https://rustup.rs) 1.85 or newer and, on Windows, Visual Studio's C++ tools.

```console
git clone https://github.com/keepvaibin/lumalink.git
cd lumalink
cargo build --release
cargo test
```

This builds `target/release/lumalink` and `target/release/lumalink-gui`. Conversion uses Nebula's `nebula-import` crate. `.rar` files need the system `tar` (Windows 10+, macOS, most Linux).

## Layout

| Folder | What it is |
|---|---|
| `crates/lumalink-core` | The engine: archives, layouts, Riivolution, plans, packages, checks |
| `crates/lumalink` | The command line |
| `crates/lumalink-gui` | The window |
| `docs` | The guide, the file format and plugins |
| `examples` | Complete mods, none holding any of the game's files |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). For a mod that converts badly, send its name and `report.txt`, not game files.

## License

GPL-3.0-only. See [LICENSE](LICENSE).

The window's font is M PLUS Rounded 1c, under the SIL Open Font License: see `crates/lumalink-gui/assets`.

LumaLink doesn't include any game files, and you need your own copy of the game. Super Mario Galaxy is a trademark of Nintendo. This project is not affiliated with or endorsed by Nintendo.
