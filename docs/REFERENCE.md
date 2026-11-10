# Nebula mods and LumaLink: the complete reference

Everything about making, converting, packaging and running mods for Nebula, in one place. [guide.md](guide.md) teaches by example; [format.md](format.md) and [plugins.md](plugins.md) are shorter versions of parts of this page.

## Contents

1. [The parts](#1-the-parts)
2. [Using a mod](#2-using-a-mod)
3. [LumaLink](#3-lumalink)
4. [What conversion does](#4-what-conversion-does)
5. [Code patches become game settings](#5-code-patches-become-game-settings)
6. [The `.luma` file](#6-the-luma-file)
7. [Files, models, textures, sound](#7-files-models-textures-sound)
8. [Levels and tables](#8-levels-and-tables)
9. [Cutscenes, movies, Lumas](#9-cutscenes-movies-lumas)
10. [Galaxies and progression](#10-galaxies-and-progression)
11. [Plugins: the interface](#11-plugins-the-interface)
12. [Plugins: writing a whole boss](#12-plugins-writing-a-whole-boss)
13. [Plugins: the game's headers and hooks](#13-plugins-the-games-headers-and-hooks)
14. [Limits that are gone](#14-limits-that-are-gone)
15. [Shaders](#15-shaders)
16. [Devices](#16-devices)
17. [Messages and what they mean](#17-messages-and-what-they-mean)
18. [Questions](#18-questions)

---

## 1. The parts

| Part | What it is | Where it runs |
|---|---|---|
| **Nebula** | The game, built to run natively. It reads `.luma` files | Every device |
| **LumaLink** | The converter and modding tool. Takes any Super Mario Galaxy mod as it was published and makes a `.luma` | A computer |
| **`.luma`** | One mod in one file, for every version of the game and every device | Anywhere |
| **Plugin** | A mod's own compiled code, inside a `.luma` | Inside Nebula |

A mod is made once. Nothing about it depends on one build of Nebula: the game prepares what it needs when the mod is first used, and again if its own needs change.

## 2. Using a mod

**On a computer.** Double-click a `.luma`, or Settings, Mods, Import. Nebula's settings open on the Mods page with the mod in the list. Turn it on and press Apply.

**A mod as you downloaded it** (zip, 7z, rar or folder): open LumaLink, drop it on the window, tick it, press Convert. Then open the `.luma`.

**On a phone.** Move the `.luma` to the phone and import it from the app. A raw mod folder can't be used on a phone: convert it on a computer first. The app says so.

**The list.** The mod higher on the list wins where two change the same file. Files that many mods add to (see [Adding to shared files](#adding-to-shared-files)) keep everyone's additions.

**Options.** A mod's options are on its row. A change takes effect at the next reset. Reset with mods changed restarts the game by itself.

**Code.** The game asks before a mod's code first runs, and again if that code changes. If the game crashes in a plugin, the report names the mod, and the next start offers to turn its code off.

**Saves.** A mod says which saves it plays on. A mod that adds a galaxy always gets its own, since a save's shape follows the list of galaxies. Your normal saves are never touched by a mod with saves of its own.

## 3. LumaLink

### The window

A grid of tiles, one per mod. Drop files or folders on it. Tick the ones you want and press Convert. Each mod has a page saying what was found, what was converted and anything that couldn't be. It runs in 11 languages.

### Commands

| Command | What it does |
|---|---|
| `lumalink inspect MOD` | Says what a mod is and what would happen to it. Writes nothing |
| `lumalink convert MOD [-o FOLDER] [--game FILE] [--game-id ID]` | Makes a `.luma` from a mod folder, `.zip`, `.7z` or `.rar` |
| `lumalink batch FOLDER -o FOLDER [--game FILE]` | Converts everything in a folder. One that fails doesn't stop the others |
| `lumalink new "NAME" [--plugin]` | Starts a project folder with a `luma.json` |
| `lumalink pack FOLDER [-o FOLDER]` | Builds a project into a `.luma`, checking it first |
| `lumalink check PATH` | Lists every problem with a project or a `.luma` |
| `lumalink unpack FILE -o FOLDER` | Opens a `.luma` into a project folder |
| `lumalink shader MODEL -o FOLDER` | Writes the pixel program Nebula makes for each material of a model (.bdl, .bmd, .glb, .gltf, or an archive), to start a hand-written one |
| `lumalink cook FILE` | Converts one file alone, to test a model, texture or track |

`--game` is your imported game, Nebula's `game.pak`. Mods that name files only by their name (loose `.arc` files with no folders) need it, to find where each one goes. `--game-id` picks the version to check against when you have no game file; a `.luma` works on every version anyway.

LumaLink needs no compilers. Nebula builds the programs a mod's materials need on the player's device (section 15).

## 4. What conversion does

A mod goes through these steps. Nothing in LumaLink knows any mod by name: each rule is about a kind of thing a mod can contain.

**Unpacking.** Zip (stored, deflate, bzip2, LZMA), 7z, rar. Paths that leave the folder, devices, links and bombs are refused.

**Finding the mod's shape.** Riivolution patch files (`riivolution/*.xml`, any case, BOM or not, any encoding), Dolphin texture packs (`tex1_` files with their game's folder), loose folders named as the game's (`StageData`, `ObjectData`, ...), and loose single archives, placed by name from your game. A folder that holds a mod is found wherever it is in the download.

**The game it's for.** `RMG` is this game. `SB4` is Super Mario Galaxy 2 and the mod is refused, saying so. A mod for Galaxy 2 needs Galaxy 2's game and engine.

**Patch files.** For each `<patch>`:

| In the file | Becomes |
|---|---|
| `<file>`, `<folder>` | Files in a layer, by their place in the game. A named folder a download doesn't bring is skipped, as on a console |
| `<memory>` | A game setting where it means one; see [section 5](#5-code-patches-become-game-settings) |
| `<savegame>` | The mod's save mode. `clone` is `copy` |
| A section's options | Options the player chooses in the game, unless only the version of the game differs |

**Versions.** An option that only asks which version you have becomes layers by version, chosen by the game. A mod with one patch file per version (`mod_E.xml`, `mod_P.xml`) is one mod: the file for the version LumaLink's maps know says what it does, and the others add only the files their version needs (a European text folder, for example).

**Options.** Choices that come to the same thing are one choice: a mod offering "With SD save" and "With Wii save" becomes Off and On, since Nebula keeps saves itself. A mod whose one option is itself is always on. A mod with options all off and nothing else starts with the first on.

**Files.** Each is checked. A file empty in the mod is left out, with a note, so the game's stays: an empty file on a console means a crash or silence, never something a modder meant. Files the game has no use for (readmes, thumbnails, tools) are not copied.

**Materials.** Models a mod brings are converted as Nebula will, and the report counts the programs their materials use.

**Writing.** A zip with `luma.json` first. The result is checked by the same code `lumalink check` uses.

**What you see.** Each mod gets: files found and converted; gameplay changes that work; code changes with no version yet; good to know; didn't make it.

## 5. Code patches become game settings

Console mods change the game by writing over its program. Nebula isn't that program, so each write is read for what it changes in the game and becomes a setting Nebula has natively. There is no emulation, and no code from a mod's patch ever runs.

The rule: **a write is matched by where it is in the game, and its value is read out of the bytes.** Any mod writing any value there works. A mod patching a place with a different meaning stays on the "code changes" list instead of being guessed at.

### Settings

Written into a layer's `tunables` (and `"requires": ["tunables"]`), each the game's own tuning number by
`"<file stem>.<name>"` (or `.<function>.<k>`, `.<function>.<call>.<k>`). The [settings reference](reference/settings.md) has every one with the game's value.

| Tunable | Read from the write | What it does |
|---|---|---|
| `GameRules.cPlayerHealth` | The number | The health and most health a player starts with (3) |
| `MarioActor.cDarkCometOneHealth` | false | Daredevil comets don't change the player's health |
| `MarioActor.cHitTakesAll` | true | A hit takes all the player's health |
| `GameRules.cLifeUpHealth` | The number | The most health a Life Up gives (6). 0: none |
| `MarioEffect.cLowHealthWarning` | false | No warning sound at low health |
| `EventUtil.cDarkCometRules` | false | A Daredevil comet is a comet like any other |
| `FlipPanel.cPanelGroupSize`, `FlipPanel.cObserverGroupSize` | The number | A flip panel group's size |
| `WaterBazooka.cShotInvalidSpinKill` | true | A spin doesn't break the water bazooka's shots |
| `BossKameckBarrier.cOffsetY` | The float the bits hold | Kamek's barrier's height |
| `SkeletalFishBaby.cBreakAirBubble` | false | No air bubble from a hit skeletal fish |
| `GameRules.cDodoryuShellOnly` | true | Dodoryu is hurt by shells alone |
| `SequenceUtil.cMissStarPieceNum` | Signed number | Star bits added after a miss (-999: all lost) |
| `SceneUtil.cHiddenStarTable` | false | Hidden stars without their table |
| `SceneUtil.cSuddenDeathDodoryu` | false | No sudden-death Dodoryu |
| `GameRules.cBegomanFireProof` | true | Begomans survive fire ground |
| `KinopioAstro.cMessangerAlwaysEx` | true | The 1-up messenger gives its larger supply |
| `KinopioAstro.sMessangerOneUpExNum` | The number | How many 1-ups that is (20) |

### Tables

| Write over | Reads as |
|---|---|
| The game's object table | [`object_changes`](#object-changes): zeros are an object gone; another row's pointers are one object becoming the other |
| A treasure box's item numbers | `object_changes` in `treasure_boxes` |
| The display model lookup | `object_changes` in `display_models` |
| A comet time limit hook, with a table in spare memory | [`comet_time_limits`](#comet-time-limits): LumaLink follows the hook into the mod's own code and reads its tables |
| The hungry Lumas' table | `event_flags` |
| Each dome's comet table | `comet_schedule` |
| The galaxy table | `NebulaData/GalaxyInfoTable.bcsv` |

A value kept in a file (`valuefile`) reads the same as one written in the patch. Spare memory (0x80001000 to 0x80003000) is held until the whole patch is read, then whatever a hook used is accounted for and anything else stays on the list.

### What's left over

A write nothing here can read is listed under "code changes Nebula has no version of yet". The mod still converts and everything else in it works. Each one left over means a new setting: add a row where the game keeps it, and a place in the game that reads it. Never a rule about one mod.

## 6. The `.luma` file

A zip, with `luma.json` as its first entry. Any zip tool can make one.

```text
My Mod.luma
├── luma.json                what the mod is
├── icon.png                 optional, square, 256x256 or smaller
├── layers/
│   └── base/
│       ├── files/           files by their place in the game
│       ├── merge/           what it adds to files many mods share
│       ├── textures/        replacement textures
│       └── movies/          replacement videos
├── save/                    optional, starting files for its own saves
└── plugins/                 optional, the mod's code
```

### luma.json

```json
{
  "format": 1,
  "id": "my-mod",
  "name": "My Mod",
  "version": "1.0.0",
  "game": "RMG",
  "authors": ["Someone"],
  "description": { "default": "What it does.", "ja": "内容。" },
  "layers": [ { "path": "layers/base" } ]
}
```

| Field | Needed | Meaning |
|---|---|---|
| `format` | yes | `1`. A later number gets THIS MOD NEEDS A NEWER NEBULA |
| `id` | yes | Lower-case letters, digits, dashes. A file with the same `id` is an update of it. Reserved names are refused |
| `name` | yes | Text |
| `version` | yes | Numbers with dots, compared number by number |
| `game` | yes | `RMG`. `SB4` and `SB5` are refused as Galaxy 2, others as another game |
| `versions` | no | The versions it works on: `E`, `P`, `J`, `K`, `W`, `C`. Left out: all |
| `nebula` | no | The oldest Nebula it works on |
| `requires` | no | Format features it can't work without, such as `custom-shaders` |
| `authors`, `license`, `homepage` | no | Shown on its page |
| `description` | no | Text |
| `icon` | no | A PNG in the file |
| `needs` | no | `[{ "id": "other", "version": "1.2.0" }]` |
| `conflicts` | no | `[{ "id": "other", "below": "2.0.0" }]`. Without `below`: every version |
| `load_after`, `load_before` | no | Where it goes in the list when first added. The player's order then wins |
| `options` | no | What the player can choose |
| `layers` | yes | What the mod does |
| `save` | no | Which saves it plays on |
| `galaxies`, `music`, `event_flags`, `comet_schedule`, `text`, `plugins` | no | See their sections |
| `tool` | no | What made it. Not used by the game |

### Text

Anything shown to players is a string, or languages with a `default`:

```json
"name": { "default": "Green Stars", "ja": "グリーンスター", "fr": "Étoiles vertes" }
```

Languages: `en`, `en-GB`, `fr`, `fr-CA`, `es`, `es-MX`, `de`, `it`, `nl`, `pt`, `ja`, `ko`, `zh-Hans`, `zh-Hant`. The game takes its own language, then that language without its region, then `default`.

### Layers

A layer is a folder and when to use it. Layers apply in order; a later layer's file replaces an earlier one's. Everything that differs by version, language, device or option is a layer, so one file carries it all.

| Field | Meaning |
|---|---|
| `path` | Its folder. May be left out for a layer of settings alone |
| `tunables` | The game's tuning numbers by `"<file stem>.<name>"`, with values ([settings](reference/settings.md)) |
| `object_changes` | Objects changed or gone |
| `object_classes` | New object names, each made as one of the game's: `{ "name": "Bramblet", "like": "Kuribo" }`, its files `ObjectData/<name>.arc` ([tutorial](tutorials/08-new-objects.md)) |
| `player_constants` | The numbers of how the player moves, by name ([player numbers](reference/player-numbers.md)) |
| `player_animations` | New player animations from one of the game's or the layer's `ObjectData/MarioAnime.arc`: `{ "name": "GoldFlowerSpin", "file": "FireSpin", "rate": 1.5, "blend_frames": 3 }`; `rate` over 0 up to 100, `blend_frames` up to 600. The mod says `"requires": ["player-animations"]`. A plugin plays one by name |
| `sound_effects` | A sound the game has, by name, played from the layer's file: `{ "name": "SE_PV_JUMP_S", "file": "AudioRes/Stream/Hup.wav" }`; `@luigi` after the name for Luigi's own ([tutorial](tutorials/04-music-and-sound.md)) |
| `stages` | Rules for stages ([stage rules](reference/stage-rules.md)) |
| `text` | Messages for this layer alone, as the top-level `text` |
| `comet_time_limits` | Comet missions' time limits |
| `materials` | Hand-written shaders by material name |
| `when` | When it's used. Left out: always |

`when` may hold any of these, and all given must hold. A key this Nebula doesn't know makes it false.

| Key | Holds when |
|---|---|
| `version` | The game is one of these: `["E", "P"]` |
| `language` | The player's language folder is one of these: `["UsEnglish", "EuEnglish"]` |
| `platform` | The device is one of `windows`, `android`, `linux`, `macos`, `ios` |
| `option` | Each named option is on one of its listed choices: `{ "lives": ["one"] }` |

### Object changes

One of the game's objects becomes another, or is gone. It applies wherever the game looks the name up, so every level gets it.

```json
"object_changes": [
  { "object": "RestartCube" },
  { "object": "BenefitItemLifeUp", "becomes": "BenefitItemOneUp" },
  { "object": "KinokoLifeUp", "becomes": "KinokoOneUp", "in": "treasure_boxes" },
  { "object": "KinokoLifeUp", "becomes": "KinokoOneUp", "in": "display_models" }
]
```

| Key | Meaning |
|---|---|
| `object` | Its name in the game |
| `becomes` | What's made instead, with that object's files. Left out: it's gone, and levels placing it carry on without it |
| `in` | `placed` (the default): anything made by name. `treasure_boxes`: what a box holds (`Coin`, `YellowChip`, `BlueChip`, `KinokoOneUp`, `KinokoLifeUp`, `AirBubble`). `display_models`: the still model shown before a thing is collected |

The mod highest on the list wins where two change one object. Removing `RestartCube`, for example, removes every checkpoint in the game.

### Comet time limits

```json
"comet_time_limits": [ { "galaxy": "BattleShipGalaxy", "mission": 4, "seconds": 150 } ]
```

A mission here is timed on Cosmic and Daredevil comets, and this comes before the game's own table. Without a row, the game's own limit applies.

### Options

| Field | Meaning |
|---|---|
| `id` | Lower-case letters, digits, dashes |
| `name`, `description` | Text |
| `default` | A choice's `id` |
| `choices` | Each `{ "id", "name" }`. `off` is shown in the player's language and means this adds nothing |

### Saves

```json
"save": { "mode": "own", "tag": "kaizo", "path": "save", "player_can_change": false }
```

| Field | Meaning |
|---|---|
| `mode` | `shared`: normal saves. `own`: its own, new or from `path`. `copy`: its own, starting as a copy of the player's, which stay untouched |
| `tag` | Mods with one tag share saves. Left out: the `id` |
| `path` | A folder in the file, laid out as a save folder |
| `player_can_change` | The player may choose on the Mods page |

Saves of its own live per tag, outside the mod, so they outlast updating or removing it.

### Adding to the format

A reader ignores keys it doesn't know. A new key a mod can't work without goes with a name in `requires`, and a Nebula without that name refuses the mod rather than half-applying it. `format` changes only if neither can hold. Nebula reads every format it has ever read.

## 7. Files, models, textures, sound

### Files

`files/` holds files at the paths the game knows them by, in any case. New paths are new files.

A folder named as an archive changes that archive file by file, and the rest comes from the player's own game, so a mod carries only its own work:

```text
layers/base/files/ObjectData/Kuribo.arc/kuribo.bdl
```

### Modern files

Turned into the game's own forms when the player applies the mod.

| Put in | Becomes | Notes |
|---|---|---|
| `.glb`, `.gltf` | Model `.bdl`/`.bmd`; per animation `.bck` (joints), `.bpk` (material colour), `.btk` (texture matrix), `.btp` (texture swaps) | glTF 2. Triangles and skins, up to the game's joints. One colour and one PNG or JPEG per material, its `KHR_texture_transform` a texture matrix. Signed, normalized and sparse numbers (`KHR_mesh_quantization`). A metre is 100 units. The material's name is what `materials` uses. `KHR_animation_pointer` moves `baseColorFactor` and the base texture's `KHR_texture_transform`; `extras.nebula.texture_swaps` swaps textures. An animation loops unless its `extras.nebula.play` is `once`, `once_then_first`, `there_and_back` or `loop_there_and_back`. A file requiring any other extension is refused by name |
| `.kcl.glb` | Solid ground `.kcl` and its surfaces `.pa` | Each material's `extras` are its surface's columns (`Floor_code`, `Wall_code`, `Sound_code`, `Camera_id`, `Camera_through`) |
| `.png` | `.bti`; in an archive's `timg` folder, `.tpl` | Any size |
| `.wav` | Music stream `.ast` | 16-bit PCM. Loop from the file's `smpl` chunk; without one the whole track loops |
| `.ogg` | Music stream `.ast` | Vorbis. Loop from `LOOPSTART` with `LOOPEND` or `LOOPLENGTH` |
| `.bcsv.json` | A table | See [Levels and tables](#8-levels-and-tables) |
| `StageData/<zone>.arc/jmp/.../<table>.json`, `.bcam.json` | A zone's placement table, its cameras | The same JSON |

### Textures

| To change | Do this |
|---|---|
| A texture wherever it's used | A pack: `tex1_` files in `layers/base/textures/`, from any Dolphin pack |
| A file in an object's archive | `files/ObjectData/Thing.arc/name.png` |
| A picture in the interface | `files/LayoutData/CoinCounter.arc/timg/name.png` |

### Music

To replace a track, put a file with its name in `files/AudioRes/Stream/`. A new track is a file and a row:

```json
"music": [{
  "name": "MyTheme",
  "file": "AudioRes/Stream/MyTheme.ogg",
  "stages": [
    { "stage": "MyGalaxy" },
    { "stage": "MyGalaxy", "scenario": 3 },
    { "stage": "MyGalaxy", "change_area": 1 }
  ]
}]
```

`name` is yours; names starting `SE_`, `BGM_`, `MBGM_`, `STM_`, `ME_` are the game's. `scenario` is one mission; without it, every mission without its own row. `change_area` is what a `ChangeBgmCube` with that `Obj_arg1` starts. A row here comes before the game's own music for that stage, comets and fortresses included. A plugin plays a track by this name.

### Sound effects

With any mod on, every sound effect in the game is ready in every stage, so an object placed or spawned away from its home stage still sounds right. Replacing a sound is a file at its place in `AudioRes/Waves` or in the object's archive. An object's sounds come from its archive's sound table, played by the action or by a plugin with `NEBULA_PLAY_SOUND`.

### Messages

```json
"text": {
  "GalaxyName_MyGalaxy": { "default": "My Galaxy", "fr": "Ma Galaxie" },
  "GalaxyNameShort_MyGalaxy": "My Galaxy",
  "ScenarioName_MyGalaxy1": "The First Star"
}
```

New or in place of the game's, in as many languages as you like. Any number of mods can add messages, which replacing the message files doesn't allow. If a mod sets text in some language folders and not the one being played, the game shows the mod's text from the same language of another region, then English, then what it has.

## 8. Levels and tables

### Adding to shared files

A file in `files/` replaces the whole file, so only one mod's survives. A layer's `merge/` folder adds to it, and every mod's additions are kept. Paths go inside archives.

```text
layers/base/merge/StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo.json
layers/base/merge/ParticleData/Effect.arc/AutoEffectList.bcsv.json
layers/base/merge/ParticleData/Effect.arc/ParticleNames.bcsv.json
layers/base/merge/ParticleData/Effect.arc/Particles.jpc
```

```json
{ "rows": [ { "name": "Coin", "l_id": 900, "pos_x": 0, "pos_y": 100, "pos_z": 0, "scale_x": 1, "scale_y": 1, "scale_z": 1 } ] }
```

A column is named, or written as `0x` and its hash. One left out is zero or empty. In placement tables, "no switch" and "no argument" are `-1`: write those out, `"SW_A": -1`.

`Particles.jpc` holds only new particles. Each one's number is its place in `ParticleNames.bcsv.json`, from 0. Nebula renumbers them and their textures when adding them.

### New tables

`Name.bcsv.json` in `files/` makes a table. Each column's kind comes from its values. Say it yourself for a float whose values are whole, or a column with no values yet:

```json
{ "columns": { "Speed": "float" }, "rows": [ { "Name": "Fast", "Speed": 2 } ] }
```

A column whose name nobody knows is named by its hash, as `lumalink extract` writes it: `"0xe4ef3738"`.

### Placing objects

Levels use the game's rows and names, from a level editor such as Whitehole. Every column of an object's row reaches a plugin's object by its name through `placement_number` and `placement_text` (`Obj_arg0` to `Obj_arg7`, `ShapeModelNo`, `MessageId`...); `SW_A`, `SW_B`, `SW_APPEAR`, `SW_DEAD` through its switches; `CommonPath_ID` as its rail.

## 9. Cutscenes, movies, Lumas

### In-game cutscenes

A cutscene is data: a `DemoGroup` placed in a level, and its sheets in `ObjectData/DemoSheet.arc`. No code is needed.

| File | What it does |
|---|---|
| `files/ObjectData/DemoSheet.arc/demoNAME/demoNAMEtime.bcsv.json` | Its parts and how long each lasts, in frames |
| `...player.bcsv.json` | The player's animation in each part |
| `...sound.bcsv.json` | Music and sounds in each part |
| `...wipe.bcsv.json` | Fades |
| `...camera.bcsv.json` | Cameras |
| `...action.bcsv.json` | What each object does |
| `...subpart.bcsv.json` | Parts within parts |
| `merge/StageData/.../demoobjinfo.json` | The `DemoGroup`: `DemoName`, `TimeSheetName`, `SW_APPEAR` |
| `merge/StageData/.../areaobjinfo.json` | A `SwitchCube` that sets that switch when the player walks in |

Leave out any sheet you don't need. A wrong name in a sheet does nothing and the cutscene carries on.

### Movies

A video in `layers/base/movies/`, or a `.thp` in `files/MovieData/`, replaces a movie. A plugin plays one in a stage, over the game, with `play_movie`.

### Lumas

Each kind is placed by name.

| Name | What it is | In its row |
|---|---|---|
| `Tico`, `TicoBaby` | A Luma that talks | `MessageId` |
| `TicoComet` | A Luma for the comet | |
| `TicoShop` | Sells for star bits | `Obj_arg0`: price |
| `TicoFat` | Hungry, gives an item | `Obj_arg0`: 0 1-up, 1 life mushroom. `Obj_arg1`: how much it eats |
| `TicoGalaxy` | Hungry, opens a galaxy | `Obj_arg1`: how much. `Obj_arg7`: slot. `CommonPath_ID`: where it flies. A `GroupId` shared with a launch star |
| `TicoRail` | Moves along a path | `CommonPath_ID` |
| `TicoReading` | Listens in the library | |
| `TicoStarRing` | Launches to a galaxy | `Obj_arg0`: which |
| `TicoAstro` | In the Observatory by green stars | `Obj_arg0`, `Obj_arg2` |
| `TicoDomeLecture` | The dome's lecture | In a `DemoGroup` |
| `CollectTico` + `StrayTico` | Gather lost Lumas for a star | One group, with a `PowerStar` in the mission. `CollectTico` needs its group: alone it costs a life |

## 10. Galaxies and progression

### A new galaxy

Its files, `StageData/<name>/<name>Scenario.arc` and `StageData/<name>.arc`, and a row:

```json
"galaxies": [{
  "name": "MyGalaxy",
  "map_position": [-60, -100],
  "dome": 1,
  "stars_to_open": 0,
  "open_conditions": [],
  "hungry_luma": { "slot": 7, "star_bits": 400 }
}],
"save": { "mode": "own" }
```

| Key | Meaning |
|---|---|
| `name` | Its folder's name |
| `map_position` | `[right, up]` from the middle of its dome's page, or the grand map with no dome. The page is 608 by 456. The game makes the icon |
| `map_pane` | A pane of `MapGrandGalaxy` showing it, instead |
| `dome` | 1 to 6; 7 to 16 for domes the mod builds ([tutorial](tutorials/09-new-galaxy.md#domes-past-six)). 0: the grand map |
| `stars_to_open` | Power stars to open |
| `open_conditions` | Up to three event flags |
| `hungry_luma` | Placed as `TicoGalaxy` with `Obj_arg7` at `slot`. The game's are 0 to 6, and using one takes its place. `star_bits` is the price |

A galaxy with no `GalaxyName_` message shows an empty name. A planet of its own needs a `PlanetMapDataTable` row ([tutorial](tutorials/09-new-galaxy.md#a-planet)).

### The game's progression

```json
"event_flags": [
  { "name": "ExOpen1CocoonExGalaxy", "kind": 3, "a": 1, "b": 0, "first": "SandClockGalaxy" },
  { "name": "StarPieceCocoonExGalaxy", "kind": 8, "a": 5, "b": 80, "first": "CocoonExGalaxy" }
],
"comet_schedule": [
  { "dome": 2, "comets": [ { "galaxy": "BattleShipGalaxy", "coins": 1 } ] }
]
```

| Kind | The row says | `a`, `b` | `first`, `second` |
|---|---|---|---|
| 1 | How many stars | `a`: the number | |
| 2 | A galaxy's stars to open it | | `first`: the galaxy |
| 3 | A special star was got | `a`: which | `first`: its galaxy |
| 4 | Two other rows are both on | | `first`, `second`: the rows |
| 8 | A hungry Luma | `a`: slot. `b`: price | `first`: what it opens |

Where galaxies are on the map and their star counts are the table `NebulaData/GalaxyInfoTable.bcsv` (or `.bcsv.json`), the one table of Nebula's a layer can replace. A mod that changes these is a different journey through the game: give it its own saves.

## 11. Plugins: the interface

A plugin is a native library: `.dll`, `.so`, `.dylib`. The game loads it and calls it directly. **It is not sandboxed**: it can do what the game can do. Players are asked before it first runs.

```json
"plugins": [{
  "api": 2,
  "binaries": { "windows-x64": "plugins/my.dll", "linux-x64": "plugins/my.so" },
  "when": { "option": { "mode": ["hard"] } }
}]
```

Devices: `windows-x64`, `windows-arm64`, `linux-x64`, `linux-arm64`, `macos-x64`, `macos-arm64`, `android-arm64`, `android-x64`.

```c
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
}
```

```console
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -I <nebula>/native/include -o plugins/my.dll my.c
cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/my.so my.c
```

These calls work only in `nebula_plugin_start`: `subscribe`, `register_object`, `register_creator`, `hook`, and the rows (`add_*`, `set_*_music`, `set_event_flag`, `set_comet_schedule`, `set_object_change`, `set_comet_time_limit`). Calls about the player do nothing, and return 0 or null, between stages and on the file select screen.

### Events

`subscribe(event, callback, user)`. Return `data->value` to change nothing. An event nobody subscribed to costs one load and one branch.

| `NEBULA_MOD_EVENT_` | When | `value` | You return |
|---|---|---|---|
| `FRAME` | Every frame, menus too | | |
| `STAGE_FRAME` | Every frame of a stage, before objects move | | |
| `STAGE_START` | A stage starts. `name`: the stage | Its mission | |
| `PLAYER_DAMAGE` | About to lose health | 1 | 0 refuses the hit |
| `PLAYER_MISS` | About to die | How | |
| `PLAYER_SPIN` | A spin starts | | |
| `POWER_UP` | The power-up is about to change | The new one | Used instead |
| `COIN` | Coins about to be added | How many | Added instead |
| `STAR_BIT` | Star bits about to be added | How many | Added instead |
| `STAR_GET` | A star is won. `name`: the galaxy | Its number | |
| `CUTSCENE_START`, `CUTSCENE_END` | `name`: the cutscene | | |
| `ACTOR_APPEAR`, `ACTOR_KILL` | Any object. `actor`, `name` | | |

### Calls

**General**

| Call | What it does |
|---|---|
| `log(self, text)` | A line in the game's log |
| `option(self, id)` | The player's choice for one of the mod's options: its choice id, or null |
| `store_get(self, key, missing)`, `store_set(self, key, value)` | A number kept for this mod across plays |
| `stage_name()` | The stage, or null |

**The player**

| Call | What it does |
|---|---|
| `player()` | The player as an actor, or null |
| `value(what)` | A number, below |
| `set_value(what, value)` | Life (at 0 or below, a miss; capped at the most), most life (1 or more), power-up (any power-up, anywhere), coins, star bits, lives |
| `kill_player()`, `jump_player(velocity)` | |
| `player_front(xyz)` | The way the player faces |
| `set_player_control(on)` | Stops or gives back the player's control |
| `buttons_held()`, `buttons_pressed()` | `NEBULA_BUTTON_JUMP`, `SPIN` (held while it lasts, pressed the frame it starts), `CROUCH`, `CAMERA`, `FIRE` |
| `stick(x, y)` | |

`NEBULA_VALUE_`: `LIFE`, `MAX_LIFE`, `POWER_UP`, `ON_GROUND`, `STAR_BITS`, `COINS`, `LIVES`, `MISSION`, `POWER_STARS`, `WATER_LIFE`, and 0 or 1: `SPINNING`, `JUMP_RISING`, `DAMAGED`, `DEAD`, `GROUND_POUNDING`, `SKATING`, `CROUCHING`, `FLYING`, `SWIMMING`, `LUIGI`.

Power-up numbers: 0 normal, 1 invincible, 2 fire, 3 ice, 4 bee, 5 spring, 6 Boo, 7 red star, 9 tornado, and whatever `add_power_up` answers.

**Any actor**

| Call | What it does |
|---|---|
| `get_position`, `set_position`, `get_velocity`, `set_velocity`, `get_rotation`, `set_rotation`, `get_scale`, `set_scale`, `get_gravity` | Three floats |
| `appear(actor)`, `kill(actor)`, `is_dead(actor)`, `show_model(actor, show)` | |
| `actor_name(actor)` | |
| `spawn(name, xyz)` | Makes any object, the mod's or the game's, on any stage. Its files load now if the stage hasn't them. Null if there's no such object |
| `play(actor, what, name)` | Below. 1 if it started |
| `is_animation_done(actor)`, `animation_frame`, `animation_length`, `set_animation_frame`, `set_animation_rate` | The animation last started |
| `stop_effect(actor, name)` | Null: all |
| `stop_sound(actor, name)` | |
| `is_on_ground(actor)`, `is_touching_wall(actor)` | |
| `distance_to_player(actor)` | |
| `get_joint_position(actor, joint, xyz)` | |
| `has_switch(actor, which)`, `is_switch_on`, `set_switch` | `NEBULA_SWITCH_A`, `B`, `APPEAR`, `DEAD` |
| `has_rail(actor)`, `move_on_rail(actor, speed)`, `rail_at_end`, `rail_turn_back`, `rail_coord`, `set_rail_coord`, `rail_length` | |

`NEBULA_PLAY_`: `ANIMATION` (its action: animations, effects and sounds together), `EFFECT`, `SOUND`, `PLAYER_ANIMATION`, and one part of a model alone, leaving the others: `SKELETON` (.bck), `TEXTURE_MOVE` (.btk), `COLOUR` (.brk), `SHOWN_PARTS` (.bva), `TEXTURE_FRAMES` (.btp), `MATERIAL_COLOUR` (.bpk).

**Objects of your own**

```c
static NebulaObjectClass boss = {
    sizeof(NebulaObjectClass),
    "LittleBoss",    /* the name levels place */
    "LittleBoss",    /* ObjectData/LittleBoss.arc; null: nothing to see */
    150.0f,          /* what it touches with */
    0,               /* your pointer, passed back */
    start, update, touch_player, attacked, attacked_by,
};
game->register_object(&boss);
```

A name the game already uses takes that object's place. Each callback can be null.

| Callback | When |
|---|---|
| `start(actor, user)` | Once, as it's made |
| `update(actor, user)` | Every frame |
| `touch_player(actor, user)` | The player is touching it |
| `attacked(actor, user)` | The player attacked it. Return 1 if it took the attack |
| `attacked_by(actor, how, user)` | The same, saying how, in place of `attacked`: `NEBULA_ATTACK_OTHER`, `SPIN`, `STOMP`, `GROUND_POUND`, `STAR_BIT`, `FIRE`, `INVINCIBLE`, `SHELL` |

| Call | What it does |
|---|---|
| `set_state(actor, n)`, `state(actor)`, `state_frames(actor)` | Its step in what it's doing. Starts at 0. `state_frames` counts from the `set_state` that began it |
| `set_front(actor, xyz)`, `get_front`, `turn_to_player(actor, degrees)` | The way it faces, standing against gravity on any planet. Its model turns with this, not its rotation |
| `set_touchable(actor, touchable)` | |
| `hit_player(actor, how)` | `NEBULA_HIT_NORMAL`, `STRONG`, `FIRE`, `ELECTRIC`, `FREEZE`, `FLIP` |
| `placement_number(actor, column, &value)`, `placement_text(actor, column)` | A column of its row in the level by name (`"Obj_arg0"`; -1 where the level sets none). 0 or null: no such column, or not placed by the level |
| `declare_item(actor, what, count)` | In `start`: `NEBULA_ITEM_COINS`, `STAR_BITS`, `POWER_STAR` (count: its number, -1 the next) |
| `appear_item(actor, what, xyz, count)` | The moment it comes out |

**The world**

| Call | What it does |
|---|---|
| `raycast(from, to, hit)` | 1 and where, if the line meets the map |
| `is_in_water(xyz)` | |
| `shake_camera()`, `shake_camera_by(how)` | `NEBULA_SHAKE_WEAK`, `NORMAL`, `NORMAL_STRONG`, `STRONG`, `VERY_STRONG` |
| `start_camera(name, frames)`, `end_camera(name, frames)` | An event camera from the level |
| `fade_out(frames)`, `fade_in(frames)` | |
| `start_cutscene(actor, name)` | 1 when it began. Another may be playing: ask again next frame |
| `end_cutscene(actor, name)`, `is_cutscene_playing()` | |
| `show_message(text, centre)`, `hide_message()` | |
| `change_stage(galaxy, mission)`, `back_to_dome()` | |
| `is_event_flag_on(name)`, `set_event_flag_on(name)`, `has_star(galaxy, star)` | |

**Music and movies**: `add_music(name, file)`, `set_stage_music(stage, scenario, name)`, `set_area_music(stage, area, name)`, `play_music(name)`, `stop_music(frames)`, `play_movie(file)`, `stop_movie()`, `is_movie_playing()`.

**Rows at start**: `add_galaxy`, `add_hungry_luma(galaxy, slot, star_bits)`, `add_text(id, utf8)`, `add_power_up`, `set_event_flag`, `set_comet_schedule`, `set_object_change(where, object, becomes)`, `set_comet_time_limit(galaxy, mission, seconds)`.

### New power-ups

```c
static NebulaPowerUpClass super_jump = { sizeof(NebulaPowerUpClass), "SuperJump", 1800, "BGM_MUTEKI_A", 0, 0, start, update, end };
```

`add_power_up` answers its number; give it with `set_value(NEBULA_SET_POWER_UP, number)`, on any stage. `frames`: how long it lasts, at 60 a second; 0 until something ends it. `music`: while it lasts. `color_frame`: a frame of `Mario.arc`'s ColorChange (0 his own). The callbacks run as it starts, every frame, and as it ends.

## 12. Plugins: writing a whole boss

[`examples/little-boss`](../examples/little-boss) is complete. The shape every boss takes: **a number for what it's doing, and how long it's been doing it**. Each step checks one thing and moves on, so nothing happens until the step before has finished.

```c
enum { WAITING, NOTICING, WALKING, WINDING_UP, CHARGING, STUNNED, HURT, DYING, GONE };

static void update(NebulaActor* actor, void* user) {
    const float pace = 1.0f + (float)(3 - health) * 0.5f;   /* faster each phase */
    const int time = game->state_frames(actor);
    switch (game->state(actor)) {
    case WAITING:
        if (game->player() != 0 && game->distance_to_player(actor) < 2000.0f) {
            game->play_music("LittleBossTheme");
            game->set_state(actor, NOTICING);
        }
        break;
    case WINDING_UP:
        game->turn_to_player(actor, 2.0f);
        if (time >= 40) {
            game->set_state(actor, CHARGING);
        }
        break;
    /* ... */
    }
}
```

To wait for an animation rather than a time, start it when the state begins and move on when it ends:

```c
case ROARING:
    if (time == 0) {
        game->play(actor, NEBULA_PLAY_ANIMATION, "Roar");
    }
    if (game->is_animation_done(actor)) {
        game->set_state(actor, WALKING);
    }
    break;
```

Being hit is `attacked_by`: refuse it outside the state where it's vulnerable, so the sequence decides what can hurt it. Dying is a state as well: effects, music fading, `appear_item` for coins and the star, then `kill`.

- **An intro or a phase change** is `start_cutscene` in one state, with the cutscene's own sheets running the camera, and `end_cutscene` when it's over.
- **A phase's camera** is `start_camera`.
- **Parts of the arena** are switches: `set_switch` opens what the level wired to it, and `is_switch_on` starts the boss when the player reaches something.
- **Moving between points** is a rail from the level: `move_on_rail`, `rail_at_end`.
- **Attacks that fly** are objects of your own, made with `spawn`.

## 13. Plugins: the game's headers and hooks

The stable interface above survives Nebula updates. For anything beyond it:

| Level | What it gives you | Survives updates |
|---|---|---|
| Stable interface | This page | Yes |
| The game's headers | Subclass `LiveActor`, call any `MR::` function, read any field | Until those headers change |
| Hooks | Your code in place of any function, calling the original when you choose | While the function exists |

Every function of the game is reachable: the game's module exports all of them, so there is no list of what's allowed.

```cpp
#include <nebula_game_sdk.h>   /* first */

class Pulse : public LiveActor { /* written as the game's own objects are */ };
void* createPulse(const char* name) { return new Pulse(name); }

NEBULA_GAME_PLUGIN_ENTRY(api, self) {
    api->register_creator("Pulse", createPulse, nullptr);
}
```

Build with the flags the game uses, linking `smg_game.lib`. The plugin names the exact headers it was built from; on a Nebula built from others it isn't started, and the log says so.

```cpp
void (*original)(MarioActor*, u16);
void myDecLife(MarioActor* self, u16 amount) { original(self, amount); }

void* target = api->find_function("?decLife@MarioActor@@QEAAXG@Z");
api->hook(target, (void*)myDecLife, (void**)&original);
```

- Only in `nebula_plugin_start`. Windows x64, Linux and Mac on x64 and arm64.
- Symbols are the compiler's: `?decLife@MarioActor@@QEAAXG@Z` on Windows, `_ZN10MarioActor7decLifeEt` on Linux and Mac.
- Several plugins can hook one function. The last one started runs first; each one's `original` calls the one before.
- A function whose first instructions can't be moved answers -1. A function the compiler put inside its callers can't be reached: use an event, or hook the caller. The linker can merge identical functions, so a hook on one may catch another.

When a plugin can't start it's left out and the log says why:

| In the log | Why |
|---|---|
| a library it needs isn't there | It depends on a `.dll` you didn't include |
| it was built for another version of the game | A function it calls is gone, or the headers changed |
| it is not for this kind of computer | The wrong build |
| it is not a Nebula plugin | It doesn't export `nebula_plugin_start` |
| it needs a newer Nebula | Built against a later interface |

A plugin runs inside the game, so a crash in one takes the game down. The report names the mod, and the next start offers to turn its code off.

## 14. Limits that are gone

The Wii decided a lot of how this game works, for reasons that don't apply to Nebula. Where a mod can reach one, it's removed, not copied. Without any mods the game loads what it always did.

| On a console | In Nebula, with mods on |
|---|---|
| A stage holds a power-up's model only if its level places the item, so giving one elsewhere crashes | **Every power-up is ready on every stage.** A spring in the Comet Observatory just works, from a plugin or a placed item |
| An object can only appear if the stage loaded it | **`spawn` loads what it needs**, for the mod's objects and the game's |
| Sound effects come in banks per stage, so an object away from home is silent | **Every sound is ready on every stage** |
| Music is a fixed table in the program | **New tracks and rows**, per stage, mission and area |
| Comets, hungry Lumas, what opens what are fixed tables | **Rows in `luma.json`** |
| The objects a level can use are fixed | **`object_changes`** and new kinds through plugins |
| A mod patches the program, for one version of the game | **One file for every version** |
| Saves in one fixed shape | **Saves per mod**, and a galaxy list of any length |
| 24 MB + 64 MB of memory, fixed heaps | **No such limits** |
| 640x456 canvas, level-of-detail models, low-resolution effect buffers | **Removed in the base game** |

## 15. Shaders

Nobody writes shaders for a mod, and nobody compiles them. A model's materials say what it needs: when the player applies the mod, Nebula's conversion writes each material's program text, and Nebula builds the ones it lacks on that device, once, with a progress screen. LumaLink puts no compiled programs in a `.luma`: one fits only the Nebula it was built for.

| Device | What builds a mod's shaders |
|---|---|
| Windows | Setup installs Microsoft's dxc beside the game. No download at play time |
| Linux, Android, Mac | glslang and the SPIR-V tools, built into the game. Nothing to download |

To write one yourself, `lumalink shader model.glb -o folder` (or the game's `.bdl`), change the pixel program it writes for a material, and name it:

```json
"requires": ["custom-shaders"],
"layers": [{ "path": "layers/base", "materials": { "WaterSurface": "layers/base/shaders/water.hlsl" } }]
```

One HLSL file for every device. It takes the place of that material's plain program, and includes `program.hlsli` (and `program_lit.hlsli` for a lit one) as generated ones do; the command writes both beside it. One that doesn't build leaves its material undrawn, and the log says why. It applies to materials of that name in models the mod brings, so a game model has to be in the layer's `files/` for its materials to change.

## 16. Devices

A `.luma` runs on every device with no change, apart from plugins, which need a library per device: list each in `binaries`. A device with none says THIS MOD HAS NO VERSION FOR THIS DEVICE and nothing of that mod is used. A plugin that only needs the stable interface builds with any C compiler for that device.

Converting happens on a computer. Phones load.

## 17. Messages and what they mean

### In the game

A mod that can't be used stays in the list, off, with one line:

| Line | When |
|---|---|
| THIS MOD NEEDS A NEWER NEBULA | `format` above what it reads, a `requires` it lacks, or `nebula` above its version |
| THIS MOD IS FOR SUPER MARIO GALAXY 2 | `game` is `SB4` or `SB5` |
| THIS MOD IS FOR ANOTHER GAME | Any other `game` |
| THIS MOD DOESN'T WORK ON THIS VERSION OF THE GAME | `versions` leaves it out |
| THIS MOD NEEDS: {NAME} | A mod in `needs` is missing, off or unusable |
| THIS MOD NEEDS A NEWER VERSION OF: {NAME} | |
| THIS MOD CAN'T RUN WITH: {NAME} | A mod in `conflicts` is on |
| THIS MOD HAS NO VERSION FOR THIS DEVICE | A plugin has no library for it |
| THIS FILE IS DAMAGED | Not a zip, `luma.json` unreadable, or a listed path missing |

### In LumaLink

| Line | Meaning |
|---|---|
| CAN'T CONVERT: This mod is for Super Mario Galaxy 2 | Its patch files name `SB4` |
| CAN'T CONVERT: These textures are for Super Mario Galaxy 2 | A pack whose files name `SB4` |
| CAN'T CONVERT: This doesn't look like a mod | Nothing in it could be used |
| left out: FILE: the mod's file is empty | The game's stays |
| N code changes need a Nebula update or a plugin | Writes with no setting yet. Everything else works |
| named without a folder; choose your game store | A loose file: give `--game` |
| note: it doesn't name its authors | From `check`, for your information |

### `lumalink check`

`luma.json` first, valid, with its needed fields. Every `path`, `icon`, `save.path` exists. No path leaves the file. `when` names only declared options and choices. `default` is one of its choices. Every tunable is `"<file stem>.<name>"` with a value it can take; `switches` is refused, naming each one's tunable. Every file in `files/` is a kind the game can read. Each plugin library is really for the device it's listed under. Music and progression rows make sense.

## 18. Questions

**Does a mod need the game's files in it?** No, and it shouldn't carry them. Name an archive as a folder and change only what's yours.

**Can two mods change the same level?** Yes, through `merge/`. A whole-file replacement goes to the higher on the list.

**A mod patched memory. Will it work?** Conversion lists what became settings and what's left over. Almost everything else in it still works.

**Can I use a Galaxy 2 mod?** No. Nebula is the first game. Its levels need Galaxy 2's objects and engine.

**Can a plugin crash the game?** Yes; it runs inside it. The stable interface refuses harmless mistakes (a null actor from `spawn` is yours to check), and the crash report names the mod.

**Can I add a new power-up model?** Yes: `add_power_up` for the rules and `play` or a hook for the look, or a file at `ObjectData/IceMario.arc` to reskin one.
