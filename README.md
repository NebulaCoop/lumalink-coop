# LumaLink

LumaLink turns Super Mario Galaxy mods and texture packs into `.luma` files that [Nebula](https://github.com/keepvaibin/nebula), the native version of the game, runs directly.

One mod makes one file. That file works on every version of the game and on every device the mod has builds for. Nebula prepares its files itself, so a `.luma` made today keeps working on later versions of Nebula.

There's a window, for dropping mods on and ticking the ones to convert, and a command line that does the same.

## Quick start

Open LumaLink, drop a mod on it, and press Convert. Or:

```console
lumalink convert "Some Mod.zip" --game "path/to/game.pak"
```

This writes `Some Mod.luma` and says what happened to everything in the mod. Open the file in Nebula: Settings, Mods, Import.

`game.pak` is the game you imported into Nebula. LumaLink reads it to find where files named without a folder belong. Nothing from it goes into the converted file.

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
| Models with materials Nebula doesn't have | Nebula builds their shaders on the player's device, with a progress screen. LumaLink adds a head start if it has the compilers |
| A galaxy the mod adds | Listed in the file, and given its own saves |
| Known code patches | Turned into Nebula's native switch for the same change |
| Other code patches, `CustomCode`, `.gct` and `.dol` files | **Can't be converted.** Named in the report. The rest of the mod still applies |
| Mods for Super Mario Galaxy 2 | Refused with a clear message |

### Why Wii code can't be converted

Code patches and custom code are PowerPC instructions aimed at exact addresses in the Wii program. Nebula doesn't contain that program, so there is nothing for them to attach to. What such a patch does has to be written again, as a switch or a plugin. [`known_patches.rs`](crates/lumalink-core/src/known_patches.rs) lists the ones done so far.

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

`lumalink-core` is the library behind LumaLink, if you'd rather use it.

## Shader compilers

A `.luma` doesn't need compiled shaders: Nebula builds them on the player's device. For a head start, so that players don't wait, give LumaLink two compilers: Microsoft's [`dxc`](https://github.com/microsoft/DirectXShaderCompiler) and Nebula's `galaxy_native_spirv`. Put them in a `tools` folder beside the program, or name them in `LUMALINK_DXC` and `LUMALINK_SPIRV`.

## Building

You need [Rust](https://rustup.rs) 1.85 or newer and, on Windows, Visual Studio's C++ tools.

```console
git clone https://github.com/keepvaibin/lumalink.git
cd lumalink
cargo build --release
cargo test
```

The results are `target/release/lumalink` and `target/release/lumalink-gui`. LumaLink uses Nebula's own converter, the `nebula-import` crate, so a converted mod matches what Nebula expects. Reading a `.rar` uses your system's `tar` command, which Windows 10 and later, macOS and most Linux systems include.

## Layout

| Folder | What it is |
|---|---|
| `crates/lumalink-core` | The engine: archives, layouts, Riivolution, plans, packages, checks |
| `crates/lumalink` | The command line |
| `crates/lumalink-gui` | The window |
| `docs` | The guide, the file format and plugins |
| `examples` | Complete mods, none holding any of the game's files |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Reports of mods that convert badly are especially useful. Please send the mod's name and `report.txt`, not game files.

## License

GPL-3.0-only. See [LICENSE](LICENSE).

The window's font is M PLUS Rounded 1c, under the SIL Open Font License: see `crates/lumalink-gui/assets`.

LumaLink doesn't include any game files, and you need your own copy of the game. Super Mario Galaxy is a trademark of Nintendo. This project is not affiliated with or endorsed by Nintendo.
