# Stage rules

The game keeps many rules by a stage's name: a Bowser arena's star, a hub's counters, an ocean's swimming. A row of a layer's `stages` gives any stage those rules, or takes them away, with no code.

```json
"stages": [
  { "stage": "MyArena", "acts_as": ["KoopaBattleVs3Galaxy"] },
  { "stage": "MyHub", "acts_as": ["AstroGalaxy"] },
  { "stage": "OceanRingGalaxy", "own_rules": false },
  { "stage": "MyGalaxy", "scenario": 3, "acts_as": ["CosmosGardenGalaxy"] }
]
```

| Key | Meaning |
|---|---|
| `stage` | The stage this row is about |
| `scenario` | One mission of it. Left out: all of them |
| `acts_as` | The stages whose rules it also plays by |
| `own_rules` | `false`: it no longer counts as itself for these rules |

A plugin says the same with `set_stage_rules`.

These are what each name brings. `lumalink list rules` prints this.

| Name | Rules |
|---|---|
| `AstroGalaxy` | The hub: counters stay up, no 1-up from star bits, Mario walks the Observatory's way, shadows as the hub's, star bits from Lumas count for the dome |
| `AstroDome` | A dome's inside: the map opens there, its pause menu |
| `LibraryRoom` | The library: a hub location |
| `PeachCastleGardenGalaxy` | The prologue's stage: its wipe, its pause menu, its shadows |
| `PeachCastleFinalGalaxy` | The ending's castle: its Toads |
| `EpilogueDemoStage` | The epilogue: no pause menu, no wipe |
| `KoopaBattleVs1Galaxy` | A Bowser arena: the grand star's fall, demo and sequence |
| `KoopaBattleVs2Galaxy` | A Bowser arena, and enemies that drop items as this one's do |
| `KoopaBattleVs3Galaxy` | The last arena: the last grand star's long sequence |
| `OceanRingGalaxy` | Swimming with a limit on how far the player can tilt; the ocean's water sound |
| `OceanPhantomCaveGalaxy` | Water bazookas as here; Syati's and the music's rules |
| `HeavensDoorGalaxy` | Planets from the start, its flip panels, its Butterflies, its pipes; fades in |
| `HeavenlyBeachGalaxy` | Water as here |
| `HellProminenceGalaxy` | Mission 5's sideways camera, the Tamakoro ball's rules |
| `CannonFleetGalaxy` | Mogu and boss music as here |
| `BattleShipGalaxy` | Boss music, and the player's fire as here |
| `ReverseKingdomGalaxy` | Boss music as here |
| `SandClockGalaxy` | The player's sand rules |
| `TriLegLv1Galaxy` | Bowser Jr.'s entrance, the player's walking on the walker |
| `CosmosGardenGalaxy` | Mission 4 is the sudden-death Dodoryu |
| `FactoryGalaxy` | Onimasu's rules, the opening's rules |
| `EggStarGalaxy` | The first opening, mission 2's star bit limit |
| `HoneyBeeKingdomGalaxy` | Its opening |
| `IceVolcanoGalaxy` | Music, wipe |
| `SkullSharkGalaxy` | Water, its steam |
| `TearDropGalaxy` | Water sphere as here |
| `TeresaMario2DGalaxy` | Fans as here |
| `FishTunnelGalaxy` | Toad as here |
| `SurfingLv1Galaxy` | The surfing race's restart |
| `SurfingLv2Galaxy` | The same, harder |
