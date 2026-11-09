# Start here

Ten minutes, and you'll have changed the game. You need Nebula with your game imported, and LumaLink.

## 1. Make a project

```console
lumalink new "My First Mod"
```

```text
My First Mod/
├── luma.json                what the mod is
└── layers/base/
    ├── files/               files by their place in the game
    └── textures/            replacement textures
```

`luma.json` already says enough to be a mod:

```json
{
  "format": 1,
  "id": "my-first-mod",
  "name": "My First Mod",
  "version": "1.0.0",
  "game": "RMG",
  "layers": [ { "path": "layers/base" } ]
}
```

## 2. Change how high Mario jumps

Add this to the layer:

```json
"layers": [ {
  "path": "layers/base",
  "player_constants": { "mario": { "mJumpHeight": [null, null, 60.0] } }
} ]
```

`mJumpHeight` is a list of nine, for each kind of jump. The third is the triple jump, 36 in the game. `null` keeps the game's number. There are 493 numbers like this: see [Player numbers](reference/player-numbers.md).

## 3. Rename a galaxy

Add this beside `layers`:

```json
"text": { "GalaxyName_EggStarGalaxy": "The Very Good Egg" }
```

## 4. Give Mario six health from the start, and a thousand lives

```json
"requires": ["tunables"],
"layers": [ {
  "path": "layers/base",
  "player_constants": { "mario": { "mJumpHeight": [null, null, 60.0] } },
  "tunables": { "GameRules.cPlayerHealth": 6, "GameDataPlayerStatus.cMaxPlayerLeft": 999, "GameDataPlayerStatus.cStartingPlayerLeft": 100 }
} ]
```

## 5. Check it and run it

```console
lumalink check "My First Mod"
lumalink run "My First Mod"
```

`check` lists every mistake it can find. `run` builds the `.luma` and opens it in Nebula's Mods page. Turn it on, press Apply, and start a game.

To make the file without opening it: `lumalink pack "My First Mod"`.

## What next

Anything you can see or hear is a file you can replace: [textures](tutorials/01-textures.md), [models](tutorials/02-models.md), [music](tutorials/04-music-and-sound.md). Levels are [tables of objects](tutorials/03-levels.md). New objects and whole galaxies take [no code](tutorials/08-new-objects.md). When data can't do it, [plugins](tutorials/10-first-plugin.md) can do anything the game can.

To see how anyone else did something:

```console
lumalink unpack "Some Mod.luma" -o folder
```

To see what the game has:

```console
lumalink list objects --filter Coin
lumalink files StageData/EggStarGalaxy --game path/to/game.pak
lumalink extract StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo --game path/to/game.pak -o out
```
