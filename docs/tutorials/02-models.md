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
| One colour and one PNG per material | The material. The material's name is what [hand-written shaders](../reference/shaders.md) use |
| A skin | Skinned joints |
| Each animation | One `.bck` file named after it, beside the model |

A game's object often asks for animations by name. Run `lumalink files ObjectData/Kuribo.arc --game game.pak` to see what the original has, and name yours the same.

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
