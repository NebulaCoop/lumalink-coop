# Making mods for Nebula

This guide starts from nothing and ends with a galaxy of your own. Every example here has been built with LumaLink and run in the game. The ones in [`examples/`](../examples) are complete projects: copy one and change it.

You need LumaLink, your game imported into Nebula (Nebula made a `game.pak` from your disc), and for code, [clang](https://llvm.org). The full rules of the file are in [format.md](format.md). The plugin interface is in [plugins.md](plugins.md).

## Your first mod

```console
lumalink new "My Mod"
```

This makes a folder:

```text
My Mod/
├── luma.json                what the mod is
└── layers/base/
    ├── files/               files by their place in the game
    └── textures/            replacement textures
```

A file in `layers/base/files/` takes the place of the game's file at the same path. To change a model, put yours at `layers/base/files/ObjectData/Kuribo.arc`.

When it's ready:

```console
lumalink check "My Mod"
lumalink pack "My Mod"
```

`check` lists every mistake it can find. `pack` writes `My Mod.luma`. Open that file in Nebula: Settings, Mods, Import. Then turn the mod on and press Apply.

To see what's inside anyone's mod, run `lumalink unpack "Some Mod.luma" -o folder`.

## Changing one file inside an archive

The game keeps most things in `.arc` archives. You don't have to bring a whole archive to change one file in it. Name the archive as a folder:

```text
layers/base/files/ObjectData/Kuribo.arc/kuribo.bdl
```

The rest of the archive comes from the player's own game. So your mod holds only your work, and it doesn't carry any of the game's files.

## Models and animations

Put a `.glb` or `.gltf` where the game's model goes:

```text
layers/base/files/ObjectData/MyThing.arc/mything.glb
```

Nebula turns it into the game's own model when the player applies the mod, with one animation file for each animation in it. Materials get their shaders automatically. Vertex skinning works, up to the game's limit of joints.

You can also use the game's own formats (`.bdl`, `.bmd`, `.bck`, and so on) from any tool that writes them.

## Textures

Three ways, depending on what you're changing.

| You want to change | Do this |
|---|---|
| Any texture, wherever it's used | A texture pack: `tex1_` files in `layers/base/textures/` |
| A texture file in an object's archive | `layers/base/files/ObjectData/Thing.arc/name.png` becomes `name.bti` |
| A picture in the game's interface | `layers/base/files/LayoutData/CoinCounter.arc/timg/name.png` becomes `name.tpl` |

See [`examples/interface-picture`](../examples/interface-picture).

## Music

To replace one of the game's tracks, put a `.wav` or `.ogg` with the same name in `layers/base/files/AudioRes/Stream/`.

A new track is a file with a row in `luma.json`:

```json
"music": [{
  "name": "MyTheme",
  "file": "AudioRes/Stream/MyTheme.ogg",
  "stages": [{ "stage": "EggStarGalaxy" }]
}]
```

A row with `"scenario": 3` is for that mission. A row with `"change_area": 1` is what a `ChangeBgmCube` with that `Obj_arg1` starts. See [`examples/stage-music`](../examples/stage-music). Code can start a track too, as [`examples/boss-music`](../examples/boss-music) does.

Loops come from the file: a WAV's `smpl` chunk, an Ogg's `LOOPSTART` and `LOOPEND` comments. Without them the whole track loops.

## Text

Any message in the game, by its id, in as many languages as you like:

```json
"text": {
  "GalaxyName_EggStarGalaxy": { "default": "The Very First Galaxy", "fr": "La toute première galaxie" }
}
```

See [`examples/new-text`](../examples/new-text).

## Adding to levels and tables

Replacing a whole file means only one mod can change it. A layer's `merge/` folder adds rows instead, so any number of mods can add to the same level.

```text
layers/base/merge/StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo.json
```

```json
{ "rows": [ { "name": "Coin", "l_id": 900, "pos_x": 0, "pos_y": 100, "pos_z": 0, "scale_x": 1, "scale_y": 1, "scale_z": 1 } ] }
```

A column you leave out is zero or empty. In placement tables, "no switch" and "no argument" are `-1`, so write those out: `"SW_A": -1`.

Particles add the same way: see "Adding to shared files" in [format.md](format.md).

To make a table that doesn't exist yet, write `Name.bcsv.json` in `files/`. The kind of each column comes from its values. Say it yourself for a float whose values are all whole numbers, or for a column with no values yet:

```json
{ "columns": { "Speed": "float" }, "rows": [ { "Name": "Fast", "Speed": 2 } ] }
```

## In-game cutscenes

A cutscene is data: a `DemoGroup` placed in a level, and its sheets in `ObjectData/DemoSheet.arc`. [`examples/new-cutscene`](../examples/new-cutscene) makes a whole one with no code and none of the game's files:

| File | What it does |
|---|---|
| `files/ObjectData/DemoSheet.arc/demomycutscene/demomycutscenetime.bcsv.json` | Its parts, and how long each lasts, in frames |
| `...player.bcsv.json` | The player's animation in each part |
| `...sound.bcsv.json` | Music and sounds in each part |
| `...wipe.bcsv.json` | Fades in and out |
| `...camera.bcsv.json` | Cameras |
| `...action.bcsv.json` | What each object in the cutscene does |
| `merge/StageData/.../demoobjinfo.json` | The `DemoGroup`: its name, its sheets' name, and the switch that starts it |
| `merge/StageData/.../areaobjinfo.json` | A `SwitchCube` that turns that switch on when the player walks in |

Leave out any sheet you don't need. A wrong name in a sheet (a part, an animation, a position, a sound, a wipe, a cast) does nothing: the cutscene carries on without it.

Code can tell when a cutscene starts and ends: see [`native/plugin_sdk/examples/cutscene_log`](https://github.com/keepvaibin/nebula/tree/main/native/plugin_sdk/examples/cutscene_log).

To replace one of the game's own, change its sheet the same way.

## Movies

To replace a movie, put a video in `layers/base/movies/` or a `.thp` in `files/MovieData/`. A plugin can play a movie of the mod's in any scene:

```c
game->play_movie("MovieData/MyMovie.thp");
```

## Lumas

Each kind is placed as an object, by its name.

| Name | What it is | Set in its row |
|---|---|---|
| `Tico`, `TicoBaby` | A Luma that talks | `MessageId` for what it says |
| `TicoComet` | A Luma for the comet | |
| `TicoShop` | Sells for star bits | `Obj_arg0`: the price |
| `TicoFat` | Hungry, gives an item when fed | `Obj_arg0`: the item (0: 1-up, 1: life mushroom). `Obj_arg1`: how much it eats |
| `TicoGalaxy` | Hungry, opens a galaxy when fed | `Obj_arg1`: how much it eats. `Obj_arg7`: its slot. `CommonPath_ID`: where it flies. A `GroupId` shared with a launch star |
| `TicoRail` | Moves along a path | `CommonPath_ID` |
| `TicoReading` | Listens in the library | |
| `TicoStarRing` | Launches you to a galaxy | `Obj_arg0`: which |
| `TicoAstro` | In the Observatory, by green stars | `Obj_arg0`, `Obj_arg2` |
| `TicoDomeLecture` | The dome's lecture | Belongs in a `DemoGroup` |
| `CollectTico` + `StrayTico` | Gather the lost Lumas for a star | In one group, with a `PowerStar` row in the mission |

For a new look, give a kind a new model file: `ObjectData/TicoShop.arc`.

Row details follow the game's own levels: `lumalink unpack` a mod or read the disc's with a level editor.

## Your own galaxy

A galaxy beside the game's is its files and a row:

```text
layers/base/files/StageData/MyGalaxy/MyGalaxyScenario.arc
layers/base/files/StageData/MyGalaxy.arc
```

```json
"galaxies": [{
  "name": "MyGalaxy",
  "dome": 1,
  "map_position": [-60, -100],
  "hungry_luma": { "slot": 7, "star_bits": 400 }
}],
"save": { "mode": "own" },
"text": {
  "GalaxyName_MyGalaxy": "My Galaxy",
  "ScenarioName_MyGalaxy1": "The First Star"
}
```

| Key | Meaning |
|---|---|
| `dome` | Its dome, 1 to 6. 0: on the grand map |
| `map_position` | Where its icon goes, right and up from the middle of the page. The game makes the icon |
| `hungry_luma` | The Luma that opens it. Place a `TicoGalaxy` in the Observatory with its `Obj_arg7` at this slot, through `merge/` |
| `save` | A galaxy needs its own saves. `check` tells you if you forget |

Your levels are level files, from a level editor such as Whitehole. Your stars are rows in the scenario archive's `ScenarioData`. Bosses and enemies are placed objects: new looks are new models, new behaviour is a plugin, as [`examples/little-boss`](../examples/little-boss) shows.

To rearrange the game's own journey, as Kaizo does, see "The game's progression" in [format.md](format.md).

## Options for the player

```json
"options": [{
  "id": "difficulty", "name": "Difficulty", "default": "normal",
  "choices": [ { "id": "normal", "name": "Normal" }, { "id": "hard", "name": "Hard" } ]
}],
"layers": [
  { "path": "layers/base" },
  { "path": "layers/hard", "when": { "option": { "difficulty": ["hard"] } } }
]
```

Players choose on the Mods page, and the choice lasts through updates to the mod.

## Saves

| `save.mode` | What the player gets |
|---|---|
| `shared` (the default) | Their normal saves |
| `own` | Saves of its own, starting new or from `save.path` |
| `copy` | Saves of its own, starting from a copy of theirs |

`save.tag` names the family: mods with the same tag share saves. `save.player_can_change` lets the player choose on the Mods page.

## Your own shaders

Materials build their own shaders, so most mods never think about this. For a special effect, run `lumalink shader model.glb -o folder`, change the HLSL it writes, and name it:

```json
"requires": ["custom-shaders"],
"layers": [{ "path": "layers/base", "materials": { "WaterSurface": "layers/base/shaders/my-water.hlsl" } }]
```

The game builds it for Direct3D 12 and Vulkan on the player's device.

## Code

When data can't do it, write a plugin.

```console
lumalink new "My Mod" --plugin
```

```c
/* Any hit is the last one. */
#include "nebula_mod_api.h"

static const NebulaModApi* game;

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    (void)user;
    if (game->value(NEBULA_VALUE_LIFE) > 1) {
        game->set_value(NEBULA_SET_LIFE, 1);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
```

Build it with `build.cmd`. The game asks the player before it first runs a mod's code, and again if the code changes.

| Example | What it shows |
|---|---|
| [`one-health`](../examples/one-health) | Reading and setting the player's state |
| [`coin-kills`](../examples/coin-kills) | Events |
| [`half-damage`](../examples/half-damage) | Replacing a game function (`find_function`, `hook`) |
| [`achievements`](../examples/achievements) | The mod's own achievements: three unlocked by events, one by its code |
| [`boss-music`](../examples/boss-music) | Starting and stopping music |
| [`double-jump`](../examples/double-jump) | A new move for the player: the buttons, gravity, an animation |
| [`little-boss`](../examples/little-boss) | A new enemy: its model, music, health, placement and what it does |
| [`coin-bank`](../examples/coin-bank) | Keeping numbers and words across plays (`store_get`, `store_set_text`), a message when the player has control |
| [`goomba-tempo`](../examples/goomba-tempo) | Every object of a kind (`find_actor`), its animation's speed, an option |
| [`first-person`](../examples/first-person) | A camera of the plugin's own every frame (`set_camera`, `set_camera_near`), the player's model hidden, the game's cameras back for cutscenes ([tutorial 18](tutorials/18-first-person.md)) |
| [`pulse`](https://github.com/keepvaibin/nebula/tree/main/native/plugin_sdk/examples/pulse) | A new object with the game's headers, and a hook |
| [`cube`](https://github.com/keepvaibin/nebula/tree/main/native/plugin_sdk/examples/cube) | A new object with the stable interface and a glTF model |

A new object is one call, `register_object`, and then levels place it by its name. A new power-up is `add_power_up`. A new Mario move is the player's buttons and `jump_player` or `play`, or a hook on the player's functions. All of it is in [plugins.md](plugins.md).

## Publishing

Put the `.luma` anywhere people can download it. Before you do:

- Run `lumalink check` on the finished file.
- Give it a real `version`. A file with the same `id` and a higher version updates the mod and keeps the player's settings.
- Bring only your own work. Name archives as folders so the player's game supplies the rest.
- If your mod has code, build it for each device it should run on.
