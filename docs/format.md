# The `.luma` format, version 1

A `.luma` file is one mod. It works on every version of the game, every device Nebula runs on, and every Nebula from now on.

It is a zip file with `luma.json` as its first entry. Inside are the mod's files as modders make them, not a form tied to one build of the game. Nebula prepares them for itself when the mod is first used and again whenever its own needs change, so a mod never has to be converted twice.

```
My Mod.luma
├── luma.json                  what the mod is
├── icon.png                   optional: a PNG, square looks best, 256x256 is plenty
├── pictures/                  optional: screenshots and art for the mod's page
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
  "pictures": ["pictures/first-galaxy.png", "pictures/boss.png"],
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
| `regions` | no | `"any"`: every disc (`lumalink new` writes this). Or the discs it is for, from `US`, `PAL` (Europe and Australia), `JP`, `KR`: `["US", "PAL"]`. On another disc the mod is blocked, and its page names the discs it is for. Only for a mod that truly cannot work on the others, such as one that replaces one region's text or layouts. Any other value refuses the file. |
| `versions` | no | The same by the disc's letter, from older files: `E` (USA), `P` (Europe), `J` (Japan), `K` (Korea), `W` (Taiwan), `C` (China). Left out: all of them. Write `regions` instead. |
| `nebula` | no | The oldest Nebula it works on: `2.1.0`. |
| `requires` | no | Names of format features it cannot work without (see "Later additions"). |
| `authors` | no | Names. |
| `description` | no | Text. |
| `license` | no | Text: a license's name (`CC-BY-4.0`, `MIT`) or its own words. Shown on the mod's page. |
| `homepage` | no | A web address starting with `https://` (or `http://`). Shown on the mod's page. |
| `icon` | no | A PNG in the file. The first picture of the mod's page. |
| `pictures` | no | PNGs in the file, shown after the icon on the mod's page, in order. The game shows the first 16. |
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
| `achievements` | no | The mod's own achievements: see "Achievements". |
| `story` | no | Where a new file starts and where the player comes back to: see "The journey". |
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
| `path` | The layer's folder. May be left out for a layer of rows alone. |
| `tunables` | The game's tuning numbers by `"<file stem>.<name>"` (or `.<function>.<k>`, `.<function>.<call>.<k>`): a number, `true`/`false`, a list (`null` keeps a place) or `{ "option": "id" }`. The mod says `"requires": ["tunables"]`. See [settings](reference/settings.md). A layer's `switches`, from earlier versions, is refused, naming the tunable for each. |
| `object_changes` | One of the game's objects made as another, or gone: `{ "object": "BenefitItemLifeUp", "becomes": "BenefitItemOneUp" }`, with `"in"` of `placed` (the default), `treasure_boxes` or `display_models`. See [Object changes](REFERENCE.md#object-changes). |
| `forms` | Power-ups from data: `{ "name": "gold", "acts_like": "fire", "looks_like": "fire", "color_frame": 2, "frames": 600, "music": "BGM_FIRE_A", "model": "GoldMario", "script": "gold" }`. `acts_like` is what the player can do in it (one of the game's: normal, invincible, fire, ice, bee, spring, boo, flying, tornado), and its music and time where the row gives none; `looks_like` or `color_frame` (a frame of Mario.arc's ColorChange) or `model` (ObjectData/<model>.arc) its look. A script runs on the player while he is in it: see [forms](reference/lumascript.md#forms). Scripts give it with `nebula.set_value(2, registry("form/gold"))`. |
| `object_classes` | New object names, each made as one of the game's: `{ "name": "Bramblet", "like": "Kuribo" }`. Its files are `ObjectData/<name>.arc`. See [tutorial 8](tutorials/08-new-objects.md). A `script` runs on every object placed by the name, a new one or the game's own (`{ "name": "Kuribo", "script": "angry" }`): see [class scripts](reference/lumascript.md#class-scripts). |
| `player_constants` | How the player moves, by name: `{ "mario": {...}, "luigi": {...}, "both": {...} }`, each a number, a list (`null` keeps a place) or `{ "option": "id" }`; a character's id (`"wario"`) for a mod's character. See [player numbers](reference/player-numbers.md). |
| `characters` | Playable characters (NebulaCoop), as sm64coopdx's character table: `{ "id": "wario", "name": "WARIO", "like": "mario", "archive": "Wario" }`. The model is the layer's `files/ObjectData/<archive>.arc` with the base's skeleton; `like` (mario or luigi) gives the animations, voice and the game's code. Its other models follow the game's names with the base's name swapped (`MarioFace` → `WarioFace`, `BeeMario` → `BeeWario`); one left out is the base's. `name` defaults to the id in capitals, `archive` to the id with a capital first letter. Players pick it on Options' PLAYER page; others in a room see it. The mod says `"requires": ["characters"]`. See nebula-coop's docs/modding/CHARACTERS.md. |
| `player_animations` | New player animations from the game's or the layer's `ObjectData/MarioAnime.arc`: `{ "name": "GoldFlowerSpin", "file": "FireSpin", "rate": 1.5, "blend_frames": 3 }`. `rate` is over 0, up to 100; `blend_frames` 0 to 600. The mod says `"requires": ["player-animations"]`. |
| `sound_effects` | One of the game's sounds, by name, played from the layer's file: `{ "name": "SE_PV_JUMP_S", "file": "AudioRes/Stream/Hup.wav" }`. `@luigi` after the name for Luigi's own. |
| `stages` | Rules for stages: `{ "stage": "MyHub", "scenario": 1, "acts_as": ["AstroGalaxy"], "own_rules": true }`. See [stage rules](reference/stage-rules.md). |
| `comet_time_limits` | Comet missions' time limits: `{ "galaxy": "BattleShipGalaxy", "mission": 4, "seconds": 150 }`. |
| `materials` | Hand-written shaders by material name: see "Hand-written shaders". |
| `text`, `story`, `galaxies`, `music`, `event_flags`, `comet_schedule` | As the same keys at the top, for this layer alone: so a galaxy, a track or a message can depend on an option or a version. |
| `when` | Conditions. All of those given must hold. Left out: always. |

A layer's folder holds `files/` (whole files, see "Files"), `merge/` (additions to shared files, see "Adding to shared files"), `textures/` and `movies/`.

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
| `choices` | Each `{ "id": ..., "name": ... }`. A choice with the id `off` is shown in the player's language and means this option adds nothing. Without one, the option has no Off. |
| `range` | A choice for each number: `{ "from": 1, "to": 20, "step": 1 }`, each choice's id being its number (counting down when `from` is the larger), at most 10001. `default` is then a number. For tunables and player numbers taken from `{ "option": "id" }`. |

An option needs `choices` or a `range`, or both.

### Files

`files/` holds files at the paths the game knows them by, in any case: `files/StageData/RedBlueExGalaxy/RedBlueExGalaxyMap.arc`. New paths are new files. Files stay in the form mods use: `.arc`, `.szs`, `.bcsv`, `.bmd`, `.brstm` and the rest. Files can also be modern, turned into the game's forms when the player applies the mod: glTF models and solid ground (`.glb`, `.gltf`, `.kcl.glb`), PNG textures, WAV and Ogg music, and tables as JSON (`.bcsv.json`, a zone's `jmp` tables as `.json`). See [Modern files](REFERENCE.md#modern-files). They need no `requires`.

Text for languages the mod leaves out: if a mod sets files in some language folders and not the one being played, Nebula shows the mod's text anyway, from the same language of another region, then English, then what it has. A player can turn this off for a mod.

A language folder the disc doesn't have (`files/EuGerman/` for a US disc) adds that language: it is listed with the game's languages, and files it lacks come from the disc's English. See [tutorial 5](tutorials/05-text.md#a-language-from-another-release).

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

Words may hold the game's message tags in brackets: `[wait 30]`, `[page]`, `[center]`, `[sound SE_NAME]`, `[icon 7]`, `[small]`, `[normal]`, `[large]`, `[player]`, `[color 1]`, `[tag G T u...]`; `[[` is one `[`. A new message takes the talk settings of a game message named in `like`: `"MyNpc_Hello": { "default": "Hi, [player]!", "like": "AstroGalaxy_Kinopio012" }`. See [tutorial 5](tutorials/05-text.md).

### The journey

```json
"story": { "first_stage": "MyFirstGalaxy", "first_mission": 1, "hub": "MyHub", "story_scenes": false }
```

| Key | Meaning |
|---|---|
| `first_stage`, `first_mission` | Where a new file begins, in place of the prologue |
| `hub` | Where the player goes after a galaxy, in place of the Comet Observatory. A file with a star opens there |
| `story_scenes` | `false`: none of the game's own story scenes |

See [tutorial 16](tutorials/16-total-conversion.md).

### Hand-written shaders

Most mods never need this: a model's materials get their shaders by themselves. To write one yourself, name the material in a layer and list `custom-shaders` in `requires`:

```json
"requires": ["custom-shaders"],
"layers": [{
  "path": "layers/base",
  "materials": { "WaterSurface": "layers/base/shaders/water.hlsl" }
}]
```

`lumalink shader <model>` writes the pixel program Nebula makes for each of a model's materials (a `.glb` or `.gltf` as Nebula converts it, or a `.bdl`/`.bmd`), to start from; a hand-written one takes the place of that material's plain program. The file is HLSL, one file for every device. It includes `program.hlsli` (and `program_lit.hlsli` for a lit material), as the programs Nebula generates do, and the command writes those beside it; and Nebula builds it for the device's graphics interface with a progress screen. A shader that doesn't build leaves that material undrawn, and the log says why. It applies to materials of that name in the models the mod brings: a model of the game's own has to be in the layer's `files/` for its materials to change, and `lumalink check` says when no model of the mod has a material of that name. A program has no clock: for motion, give the material an animation (a `.btk` texture matrix, [tutorial 2](tutorials/02-models.md#a-material-that-moves)) and read its moving coordinates. Example: `examples/glowing-lantern`.

### Adding to shared files

Files in `files/` can be in modern forms, turned into the game's when the player applies the mod: `.glb` and `.gltf` models and their animations, `.png` (a `.tpl` in an archive's `timg` folder, otherwise a `.bti`), `.wav` and `.ogg` streams, and `.bcsv.json` tables. See [guide.md](guide.md).

A file in `files/` replaces the whole file: the highest mod on the player's list wins. Some files many mods need to add to, such as the game's list of effects. A layer's `merge/` folder adds to them, so every mod's additions are kept. Its paths are places in the game, going inside archives:

```
layers/base/merge/ParticleData/Effect.arc/AutoEffectList.bcsv.json     rows for a table
layers/base/merge/ParticleData/Effect.arc/ParticleNames.bcsv.json      the names of new particles
layers/base/merge/ParticleData/Effect.arc/Particles.jpc                those particles
```

A table's rows are JSON. A column is named, or written as `0x` and its hash. A column left out is zero or empty. A table merge can also take rows away and change them:

```json
{
  "remove": [ { "name": "EarthenPipe" } ],
  "change": [ { "where": { "name": "SuperSpinDriver" }, "set": { "scale_x": 2.0 } } ],
  "rows": [ { "GroupName": "MyEnemy", "UniqueName": "Smoke", "EffectName": "MyFire", "JointName": "Center" } ]
}
```

| Key | Does |
|---|---|
| `remove` | Takes away every row whose columns hold all the values given |
| `change` | Sets columns (`set`) of every row matching `where` |
| `rows` | Adds rows |

They run in that order, mod by mod down the player's list. A row that matches nothing is not an error: another mod may have taken it. Two mods setting one cell: the higher on the list wins, and the Mods page shows the two overlap.

Messages merge too: `merge/MessageData/messages.json` (every language) or `merge/<Language>/MessageData/Message.arc/message.bmg.json` (one) adds talk lines and changes what follows each. See [tutorial 5](tutorials/05-text.md#talk-new-lines-and-what-follows-them).

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

### Achievements

A mod's own achievements, beside the game's. Each unlocks once, for good: the game keeps it in `achievements.ini` with
the game's own, under the key `mod:<mod id>/<id>`, so it stays unlocked across save files, updates of the mod, and a
mod turned off and on again. When one unlocks, the game's pop-up shows its name. The achievement board gets a Mods tab
(only while a mod on has achievements) listing each with its description, and the All tab lists them after the game's.

```json
"achievements": [
  { "id": "first-coin", "name": { "default": "Pocket Change", "ja": "はじめてのコイン" }, "description": "Collect a coin.",
    "when": { "event": "coin" } },
  { "id": "kameck", "name": "Wand Breaker", "description": "Defeat a Magikoopa.", "hidden": true,
    "when": { "event": "actor_kill", "name": "Kameck" } },
  { "id": "fifty-coins", "name": "Coin Purse", "description": "Collect 50 coins in one sitting." }
]
```

| Field | Needed | Meaning |
|---|---|---|
| `id` | yes | Lower-case letters, digits and dashes, unique in the mod. Keep it for good: a new id is a new achievement, and the old one's unlock is no longer shown. |
| `name` | yes | Text, shown on the pop-up and the board. |
| `description` | no | Text: what earns it, shown on the board's card. |
| `hidden` | no | `true`: the board lists it only once it is unlocked. |
| `when` | no | The game event that unlocks it, from the list below. Without `when`, only the mod's code unlocks it. |
| `when.name` | no | Only that event with this name: the galaxy for `star_get`, the object's name for `actor_kill`, the message id for `talk_line` (the names [events](reference/events.md) gives each event). |
| `when.value` | no | Only that event with this value: the star's number for `star_get`, the action for `player_action`. |

Events `when` can name: `frame`, `stage_frame`, `stage_start`, `player_damage`, `player_miss`, `power_up`, `coin`,
`star_bit`, `star_get`, `cutscene_start`, `cutscene_end`, `actor_appear`, `actor_kill`, `player_spin`, `player_jump`,
`player_land`, `pause`, `stage_end`, `game_save`, `power_star_appear`, `talk_line`, `talk_choice`, `cutscene_part`,
`camera_change`, `sound_start`, `effect_start`, `screen`, `actor_message`, `player_action`. Each is the
`NEBULA_MOD_EVENT_` of the same name; [events](reference/events.md) says when it comes and what its name and value are.

Anything a single event cannot say (fifty coins, a galaxy with no damage, a time) is the mod's code: it counts what it
needs and calls `unlock_achievement(self, "fifty-coins")` (see [plugins](plugins.md)); it returns 1 when that unlocked
it now. The game keeps the first 64 achievements of a mod; one with no name, an id that is not an id, or the id of one
before it is left out.

The `achievements` example in LumaLink's `examples` folder has three unlocked by events and one by its code.

## What the game says

A mod that can't be used stays in the list, off, with one line saying why. These lines are in the game's own text, in the player's language.

| Line | When |
|---|---|
| THIS MOD NEEDS A NEWER NEBULA | `format` above what it reads, a `requires` name it lacks, or `nebula` above its version. |
| THIS MOD IS FOR SUPER MARIO GALAXY 2 | `game` is `SB4` or `SB5`. |
| THIS MOD IS FOR ANOTHER GAME | Any other `game`. |
| THIS MOD IS ONLY FOR: {REGIONS} | `regions` leaves out the disc being played. The words are in the player's language; the region names stay as written. |
| THIS MOD DOESN'T WORK ON THIS VERSION OF THE GAME | `versions` leaves out the version being played. |
| THIS MOD NEEDS: {NAME} | A mod in `needs` is missing, off, or can't be used. |
| THIS MOD NEEDS A NEWER VERSION OF: {NAME} | A mod in `needs` is older than it says. |
| THIS MOD CAN'T RUN WITH: {NAME} | A mod in `conflicts` is on above it in the list (of two that can't run together, the lower one is left off, whichever names the other), or a mod above it plays on another family of its own saves. |
| ANOTHER COPY OF THIS MOD IS ON: {FOLDER} | Two folders hold the same `id` (a copy made by hand): the one higher in the list is used. |
| THIS MOD HAS NO VERSION FOR THIS DEVICE | A plugin has no library for this device. |
| THIS MOD'S CODE IS FOR AN OLDER NEBULA: REBUILD IT WITH THIS NEBULA'S NEBULA_MOD_API.H | A plugin's `api` is older than the game's. |
| THIS MOD'S CODE IS OFF AFTER A CRASH. RESTART TO TRY IT AGAIN | The last run crashed while mods' code ran: this run starts without it. |
| THIS FILE IS DAMAGED | Not a zip, `luma.json` missing or not readable, or a listed path missing. |

A mod's needs, conflicts, copies and saves are worked out again whenever the player turns a mod on or off or moves one, until nothing changes: a mod that loses what it needs takes with it the mods that need it.

## Later additions

Added since version 1 began, each with its `requires` name: `tunables`, `custom-shaders` (a layer naming a material and an HLSL file that Nebula builds for every device), `player-animations`, and in NebulaCoop `coop` and `characters`. Modern asset files came without one.

This co-op fork of LumaLink adds `coop`: a mod made for NebulaCoop's online co-op. `lumalink pack` writes such a mod as
a `.lumacoop` instead of a `.luma`. It's the same file inside. NebulaCoop reads both; upstream Nebula doesn't know
`coop`, so it refuses the mod even when it's renamed to `.luma`.


The format stays version 1 for as long as a change can be read safely by older games:

- A reader ignores keys it does not know.
- New keys that a mod can't work without go with a name in `requires`. A Nebula without that name refuses the mod, with the first line above, and never half-applies it.
- `format` only changes if none of this can hold.

Nebula reads every format it has ever read.

## Checks

`lumalink check my-mod.luma` reports each of these:

- `luma.json` is the first entry, is valid, and has its needed fields.
- Every `path`, `icon`, `pictures` entry and `save.path` exists in the file.
- `icon` and `pictures` are PNGs of at most 8192 pixels a side and 64 MB; at most 16 pictures. Over 1024 pixels a side gets a note: the game shows a picture at most 1024 a side.
- `homepage` starts with `https://` or `http://`; `license` is text.
- Each achievement has an id (unique in the mod) and a name; its `when` names an event the game has, `when.value` is a whole number, and one without `when` is in a mod with code; at most 64.
- No path leaves the file.
- `when` names only options the file declares, and only their choices.
- `default` is one of its option's choices.
- Every tunable key is `"<file stem>.<name>"` with a number, `true`/`false`, a list or an option, and the mod requires `tunables`. A layer's `switches` is refused, naming the tunable for each.
- Every file in a layer's `files/` is a kind the game can read.
