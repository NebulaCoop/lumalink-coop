# Give players options

## Choices

```json
"options": [{
  "id": "difficulty",
  "name": "Difficulty",
  "description": "How hard it hits",
  "default": "normal",
  "choices": [
    { "id": "normal", "name": "Normal" },
    { "id": "hard", "name": "Hard" }
  ]
}],
"layers": [
  { "path": "layers/base" },
  { "when": { "option": { "difficulty": ["hard"] } }, "tunables": { "MarioActor.cHitTakesAll": true } }
]
```

Players choose on the Mods page. The choice lasts through updates to the mod. `off` as a choice's id is shown in the player's own language and means this adds nothing.

## Numbers

A range is a choice for each number:

```json
"options": [{
  "id": "health", "name": "Health", "default": 6,
  "range": { "from": 1, "to": 20, "step": 1 }
}],
"layers": [{
  "tunables": { "GameRules.cPlayerHealth": { "option": "health" } },
  "player_constants": { "both": { "mFireModeTime": { "option": "fire-time" } } }
}]
```

A tunable or a player number takes the chosen number directly, with no layer per value.

## `when`

A layer is used when all of its `when` hold.

| Key | Holds when |
|---|---|
| `version` | The game is one of these: `["E", "P", "J", "K", "W", "C"]` |
| `language` | The language folder being played is one of these: `["UsEnglish", "EuFrench"]` |
| `platform` | The device is one of `windows`, `android`, `linux`, `macos`, `ios` |
| `option` | Each named option is on one of its listed choices |
| `mods` | Each of these mods is on: a patch that makes two mods work together |
| `not` | This isn't so. One `when`, or a list of them, none of which may hold |
| `any` | A list of `when`: one of them holds |

```json
{ "path": "layers/with-green-stars", "when": { "mods": ["green-stars"], "not": { "platform": ["android"] } } }
```

A key this Nebula doesn't know makes it false, so a mod made for a later version loses the layer rather than breaking.

## Anything can be switched

Everything a mod can say at its top level can also go in a layer: `text`, `music`, `galaxies`, `event_flags`, `comet_schedule`, `story`. So an option can turn on a new galaxy, a soundtrack or a translation.

## In a plugin

```c
const char* mode = game->option(me, "difficulty");  /* its choice's id */
```

A plugin's own row can have a `when` as a layer does: only its code for that choice runs.
