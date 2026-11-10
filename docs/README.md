# Modding Super Mario Galaxy with Nebula

Nebula is Super Mario Galaxy rebuilt as a native program. LumaLink makes mods for it: it converts what people have already made, and it builds new ones. A mod is one `.luma` file that works on every version of the game and every device.

A mod can change files, levels, objects, the player's physics, power-ups, the star count, the story's order, music, text, shaders and cutscenes, and add new enemies, characters and galaxies. Most of it needs no code; the plugin interface covers the rest.

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
| [`luma.json`](format.md) | Every key of a mod |
| [Files](REFERENCE.md#7-files-models-textures-sound) | Every kind of file, what becomes what, where each goes |
| [Tables](REFERENCE.md#8-levels-and-tables) | Levels and data tables: adding, changing, removing rows |
| [Game settings](reference/settings.md) | Every setting a layer can set |
| [Player numbers](reference/player-numbers.md) | Every number of how the player moves |
| [Objects](reference/objects.md) | Every object of the game |
| [Stage rules](reference/stage-rules.md) | The rules the game keeps by a stage's name |
| [Plugin interface](reference/plugin-api.md) | Every call, event and number |
| [Hooks and the game's headers](REFERENCE.md#13-plugins-the-games-headers-and-hooks) | Reaching any function of the game |
| [Shaders](REFERENCE.md#15-shaders) | Hand-written materials |
| [LumaLink commands](REFERENCE.md#3-lumalink) | The command line and the window |
| [What conversion reads](REFERENCE.md#4-what-conversion-does) | How each thing in a Wii mod becomes something in Nebula |
| [Limits that are gone](REFERENCE.md#14-limits-that-are-gone) | What the Wii decided, and what Nebula does |
| [Messages](REFERENCE.md#17-messages-and-what-they-mean) | Every message, what it means, what to do |
| [Questions](REFERENCE.md#18-questions) | |

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
| `gold-flower` | A new power-up: its item, model, particles, sounds, spin animation, fire damage and higher jump |
| `new-galaxy` | A whole galaxy: planet, ground, gravity, a new enemy, stars, a cutscene with a glTF camera |
| `new-npc` | A new character made like a Toad, with its own model and talk |
| `new-talk` | A branching yes/no talk on one of the game's Toads |
| `new-voice` | The player's voices from the mod's own sounds |
