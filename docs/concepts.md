# How mods work

## The pieces

| Piece | What it is | Where it runs |
|---|---|---|
| **Nebula** | The game, built to run natively. It reads `.luma` files | Every device |
| **LumaLink** | The converter and modding tool | A computer |
| **`.luma`** | One mod in one file, for every version of the game and every device | Anywhere |
| **Plugin** | A mod's own compiled code, inside a `.luma` | Inside Nebula |

A mod is made once. Nothing about it depends on one build of Nebula: the game prepares what it needs when the mod is first used, and again if its own needs change.

## Inside a `.luma`

A zip file with `luma.json` as its first entry. Any zip tool can make one, and `lumalink unpack` opens one.

```text
My Mod.luma
├── luma.json                what the mod is
├── icon.png                 optional, square
├── layers/
│   └── base/
│       ├── files/           files by their place in the game
│       ├── merge/           what it adds to, or changes in, files many mods share
│       ├── textures/        replacement textures
│       └── movies/          replacement videos
├── save/                    optional, starting files for its own saves
├── shaders/                 optional, ready-built shaders
└── plugins/                 optional, the mod's code
```

## The ways to change the game

From the simplest to the most powerful. Use the first one that does what you want.

| Way | What it changes | Two mods can both do it? |
|---|---|---|
| **Settings** (`switches`) | Numbers and rules: health, lives, caps | Yes, the higher wins |
| **Player numbers** (`player_constants`) | How the player moves, how long power-ups last | Yes, per number |
| **Rows** (`music`, `text`, `event_flags`, `stages`, `object_classes`, ...) | The game's tables: what plays where, what opens what | Yes, per row |
| **Merge** (`merge/`) | Rows in levels and tables: add, change, remove | Yes, all kept |
| **Files** (`files/`) | Anything that's a file: models, textures, levels, sounds | The higher replaces |
| **Plugin, stable interface** | Anything, through 140 plain C calls and events. Survives Nebula updates | Yes |
| **Plugin, game's headers and hooks** | Any function of the game | Yes, in a chain |

## Layers and options

A layer is a folder and when to use it. Layers apply in order, and later ones win. Everything that differs by version, language, device or the player's choice is a layer, so one file carries it all.

```json
"layers": [
  { "path": "layers/base" },
  { "path": "layers/hard", "when": { "option": { "difficulty": ["hard"] } } },
  { "path": "layers/japanese", "when": { "version": ["J"] } }
]
```

Everything a mod can say can go in a layer, so an option can turn on a galaxy, a track, a setting or a line of text.

## With no mods on

Nebula with no mods is the game as it always was. Some changes apply only once any mod is on: every power-up's model and every sound bank are then loaded on every stage, so an object works wherever a mod puts it. That costs a little memory, and no one playing without mods pays for it.

## Why there's no code from the Wii

Wii mods change the game by writing over its program. Nebula isn't that program. So LumaLink reads each write for what it changes in the game, and turns it into a setting: a write over the player's table becomes `player_constants`, a write over the object table becomes `object_changes`. Any mod writing any value there works, since the value is read out of the bytes. No code from a mod's patch ever runs. A write that LumaLink can't read stays on a list, and everything else in the mod still works. See [What conversion reads](reference/conversion.md).
