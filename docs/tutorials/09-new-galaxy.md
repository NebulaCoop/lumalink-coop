# A new galaxy

A galaxy beside the game's is its files and a row.

## The files

```text
layers/base/files/StageData/MyGalaxy/MyGalaxyScenario.arc    its missions and zones
layers/base/files/StageData/MyGalaxy.arc                     its main zone
layers/base/files/StageData/MyPlanetZone.arc                 any other zones
```

Levels come from a level editor such as Whitehole. You can also start from one of the game's: `lumalink extract StageData/EggStarGalaxy --game game.pak -o out` writes the scenario's tables as JSON.

`examples/new-galaxy` is a whole one, every file written by `source/make_galaxy.py`: a planet, a new enemy, two missions, a cutscene and a theme.

### A zone's tables as JSON

Each of the zone's placement tables can be JSON, at its path with `.json` added:

```text
layers/base/files/StageData/MyGalaxy.arc/jmp/start/common/startinfo.json          where the player starts
layers/base/files/StageData/MyGalaxy.arc/jmp/list/stageinfo.json
layers/base/files/StageData/MyGalaxy.arc/jmp/placement/common/objinfo.json        objects in every mission
layers/base/files/StageData/MyGalaxy.arc/jmp/placement/common/planetobjinfo.json  gravity
layers/base/files/StageData/MyGalaxy.arc/jmp/placement/layera/objinfo.json        objects in layer A's missions
layers/base/files/StageData/MyGalaxy.arc/camera/cameraparam.bcam.json             cameras, if any
```

A table is its rows. A column whose numbers have fractions is a float on its own; one whose values all happen to be whole says so in `columns`:

```json
{ "columns": { "pos_x": "float", "pos_y": "float", "pos_z": "float" },
  "rows": [ { "name": "Kuribo", "l_id": 10, "pos_x": 0, "pos_y": 1510, "pos_z": 0 } ] }
```

A column nobody has a name for is written by its hash, as `lumalink extract` writes it: `"0xe4ef3738": "Normal"`. A zone without cameras uses the game's default camera.

### A planet

A planet is a model with solid ground, `ObjectData/MyPlanet.arc/MyPlanet.glb` and `MyPlanet.kcl.glb` ([models](02-models.md)), placed in `objinfo` by its name. The game makes planets of the names in `ObjectData/PlanetMapDataTable.arc/PlanetMapDataTable.bcsv`, so add a row:

```text
layers/base/merge/ObjectData/PlanetMapDataTable.arc/PlanetMapDataTable.bcsv.json
```

```json
{ "rows": [ { "PlanetName": "MyPlanet", "LowFlag": 0, "MiddleFlag": 0, "BloomFlag": 0, "WaterFlag": 0, "IndirectFlag": 0 } ] }
```

The flags say it has other models of its own beside it (`MyPlanetBloom`, `MyPlanetWater`, ...). Its pull is a gravity in `planetobjinfo`: `GlobalPointGravity` at its middle pulls towards it, as far out as its `Range`.

### A sky

A galaxy without one floats in black. Place one of the game's skies in `objinfo`, anywhere: it stays round the camera. `VROrbit` is the Good Egg's; the rest are the rows of class `Sky` or `ProjectionMapSky` in [objects](../reference/objects.md). To change how one looks, bring a model in its place, `ObjectData/VROrbit.arc/VROrbit.bdl.glb` ([models](02-models.md)).

```json
{ "name": "VROrbit", "l_id": 3, "pos_x": 0, "pos_y": 0, "pos_z": 0 }
```

### Missions

`MyGalaxyScenario.arc` holds `ScenarioData.bcsv`, one row per mission, and `ZoneList.bcsv`, one row per zone. Either can be `.bcsv.json`:

```json
{ "rows": [
  { "ScenarioNo": 1, "PowerStarId": 1, "MyGalaxy": 1, "ErrorCheck": 1, "IsHidden": 0, "Comet": "", "AppearPowerStarObj": "" },
  { "ScenarioNo": 2, "PowerStarId": 2, "MyGalaxy": 3, "ErrorCheck": 1, "IsHidden": 0, "Comet": "", "AppearPowerStarObj": "" }
] }
```

The column named after each zone holds that mission's layers, as bits: 1 is layer A, 2 layer B, 3 both.

There's no limit of eight stars. A galaxy with more shows six at a time on its star screen, with the page beside its name; left and right turn it, as does pressing at either side of the screen. The save keeps them all.

## The row

```json
"galaxies": [{
  "name": "MyGalaxy",
  "dome": 1,
  "map_position": [-60, -100],
  "stars_to_open": 0,
  "open_conditions": [],
  "hungry_luma": { "slot": 7, "star_bits": 400 }
}],
"save": { "mode": "own" },
"text": {
  "GalaxyName_MyGalaxy": "My Galaxy",
  "GalaxyNameShort_MyGalaxy": "My Galaxy",
  "ScenarioName_MyGalaxy1": "The First Star"
}
```

| Key | Meaning |
|---|---|
| `name` | Its folder's name |
| `dome` | Its dome, 1 to 6, or up to 16: see below. 0: the grand map |
| `map_position` | `[right, up]` from the middle of its dome's page, which is 608 by 456. The game makes the icon |
| `map_pane` | A pane of `MapGrandGalaxy` showing it, instead |
| `stars_to_open` | Power stars it takes |
| `open_conditions` | Up to three event flags |
| `hungry_luma` | The Luma that opens it: place a `TicoGalaxy` with its `Obj_arg7` at `slot`. The game's are 0 to 6. `star_bits` is the price |

A galaxy needs its own saves, as a save's shape follows the list of galaxies. `lumalink check` tells you if you forget.

## Getting there

A hungry Luma, a launch star, or a plugin: `game->change_stage("MyGalaxy", 1)`, or `game->open_scenario_select("MyGalaxy")` for its star screen.

To place the Luma in the Observatory:

```text
layers/base/merge/StageData/AstroGalaxy.arc/jmp/placement/common/objinfo.json
```

## The game's progression

```json
"event_flags": [
  { "name": "ExOpen1CocoonExGalaxy", "kind": 3, "a": 1, "b": 0, "first": "SandClockGalaxy" },
  { "name": "StarPieceCocoonExGalaxy", "kind": 8, "a": 5, "b": 80, "first": "CocoonExGalaxy" },
  { "name": "PowerStarComplete", "kind": 1, "a": 186 }
]
```

A name the game has is changed, a new one added. The numbers go past the Wii's 255.

| Kind | The row says | `a`, `b` | `first`, `second` |
|---|---|---|---|
| 1 | How many stars | `a`: the number | |
| 2 | A galaxy's stars to open it | | `first`: the galaxy |
| 3 | A special star was got | `a`: which | `first`: its galaxy |
| 4 | Two other rows are both on | | `first`, `second`: the rows |
| 8 | A hungry Luma | `a`: slot. `b`: price | `first`: what it opens |

Where galaxies are on the map and their star counts are the table `NebulaData/GalaxyInfoTable.bcsv` (or `.bcsv.json`), which a layer can replace or edit through `merge/`. So is `NebulaData/StoryProgressTable.bcsv`.

## Stars

A star's number comes from `PowerStarId` and the mission, as a `PowerStar` placed in the mission's layer or an object that gives one. Green and red stars, Luigi's stars and grand stars are `event_flags` rows named `SpecialStarGreen`, `SpecialStarRed`, `SpecialStarFindingLuigi` and `SpecialStarGrand`, of kind 3.

The total isn't fixed at 121. `PowerStarComplete` is only a row: set it to what your mod has.

## Domes past six

`dome` goes to 16, and `comet_schedule` with it. A dome past six is built by you: its hub objects (`AstroDome`, `AstroDomeEntrance`, `AstroDomeSky`, `AstroStarPlate`) take their dome from `Obj_arg0` and their model by its number, as `ObjectData/AstroDomeEntrance7.arc`. Comets past the sixth dome aren't kept in the save, and start over each play. It has no icon on the grand map unless you give it one with `map_pane`.

## Your own journey

To start at your galaxy instead of the game's prologue: [A total conversion](16-total-conversion.md).
