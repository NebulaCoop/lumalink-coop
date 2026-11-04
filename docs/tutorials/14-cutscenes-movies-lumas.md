# Cutscenes, movies and Lumas

## In-game cutscenes

A cutscene is data: a `DemoGroup` placed in a level, and its sheets in `ObjectData/DemoSheet.arc`. [`examples/new-cutscene`](../../examples/new-cutscene) makes a whole one with no code.

| File | What it does |
|---|---|
| `files/ObjectData/DemoSheet.arc/demoNAME/demoNAMEtime.bcsv.json` | Its parts, and how long each lasts, in frames |
| `...player.bcsv.json` | The player's animation in each part |
| `...sound.bcsv.json` | Music and sounds in each part |
| `...wipe.bcsv.json` | Fades |
| `...camera.bcsv.json` | Cameras |
| `...action.bcsv.json` | What each object does |
| `...subpart.bcsv.json` | Parts within parts |
| `merge/StageData/.../demoobjinfo.json` | The `DemoGroup`: `DemoName`, `TimeSheetName`, `SW_APPEAR` |
| `merge/StageData/.../areaobjinfo.json` | A `SwitchCube` that sets that switch when the player walks in |

Leave out any sheet you don't need. A wrong name in a sheet does nothing and the cutscene carries on. To change one of the game's, change its sheet the same way.

A plugin starts one from an object: `game->start_cutscene(actor, "MyCutscene")`, and hears about any through `NEBULA_MOD_EVENT_CUTSCENE_START`.

## Movies

A video in `layers/base/movies/`, or a `.thp` in `files/MovieData/`, replaces a movie by name.

```c
game->play_movie("MovieData/MyMovie.thp");  /* over the game; it waits */
```

## Lumas

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

For a new look, a new file at `ObjectData/TicoShop.arc`.
