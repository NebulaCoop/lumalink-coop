# Change a level

A stage is tables: which objects stand where, what its zones are, where the player starts.

## Where things are

```text
StageData/<Galaxy>/<Galaxy>Scenario.arc      its missions and zones
StageData/<Zone>.arc                         one zone: placement, paths, cameras, gravity
StageData/<Zone>.arc/jmp/placement/common/objinfo      objects in every mission
StageData/<Zone>.arc/jmp/placement/layera/objinfo      objects when layer A is on
```

Look at one:

```console
lumalink files StageData/EggStarGalaxy.arc --game game.pak
lumalink extract StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo --game game.pak -o out
```

That writes the table, and beside it `objinfo.json`, each row as words and numbers.

## With a level editor

Tools such as Whitehole read and write the game's own files. Save your level, and put the archives in `files/`:

```text
layers/base/files/StageData/MyZone.arc
```

Or keep the disc's folders as the tool writes them; LumaLink finds them by their names. This replaces the whole file, so only one mod's survives. For one mod that's fine.

## With `merge/`: add, change, remove

A file in a layer's `merge/` folder edits a table, and any number of mods can edit one table. The path goes inside the archive, and the name ends in `.json`.

```text
layers/base/merge/StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo.json
```

```json
{
  "remove": [ { "name": "EarthenPipe" } ],
  "change": [
    { "where": { "name": "SuperSpinDriver" }, "set": { "scale_x": 2.0, "scale_y": 2.0, "scale_z": 2.0 } }
  ],
  "rows": [
    {
      "name": "Coin", "l_id": 900,
      "pos_x": -2200, "pos_y": -12700, "pos_z": -14500,
      "dir_x": 0, "dir_y": 0, "dir_z": 0,
      "scale_x": 1, "scale_y": 1, "scale_z": 1,
      "Obj_arg0": -1, "Obj_arg1": -1, "Obj_arg2": -1, "Obj_arg3": -1,
      "Obj_arg4": -1, "Obj_arg5": -1, "Obj_arg6": -1, "Obj_arg7": -1,
      "SW_A": -1, "SW_B": -1, "SW_APPEAR": -1, "SW_DEAD": -1, "SW_SLEEP": -1,
      "CommonPath_ID": -1, "ClippingGroupId": -1, "GroupId": -1, "DemoGroupId": -1,
      "MapParts_ID": -1, "ViewGroupId": -1, "CameraSetId": -1, "CastId": -1,
      "MessageId": -1, "ShapeModelNo": -1
    }
  ]
}
```

They run in that order: remove, change, add. A row that matches nothing is not an error, since another mod may have taken it. See [Tables](../REFERENCE.md#8-levels-and-tables).

In placement tables, "no switch" and "no argument" are `-1`, so write those out. A column you leave out is zero, which means "switch number 0": a real one.

## What a row means

| Column | Meaning |
|---|---|
| `name` | The object: see [Objects](../reference/objects.md) |
| `l_id` | Its number, unique in the table. Use high numbers for your own, 3000 and up |
| `pos_*`, `dir_*`, `scale_*` | Where it is, turned in degrees, its size |
| `Obj_arg0`..`7` | Numbers that mean what the object says |
| `SW_A`, `SW_B`, `SW_APPEAR`, `SW_DEAD`, `SW_SLEEP` | Switches: how objects talk to each other |
| `CommonPath_ID` | Its rail |
| `MessageId` | What it says |
| `GroupId`, `ClippingGroupId` | Objects that belong together |

The easiest way to learn an object's arguments: find one in a level with `extract`, and copy its row.

## Missions and layers

A galaxy's `ScenarioData` table says, per mission, which layers of each zone are on: a column named after the zone, holding bits. Bit 0 is `LayerA`. A zone has 32 layers now, `LayerA` to `LayerZ` then `LayerAA` to `LayerAF`, not the Wii's 16.

```text
StageData/MyGalaxy/MyGalaxyScenario.arc/scenariodata.bcsv.json
```

| Column | Meaning |
|---|---|
| `ScenarioNo` | The mission's number |
| `PowerStarId` | The star's number in the stage |
| `IsHidden` | 1: hidden until found |
| `Comet` | `Red`, `Dark`, `Blue` etc: a comet mission |
| `<ZoneName>` | Bits: the layers of that zone on in this mission |

More than eight stars in a galaxy is fine. The star-choosing screen shows pages of six, the save keeps them all, and left and right turn the page. See [A new galaxy](09-new-galaxy.md).

## Stage rules

Some rules go by a stage's name. To make your stage a hub, a Bowser arena or an ocean level, see [Stage rules](../reference/stage-rules.md).
