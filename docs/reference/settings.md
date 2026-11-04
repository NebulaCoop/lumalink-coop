# Game settings

Named in a layer's `switches`. A setting is a number, or `{ "option": "id" }` to take the number from one of the mod's number options. Where two mods set one, the mod higher on the list wins. `lumalink list settings` prints these; a plugin reads them with `setting`.

```json
"switches": { "player_health": 6, "max_lives": 999, "coins_per_one_up": 100 }
```

| Setting | The game has | What it does |
|---|---|---|
| `player_health` | 3 | The health a player starts with, and the most before a Life Up |
| `player_health_fixed` | off | Daredevil comets don't change the player's health |
| `hit_takes_all` | off | A hit takes all the player's health |
| `life_up_health` | 6 | The most health a Life Up gives. 0: none |
| `no_low_health_warning` | off | No warning sound at low health |
| `no_dark_comet_rules` | off | A Daredevil comet is a comet like any other |
| `max_lives` | 99 | The most lives |
| `starting_lives` | 4 | The lives a file starts and loads with |
| `max_star_bits` | 9999 | The most star bits kept in the save |
| `max_carried_star_bits` | 999 | The most star bits carried in a stage |
| `max_coins` | 999 | The most coins in a stage |
| `coins_per_one_up` | 50 | Coins for a 1-up. 0: none |
| `star_bits_per_one_up` | 50 | Star bits for a 1-up. 0: none |
| `purple_coins_for_star` | 100 | Purple coins that bring the star |
| `water_air` | 8 | The air the player has under water |
| `miss_star_pieces` | 0 | Star bits added after a miss, negative to take. -999: all lost |
| `flip_panel_group` | 32 | How many flip panels a group holds |
| `flip_panel_observer_group` | 32 | The same for their observers |
| `water_bazooka_shot_flag` | off | The water bazooka's shots' flag |
| `kamek_barrier_offset_y` | 0 | Kamek's barrier's height, as float bits |
| `skeletal_fish_baby_no_bubble` | off | No air bubble from a hit skeletal fish |
| `dodoryu_shell_only` | off | Dodoryu is hurt by shells alone |
| `no_hidden_star_table` | off | Hidden stars without their table |
| `no_sudden_death_dodoryu` | off | No sudden-death Dodoryu |
| `begoman_fire_proof` | off | Begomans survive fire ground |
| `messenger_always_ex` | off | The 1-up messenger gives its larger supply |
| `messenger_ex_one_ups` | 20 | How many 1-ups that is |
