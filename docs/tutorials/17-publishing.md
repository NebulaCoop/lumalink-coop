# Publishing

Put the `.luma` anywhere people can download it. Before you do:

- Run `lumalink check` on the finished file. It reads what the game reads, and names every mistake it can find.
- Give it a real `version`. A file with the same `id` and a higher version updates the mod and keeps the player's settings. Numbers with dots, compared number by number: `2.10` is above `2.9`.
- Say who made it: `authors`, `license`, `homepage`, an `icon`.
- Bring only your own work. Name archives as folders so the player's game supplies the rest. A mod carrying the game's files is larger than it needs to be, and isn't yours to share.
- If your mod has code, build it for each device it should run on: `binaries` lists them. On a device with none, the mod says so and stays off.
- Try it with no other mods on, on a new file, from the start.

## Working with other mods

```json
"needs": [ { "id": "other-mod", "version": "1.2.0" } ],
"conflicts": [ { "id": "old-thing", "below": "2.0.0" } ],
"load_after": [ "other-mod" ],
"load_before": [ "another-mod" ]
```

| Key | Meaning |
|---|---|
| `needs` | Off until that mod is on, at that version or later |
| `conflicts` | Off while that mod is on. Without `below`: every version |
| `load_after`, `load_before` | Where it goes in the list when first added. The player's order then wins |

Use `merge/` in place of replacing a whole level or table, so yours and others' additions are all kept. A layer with `"when": { "mods": ["other-mod"] }` carries a patch for only when both are on.

## Things only newer versions have

```json
"nebula": "2.1.0",
"requires": ["custom-shaders"]
```

`nebula` is the oldest Nebula it works on. A key this Nebula doesn't know is ignored; a mod that can't work without something names it in `requires`, and a Nebula without that says so rather than half-applying it. `format` only changes if neither can hold, and Nebula reads every format it has ever read.

## What players see

Its name, icon, description, authors and options are on its row of the Mods page, in the player's language where you've given one. See [`luma.json`](../format.md).
