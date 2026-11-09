# Change a model, and make solid ground

## A model from glTF

Export a `.glb` or `.gltf` from Blender or anything else, and put it where the game's model goes, named as the archive is:

```text
layers/base/files/ObjectData/Kuribo.arc/Kuribo.glb
```

Nebula turns it into the game's own model when the player applies the mod.

| In the file | Becomes |
|---|---|
| Triangles | The model. A metre is 100 of the game's units |
| One colour and one PNG or JPEG per material | The material. The material's name is what [hand-written shaders](../reference/shaders.md) use |
| A texture's `KHR_texture_transform` | Its texture matrix |
| A skin | Skinned joints |
| Each animation | Files named after it, beside the model: `.bck` for what it does to the joints, and [for materials](#a-material-that-moves) `.bpk`, `.btk` and `.btp` |

A game's object often asks for animations by name. Run `lumalink files ObjectData/Kuribo.arc --game game.pak` to see what the original has, and name yours the same.

### How an animation plays

An animation loops unless its `extras` say otherwise. In Blender, that's a custom property on the action, exported with it:

```json
{ "name": "FlatDown", "extras": { "nebula": { "play": "once" } }, "channels": [ ... ] }
```

| `play` | The animation |
|---|---|
| `loop` (or none) | Starts again at its end |
| `once` | Stops on its last frame |
| `once_then_first` | Plays once, then shows its first frame |
| `there_and_back` | Plays once forwards, then backwards |
| `loop_there_and_back` | Forwards and backwards, over and over |

A model standing in for one of the game's ([a new object](08-new-objects.md)) plays each animation the original has as the original's does, whatever this says. Code that waits for an animation's end (a squashed Goomba waits for `FlatDown`) needs it to play once.

### A material that moves

An animation can change materials as well as joints, through `KHR_animation_pointer`:

| The animation moves | Becomes | For |
|---|---|---|
| `/materials/<n>/pbrMetallicRoughness/baseColorFactor` | `.bpk`, the material's colour | A flash, a glow, fading out |
| `/materials/<n>/pbrMetallicRoughness/baseColorTexture/extensions/KHR_texture_transform/offset`, `scale`, `rotation` | `.btk`, its texture's matrix | Flowing water, a scrolling belt, a spinning pattern |

A texture that turns needs equal scales across and up: glTF scales before turning and the game after, which come out the same only then.

Swapping one texture for another (eyes that blink, a face that changes) is the animation's `extras`:

```json
{ "name": "Blink",
  "extras": { "nebula": { "texture_swaps": [ { "material": 0, "keys": [[0, 3], [0.1, 4], [0.2, 3]] } ] } } }
```

Each key is `[second, texture]`: from that second, the material's base colour texture is glTF texture `texture`. The textures swapped in become part of the model even if no material starts with them. That becomes `.btp`.

An animation needs no joints to move: one with only these is only these files. The game's code asks for a material's animations by name as it does the joints': a Goomba's `Blink` is its `.btp`.

You can use the game's own formats too (`.bdl`, `.bmd`, `.bck`, `.btk`, `.brk`, `.bpk`, `.btp`, `.bva`) from any tool that writes them.

## Solid ground

What the player stands on is separate from what they see, so a platform can have a detailed look and a simple shape. Export the simple shape, and name it `.kcl.glb`:

```text
layers/base/files/ObjectData/MyPlatform.arc/MyPlatform.glb        what you see
layers/base/files/ObjectData/MyPlatform.arc/MyPlatform.kcl.glb    what you stand on
```

It becomes the game's `MyPlatform.kcl` and `MyPlatform.pa`. Faces point out the way they do in the model: counter-clockwise from the side you stand on.

### What the surface is

Each material of the collision model is one kind of surface. Its glTF `extras` set the columns of the game's attribute table:

```json
"materials": [ { "name": "Ice", "extras": { "Floor_code": 3, "Sound_code": 5 } } ]
```

| Key | What it is |
|---|---|
| `Floor_code` | What the floor does: slippery, sand, death |
| `Wall_code` | What a wall does |
| `Sound_code` | What steps sound like |
| `Camera_id` | The camera for standing there |
| `Camera_through` | Whether the camera passes through it |

To see which number means what, read what the game's own use: `lumalink extract ObjectData/SomeObject.arc --game game.pak -o out` gives each `.pa` as JSON.

A collision model holds at most 65,534 triangles and 65,536 distinct points: that's the form of the game's file. Past that, split it into parts: many objects, or many zones.

## An object that uses it

If you changed a game's object, you're done. For a new one, see [A new object, without code](08-new-objects.md).

## Mario

Mario's model is `ObjectData/Mario.arc`, Luigi's `ObjectData/Luigi.arc`, and each power-up's body has its own: `BeeMario`, `IceMario`, `HopperMario`, `InvincibleMario` and the Luigi versions. Keep the skeleton: the game's animations move joints by name. A costume mod is usually a texture and a model in those archives, with no code.
