# Modding Super Mario Galaxy with Nebula

Nebula is Super Mario Galaxy rebuilt as a native program. LumaLink makes mods for it: it converts what people have already made, and it builds new ones. A mod is one `.luma` file that works on every version of the game and every device.

There's almost nothing a mod can't change. Files, levels, objects, the player's physics, power-ups, the star count, the story's order, music, text, shaders and whole new enemies, with no code at all for most of it and a full plugin interface for the rest.

## Where to start

| You want to | Read |
|---|---|
| Use a mod someone made | [Using mods](using-mods.md) |
| Convert a mod made for the Wii | [Converting a mod](tutorials/15-converting.md) |
| Make your first mod in ten minutes | [Start here](start-here.md) |
| Understand how the pieces fit | [How mods work](concepts.md) |
| Look up an exact name or key | [Reference](#reference) |

## Tutorials

Each one starts from nothing and ends with something running in the game.

| | Tutorial | No code |
|---|---|---|
| 1 | [Change a texture](tutorials/01-textures.md) | ✓ |
| 2 | [Change a model, and make solid ground](tutorials/02-models.md) | ✓ |
| 3 | [Change a level](tutorials/03-levels.md) | ✓ |
| 4 | [Music and sound](tutorials/04-music-and-sound.md) | ✓ |
| 5 | [Text, and other languages](tutorials/05-text.md) | ✓ |
| 6 | [Change how the game plays](tutorials/06-rules-and-physics.md) | ✓ |
| 7 | [Give players options](tutorials/07-options.md) | ✓ |
| 8 | [A new object, without code](tutorials/08-new-objects.md) | ✓ |
| 9 | [A new galaxy](tutorials/09-new-galaxy.md) | ✓ |
| 10 | [Your first plugin](tutorials/10-first-plugin.md) | |
| 11 | [A whole boss](tutorials/11-a-boss.md) | |
| 12 | [A new power-up](tutorials/12-power-ups.md) | |
| 13 | [Your own screen: counters and pop-ups](tutorials/13-screens.md) | |
| 14 | [Cutscenes, movies and Lumas](tutorials/14-cutscenes-movies-lumas.md) | ✓ |
| 15 | [Converting a mod made for the Wii](tutorials/15-converting.md) | ✓ |
| 16 | [A total conversion: your own game](tutorials/16-total-conversion.md) | ✓ |
| 17 | [Publishing](tutorials/17-publishing.md) | ✓ |

## Reference

| Page | What's in it |
|---|---|
| [`luma.json`](reference/luma-json.md) | Every key of a mod |
| [Files](reference/files.md) | Every kind of file, what becomes what, where each goes |
| [Tables](reference/tables.md) | Levels and data tables: adding, changing, removing rows |
| [Game settings](reference/settings.md) | Every setting a layer can set |
| [Player numbers](reference/player-numbers.md) | Every number of how the player moves |
| [Objects](reference/objects.md) | Every object of the game |
| [Stage rules](reference/stage-rules.md) | The rules the game keeps by a stage's name |
| [Plugin interface](reference/plugin-api.md) | Every call, event and number |
| [Hooks and the game's headers](reference/hooks.md) | Reaching any function of the game |
| [Shaders](reference/shaders.md) | Hand-written materials |
| [LumaLink commands](reference/commands.md) | The command line and the window |
| [What conversion reads](reference/conversion.md) | How each thing in a Wii mod becomes something in Nebula |
| [Limits that are gone](reference/limits.md) | What the Wii decided, and what Nebula does |
| [Messages](reference/messages.md) | Every message, what it means, what to do |
| [Questions](faq.md) | |

## Examples

[`examples/`](../examples) holds a working mod for each idea. Each builds with `lumalink pack`.

| Example | Shows |
|---|---|
| `new-text` | Messages in several languages |
| `interface-picture` | A picture in the game's interface, from a PNG |
| `stage-music` | New music in a stage, per mission and per area |
| `new-cutscene` | A whole cutscene from tables |
| `one-health`, `coin-kills`, `boss-music`, `double-jump` | Small plugins |
| `little-boss` | A three-phase boss: model, music, placement, attacks |
