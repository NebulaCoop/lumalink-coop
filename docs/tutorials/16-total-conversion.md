# A total conversion: your own game

Everything that's a part of the journey can be replaced.

## Where a game starts, and where it comes back to

```json
"story": {
  "first_stage": "MyFirstGalaxy",
  "first_mission": 1,
  "hub": "MyHub",
  "story_scenes": false
},
"save": { "mode": "own" }
```

| Key | Meaning |
|---|---|
| `first_stage`, `first_mission` | Where a new file begins, in place of the prologue |
| `hub` | Where the player goes after a galaxy, in place of the Comet Observatory and its domes |
| `story_scenes` | `false`: none of the game's own story scenes |

With a `hub`, a file with at least one star opens there; without one, at `first_stage`.

## Make your stages behave

```json
"stages": [
  { "stage": "MyHub", "acts_as": ["AstroGalaxy"] },
  { "stage": "MyFinalArena", "acts_as": ["KoopaBattleVs3Galaxy"] }
]
```

So the hub's counters stay up and its star bits count as a hub's, and the final star falls as Bowser's last does. See [Stage rules](../reference/stage-rules.md).

## The rest of the journey

| Part | How |
|---|---|
| Galaxies | [`galaxies`](09-new-galaxy.md), any number, any number of stars each |
| What opens what | [`event_flags`](09-new-galaxy.md#the-games-progression), any count |
| Comets | `comet_schedule`, `comet_time_limits` |
| The map | `map_position`, and `LayoutData/MapGrandGalaxy.arc` |
| Your hero | `ObjectData/Mario.arc`, `MarioAnime.arc`, and `player_constants` |
| The numbers | `max_lives`, `starting_lives`, `player_health`... |
| The title | `LayoutData` for its layouts, and the stage `FileSelect` |
| The intro and ending movies | `MovieData`, and `movies/` |
| The credits | `text`, and `StaffRoll` in the message files |
| Every word | `text`, or `Message.bmg` in each language |

## Keep what doesn't change

A mod carries only its own work, so a conversion can be small even if it changes everything: anything you don't replace comes from the player's own game. Don't include the game's files in a mod you publish.

## Test it

```console
lumalink check "My Game"
lumalink run "My Game"
```

Then start a new file: it's the only way to see `first_stage` and the first steps of your journey.
