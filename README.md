<p align="center"><img src="assets/icon/lumalink.svg" width="128" height="128" alt="LumaLink icon"></p>

# LumaLink

Makes, checks, builds and packs mods for [Nebula](https://github.com/keepvaibin/nebula) (`.luma` files). It checks
with the game's own validator, so a mod LumaLink packs is one the game accepts.

## Install

LumaLink Setup installs the window, the command line, the compiler that builds mod code (clang and lld of LLVM
22.1.6) and LumaKit, for your account only (no administrator password). LumaLink finds the installed game, its SDK
and LumaKit by itself, so nothing else needs setting up.

**Windows.** Download `LumaLink-Setup.exe` (`LumaLink-Setup-windows-arm64.exe` on Windows on ARM) from the
[latest release](https://github.com/keepvaibin/lumalink/releases/latest) and run it.

**Linux** (x86-64 or ARM64, glibc 2.35 or later) **and macOS** (Apple silicon or Intel). In a terminal:

```sh
curl -fLO https://github.com/keepvaibin/lumalink/releases/latest/download/LumaLink-Setup-linux-x64
chmod +x LumaLink-Setup-linux-x64
./LumaLink-Setup-linux-x64
```

On ARM64 Linux, use `LumaLink-Setup-linux-arm64`; on a Mac, `LumaLink-Setup-macos-arm64` (Apple silicon) or
`LumaLink-Setup-macos-x64` (Intel). LumaLink is not signed by Apple: download it with `curl` as shown, since
macOS stops a copy downloaded with a browser.

Setup's choices:

| Choice | Default |
|---|---|
| Add LumaLink to PATH: `lumalink` works in any new terminal | on |
| Start menu shortcut (Windows), Applications menu entry (Linux), Add to Applications (macOS) | on |
| Desktop shortcut | off |
| Check for updates when LumaLink starts | on |

| | Windows | Linux | macOS |
|---|---|---|---|
| Installed in | `%LOCALAPPDATA%\Programs\LumaLink` | `~/.local/share/lumalink` | `~/Library/LumaLink`, with `~/Applications/LumaLink.app` |
| PATH | your user PATH | a line in each shell's startup file (`~/.profile`, `~/.bashrc`, `~/.zshenv`, fish's `conf.d`) that reads `env` in the folder | the same |
| Removing it | **Settings > Apps > Installed apps** | `LumaLink-Setup --uninstall` in the folder | the same |

Removing LumaLink leaves your mods where they are.

For scripts: `LumaLink-Setup --silent`, with `--folder <folder>`, `--no-path`, `--no-menu`, `--desktop-shortcut` or
`--no-update-check` as needed (`--no-start-menu` on Windows also works). `--uninstall --silent` removes LumaLink with no
window. On Linux, Setup with no display (over SSH) needs `--silent`.

## Updates

An installed LumaLink looks for a newer release by itself:

- **The window** checks when it opens, at most once an hour. A newer version shows a bar with **Update**,
  **What's new**, **Not now** and **Skip this version**. Turn the check off in the window's **Settings** menu.
- **The command line** checks at most once a day and prints one line after a command. It never prints during
  `lumalink lsp` or when its output goes to a file or another program.

`lumalink update` installs the newest version now, `lumalink update --check` only says whether there is one, and
`lumalink update --check-at-start off` turns the automatic check off. Set `LUMALINK_NO_UPDATE_CHECK=1` to turn it off
for one run.

An update downloads the release's Setup for your system and runs it only if it is signed with the LumaLink release key
and is the version the release names. Anything else is deleted without being run. Setup keeps your choices. A LumaLink
installed some other way (a package manager, a copy of the programs) is told about new releases but not updated.

## Commands

| Command | What it does |
|---|---|
| `lumalink new <folder> --id <id> --name <name> --summary <line> --author <who>` | Starts a mod: `luma.json` and an empty base layer |
| `lumalink add <what> ...` | Adds a need, option, permission, layer, file, asset, texture, text, character and more, checked before it is kept |
| `lumalink check <folder or .luma>` | Lists every problem with its code, file and line; `--json` for tools |
| `lumalink build <folder>` | Builds the mod's LumaScript and C for every platform |
| `lumalink pack <folder>` | Writes the `.luma` once every rule passes and none is left unchecked |
| `lumalink run <folder or .luma>` | Imports the mod into the installed game and starts it |
| `lumalink test <folder>` | Builds the mod and runs each `tests/**/test.json` in the game, hidden and silent |
| `lumalink convert <mod> <folder>` | Converts a Riivolution pack, disc-layout folder, loose files, texture pack or Gecko list to a mod folder |
| `lumalink format <paths>` | Formats `.lum` files; `--check` writes nothing |
| `lumalink list <.luma>` | What is inside |
| `lumalink extract <.luma> [entries] -o <folder>` | Writes entries out |
| `lumalink verify <.luma> --source <folder>` | Whether the `.luma` is exactly what the folder packs to |
| `lumalink explain L203` | What a code means |
| `lumalink list tables --game game.pak` | Tables, models, messages or files of your imported game; `--self` names table columns |
| `lumalink extract <path> --game game.pak -o <folder>` | Files out of your imported game |
| `lumalink lsp` | The editor server (Language Server Protocol over stdin and stdout) |
| `lumalink services` | Every service function of the game's SDK and whether scripts reach it |
| `lumalink update` | Installs a newer LumaLink; `--check` only says whether there is one |

Every command that reads the game uses the installed one unless `--game <game.pak>` and `--names <folder>` name
another; `--no-installed` leaves it out. Messages and help come in any of the game's 14 languages
(`--language EuGerman`, `lumalink help check --language JpJapanese`).

## Editors

`editors/vscode` is the LumaScript extension for Visual Studio Code: colours, brackets, snippets, and errors as you
type, completion, hover, go to definition, rename and formatting from `lumalink lsp`. To install it, open LumaLink,
then **Settings** > **Add LumaScript to Visual Studio Code**. Setup installs the extension packed
(`editors/lumascript.vsix`, made by `installer/vsix.py`); `code --install-extension` takes that file too. Other editors
run `lumalink lsp` with their own Language Server Protocol client; Nebula's modding docs have Neovim and Helix
settings.

## Building

LumaLink uses Nebula's validator and cooker crates. `nebula/` must be a Nebula tree: a checkout of the commit in
`nebula.commit`, or a link to a Nebula worktree.

```console
git clone https://github.com/keepvaibin/nebula.git nebula
git -C nebula checkout $(cat nebula.commit)
cargo test --workspace
```

A release: `installer/build-setup.ps1` on the Windows PC that holds the release key builds and signs
`LumaLink-Setup.exe`. Publishing the release with it and its `.sig` runs the release workflow, which builds the Linux
and macOS Setups (`installer/build-setup.sh`) and adds them. `installer/sign-release.ps1 -Tag <tag>` then signs those
on the same PC.

GPL-3.0-only.
