# Using mods

## Installing one

**A `.luma` file.** Double-click it, or in Nebula's settings choose Mods, then Import. It appears in the list. Turn it on and press Apply.

**A mod as you downloaded it** (a zip, 7z, rar or a folder, made for the Wii). Open LumaLink, drop it on the window, tick it, and press Convert. Open the `.luma` it makes.

**On a phone.** Move the `.luma` to the phone and import it from the app. A raw mod folder can't be used on a phone: convert it on a computer first.

## The list

The Mods page has two lists: Disabled and Enabled (top has priority). Mods apply from the bottom of the enabled list up, so the one on top wins where two change the same file, texture or setting. Drag a mod to move it, or select it, press A (Space on a keyboard) to pick it up, move it with the arrows (left turns it off, right on) and press A again to put it down. Files many mods add to, such as a level's objects, keep everyone's additions. Text, settings and table rows follow the same order: the higher mod's word is the one used.

The line under a selected mod says what it changes and where it overlaps another mod: OVERRIDES: {NAME} (3 FILES) means it wins those, OVERRIDDEN BY: {NAME} means the other one does.

Changes wait until you press Apply. Mods that change the game's files apply when the game restarts, and Apply offers to restart; texture packs and movies apply at once.

## A mod's page

Select a mod and press Details. Its page shows:

| Line | What it is |
|---|---|
| Status | On (its place in the list) or off; left and right turn it on or off. The line under it says why a mod can't be used. |
| Changes | What kinds of things it changes: files, settings, textures, movies, code. The line under it lists every overlap. |
| Version, By, License, Website | What its maker wrote in the mod. |
| Folder | Its folder in the mods folder. |
| Options | Its choices, the text it shows in a language it lacks, a texture pack's categories. |

About shows its description, then its icon and pictures, a page at a time. Remove removes it (to the Recycle Bin on Windows) after asking; a mod the game started with is removed after a restart without it.

## Updating

Importing a `.luma` whose `id` is already in the mods folder replaces that mod, whatever its folder is called, and keeps its place, its options and its saves. The message says which: Updated: {name} ({old} to {new}), Older version, or Replaced (the same version). A different mod that happens to have the same folder name is left alone; the new one gets a folder of its own.

Two folders holding the same mod (a copy made by hand) can't both be on: the lower one says ANOTHER COPY OF THIS MOD IS ON.

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

It stays on the list, off, with one line saying why. See [Messages](REFERENCE.md#17-messages-and-what-they-mean).

## When something's wrong

- **A new line in the log says `mods: left out:`.** Part of the mod couldn't be used, and the rest is running. The line says which part and why. The log is `nebula.log` in the settings folder.
- **It crashes.** Turn off the mods one at a time, starting from the top of the list, to find which one. If it's a plugin, the crash report names it.
- **Sounds or shaders take a moment the first time.** A mod's shaders are built on your device on first use, with a progress screen. After that, it's instant.
