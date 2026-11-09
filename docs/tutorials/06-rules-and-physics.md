# Change how the game plays

No code for any of this. All of it goes in a layer, so an [option](07-options.md) can turn any of it on.

## Tunables

Every tuning number in the game is a tunable, named after its source file and its name. The mod says
`"requires": ["tunables"]`, and a layer sets them:

```json
"tunables": {
  "GameRules.cPlayerHealth": 6,
  "GameRules.cLifeUpHealth": 12,
  "GameDataPlayerStatus.cMaxPlayerLeft": 999,
  "GameDataPlayerStatus.cStartingPlayerLeft": 20,
  "ScenePlayingResult.cCoinStepNum": 100,
  "ScenePlayingResult.cStarPieceStepNum": 0,
  "GameRules.cMaxCoins": 5000,
  "ScenePlayingResult.cPurpleCoinPowerStarAppearNum": 50,
  "MarioActor.cWaterAir": 16
}
```

Health can be anything: the meter's six parts show it in proportion and the number is exact. Numbers past what the game shows stay on one line. See [the game's rules and how to list every tunable](../reference/settings.md).

## How the player moves

```json
"player_constants": {
  "mario": { "mJumpHeight": [null, null, 60.0] },
  "luigi": { "mJumpHeight": [null, null, 45.0] },
  "both": { "mFireModeTime": 2400, "mIceModeTime": 2400 }
}
```

Mario and Luigi each have a table of 493 numbers. The ones people reach for first:

| Name | What it is |
|---|---|
| `mJumpHeight` | Each kind of jump's height. 0 normal, 1 second, 2 triple |
| `mGravityJumping` | Gravity while rising, per kind of jump |
| `mJumpFrontSpeed` | Speed forward in a jump |
| `mMaxDropSpeed` | How fast the player falls |
| `mHardLandingHeight` | The fall that hurts |
| `mFireModeTime`, `mIceModeTime`, `mMetalHoldTime`, `mTornadoHoldTime`, `mRacketHoldTime` | How long each power-up lasts, in frames |

[All of them](../reference/player-numbers.md), with what each table has.

## Which objects exist

One of the game's objects becomes another, or is gone, wherever the game makes it.

```json
"object_changes": [
  { "object": "RestartCube" },
  { "object": "BenefitItemLifeUp", "becomes": "BenefitItemOneUp" },
  { "object": "KinokoLifeUp", "becomes": "KinokoOneUp", "in": "treasure_boxes" },
  { "object": "KinokoLifeUp", "becomes": "KinokoOneUp", "in": "display_models" }
]
```

Removing `RestartCube` removes every checkpoint in the game. See [`luma.json`](../reference/luma-json.md#object_changes).

## Comets

```json
"comet_time_limits": [ { "galaxy": "BattleShipGalaxy", "mission": 4, "seconds": 150 } ],
"comet_schedule": [ { "dome": 2, "comets": [ { "galaxy": "BattleShipGalaxy", "coins": 1 } ] } ]
```

## What opens what

Event flags are the game's progression: what each dome needs, what each hungry Luma costs. See [A new galaxy](09-new-galaxy.md#the-games-progression).

## Rules that go by a stage's name

```json
"stages": [ { "stage": "OceanRingGalaxy", "own_rules": false } ]
```

Takes the swim-angle limit off the ocean. See [Stage rules](../reference/stage-rules.md).

## One thing to keep in mind

A mod that changes what's needed to reach things is a different journey through the game: give it its own saves with `"save": { "mode": "own" }`.
