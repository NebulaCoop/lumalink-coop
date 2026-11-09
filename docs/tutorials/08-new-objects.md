# A new object, without code

A new name that levels can place, behaving as one of the game's objects, with its own model.

## 1. Choose what it behaves as

[Objects](../reference/objects.md) lists everything. For a platform that just sits there, `FloaterLandPartsFrame` is a plain map object. For a Goomba with a new look, `Kuribo`.

## 2. Say so

```json
"layers": [ {
  "path": "layers/base",
  "object_classes": [ { "name": "BigStoneBlock", "like": "FloaterLandPartsFrame" } ]
} ]
```

## 3. Give it files

Its files are `ObjectData/<name>.arc`:

```text
layers/base/files/ObjectData/BigStoneBlock.arc/BigStoneBlock.glb
layers/base/files/ObjectData/BigStoneBlock.arc/BigStoneBlock.kcl.glb
```

If the object it's like asks for a model by its own name (a `Kuribo` asks for `Kuribo`), it gets yours instead. Animations are asked for by name: copy the original's names, with `lumalink files ObjectData/Kuribo.arc --game game.pak` to see them. Its joints too, where the original's code holds something by a joint (a Goomba's `Head`).

What your archive doesn't have comes from the original's:

| Missing from yours | What happens |
|---|---|
| A table (`actoranimctrl.bcsv`, `storydemoshadow.bcsv`, ...) | The original's is read |
| A sound animation (`walk.bas`: footsteps on frames) | The original's plays with your animation of that name |
| Effects (no `AutoEffectList` row for your name) | The original's: a squashed Bramblet gives off a Goomba's puff |
| An animation the code plays | Nothing plays for it, and nothing breaks: a model with no colour animations (`.brk`) is still a Toad |

An animation standing for one of the original's plays as that one does: once (a Goomba's `FlatDown`), or looping (`Walk`). The code waits for the end of those it plays once, so this matters. To say it yourself, see [models](02-models.md#how-an-animation-plays).

`examples/new-galaxy` makes a new enemy this way: `Bramblet`, like `Kuribo`, its model and 28 animations from one glTF file.

## 4. Place it

```text
layers/base/merge/StageData/EggStarGalaxy.arc/jmp/placement/common/objinfo.json
```

```json
{ "rows": [ {
  "name": "BigStoneBlock", "l_id": 3200,
  "pos_x": -2200, "pos_y": -12700, "pos_z": -14500,
  "dir_x": 0, "dir_y": 0, "dir_z": 0, "scale_x": 1, "scale_y": 1, "scale_z": 1,
  "Obj_arg0": -1, "Obj_arg1": -1, "Obj_arg2": -1, "Obj_arg3": -1,
  "Obj_arg4": -1, "Obj_arg5": -1, "Obj_arg6": -1, "Obj_arg7": -1,
  "SW_A": -1, "SW_B": -1, "SW_APPEAR": -1, "SW_DEAD": -1, "SW_SLEEP": -1,
  "CommonPath_ID": -1, "ClippingGroupId": -1, "GroupId": -1, "DemoGroupId": -1,
  "MapParts_ID": -1, "ViewGroupId": -1, "CameraSetId": -1, "CastId": -1,
  "MessageId": -1, "ShapeModelNo": -1
} ] }
```

The player stands on it, and it draws, sounds and counts as what it's like.

## What it can and can't change

The look, the shape, the sounds in its archive and any table in its archive are yours, and so are effects given in an `AutoEffectList` row of its name. What it does is the original's. Its arguments mean what the original's do. For behaviour of its own, [a plugin](10-first-plugin.md) makes a new object in a few lines.

## If a file is missing

With any mod on, a level placing an object whose archive isn't there leaves that object out, and the log says so. The level goes on.
