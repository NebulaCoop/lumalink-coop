# Change a texture

Three ways, depending on what you're changing.

| You want to change | Do this |
|---|---|
| A texture wherever it's used, by its look | A texture pack: `tex1_` files in `layers/base/textures/` |
| A texture file in one object's archive | `layers/base/files/ObjectData/Thing.arc/name.png` |
| A picture in the interface | `layers/base/files/LayoutData/CoinCounter.arc/timg/name.png` |

## A texture pack

Any Dolphin texture pack works as it is: its `tex1_` files go in `layers/base/textures/`, in any folders. PNG and DDS both work, mipmaps too, at any size. To make one, dump the game's textures in Dolphin, edit what you want, and keep only those.

A pack for the Wii version converts without changes: drop it on LumaLink. Players can turn a pack's categories on and off on the Mods page.

## One object's texture

```text
layers/base/files/ObjectData/Kuribo.arc/kuribo_body.png
```

A folder named as an archive changes that archive file by file, and the rest comes from the player's own game. The PNG becomes the game's `.bti` when the player applies the mod.

To see what an archive holds:

```console
lumalink files ObjectData/Kuribo.arc --game path/to/game.pak
```

Names are the files in the archive, in any case, without the ending.

## A picture in the interface

The interface keeps its pictures in another form, in a folder named `timg`. A PNG there becomes that form, under the name of the picture it replaces. See [`examples/interface-picture`](../../examples/interface-picture), which turns the coin counter's coin and × blue.

```text
layers/base/files/LayoutData/CoinCounter.arc/timg/myiconcoin.png
```

`lumalink files LayoutData/CoinCounter.arc --game game.pak` lists an archive's pictures by name.

## Sizes

Any size. The game's own limit of 1024 doesn't apply to files from mods. Textures larger than the original draw sharper; they don't need the same proportions, but the model's coordinates decide how they map.

## Running it

```console
lumalink check "My Mod"
lumalink run "My Mod"
```
