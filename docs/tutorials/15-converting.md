# Converting a mod made for the Wii

Nothing about a mod has to change. Nothing in LumaLink knows any mod by name: each rule is about a kind of thing a mod can contain.

## In the window

Open LumaLink, drop a mod on it, tick it and press Convert. Each mod gets a page saying what was found, what was converted and anything that couldn't be. It runs in 11 languages.

## On the command line

```console
lumalink inspect "Some Mod.zip"
lumalink convert "Some Mod.zip" -o out --game path/to/game.pak
lumalink batch folder-of-mods -o out --game path/to/game.pak
```

`--game` is your imported game, Nebula's `game.pak`. Mods that name files only by their name need it, to find where each one goes.

## What it can read

| A mod that's | Works by |
|---|---|
| A zip, 7z or rar | Unpacked, wherever its folders are inside |
| Riivolution (`riivolution/*.xml`) | Each patch's files, folders and memory |
| Patch files for one version of the game, or one for each | Any of `RMGE`, `RMGP`, `RMGJ`, `RMGK`. All of a mod's versions become one file |
| A folder of the disc's folders (`StageData`, `ObjectData`...) | Files by place |
| Loose `.arc` files | Placed by name, with `--game` |
| A Dolphin texture pack | `tex1_` files, sorted into categories |
| Gecko codes (`.gct`, a Dolphin `RMGE01.ini`) | Each plain write read like a patch's |
| A changed `main.dol` | Compared to your game's own, with `--base`; each change read like a patch's |
| A whole changed disc (`.iso`, `.wbfs`, `.rvz`, an extracted folder) | Compared to your game's own, with `--base`; only what differs goes in |
| A disc patch (`.xdelta`, `.bps`, `.ips`, `.ppf`) | Apply it with its tool first, then give LumaLink the result and `--base` |

`--base` names your own `main.dol` or disc. On a device with Nebula installed, LumaLink finds it by itself, beside `game.pak`.

## Code patches

Nebula isn't the Wii's program, so a write over that program is read for what it changes in the game. The value comes out of the patch's own bytes, so any mod writing any value works.

| A write over | Becomes |
|---|---|
| The player's tables | [`player_constants`](../reference/player-numbers.md) |
| A number the game keeps in its code (health, lives, caps, timers) | [A setting](../reference/settings.md) |
| The object table, treasure boxes, display models | `object_changes` |
| A comet time limit hook, with its tables in spare memory | `comet_time_limits` |
| The hungry Lumas' table, each dome's comets | `event_flags`, `comet_schedule` |
| The galaxy table | `NebulaData/GalaxyInfoTable.bcsv` |

A write nothing here can read is listed under "code changes Nebula has no version of yet". The mod still converts and everything else in it works. See [What conversion reads](../REFERENCE.md#4-what-conversion-does).

## Popular Gecko codes

A popular code in a Dolphin `RMGE01.ini` is known by its name, and its writes are not read: the report names the mod
in `examples/` that does the same thing natively, or the Nebula setting that does. Each of those mods builds for every
device with `lumalink build examples/<name>` and becomes a file with `lumalink pack examples/<name>`.

| A code named like | Native way |
|---|---|
| Moon jump | `gecko-moon-jump` |
| Infinite health, invincible | `gecko-infinite-health` |
| Lives | `gecko-infinite-lives` |
| Star bits, coins | `gecko-max-star-bits` |
| Gravity | `gecko-low-gravity` |
| Rainbow | `gecko-rainbow-forever` |
| Power-ups never end, infinite power | `gecko-power-ups-last` |
| Fire, Bee, Boo or Ice Mario, power-up anywhere | `gecko-power-up-anywhere` |
| Timer | `gecko-freeze-timer` |
| Free camera | `gecko-free-camera` |
| FOV | `gecko-wide-fov` |
| HUD | `gecko-hide-hud` |
| Fall | `gecko-no-fall-death` |
| Slow motion, frame advance, speed hack | `gecko-slow-motion` |
| Position, coordinates | `gecko-position-display` |
| Freeze enemies | `gecko-freeze-enemies` |
| Stage select, level select, warp | `gecko-warp-anywhere` |
| Mario size, big, giant, tiny or mini Mario | `gecko-mario-size` |
| Spin cooldown, always spin, infinite spin | `gecko-no-spin-cooldown` |
| Star pointer, spin from the start | `gecko-spin-and-pointer` |
| Fast text, text speed, instant text | Text speed, in Nebula's settings |

## What it decides for you

- **Versions.** An option that only asks which version you have becomes layers by version, chosen by the game.
- **Options.** Choices that come to the same thing are one choice: "With SD save" and "With Wii save" become Off and On, since Nebula keeps saves itself. A mod whose one option is itself is always on.
- **Saves.** A mod that adds a galaxy keeps its own.
- **Empty files.** One where the game's own is empty too changes nothing: left out with a note. Any other the game leaves out as it starts, saying so in its log, and its own stays: an empty file on a console means a crash or silence, never something a modder meant.
- **Files the game has no use for.** Readmes, thumbnails, tools: not copied.

## Super Mario Galaxy 2 mods

Refused, saying so. Their levels need Galaxy 2's objects and engine.

## After converting

```console
lumalink check out/Some\ Mod.luma
```

Then open it in Nebula, and play. A mod can always be opened: `lumalink unpack "Some Mod.luma" -o folder`, and its `luma.json` can be edited by hand and `pack`ed again.

## If something didn't convert

Each mod's page says what. A line with an address is a write with no setting yet; the address and what it did are what's needed to add one. See [Messages](../REFERENCE.md#17-messages-and-what-they-mean).
