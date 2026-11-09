# The `.luma` format, version 1

A `.luma` file is one mod. It works on every version of the game, every device Nebula runs on, and every Nebula from now on.

It is a zip file with `luma.json` as its first entry. Inside are the mod's files as modders make them, not a form tied to one build of the game. Nebula prepares them for itself when the mod is first used and again whenever its own needs change, so a mod never has to be converted twice.

```
My Mod.luma
├── luma.json                  what the mod is
├── icon.png                   optional, square, 256x256 or smaller
├── layers/
│   └── base/
│       ├── files/             files by their place in the game: StageData/MyGalaxy.arc
│       ├── merge/             what it adds to files many mods share, see "Adding to shared files"
│       ├── textures/          replacement textures
│       └── movies/            replacement videos
├── save/                      optional, starting files for its own saves
└── plugins/                   optional, the mod's code, see "Plugins"
```

Any zip tool can make one. `lumalink pack` makes one from a folder and checks it. `lumalink check` checks one made any other way.

## luma.json

```json
{
  "format": 1,
  "id": "kaizo-mario-galaxy",
  "name": "Kaizo Mario Galaxy",
  "version": "2.1.0",
  "game": "RMG",
  "authors": ["Someone"],
  "description": { "default": "The whole game, harder.", "ja": "ゲーム全体がより難しく。" },
  "license": "CC-BY-4.0",
  "homepage": "https://example.com/kaizo",
  "icon": "icon.png",
  "options": [
    {
      "id": "lives",
      "name": "Lives",
      "default": "normal",
      "choices": [
        { "id": "normal", "name": "Normal" },
        { "id": "one", "name": "One health" }
      ]
    }
  ],
  "layers": [
    { "path": "layers/base", "tunables": { "SkeletalFishBaby.cBreakAirBubble": false } },
    { "path": "layers/one-health", "when": { "option": { "lives": ["one"] } } },
    { "path": "layers/japan-text", "when": { "version": ["J"] } }
  ]
}
```

| Field | Needed | Meaning |
|---|---|---|
| `format` | yes | `1`. A later number gets a line saying a newer Nebula is needed. |
| `id` | yes | Lower-case letters, digits and dashes. It says which mod this is: a file with the same `id` is an update of it. |
| `name` | yes | Text, shown to players. |
| `version` | yes | Numbers with dots: `2.1.0`. Compared number by number. |
| `game` | yes | `RMG`: Super Mario Galaxy. Any other value is another game and the mod is refused by name. |
| `versions` | no | The versions it works on, from `E` (USA), `P` (Europe), `J` (Japan), `K` (Korea), `W` (Taiwan), `C` (China). Left out: all of them. Only for a mod that truly cannot work on the others. |
| `nebula` | no | The oldest Nebula it works on: `2.1.0`. |
| `requires` | no | Names of format features it cannot work without (see "Later additions"). |
| `authors` | no | Names. |
| `description` | no | Text. |
| `license`, `homepage` | no | Strings. |
| `icon` | no | A PNG in the file. |
| `needs` | no | Other mods it needs: `[{ "id": "other", "version": "1.2.0" }]`, where `version` is the oldest that will do. |
| `conflicts` | no | Mods it can't run with: `[{ "id": "other", "below": "2.0.0" }]`. Without `below`: every version. |
| `load_after`, `load_before` | no | Ids. Where this mod goes in the player's list when first added. The player's own order then wins. |
| `options` | no | What the player can choose, in the game. |
| `layers` | yes | What the mod does. |
| `save` | no | Which saves it plays on: see "Saves". Left out: the player's normal saves. |
| `galaxies` | no | Galaxies it adds: see "Galaxies". |
| `music` | no | Tracks it adds: see "Music". |
| `event_flags` | no | Rows of the game's progression: see "The game's progression". |
| `comet_schedule` | no | Each dome's comets: see "The game's progression". |
| `text` | no | Messages of the game by id: see "Messages". |
| `plugins` | no | The mod's code: see "Plugins". |
| `tool` | no | What made the file. Not used by the game. |

### Text

Anything shown to players is a string, or an object from language to string with a `default`:

```json
"name": { "default": "Green Stars", "ja": "グリーンスター", "fr": "Étoiles vertes" }
```

Languages: `en`, `en-GB`, `fr`, `fr-CA`, `es`, `es-MX`, `de`, `it`, `nl`, `pt`, `ja`, `ko`, `zh-Hans`, `zh-Hant`. The game takes its own language, then the language without its region, then `default`.

### Layers

A layer is a folder and the conditions for using it. Layers apply in order; a later layer's file replaces an earlier one's. This is how one file carries everything: differences between regions, languages, devices and options are layers, not separate files.

| Field | Meaning |
|---|---|
| `path` | The layer's folder. It may hold `files/`, `textures/` and `movies/`. May be left out for a layer that only sets tunables. |
| `tunables` | The game's tuning numbers by `"<file stem>.<name>"`: a number, `true`/`false`, a list (`null` keeps a place) or `{ "option": "id" }`. The mod says `"requires": ["tunables"]`. See [settings](reference/settings.md). A layer's `switches`, from earlier versions, is refused, naming the tunable for each. |
| `when` | Conditions. All of those given must hold. Left out: always. |

`when` may hold:

| Key | Holds when |
|---|---|
| `version` | The game is one of these versions: `["E", "P"]`. |
| `language` | The player's language is one of these game folders: `["UsEnglish", "EuEnglish"]`. |
| `platform` | The device is one of `windows`, `android`, `linux`, `macos`, `ios`. |
| `option` | Each named option is set to one of its listed choices: `{ "lives": ["one"] }`. |

A `when` with a key this version of Nebula does not know is false: the layer is not used.

### Options

Players choose in the game, on the Mods page, and can change their minds without the mod being converted again. Changing a choice takes effect at the next reset.

| Field | Meaning |
|---|---|
| `id` | Lower-case letters, digits and dashes. |
| `name` | Text. |
| `description` | Text, optional. |
| `default` | A choice's `id`. |
| `choices` | At least one, each `{ "id": ..., "name": ... }`. A choice with the id `off` is shown in the player's language and means this option adds nothing. Without one, the option has no Off. |

### Files

`files/` holds files at the paths the game knows them by, in any case: `files/StageData/RedBlueExGalaxy/RedBlueExGalaxyMap.arc`. New paths are new files. Files stay in the form mods use: `.arc`, `.szs`, `.bcsv`, `.bmd`, `.brstm` and the rest. Files can also be modern, turned into the game's forms when the player applies the mod: glTF models and solid ground (`.glb`, `.gltf`, `.kcl.glb`), PNG textures, WAV and Ogg music, and tables as JSON (`.bcsv.json`, a zone's `jmp` tables as `.json`). See [Modern files](REFERENCE.md#modern-files). They need no `requires`.

Text for languages the mod leaves out: if a mod sets files in some language folders and not the one being played, Nebula shows the mod's text anyway, from the same language of another region, then English, then what it has. A player can turn this off for a mod.

### Saves

```json
"save": { "mode": "own", "tag": "kaizo", "path": "save", "player_can_change": false }
```

| Field | Meaning |
|---|---|
| `mode` | `shared`: the player's normal saves. `own`: saves of its own, starting fresh or from `path`. `copy`: saves of its own, starting from a copy of the player's, which stay untouched. |
| `tag` | Mods with the same tag share saves. Left out: the mod's `id`. |
| `path` | A folder in the file with starting files, laid out as the save folder is. |
| `player_can_change` | The player may choose between the mod's saves and normal ones on the Mods page. |

Own saves live in a folder per tag, outside the mod, so they survive updating or removing it, and a mod can't load a save from another family. Two mods wanting different saves can't run together. A mod that adds a galaxy and doesn't ask for its own saves gets them anyway: a save's shape follows the galaxy list.

### Shaders

Nobody writes shaders for a mod, and nobody compiles them. A model's materials say what it needs: when the player applies the mod, Nebula's conversion writes each material's program text, and Nebula builds the ones it lacks on that device, once, with a progress screen. LumaLink puts no compiled programs in a `.luma`: one fits only the Nebula it was built for.

### Modern files

Files in a layer's `files/` can be modern ones. Nebula makes them into the game's own when it prepares the mod, under the same name with the game's ending.

| Put in | Becomes | Notes |
|---|---|---|
| `.glb`, or `.gltf` with its files | A model, `.bmd` | glTF 2. Triangles and skins. One colour and one PNG texture per material. One metre is 100 of the game's units. A material's name is the name `materials` uses |
| `.png` | An image, `.bti` | Any size. For textures inside models, use a model or `textures/` |
| `.wav` | A music stream, `.ast` | 16-bit PCM. The file's `smpl` loop is kept; without one the whole track loops |
| `.ogg` | A music stream, `.ast` | Vorbis. `LOOPSTART` with `LOOPEND` or `LOOPLENGTH` set the loop |

A new object's model, in a folder named as its archive:

```
layers/base/files/ObjectData/MyEnemy.arc/MyEnemy.glb
```

A folder named `.arc` in `files/` changes the game's archive of that name file by file: the rest of it stays the game's.

### Messages

`text` gives words for any message of the game by its id, new or in place of the game's. Each is Text, so each can have its languages. Two mods can both add messages, which replacing the game's message files doesn't allow.

```json
"text": {
  "GalaxyName_MyGalaxy": { "default": "My Galaxy", "fr": "Ma Galaxie" },
  "GalaxyNameShort_MyGalaxy": "My Galaxy",
  "ScenarioName_MyGalaxy1": "The First Star"
}
```

A new galaxy with no `GalaxyName_` message shows an empty name.

### Hand-written shaders

Most mods never need this: a model's materials get their shaders by themselves. To write one yourself, name the material in a layer and list `custom-shaders` in `requires`:

```json
"requires": ["custom-shaders"],
"layers": [{
  "path": "layers/base",
  "materials": { "WaterSurface": "layers/base/shaders/water.hlsl" }
}]
```

`lumalink shader <model>` writes the pixel program Nebula makes for each of a model's materials, to start from; a hand-written one takes the place of that material's plain program. The file is HLSL, one file for every device. It includes `program.hlsli` (and `program_lit.hlsli` for a lit material), as the programs Nebula generates do, and the command writes those beside it; and Nebula builds it for the device's graphics interface with a progress screen. A shader that doesn't build leaves that material undrawn, and the log says why. It applies to materials of that name in the models the mod brings: a model of the game's own has to be in the layer's `files/` for its materials to change.

### Adding to shared files

Files in `files/` can be in modern forms, turned into the game's when the player applies the mod: `.glb` and `.gltf` models and their animations, `.png` (a `.tpl` in an archive's `timg` folder, otherwise a `.bti`), `.wav` and `.ogg` streams, and `.bcsv.json` tables. See [guide.md](guide.md).

A file in `files/` replaces the whole file: the highest mod on the player's list wins. Some files many mods need to add to, such as the game's list of effects. A layer's `merge/` folder adds to them, so every mod's additions are kept. Its paths are places in the game, going inside archives:

```
layers/base/merge/ParticleData/Effect.arc/AutoEffectList.bcsv.json     rows for a table
layers/base/merge/ParticleData/Effect.arc/ParticleNames.bcsv.json      the names of new particles
layers/base/merge/ParticleData/Effect.arc/Particles.jpc                those particles
```

A table's rows are JSON. A column is named, or written as `0x` and its hash. A column left out is zero or empty.

```json
{ "rows": [ { "GroupName": "MyEnemy", "UniqueName": "Smoke", "EffectName": "MyFire", "JointName": "Center" } ] }
```

`Particles.jpc` holds only the new particles. Each one's number in the file is its place in `ParticleNames.bcsv.json`, starting at 0. Nebula renumbers them, and their textures, when it adds them to the game's.

### Galaxies

A galaxy beside the game's own is its files, `StageData/<name>/<name>Scenario.arc` and `StageData/<name>.arc`, and a row here. Everything but `name` is optional.

```json
"galaxies": [{
  "name": "MyGalaxy",
  "map_position": [-60, -100],
  "dome": 1,
  "stars_to_open": 0,
  "open_conditions": [],
  "hungry_luma": { "slot": 7, "star_bits": 400 }
}]
```

| Key | Meaning |
|---|---|
| `name` | Its folder's name. |
| `map_position` | Where its icon goes: `[right, up]` from the middle of its dome's page, or of the grand map if it has no dome. The page is 608 wide by 456 high. The game makes the icon itself. |
| `map_pane` | The pane in the map's layout (`MapGrandGalaxy`) that shows it. Without one, or if your layout has no such pane, it isn't on the map. |
| `dome` | The dome it belongs to, 1 to 6. |
| `stars_to_open` | Power stars it takes to open. |
| `open_conditions` | Up to three event flags that open it. |
| `hungry_luma` | A hungry Luma that opens it: placed in a level as `TicoGalaxy` with its `Obj_arg7` at `slot`. The game's are slots 0 to 6, and using one takes its place. `star_bits` is the price. |

A mod that adds a galaxy keeps its own saves: `save.mode` has to be `own` or `copy`.

### Music

To replace one of the game's tracks, a layer has a file of the same name. A track of your own is a file in a layer, `files/AudioRes/Stream/MyTrack.wav` (or `.ogg`, or the game's `.ast`), and a row here.

```json
"music": [{
  "name": "MyTheme",
  "file": "AudioRes/Stream/MyTheme.wav",
  "stages": [
    { "stage": "MyGalaxy" },
    { "stage": "MyGalaxy", "scenario": 3 },
    { "stage": "MyGalaxy", "change_area": 1 }
  ]
}]
```

| Key | Meaning |
|---|---|
| `name` | Yours to choose. Names starting `SE_`, `BGM_`, `MBGM_`, `STM_` or `ME_` are the game's. A plugin starts a track by this name with `play_music`. |
| `file` | Its place in the game. A WAV's `smpl` chunk and an Ogg's `LOOPSTART` and `LOOPEND` comments say where it loops; without them it loops from the start. |
| `stages` | Where it plays. `scenario` is a mission, leaving it out is every mission without a row of its own. `change_area` is what a `ChangeBgmCube` in that stage starts, with its `Obj_arg1` at that number. |

A row here comes before the game's own music for that stage, comets and Bowser's fortresses included.

### The game's progression

What opens what, and what each hungry Luma costs, is a table of rows by name. A row here changes the game's row of that name, or adds a new one.

```json
"event_flags": [
  { "name": "ExOpen1CocoonExGalaxy", "kind": 3, "a": 1, "b": 0, "first": "SandClockGalaxy" },
  { "name": "StarPieceCocoonExGalaxy", "kind": 8, "a": 5, "b": 80, "first": "CocoonExGalaxy" }
],
"comet_schedule": [
  { "dome": 2, "comets": [ { "galaxy": "BattleShipGalaxy", "coins": 1 }, { "galaxy": "OceanRingGalaxy", "coins": 0 } ] }
]
```

| Kind | What the row says | `a` and `b` | `first` and `second` |
|---|---|---|---|
| 1 | How many stars | `a`: the number | |
| 2 | A galaxy's stars to open it | | `first`: the galaxy |
| 3 | A special star was got | `a`: which star | `first`: its galaxy |
| 4 | Two other rows are both on | | `first` and `second`: the rows |
| 8 | A hungry Luma | `a`: its slot. `b`: its price | `first`: the galaxy it opens |

Where each galaxy is on the map and how many stars it takes is the table `NebulaData/GalaxyInfoTable.bcsv`, which is the one table of Nebula's a layer can replace: `layers/base/files/NebulaData/GalaxyInfoTable.bcsv` or `.bcsv.json`. To add a galaxy, use `galaxies`.

Keep these together: a mod that changes them is a different journey through the game, so it should keep its own saves.

### Plugins

A mod's code: a native library per device, with what it needs named in [plugins.md](plugins.md).

```json
"plugins": [{
  "api": 2,
  "binaries": { "windows-x64": "plugins/my.dll", "linux-x64": "plugins/my.so" },
  "when": { "option": { "mode": ["hard"] } }
}]
```

`api` is the plugin interface it was built for. A Nebula with an older one says it needs a newer Nebula. A device with no entry in `binaries` says the mod has no version for it, and nothing of the mod is used.

## What the game says

A mod that can't be used stays in the list, off, with one line saying why. These lines are in the game's own text, in the player's language.

| Line | When |
|---|---|
| THIS MOD NEEDS A NEWER NEBULA | `format` above what it reads, a `requires` name it lacks, or `nebula` above its version. |
| THIS MOD IS FOR SUPER MARIO GALAXY 2 | `game` is `SB4` or `SB5`. |
| THIS MOD IS FOR ANOTHER GAME | Any other `game`. |
| THIS MOD DOESN'T WORK ON THIS VERSION OF THE GAME | `versions` leaves out the version being played. |
| THIS MOD NEEDS: {NAME} | A mod in `needs` is missing, off, or can't be used. |
| THIS MOD NEEDS A NEWER VERSION OF: {NAME} | A mod in `needs` is older than it says. |
| THIS MOD CAN'T RUN WITH: {NAME} | A mod in `conflicts` is on. |
| THIS MOD HAS NO VERSION FOR THIS DEVICE | A plugin has no library for this device. |
| THIS FILE IS DAMAGED | Not a zip, `luma.json` missing or not readable, or a listed path missing. |

## Later additions

Added since version 1 began, each with its `requires` name: `tunables`, `custom-shaders` (a layer naming a material and an HLSL file that Nebula builds for every device) and `player-animations`. Modern asset files came without one.


The format stays version 1 for as long as a change can be read safely by older games:

- A reader ignores keys it does not know.
- New keys that a mod can't work without go with a name in `requires`. A Nebula without that name refuses the mod, with the first line above, and never half-applies it.
- `format` only changes if none of this can hold.

Nebula reads every format it has ever read.

## Checks

`lumalink check my-mod.luma` reports each of these:

- `luma.json` is the first entry, is valid, and has its needed fields.
- Every `path`, `icon` and `save.path` exists in the file.
- No path leaves the file.
- `when` names only options the file declares, and only their choices.
- `default` is one of its option's choices.
- Every tunable key is `"<file stem>.<name>"` with a number, `true`/`false`, a list or an option, and the mod requires `tunables`. A layer's `switches` is refused, naming the tunable for each.
- Every file in a layer's `files/` is a kind the game can read.
