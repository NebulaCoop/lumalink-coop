# Using mods

## Installing one

**A `.luma` file.** Double-click it, or in Nebula's settings choose Mods, then Import. It appears in the list. Turn it on and press Apply.

**A mod as you downloaded it** (a zip, 7z, rar or a folder, made for the Wii). Open LumaLink, drop it on the window, tick it, and press Convert. Open the `.luma` it makes.

**On a phone.** Move the `.luma` to the phone and import it from the app. A raw mod folder can't be used on a phone: convert it on a computer first.

## The list

Mods apply from the bottom of the list up, so the one on top wins where two change the same file. Drag to reorder. Files many mods add to, such as a level's objects, keep everyone's additions. Text, settings and rows follow the same order: the higher mod's word is the one used.

A mod's options are on its row. A change takes effect at the next reset: Reset with mods changed restarts the game by itself.

## Mods with code

Some mods hold their own compiled code. The game asks before that code first runs, and again whenever it changes. It runs with the same rights as the game, so turn it on only for mods you trust. If the game crashes in a plugin, the report names the mod and the next start offers to turn its code off.

## Saves

A mod says which saves it plays on.

| It says | You get |
|---|---|
| `shared` | Your normal saves |
| `own` | Saves of its own, starting new or from what the mod brings |
| `copy` | Saves of its own, starting as a copy of yours. Yours stay as they are |

A mod that adds a galaxy always has its own, since a save's shape follows the list of galaxies. Saves of its own outlast updating or removing the mod.

## When a mod can't be used

It stays on the list, off, with one line saying why. See [Messages](reference/messages.md).

## When something's wrong

- **A new line in the log says `mods: left out:`.** Part of the mod couldn't be used, and the rest is running. The line says which part and why. The log is `nebula.log` in the settings folder.
- **It crashes.** Turn off the mods one at a time, starting from the top of the list, to find which one. If it's a plugin, the crash report names it.
- **Sounds or shaders take a moment the first time.** A mod's shaders are built on your device on first use, with a progress screen. After that, it's instant.
