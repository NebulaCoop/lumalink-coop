# Game settings (tunables)

Every tuning number the game has (a walk speed, a timer, a health count, thousands of them) is a **tunable**, named
after the game's source file:

| Key | Is |
|---|---|
| `"<file stem>.<name>"` | A named number: `GameRules.cPlayerHealth`, how many hits Mario takes |
| `"<file stem>.<function>.<k>"` | The k-th step count (frames) in a function: `Kuribo.exeFlatDown.1`, how long a flattened Goomba stays |
| `"<file stem>.<function>.<call>.<k>"` | The k-th number a function gives a motion, animation or reach call: `Kuribo.tryActive.isNearPlayerAnyTime.1`, how near the player a Goomba wakes |
| `"<file stem>.<function>.f<k>"` | The k-th other number written in a function: `Kuribo.exeAppearFromBox.f2`, how fast a Goomba jumps out of a box |
| `"<file stem>.<name>"` (a list) | A record of numbers, set as a list in order: `BossBegoman.hWaitParam`, `[speed, gravity, damping, turn]` |

A layer's `tunables` sets them, and the mod
says `"requires": ["tunables"]`. A value is a number, `true` or `false`, a list for a list of numbers (`null` keeps a
place), or `{ "option": "id" }` to take the number from one of the mod's number options. Where two mods set one, the mod
higher on the list wins. A plugin reads one with `get_tunable` and sets one with `set_tunable`.

```json
"requires": ["tunables"],
"layers": [{ "tunables": { "GameRules.cPlayerHealth": 6, "GameDataPlayerStatus.cMaxPlayerLeft": 999, "ScenePlayingResult.cCoinStepNum": 100 } }]
```

The whole list, with the game's value for each, is written by the game itself: start Nebula with
`--dump-tunables <file>`. `lumalink list settings` prints the game's rules below.

| Tunable | The game has | What it does |
|---|---|---|
| `GameRules.cPlayerHealth` | 3 | The health a player starts with, and the most before a Life Up |
| `MarioActor.cDarkCometOneHealth` | true | Daredevil comets leave the player one health |
| `MarioActor.cHitTakesAll` | false | A hit takes all the player's health |
| `GameRules.cLifeUpHealth` | 6 | The most health a Life Up gives. 0: none |
| `MarioEffect.cLowHealthWarning` | true | The warning sound at low health or air |
| `EventUtil.cDarkCometRules` | true | Daredevil comets keep their rules; false: a comet like any other |
| `GameDataPlayerStatus.cMaxPlayerLeft` | 99 | The most lives |
| `GameDataPlayerStatus.cStartingPlayerLeft` | 4 | The lives a file starts and loads with |
| `GameRules.cMaxStarBits` | 9999 | The most star bits kept in the save |
| `GameDataTemporaryInGalaxy.cMaxCarriedStarPieceNum` | 999 | The most star bits carried in a stage |
| `GameRules.cMaxCoins` | 999 | The most coins in a stage |
| `ScenePlayingResult.cCoinStepNum` | 50 | Coins for a 1-up. 0: none |
| `ScenePlayingResult.cStarPieceStepNum` | 50 | Star bits for a 1-up. 0: none |
| `ScenePlayingResult.cPurpleCoinPowerStarAppearNum` | 100 | Purple coins that bring the star |
| `MarioActor.cWaterAir` | 8 | The air the player has under water |
| `SequenceUtil.cMissStarPieceNum` | 0 | Star bits added after a miss, negative to take. -999: all lost |
| `FlipPanel.cPanelGroupSize` | 32 | How many flip panels a group holds |
| `FlipPanel.cObserverGroupSize` | 32 | The same for their observers |
| `WaterBazooka.cShotInvalidSpinKill` | false | A spin doesn't break the water bazooka's shots |
| `BossKameckBarrier.cOffsetY` | 0 | Added to Kamek's barrier's height |
| `SkeletalFishBaby.cBreakAirBubble` | true | A hit skeletal fish leaves an air bubble |
| `GameRules.cDodoryuShellOnly` | false | Dodoryu is hurt by shells alone |
| `SceneUtil.cHiddenStarTable` | true | Hidden stars take their mission from the game's table |
| `SceneUtil.cSuddenDeathDodoryu` | true | Dodoryu's one-hit run in CosmosGardenGalaxy's fourth mission |
| `GameRules.cBegomanFireProof` | false | Begomans survive fire ground |
| `KinopioAstro.cMessangerAlwaysEx` | false | The 1-up messenger always gives its larger supply |
| `KinopioAstro.sMessangerOneUpExNum` | 20 | How many 1-ups that is |

## From `switches`

Earlier mods named these rules in a layer's `switches`. Nebula and `lumalink check` refuse `switches` now, naming the
tunable for each; LumaLink writes tunables when it converts a console mod. A switch whose sense was "no ..." is the
opposite value of its tunable: `"no_low_health_warning": 1` is `"MarioEffect.cLowHealthWarning": false`.

| Switch | Tunable |
|---|---|
| `player_health`, `life_up_health` | `GameRules.cPlayerHealth`, `GameRules.cLifeUpHealth` |
| `player_health_fixed` | `MarioActor.cDarkCometOneHealth`, opposite |
| `hit_takes_all`, `water_air` | `MarioActor.cHitTakesAll`, `MarioActor.cWaterAir` |
| `no_low_health_warning` | `MarioEffect.cLowHealthWarning`, opposite |
| `no_dark_comet_rules` | `EventUtil.cDarkCometRules`, opposite |
| `max_lives`, `starting_lives` | `GameDataPlayerStatus.cMaxPlayerLeft`, `GameDataPlayerStatus.cStartingPlayerLeft` |
| `max_star_bits`, `max_coins` | `GameRules.cMaxStarBits`, `GameRules.cMaxCoins` |
| `max_carried_star_bits` | `GameDataTemporaryInGalaxy.cMaxCarriedStarPieceNum` |
| `coins_per_one_up`, `star_bits_per_one_up`, `purple_coins_for_star` | `ScenePlayingResult.cCoinStepNum`, `.cStarPieceStepNum`, `.cPurpleCoinPowerStarAppearNum` |
| `miss_star_pieces` | `SequenceUtil.cMissStarPieceNum` (a signed number) |
| `flip_panel_group`, `flip_panel_observer_group` | `FlipPanel.cPanelGroupSize`, `FlipPanel.cObserverGroupSize` |
| `water_bazooka_shot_flag` | `WaterBazooka.cShotInvalidSpinKill` |
| `kamek_barrier_offset_y` | `BossKameckBarrier.cOffsetY` (a number now, not float bits) |
| `skeletal_fish_baby_no_bubble` | `SkeletalFishBaby.cBreakAirBubble`, opposite |
| `dodoryu_shell_only`, `begoman_fire_proof` | `GameRules.cDodoryuShellOnly`, `GameRules.cBegomanFireProof` |
| `no_hidden_star_table`, `no_sudden_death_dodoryu` | `SceneUtil.cHiddenStarTable`, `SceneUtil.cSuddenDeathDodoryu`, both opposite |
| `messenger_always_ex`, `messenger_ex_one_ups` | `KinopioAstro.cMessangerAlwaysEx`, `KinopioAstro.sMessangerOneUpExNum` |
