# The plugin interface, call by call

Written from `nebula_mod_api.h` (by `tools/plugin_api_doc.py`), in its order: fields are only ever added at the end, so a plugin keeps working on later versions of Nebula. Check `game->size` before using a call newer than the oldest Nebula you support.

A call on an actor with a null pointer is your bug: nothing checks. Calls about the player or the stage do nothing, and answer 0 or null, between stages and on the file select screen.

| Call | Signature |
|---|---|
| `log` | `void log(const NebulaPlugin* self, const char* text)` |
| `option` | `const char* option(const NebulaPlugin* self, const char* id)` |
| `subscribe` | `int32_t subscribe(NebulaModEvent event, NebulaModCallback callback, void* user)` |
| `register_object` | `int32_t register_object(const NebulaObjectClass* object)` |
| `register_creator` | `int32_t register_creator(const char* name, void*(*create)(const char* name), const char* archive)` |
| `hook` | `int32_t hook(void* target, void* replacement, void* * original)` |
| `find_function` | `void* find_function(const char* symbol)` |
| `player` | `NebulaActor* player(void)` |
| `get_position` | `void get_position(NebulaActor* actor, float* xyz)` |
| `set_position` | `void set_position(NebulaActor* actor, const float* xyz)` |
| `get_velocity` | `void get_velocity(NebulaActor* actor, float* xyz)` |
| `set_velocity` | `void set_velocity(NebulaActor* actor, const float* xyz)` |
| `value` | `int32_t value(NebulaModValue what)` |
| `set_value` | `void set_value(NebulaModSet what, int32_t value)` |
| `kill_player` | `void kill_player(void)` |
| `jump_player` | `void jump_player(const float* velocity)` |
| `appear` | `void appear(NebulaActor* actor)` |
| `kill` | `void kill(NebulaActor* actor)` |
| `is_dead` | `int32_t is_dead(NebulaActor* actor)` |
| `show_model` | `void show_model(NebulaActor* actor, int32_t show)` |
| `play` | `int32_t play(NebulaActor* actor, NebulaModPlay what, const char* name)` |
| `actor_name` | `const char* actor_name(NebulaActor* actor)` |
| `stage_name` | `const char* stage_name(void)` |
| `spawn` | `NebulaActor* spawn(const char* name, const float* xyz)` |
| `buttons_held` | `uint32_t buttons_held(void)` |
| `buttons_pressed` | `uint32_t buttons_pressed(void)` |
| `stick` | `void stick(float* x, float* y)` |
| `shake_camera` | `void shake_camera(void)` |
| `add_galaxy` | `int32_t add_galaxy(const NebulaGalaxy* galaxy)` |
| `add_hungry_luma` | `int32_t add_hungry_luma(const char* galaxy, int32_t slot, int32_t star_bits)` |
| `add_text` | `int32_t add_text(const char* id, const char* utf8)` |
| `add_power_up` | `int32_t add_power_up(const NebulaPowerUpClass* power_up)` |
| `get_gravity` | `void get_gravity(NebulaActor* actor, float* xyz)` |
| `add_music` | `int32_t add_music(const char* name, const char* file)` |
| `set_stage_music` | `int32_t set_stage_music(const char* stage, int32_t scenario, const char* name)` |
| `set_area_music` | `int32_t set_area_music(const char* stage, int32_t area, const char* name)` |
| `play_music` | `int32_t play_music(const char* name)` |
| `stop_music` | `void stop_music(int32_t frames)` |
| `play_movie` | `int32_t play_movie(const char* file)` |
| `stop_movie` | `void stop_movie(void)` |
| `is_movie_playing` | `int32_t is_movie_playing(void)` |
| `set_event_flag` | `int32_t set_event_flag(const char* name, int32_t kind, int32_t a, int32_t b, const char* first, const char* second)` |
| `set_comet_schedule` | `int32_t set_comet_schedule(int32_t dome, const char* const* galaxies, const int32_t* coins, int32_t count)` |
| `set_object_change` | `int32_t set_object_change(int32_t where, const char* object, const char* becomes)` |
| `set_comet_time_limit` | `int32_t set_comet_time_limit(const char* galaxy, int32_t mission, int32_t seconds)` |
| `is_animation_done` | `int32_t is_animation_done(NebulaActor* actor)` |
| `animation_frame` | `float animation_frame(NebulaActor* actor)` |
| `animation_length` | `float animation_length(NebulaActor* actor)` |
| `set_animation_frame` | `void set_animation_frame(NebulaActor* actor, float frame)` |
| `set_animation_rate` | `void set_animation_rate(NebulaActor* actor, float rate)` |
| `stop_effect` | `void stop_effect(NebulaActor* actor, const char* name)` |
| `stop_sound` | `void stop_sound(NebulaActor* actor, const char* name)` |
| `set_state` | `void set_state(NebulaActor* actor, int32_t state)` |
| `state` | `int32_t state(NebulaActor* actor)` |
| `state_frames` | `int32_t state_frames(NebulaActor* actor)` |
| `nerve` | `const char* nerve(NebulaActor* actor)` |
| `nerve_frames` | `int32_t nerve_frames(NebulaActor* actor)` |
| `set_nerve` | `int32_t set_nerve(NebulaActor* actor, const char* nerve)` |
| `hook_nerve` | `int32_t hook_nerve(const char* nerve, NebulaNerveCallback callback, void* user)` |
| `nerve_at` | `const char* nerve_at(int32_t index)` |
| `get_rotation` | `void get_rotation(NebulaActor* actor, float* xyz)` |
| `set_rotation` | `void set_rotation(NebulaActor* actor, const float* xyz)` |
| `get_scale` | `void get_scale(NebulaActor* actor, float* xyz)` |
| `set_scale` | `void set_scale(NebulaActor* actor, const float* xyz)` |
| `set_front` | `void set_front(NebulaActor* actor, const float* xyz)` |
| `get_front` | `void get_front(NebulaActor* actor, float* xyz)` |
| `turn_to_player` | `void turn_to_player(NebulaActor* actor, float degrees)` |
| `set_touchable` | `void set_touchable(NebulaActor* actor, int32_t touchable)` |
| `hit_player` | `int32_t hit_player(NebulaActor* actor, NebulaHit how)` |
| `placement_number` | `int32_t placement_number(NebulaActor* actor, const char* column, double* value)` |
| `placement_text` | `const char* placement_text(NebulaActor* actor, const char* column)` |
| `has_switch` | `int32_t has_switch(NebulaActor* actor, NebulaSwitch which)` |
| `is_switch_on` | `int32_t is_switch_on(NebulaActor* actor, NebulaSwitch which)` |
| `set_switch` | `void set_switch(NebulaActor* actor, NebulaSwitch which, int32_t on)` |
| `has_rail` | `int32_t has_rail(NebulaActor* actor)` |
| `move_on_rail` | `void move_on_rail(NebulaActor* actor, float speed)` |
| `rail_at_end` | `int32_t rail_at_end(NebulaActor* actor)` |
| `rail_turn_back` | `void rail_turn_back(NebulaActor* actor)` |
| `rail_coord` | `float rail_coord(NebulaActor* actor)` |
| `set_rail_coord` | `void set_rail_coord(NebulaActor* actor, float coord)` |
| `rail_length` | `float rail_length(NebulaActor* actor)` |
| `raycast` | `int32_t raycast(const float* from, const float* to, float* hit)` |
| `is_on_ground` | `int32_t is_on_ground(NebulaActor* actor)` |
| `is_touching_wall` | `int32_t is_touching_wall(NebulaActor* actor)` |
| `is_in_water` | `int32_t is_in_water(const float* xyz)` |
| `distance_to_player` | `float distance_to_player(NebulaActor* actor)` |
| `get_joint_position` | `void get_joint_position(NebulaActor* actor, const char* joint, float* xyz)` |
| `player_front` | `void player_front(float* xyz)` |
| `set_player_control` | `void set_player_control(int32_t on)` |
| `declare_item` | `void declare_item(NebulaActor* actor, NebulaItem what, int32_t count)` |
| `appear_item` | `void appear_item(NebulaActor* actor, NebulaItem what, const float* xyz, int32_t count)` |
| `start_cutscene` | `int32_t start_cutscene(NebulaActor* actor, const char* name)` |
| `end_cutscene` | `void end_cutscene(NebulaActor* actor, const char* name)` |
| `is_cutscene_playing` | `int32_t is_cutscene_playing(void)` |
| `shake_camera_by` | `void shake_camera_by(NebulaShake how)` |
| `start_camera` | `void start_camera(const char* name, int32_t frames)` |
| `end_camera` | `void end_camera(const char* name, int32_t frames)` |
| `fade_out` | `void fade_out(int32_t frames)` |
| `fade_in` | `void fade_in(int32_t frames)` |
| `change_stage` | `void change_stage(const char* galaxy, int32_t mission)` |
| `back_to_dome` | `void back_to_dome(void)` |
| `is_event_flag_on` | `int32_t is_event_flag_on(const char* name)` |
| `set_event_flag_on` | `void set_event_flag_on(const char* name)` |
| `has_star` | `int32_t has_star(const char* galaxy, int32_t star)` |
| `show_message` | `void show_message(const char* utf8_or_id, int32_t centre)` |
| `hide_message` | `void hide_message(void)` |
| `store_get` | `int32_t store_get(const NebulaPlugin* self, const char* key, int32_t missing)` |
| `store_set` | `void store_set(const NebulaPlugin* self, const char* key, int32_t value)` |
| `set_stage_rules` | `int32_t set_stage_rules(const char* stage, int32_t scenario, const char* acts_as, int32_t own_rules)` |
| `set_player_constant` | `int32_t set_player_constant(int32_t character, const char* name, int32_t index, float value)` |
| `player_constant` | `float player_constant(int32_t character, const char* name, int32_t index)` |
| `set_object_class` | `int32_t set_object_class(const char* name, const char* like)` |
| `add_sound_effect` | `int32_t add_sound_effect(const char* name, const char* file)` |
| `find_actor` | `NebulaActor* find_actor(const char* name, NebulaActor* after)` |
| `actors_near` | `int32_t actors_near(const float* xyz, float radius, NebulaActor* * out, int32_t max)` |
| `attack_actor` | `int32_t attack_actor(NebulaActor* target, NebulaActor* by, NebulaAttack how)` |
| `actor_data` | `void* actor_data(NebulaActor* actor)` |
| `set_actor_data` | `void set_actor_data(NebulaActor* actor, void* data)` |
| `actor_placed_name` | `const char* actor_placed_name(NebulaActor* actor)` |
| `stop_scene` | `void stop_scene(int32_t frames)` |
| `gravity_at` | `void gravity_at(const float* xyz, float* out)` |
| `is_in_area` | `int32_t is_in_area(const char* area, const float* xyz)` |
| `camera_position` | `void camera_position(float* xyz)` |
| `camera_target` | `void camera_target(float* xyz)` |
| `screen_position` | `int32_t screen_position(const float* xyz, float* xy)` |
| `play_system_sound` | `void play_system_sound(const char* name)` |
| `pointer` | `int32_t pointer(float* xy)` |
| `event_value` | `int32_t event_value(const char* name)` |
| `set_event_value` | `void set_event_value(const char* name, int32_t value)` |
| `set_star` | `void set_star(const char* galaxy, int32_t star, int32_t have)` |
| `stars_in_galaxy` | `int32_t stars_in_galaxy(const char* galaxy)` |
| `stars_won_in_galaxy` | `int32_t stars_won_in_galaxy(const char* galaxy)` |
| `get_tunable` | `int32_t get_tunable(const char* key, double* value, double* game_value)` |
| `layout_open` | `NebulaLayout* layout_open(const char* name)` |
| `layout_show` | `void layout_show(NebulaLayout* layout, int32_t show)` |
| `layout_set_text` | `void layout_set_text(NebulaLayout* layout, const char* pane, const char* utf8)` |
| `layout_set_number` | `void layout_set_number(NebulaLayout* layout, const char* pane, int32_t number)` |
| `layout_show_pane` | `void layout_show_pane(NebulaLayout* layout, const char* pane, int32_t show)` |
| `layout_move_pane` | `void layout_move_pane(NebulaLayout* layout, const char* pane, float x, float y)` |
| `layout_play` | `void layout_play(NebulaLayout* layout, const char* animation)` |
| `layout_is_playing` | `int32_t layout_is_playing(NebulaLayout* layout)` |
| `layout_set_alpha` | `void layout_set_alpha(NebulaLayout* layout, float alpha)` |
| `store_get_text` | `int32_t store_get_text(const NebulaPlugin* self, const char* key, char* out, int32_t size)` |
| `store_set_text` | `void store_set_text(const NebulaPlugin* self, const char* key, const char* text)` |
| `open_scenario_select` | `void open_scenario_select(const char* galaxy)` |
| `set_story` | `int32_t set_story(const char* key, const char* value)` |
| `save_game` | `void save_game(void)` |
| `set_tunable` | `int32_t set_tunable(const char* key, double value)` |
| `tunable_at` | `int32_t tunable_at(int32_t index, const char* * file, const char* * name, int32_t* place, int32_t* kind, double* game_value, double* value)` |
| `set_gravity_scale` | `void set_gravity_scale(float scale)` |
| `gravity_scale` | `float gravity_scale(void)` |
| `timer` | `int32_t timer(float* seconds_left, float* seconds_limit)` |
| `set_timer` | `int32_t set_timer(float seconds_left)` |
| `pause_timer` | `void pause_timer(int32_t paused)` |
| `set_camera` | `void set_camera(const float* position, const float* target, float fov)` |
| `camera_fov` | `float camera_fov(void)` |
| `show_hud` | `void show_hud(uint32_t parts, int32_t show)` |
| `set_game_speed` | `void set_game_speed(float speed)` |
| `step_game` | `void step_game(int32_t steps)` |
| `draw_text` | `int32_t draw_text(int32_t line, float x, float y, float size, uint32_t rgba, const char* utf8)` |
| `add_text_like` | `int32_t add_text_like(const char* id, const char* utf8, const char* like_id)` |
| `add_player_animation` | `int32_t add_player_animation(const char* name, const char* file, float rate, int32_t blend_frames)` |
| `alloc` | `void* alloc(uint64_t bytes)` |
| `release` | `void release(void* memory)` |
| `format` | `int32_t format(char* buffer, uint64_t size, const char* format, ...)` |
| `set_camera_near` | `void set_camera_near(float distance)` |
| `unlock_achievement` | `int32_t unlock_achievement(const NebulaPlugin* self, const char* id)` |

## What each does

### `log`

```c
void log(const NebulaPlugin* self, const char* text)
```

One line in the game's log.

### `option`

```c
const char* option(const NebulaPlugin* self, const char* id)
```

The choice a player made for one of the mod's options, by option id: its choice id. Null: no such option.

### `subscribe`

```c
int32_t subscribe(NebulaModEvent event, NebulaModCallback callback, void* user)
```

Only in nebula_plugin_start: `callback` runs at each `event` (NEBULA_MOD_EVENT_*), with what it's about. What it returns counts where the event says so (a hit refused, coins changed); else return data->value. 0: done.

### `register_object`

```c
int32_t register_object(const NebulaObjectClass* object)
```

A class of objects levels can place by its name, run by the plugin's callbacks (NebulaObjectClass). 0: done; -1: no name, a name taken, or a class older than Nebula reads.

### `register_creator`

```c
int32_t register_creator(const char* name, void*(*create)(const char* name), const char* archive)
```

`create` makes the game's own object (a NameObj) from its placed name; `archive` is loaded for it, or null.

### `hook`

```c
int32_t hook(void* target, void* replacement, void* * original)
```

Run `replacement` in place of the game's function at `target`; `original` gets a call to what was there before it, which may be another plugin's. A function the compiler inlined everywhere can't be reached this way: use events. Windows x64, and Linux and Mac on x64 and arm64. Elsewhere, or for a function whose first instructions can't be moved, it answers -1.

### `find_function`

```c
void* find_function(const char* symbol)
```

A game function by its symbol: on Windows, as dumpbin /exports names it. Null: no such function.

### `player`

```c
NebulaActor* player(void)
```

Null when no stage is running, or outside a stage's frame.

### `get_position`

```c
void get_position(NebulaActor* actor, float* xyz)
```

Where it is: x, y, z in the level's units.

### `set_position`

```c
void set_position(NebulaActor* actor, const float* xyz)
```

It is there at once. The player is put there as after a warp; an object just moves.

### `get_velocity`

```c
void get_velocity(NebulaActor* actor, float* xyz)
```

How far it moves each frame: x, y, z. The player's is the game's own.

### `set_velocity`

```c
void set_velocity(NebulaActor* actor, const float* xyz)
```

Its movement from now: the game's gravity and friction go on from it.

### `value`

```c
int32_t value(NebulaModValue what)
```

One of the game's numbers (NEBULA_VALUE_*). The player's are 0 with no stage running.

### `set_value`

```c
void set_value(NebulaModSet what, int32_t value)
```

Sets one (NEBULA_SET_*); nothing with no stage running. Life 0 or less: the player dies; more than the most: the most. A power-up: the game's (NEBULA_POWER_UP_*) or one add_power_up gave.

### `kill_player`

```c
void kill_player(void)
```

The player dies, as from a fall out of the level.

### `jump_player`

```c
void jump_player(const float* velocity)
```

The player leaves the ground with this velocity, as a spring sends them.

### `appear`

```c
void appear(NebulaActor* actor)
```

It comes into the level (one killed, or one not yet shown): drawn, moving and touchable, as the game's appear.

### `kill`

```c
void kill(NebulaActor* actor)
```

It leaves the level, as a beaten enemy: not drawn, moved or touched. appear brings it back.

### `is_dead`

```c
int32_t is_dead(NebulaActor* actor)
```

1 while it's out of the level (killed, or not yet appeared).

### `show_model`

```c
void show_model(NebulaActor* actor, int32_t show)
```

Its model drawn (1) or not (0); it still moves and touches. Nothing for an object with no model.

### `play`

```c
int32_t play(NebulaActor* actor, NebulaModPlay what, const char* name)
```

1: it started. 0: the object has no such thing, or nothing to play it with.

### `actor_name`

```c
const char* actor_name(NebulaActor* actor)
```

The name it was made with: a placed object's is its name in the level (Kuribo); some of the game's own objects have Japanese names.

### `stage_name`

```c
const char* stage_name(void)
```

The stage being played (EggStarGalaxy); null with none.

### `spawn`

```c
NebulaActor* spawn(const char* name, const float* xyz)
```

A registered object at a place. Its archive must already be loaded: a level places it, or the stage holds it. Null: it can't be made now.

### `buttons_held`

```c
uint32_t buttons_held(void)
```

The controls held now: NEBULA_BUTTON_*, any together. 0 with no stage running.

### `buttons_pressed`

```c
uint32_t buttons_pressed(void)
```

The controls pressed on this frame only: NEBULA_BUTTON_*.

### `stick`

```c
void stick(float* x, float* y)
```

The stick, each from -1 to 1 (either may be null).

### `shake_camera`

```c
void shake_camera(void)
```

In a stage: the camera shakes, as when something heavy lands (shake_camera_by chooses how hard).

### `add_galaxy`

```c
int32_t add_galaxy(const NebulaGalaxy* galaxy)
```

Only in nebula_plugin_start. 0: done; -2: the galaxy has no scenario archive (StageData/<name>/<name>Scenario.arc in the mod, StageData/<name>Scenario in the store), so it is left out; -1: anything else (its name clashes with one the game has).

### `add_hungry_luma`

```c
int32_t add_hungry_luma(const char* galaxy, int32_t slot, int32_t star_bits)
```

A hungry Luma: placed in a level as TicoGalaxy with its Arg7 at `slot`, it opens `galaxy` when fed `star_bits`. The game's own are slots 0 to 6: using one takes its place.

### `add_text`

```c
int32_t add_text(const char* id, const char* utf8)
```

A message of the game by its id, in place of its own or new: GalaxyName_<galaxy>, ScenarioName_<galaxy><n>, a talk's id. A line of the game keeps its talk settings. The words may hold the game's message tags in brackets (nebula_text_tags.h).

### `add_power_up`

```c
int32_t add_power_up(const NebulaPowerUpClass* power_up)
```

Its number, for set_value. Below 0: it can't be added.

### `get_gravity`

```c
void get_gravity(NebulaActor* actor, float* xyz)
```

Which way is down for it, a unit length: on a planet that is no one axis.

### `add_music`

```c
int32_t add_music(const char* name, const char* file)
```

Only in nebula_plugin_start. A music track: `file` is its place in the game, AudioRes/Stream/MyTrack.wav (.ogg, .ast). `name` is yours to choose, not starting SE_, BGM_, MBGM_, STM_ or ME_, which are the game's. 0: done.

### `set_stage_music`

```c
int32_t set_stage_music(const char* stage, int32_t scenario, const char* name)
```

Only in nebula_plugin_start. What a stage plays: `scenario` is a mission, 0 for all it has no row for.

### `set_area_music`

```c
int32_t set_area_music(const char* stage, int32_t area, const char* name)
```

Only in nebula_plugin_start. What a ChangeBgmCube in the stage starts, with its Obj_arg1 at `area`.

### `play_music`

```c
int32_t play_music(const char* name)
```

In a stage: this track, a mod's or the game's by its name, in place of what's playing. 1: it started.

### `stop_music`

```c
void stop_music(int32_t frames)
```

In a stage: what's playing fades out over this many frames.

### `play_movie`

```c
int32_t play_movie(const char* file)
```

In a stage: a movie over the game, by its place in the game: MovieData/MyMovie.thp. The game waits while it plays. 1: it started.

### `stop_movie`

```c
void stop_movie(void)
```

The movie playing ends now; the game goes on.

### `is_movie_playing`

```c
int32_t is_movie_playing(void)
```

1 while a movie plays.

### `set_event_flag`

```c
int32_t set_event_flag(const char* name, int32_t kind, int32_t a, int32_t b, const char* first, const char* second)
```

Only in nebula_plugin_start. A row of the game's progression, by its name: what opens what, and what each hungry Luma costs. A name the game has is changed, a new one added. `kind`: 1 stars, 2 a galaxy's stars, 3 a special star, 4 two flags, 5 story, 6 a galaxy, 7 a comet, 8 a hungry Luma. `a` and `b` follow the kind: the star's number, or the Luma's slot and its price. 0: done.

### `set_comet_schedule`

```c
int32_t set_comet_schedule(int32_t dome, const char* const* galaxies, const int32_t* coins, int32_t count)
```

Only in nebula_plugin_start. A dome's comets (domes 1 to 6 are the game's; 7, 8 and on add domes while they run on unbroken), in the order they come, for the whole of its schedule: `galaxies[i]` with `coins[i]` at 1 for the hundred purple coins.

### `set_object_change`

```c
int32_t set_object_change(int32_t where, const char* object, const char* becomes)
```

One of the game's objects becomes another, or (`becomes` null) is gone. Where: NEBULA_OBJECTS_*. Before stages load.

### `set_comet_time_limit`

```c
int32_t set_comet_time_limit(const char* galaxy, int32_t mission, int32_t seconds)
```

A comet mission's time limit in seconds, for a galaxy and mission of the player's comets.

### `is_animation_done`

```c
int32_t is_animation_done(NebulaActor* actor)
```

An object's skeleton animation (what play's NEBULA_PLAY_ANIMATION started): 1 once it has reached its end, 0 while it plays on.

### `animation_frame`

```c
float animation_frame(NebulaActor* actor)
```

Its frame now, from 0; 0 with no model.

### `animation_length`

```c
float animation_length(NebulaActor* actor)
```

Its last frame; 0 with no model.

### `set_animation_frame`

```c
void set_animation_frame(NebulaActor* actor, float frame)
```

It goes on from this frame.

### `set_animation_rate`

```c
void set_animation_rate(NebulaActor* actor, float rate)
```

Its speed: 1 its own, 2 twice, 0 held, below 0 backwards.

### `stop_effect`

```c
void stop_effect(NebulaActor* actor, const char* name)
```

Its effect of that name stops; null: all of them.

### `stop_sound`

```c
void stop_sound(NebulaActor* actor, const char* name)
```

Its sound of that name stops.

### `set_state`

```c
void set_state(NebulaActor* actor, int32_t state)
```

An object of register_object's own step in what it's doing: a number of the plugin's own (its enum of steps), starting at 0; state_frames counts from the set_state that began it.

### `state`

```c
int32_t state(NebulaActor* actor)
```

Its state: what set_state set last, 0 before that or for an object not register_object's.

### `state_frames`

```c
int32_t state_frames(NebulaActor* actor)
```

Frames since that set_state, from 0.

### `nerve`

```c
const char* nerve(NebulaActor* actor)
```

The game's own objects (and screens, and the states objects hand their work to) run one nerve at a time: Kuribo walks in "Kuribo.Walk". A nerve's name is "<class>.<nerve>", as docs/modding/NERVES.md lists every one. nerve: the object's now; null for one without nerves (register_object's objects have set_state). nerve_frames: its frames in it, from 0. set_nerve: it goes to another nerve of the class its nerve now is of, from its next frame (the game's code of that nerve runs, as when the game sets it). 0: done; -1: no such nerve of its class.

### `nerve_frames`

```c
int32_t nerve_frames(NebulaActor* actor)
```

Its frames in the nerve, from 0.

### `set_nerve`

```c
int32_t set_nerve(NebulaActor* actor, const char* nerve)
```

It goes to another nerve of its class (as nerve names it) from its next frame. 0: done; -1: no such nerve.

### `hook_nerve`

```c
int32_t hook_nerve(const char* nerve, NebulaNerveCallback callback, void* user)
```

Only in nebula_plugin_start. `callback` runs for every object in the nerve: as it enters, every frame of it, and as it leaves. 0: done; -1: no such nerve.

### `nerve_at`

```c
const char* nerve_at(int32_t index)
```

The index-th nerve's name, for listing them all. Null: past the last.

### `get_rotation`

```c
void get_rotation(NebulaActor* actor, float* xyz)
```

Its rotation about x, y and z, in degrees, as levels place it.

### `set_rotation`

```c
void set_rotation(NebulaActor* actor, const float* xyz)
```

Its rotation, in degrees. A register_object object with a front (set_front) turns by that instead.

### `get_scale`

```c
void get_scale(NebulaActor* actor, float* xyz)
```

Its size on each axis: 1 its own.

### `set_scale`

```c
void set_scale(NebulaActor* actor, const float* xyz)
```

Its size on each axis.

### `set_front`

```c
void set_front(NebulaActor* actor, const float* xyz)
```

register_object's objects: the way it faces, standing against gravity. Its model then turns with this, not with its rotation.

### `get_front`

```c
void get_front(NebulaActor* actor, float* xyz)
```

The way it faces (set_front's, or as it turned to the player); 0, 0, 1 for others.

### `turn_to_player`

```c
void turn_to_player(NebulaActor* actor, float degrees)
```

It turns to face the player, at most `degrees` this frame.

### `set_touchable`

```c
void set_touchable(NebulaActor* actor, int32_t touchable)
```

register_object's objects: whether anything can touch it, and hurting the player.

### `hit_player`

```c
int32_t hit_player(NebulaActor* actor, NebulaHit how)
```

It hurts the player as the game's enemies do (NEBULA_HIT_*), through its body. 1 if the player took it.

### `placement_number`

```c
int32_t placement_number(NebulaActor* actor, const char* column, double* value)
```

register_object's objects: a column of its row in the level, by the column's name, as a level editor shows it: Obj_arg0 to Obj_arg7 (-1 where the level sets none), CommonPath_ID, ShapeModelNo, CameraSetId, MessageId, GroupId, ClippingGroupId, DemoGroupId, ViewGroupId, CastId, ParentID, the SW_ columns, l_id... placement_number: 1 and the number for a number column (whole or not); 0 for a column the row lacks or one of words, or an object the level didn't place (spawn). placement_text: a column of words (name), or null.

### `placement_text`

```c
const char* placement_text(NebulaActor* actor, const char* column)
```

A column of words of its row (its name in the level, a camera's name), or null.

### `has_switch`

```c
int32_t has_switch(NebulaActor* actor, NebulaSwitch which)
```

Its switches (NEBULA_SWITCH_*): 1 when the level set that one. The other switch calls do nothing for one it didn't.

### `is_switch_on`

```c
int32_t is_switch_on(NebulaActor* actor, NebulaSwitch which)
```

1 while that switch is on.

### `set_switch`

```c
void set_switch(NebulaActor* actor, NebulaSwitch which, int32_t on)
```

It turns that switch on or off, for all the level's objects that listen to it.

### `has_rail`

```c
int32_t has_rail(NebulaActor* actor)
```

Its rail from the level (CommonPath_ID): 1 when it has one. The rail calls do nothing, or answer 0, for an object without.

### `move_on_rail`

```c
void move_on_rail(NebulaActor* actor, float speed)
```

It goes `speed` along its rail (below 0: back) and is put there.

### `rail_at_end`

```c
int32_t rail_at_end(NebulaActor* actor)
```

1 once it has come to the end it moves to.

### `rail_turn_back`

```c
void rail_turn_back(NebulaActor* actor)
```

It moves the other way along it from now.

### `rail_coord`

```c
float rail_coord(NebulaActor* actor)
```

How far along it is, in the level's units from the rail's start.

### `set_rail_coord`

```c
void set_rail_coord(NebulaActor* actor, float coord)
```

It is put this far along (the next move_on_rail moves it from there).

### `rail_length`

```c
float rail_length(NebulaActor* actor)
```

The rail's whole length.

### `raycast`

```c
int32_t raycast(const float* from, const float* to, float* hit)
```

The world. 1 and where (`hit`, which may be null) if the line from `from` to `to` meets the map's collision; 0 if it's clear.

### `is_on_ground`

```c
int32_t is_on_ground(NebulaActor* actor)
```

1 while it stands on the map: the player, or an object with collision of its own.

### `is_touching_wall`

```c
int32_t is_touching_wall(NebulaActor* actor)
```

1 while it touches a wall of the map (objects with collision of their own).

### `is_in_water`

```c
int32_t is_in_water(const float* xyz)
```

1 if a point is in the stage's water.

### `distance_to_player`

```c
float distance_to_player(NebulaActor* actor)
```

How far it is from the player; 0 with no player.

### `get_joint_position`

```c
void get_joint_position(NebulaActor* actor, const char* joint, float* xyz)
```

Where one of its model's joints is now; its position with no such joint or model.

### `player_front`

```c
void player_front(float* xyz)
```

The way the player faces, a unit length; 0, 0, 1 with no player.

### `set_player_control`

```c
void set_player_control(int32_t on)
```

The player's controls: 0 takes them (the player stands still, as in a cutscene), 1 gives them back.

### `declare_item`

```c
void declare_item(NebulaActor* actor, NebulaItem what, int32_t count)
```

Items an object gives: declared in its class's `start` (the game readies them with the stage), then appear_item at the moment one comes out.

### `appear_item`

```c
void appear_item(NebulaActor* actor, NebulaItem what, const float* xyz, int32_t count)
```

One comes out at a point: coins and star bits pop out `count` at a time; a power star, the star of number `count` (below 0: the stage's).

### `start_cutscene`

```c
int32_t start_cutscene(NebulaActor* actor, const char* name)
```

A cutscene of the stage's (its DemoGroupId's, by name): the actor's part in it starts. 1 when it began; 0 while another plays (ask again next frame).

### `end_cutscene`

```c
void end_cutscene(NebulaActor* actor, const char* name)
```

The actor's part in that cutscene ends; the cutscene ends when every part has.

### `is_cutscene_playing`

```c
int32_t is_cutscene_playing(void)
```

1 while any cutscene plays, and while the stage starts (its opening camera and the player's arrival, which send no NEBULA_MOD_EVENT_CUTSCENE): the player has control when it is 0. A message is best shown then.

### `shake_camera_by`

```c
void shake_camera_by(NebulaShake how)
```

In a stage: the camera shakes, as hard as `how` (NEBULA_SHAKE_*).

### `start_camera`

```c
void start_camera(const char* name, int32_t frames)
```

In a stage: one of its event cameras (by name, as the stage's camera table has it) takes the view over `frames` frames.

### `end_camera`

```c
void end_camera(const char* name, int32_t frames)
```

That camera hands the view back over `frames` frames.

### `fade_out`

```c
void fade_out(int32_t frames)
```

In a stage: the screen fades to black over `frames` frames.

### `fade_in`

```c
void fade_in(int32_t frames)
```

In a stage: the screen fades back in over `frames` frames.

### `change_stage`

```c
void change_stage(const char* galaxy, int32_t mission)
```

The player goes to a galaxy's mission, as a launch star sends them (the stage ends, the new one loads).

### `back_to_dome`

```c
void back_to_dome(void)
```

In a stage: the player goes back to the dome, as when a galaxy is left.

### `is_event_flag_on`

```c
int32_t is_event_flag_on(const char* name)
```

1 if the save has that event flag (a name of the game's progression, or one set_event_flag added) on. "SpinAndStarPointer" is the spin and the Star Pointer the Gateway's Luma grants.

### `set_event_flag_on`

```c
void set_event_flag_on(const char* name)
```

Turns it on in the save (written with the next save). "SpinAndStarPointer" grants the spin and the Star Pointer at once, in the stage being played too.

### `has_star`

```c
int32_t has_star(const char* galaxy, int32_t star)
```

1 if the save has that galaxy's star of that number.

### `show_message`

```c
void show_message(const char* utf8_or_id, int32_t centre)
```

In a stage: the game's information box (at the top, or the middle with `centre` 1) shows a message by its id (the game's or add_text's), else these words (with the game's message tags; lines wrap between words). It stays until hide_message. Asked for while the stage starts, it appears when the player has control.

### `hide_message`

```c
void hide_message(void)
```

The information box goes.

### `store_get`

```c
int32_t store_get(const NebulaPlugin* self, const char* key, int32_t missing)
```

A number kept for this mod across plays (in the mods folder's .plugin-data, written at once). Keys: no '=' or line ends. `missing`: what a key never set answers.

### `store_set`

```c
void store_set(const NebulaPlugin* self, const char* key, int32_t value)
```

Keeps `value` under `key`.

### `set_stage_rules`

```c
int32_t set_stage_rules(const char* stage, int32_t scenario, const char* acts_as, int32_t own_rules)
```

Only in nebula_plugin_start. A stage plays by another's rules: the game's rules that go by a stage's name (a Bowser arena's star, a hub's counters, an ocean's swimming) then count it as `acts_as`. own_rules 0: it no longer counts as itself. `scenario`: one mission, 0 for all. 0: done.

### `set_player_constant`

```c
int32_t set_player_constant(int32_t character, const char* name, int32_t index, float value)
```

A number of the player's table by its name (mJumpHeight, mFireModeTime). `character`: 0 Mario, 1 Luigi. `index`: its place in a list, else 0. Any time. 0: done; -1: no such number.

### `player_constant`

```c
float player_constant(int32_t character, const char* name, int32_t index)
```

The number now; 0 with no such number.

### `set_object_class`

```c
int32_t set_object_class(const char* name, const char* like)
```

Only in nebula_plugin_start. A new name levels can place, made as the game's object `like`, with its files from ObjectData/<name>.arc. No code needed. 0: done.

### `add_sound_effect`

```c
int32_t add_sound_effect(const char* name, const char* file)
```

Only in nebula_plugin_start. A sound by its name, played from a file (AudioRes/Stream/Boom.wav, .ogg, .ast): a new name, or one of the game's (SE_OJ_COIN_GET) to replace it wherever it plays, by name or by number (the player's voices, SE_PV_*). "<name>@luigi": in its place while Luigi plays. 0: done.

### `find_actor`

```c
NebulaActor* find_actor(const char* name, NebulaActor* after)
```

Finding things. An actor by the name levels place it under; `after` null starts, a found one goes on from it. `name` null: every actor.

### `actors_near`

```c
int32_t actors_near(const float* xyz, float radius, NebulaActor* * out, int32_t max)
```

Living actors within `radius`. Their count; up to `max` go in `out`.

### `attack_actor`

```c
int32_t attack_actor(NebulaActor* target, NebulaActor* by, NebulaAttack how)
```

As the player's attack `how`, on any actor. 1 if it took it. `by`: where it comes from, null for the player.

### `actor_data`

```c
void* actor_data(NebulaActor* actor)
```

A pointer of your own on one of register_object's objects: what set_actor_data put there, else null.

### `set_actor_data`

```c
void set_actor_data(NebulaActor* actor, void* data)
```

Puts a pointer of your own on it: yours to free (at its class's `kill` or the STAGE_END event).

### `actor_placed_name`

```c
const char* actor_placed_name(NebulaActor* actor)
```

The name levels place it under.

### `stop_scene`

```c
void stop_scene(int32_t frames)
```

Run again at `frames` frames, or not at all with 0: a plain hit-stop: the whole scene holds still.

### `gravity_at`

```c
void gravity_at(const float* xyz, float* out)
```

Which way is down at a point, a unit length: from the stage's gravity, as an object there would fall.

### `is_in_area`

```c
int32_t is_in_area(const char* area, const float* xyz)
```

1 if a point is in one of the stage's areas of that kind (WaterArea, DeathArea, a ChangeBgmCube...).

### `camera_position`

```c
void camera_position(float* xyz)
```

Where the camera is.

### `camera_target`

```c
void camera_target(float* xyz)
```

A point the camera looks at, 1000 units before it.

### `screen_position`

```c
int32_t screen_position(const float* xyz, float* xy)
```

Where a point is on the screen; 0 if behind the camera.

### `play_system_sound`

```c
void play_system_sound(const char* name)
```

A sound in no place; an effect's named sound at a point.

### `pointer`

```c
int32_t pointer(float* xy)
```

The pointer on the screen, in the screen's own coordinates. 0: off it.

### `event_value`

```c
int32_t event_value(const char* name)
```

Save data. An event value (a named number the save keeps), by its name.

### `set_event_value`

```c
void set_event_value(const char* name, int32_t value)
```

Sets it, held between 0 and 65535 (the save keeps 16 bits).

### `set_star`

```c
void set_star(const char* galaxy, int32_t star, int32_t have)
```

A star as if won, or taken away.

### `stars_in_galaxy`

```c
int32_t stars_in_galaxy(const char* galaxy)
```

How many power stars a galaxy has (its scenarios with a star).

### `stars_won_in_galaxy`

```c
int32_t stars_won_in_galaxy(const char* galaxy)
```

How many of them the save has.

### `get_tunable`

```c
int32_t get_tunable(const char* key, double* value, double* game_value)
```

A tuning number by its key, as set_tunable takes it ("GameRules.cPlayerHealth"): the number now and the game's. 0: done; -1: no such number. Interface 2: it took the place of setting and setting_by_number.

### `layout_open`

```c
NebulaLayout* layout_open(const char* name)
```

A layout on the screen (LayoutData/<name>.arc, the game's or a mod's), for this stage: it goes with the stage. Null if there is no such archive.

### `layout_show`

```c
void layout_show(NebulaLayout* layout, int32_t show)
```

Shown (1) or hidden (0).

### `layout_set_text`

```c
void layout_set_text(NebulaLayout* layout, const char* pane, const char* utf8)
```

A pane's text: these words, with the game's message tags.

### `layout_set_number`

```c
void layout_set_number(NebulaLayout* layout, const char* pane, int32_t number)
```

A pane's text as a number, in the game's digits.

### `layout_show_pane`

```c
void layout_show_pane(NebulaLayout* layout, const char* pane, int32_t show)
```

A pane shown (1) or hidden (0), with the panes in it.

### `layout_move_pane`

```c
void layout_move_pane(NebulaLayout* layout, const char* pane, float x, float y)
```

A pane moved to x, y on the screen, where screen_position puts a point; it stays there.

### `layout_play`

```c
void layout_play(NebulaLayout* layout, const char* animation)
```

Its animation by name.

### `layout_is_playing`

```c
int32_t layout_is_playing(NebulaLayout* layout)
```

1 while its animation plays.

### `layout_set_alpha`

```c
void layout_set_alpha(NebulaLayout* layout, float alpha)
```

How opaque it is: 1 as made, 0 not seen.

### `store_get_text`

```c
int32_t store_get_text(const NebulaPlugin* self, const char* key, char* out, int32_t size)
```

Text kept for this mod across plays, beside store_get's numbers: up to `size` - 1 bytes and an end go in `out`; how many were read (0 for a key never set).

### `store_set_text`

```c
void store_set_text(const NebulaPlugin* self, const char* key, const char* text)
```

Keeps `text` under `key`.

### `open_scenario_select`

```c
void open_scenario_select(const char* galaxy)
```

The star-choosing screen of a galaxy, as a launch star opens it.

### `set_story`

```c
int32_t set_story(const char* key, const char* value)
```

Only in nebula_plugin_start. The game's journey: "first_stage" (and "first_mission", a number as text) is where a new file begins, in place of the prologue; "hub" is where the player goes after a galaxy, in place of the Comet Observatory; "story_scenes" "off" leaves out the game's own story scenes. 0: done.

### `save_game`

```c
void save_game(void)
```

The game writes the save, as it does after a star.

### `set_tunable`

```c
int32_t set_tunable(const char* key, double value)
```

A file's tuning number by "<file stem>.<name>" ("GameRules.cPlayerHealth"), or a list's place by "<file stem>.<name>[<i>]", as luma.json's "tunables" set them. Any time; most are read when an object starts, so the host sets luma.json's before the first stage. Whole kinds round and clamp. 0: done; -1: no such number.

### `tunable_at`

```c
int32_t tunable_at(int32_t index, const char* * file, const char* * name, int32_t* place, int32_t* kind, double* game_value, double* value)
```

The index-th tuning number (a list counts each place): its file ("Kuribo.cpp"), name, place in its list (-1: one number), kind (1 f32, 2 f64, 3 s32, 4 u32, 5 s16, 6 u16, 7 s8, 8 u8, 9 bool), the game's number and the number now. 0: done; -1: past the last.

### `set_gravity_scale`

```c
void set_gravity_scale(float scale)
```

The game's rules as a plugin changes them, any time. Each holds until set again; a stage change keeps it. How hard gravity pulls: the player's fall and every object's. 1 the game's, 0.5 half, 0 none; below 0 pulls away.

### `gravity_scale`

```c
float gravity_scale(void)
```

The scale now.

### `timer`

```c
int32_t timer(float* seconds_left, float* seconds_limit)
```

The countdown on the screen (a comet's time limit). 1 while one runs, with its seconds left and its whole length; 0 when none does.

### `set_timer`

```c
int32_t set_timer(float seconds_left)
```

Its seconds left, within its length. 0: done; -1: no countdown runs.

### `pause_timer`

```c
void pause_timer(int32_t paused)
```

1 holds every countdown where it is, 0 lets them run.

### `set_camera`

```c
void set_camera(const float* position, const float* target, float fov)
```

The camera at `position` looking at `target`, with `fov` degrees from the picture's top to its bottom. A null point or a fov of 0 leaves that to the game; all three left: the game's camera again. A position set is where the camera is that frame, with no easing, and it passes through walls.

### `camera_fov`

```c
float camera_fov(void)
```

The angle of view the camera drew with last, in degrees.

### `show_hud`

```c
void show_hud(uint32_t parts, int32_t show)
```

HUD parts (NEBULA_HUD_*) hidden (0) or shown as the game shows them (1).

### `set_game_speed`

```c
void set_game_speed(float speed)
```

The scene's speed: 1 the game's, 0.5 one step every other frame, 0 held (menus and the HUD still run). Between 0 and 1. Set in a STAGE_FRAME callback, it counts from that frame.

### `step_game`

```c
void step_game(int32_t steps)
```

While held: the scene runs this many steps, one a frame.

### `draw_text`

```c
int32_t draw_text(int32_t line, float x, float y, float size, uint32_t rgba, const char* utf8)
```

A line of text over the game, in the game's message font. `line`: its slot, 0-15. x, y: its top-left corner, 0 to 1 across and down the screen. `size`: its height, of the screen's height (0.04 reads like the HUD's numbers). `rgba`: its colour, 0xRRGGBBAA. Null or "" takes the line away; it stays until then, across stages. 0: done; -1: no such slot.

### `add_text_like`

```c
int32_t add_text_like(const char* id, const char* utf8, const char* like_id)
```

As add_text, and a new line takes the talk (camera, sound, talk and balloon type) of the game's message `like_id`; null or "": add_text. The words may hold the game's message tags in brackets ([wait 30], [color 1], [player]...: nebula_text_tags.h), as add_text's may.

### `add_player_animation`

```c
int32_t add_player_animation(const char* name, const char* file, float rate, int32_t blend_frames)
```

A player animation: `name` plays the bck `file` of the player's animations (ObjectData/MarioAnime.arc: the game's or one the mod adds there, a .bck or an animation in a .glb), at `rate` (1: the bck's own speed), blended in over `blend_frames` frames (0: the game's 6). Played by name (NEBULA_PLAY_PLAYER_ANIMATION), and where the game plays that name: a name the game has (its own animation table's) is played from the mod's bck. Called from the plugin's entry, before the first stage: 0 done; -1 a bad name or file, or too late.

### `alloc`

```c
void* alloc(uint64_t bytes)
```

Memory of the plugin's own, from the game's C library: a plugin built with none (as one built for every device is) has no malloc. alloc: null for 0 bytes or when none is left; the memory is aligned for any type. release(null) does nothing. Kept until released; a plugin's memory is never moved or reused under it.

### `release`

```c
void release(void* memory)
```

Memory of the plugin's own, from the game's C library: a plugin built with none (as one built for every device is) has no malloc. alloc: null for 0 bytes or when none is left; the memory is aligned for any type. release(null) does nothing. Kept until released; a plugin's memory is never moved or reused under it.

### `format`

```c
int32_t format(char* buffer, uint64_t size, const char* format, ...)
```

printf's formatting into `buffer` of `size` bytes, always ended with a 0 when size is above 0: what snprintf returns (the length the whole text needs; below 0 for a bad format).

### `set_camera_near`

```c
void set_camera_near(float distance)
```

The camera's near plane in units: nothing closer to the camera is drawn. 0: the game's (100; 10 in its own first-person view). Kept until set again.

### `unlock_achievement`

```c
int32_t unlock_achievement(const NebulaPlugin* self, const char* id)
```

One of the mod's achievements (its luma.json "achievements", by id) is unlocked: 1 now, 0 when it was already or the mod has none of that id. Its pop-up shows; the board keeps it under Mods.

## Numbers and names

### `NebulaPlugin`

```c
    uint32_t size;
    const char* id;       /* the mod's id */
    const char* version;  /* the mod's version */
```

### `NebulaModEvent`

```c
    /* Every frame, in any scene: menus too. player() is null here. */
    NEBULA_MOD_EVENT_FRAME = 0,
    /* Every frame of a stage, before its objects move. */
    NEBULA_MOD_EVENT_STAGE_FRAME,
    /* A stage has started. name: the stage. value: the mission. */
    NEBULA_MOD_EVENT_STAGE_START,
    /* The player is about to lose one health. Return 0 to refuse the hit. */
    NEBULA_MOD_EVENT_PLAYER_DAMAGE,
    /* The player is about to die. value: how (0 a fall out of the level). Return -1 to refuse it: the player lives on where
       they are, so move them somewhere safe (set_position) and give back life (set_value) as the plugin's rule says. */
    NEBULA_MOD_EVENT_PLAYER_MISS,
    /* The player's power-up is about to change. value: the new one (NEBULA_POWER_UP_*); what you return is used. */
    NEBULA_MOD_EVENT_POWER_UP,
    /* Coins are about to be added. value: how many; what you return is added. */
    NEBULA_MOD_EVENT_COIN,
    /* Star bits are about to be added. value: how many; what you return is added. */
    NEBULA_MOD_EVENT_STAR_BIT,
    /* A star has been won. name: the galaxy. value: the star's number. */
    NEBULA_MOD_EVENT_STAR_GET,
    /* A cutscene starts or ends. name: the cutscene. */
    NEBULA_MOD_EVENT_CUTSCENE_START,
    NEBULA_MOD_EVENT_CUTSCENE_END,
    /* Any object appears or dies. actor: the object. name: its name. Subscribing to these costs a call per object. */
    NEBULA_MOD_EVENT_ACTOR_APPEAR,
    NEBULA_MOD_EVENT_ACTOR_KILL,
    /* The player's spin starts. */
    NEBULA_MOD_EVENT_PLAYER_SPIN,
    /* The player leaves the ground in a jump; lands. */
    NEBULA_MOD_EVENT_PLAYER_JUMP,
    NEBULA_MOD_EVENT_PLAYER_LAND,
    /* The pause menu opens or closes. value: 1 open, 0 closed. */
    NEBULA_MOD_EVENT_PAUSE,
    /* The stage is ending: every actor and layout of it goes after this. name: the stage. */
    NEBULA_MOD_EVENT_STAGE_END,
    /* The game is about to write the save. */
    NEBULA_MOD_EVENT_GAME_SAVE,
    /* A power star comes into the stage. value: its number. */
    NEBULA_MOD_EVENT_POWER_STAR_APPEAR,
    /* A line of talk opens in its balloon. actor: who talks. name: the line's message id, as luma.json "text" and
       add_text name it (null for a line with no id). */
    NEBULA_MOD_EVENT_TALK_LINE,
    /* The player answered a talk's yes/no question. value: 1 yes, 0 no. actor: who asked. name: the question's
       message id. */
    NEBULA_MOD_EVENT_TALK_CHOICE,
    /* A part of a cutscene starts. value: the part's number in the cutscene, from 0. name: the part, as the
       cutscene's time sheet names it. */
    NEBULA_MOD_EVENT_CUTSCENE_PART,
    /* The camera changes. value: 0 an area's camera, 1 an event camera (a cutscene's, start_camera's). name: its
       id, as the stage's camera table names it. */
    NEBULA_MOD_EVENT_CAMERA_CHANGE,
    /* A sound effect is about to start. value: its number; name: its name. What you return plays: that number, another
       sound's, or -1 for none. Subscribing costs a call per sound. */
    NEBULA_MOD_EVENT_SOUND_START,
    /* An object starts one of its effects. actor: the object. name: the effect, as the object's effect table names
       it. Subscribing costs a call per effect. */
    NEBULA_MOD_EVENT_EFFECT_START,
    /* A screen layout (a counter, a menu, a message box) is shown or goes away. value: 1 shown, 0 gone. name: its
       name in the game's code. */
    NEBULA_MOD_EVENT_SCREEN,
    /* An object gets a message from another: an attack, a stomp, a push, a grab (docs/modding/EVENTS.md lists the
       numbers). actor: who gets it. name: its name. value: the message. Return -1 to refuse it: the object acts as if
       it never came. Subscribing costs a call per message. */
    NEBULA_MOD_EVENT_ACTOR_MESSAGE,
    /* The player starts an action of their own: a swim, a hang, a slide, a fall (docs/modding/EVENTS.md lists the
       numbers). value: the action. */
    NEBULA_MOD_EVENT_PLAYER_ACTION,
    NEBULA_MOD_EVENT_COUNT
```

### `NebulaModEventData`

```c
    int32_t value;
    NebulaActor* actor;
    const char* name;
```

### `NebulaModValue`

```c
    NEBULA_VALUE_LIFE = 0,
    NEBULA_VALUE_MAX_LIFE,
    NEBULA_VALUE_POWER_UP,
    NEBULA_VALUE_ON_GROUND,
    NEBULA_VALUE_STAR_BITS,
    NEBULA_VALUE_COINS,
    NEBULA_VALUE_LIVES,
    NEBULA_VALUE_MISSION,
    NEBULA_VALUE_POWER_STARS,
    NEBULA_VALUE_WATER_LIFE,
    /* 0 or 1, what the player is doing. */
    NEBULA_VALUE_SPINNING,
    NEBULA_VALUE_JUMP_RISING,
    NEBULA_VALUE_DAMAGED,
    NEBULA_VALUE_DEAD,
    NEBULA_VALUE_GROUND_POUNDING,
    NEBULA_VALUE_SKATING,
    NEBULA_VALUE_CROUCHING,
    NEBULA_VALUE_FLYING,
    NEBULA_VALUE_SWIMMING,
    NEBULA_VALUE_LUIGI
```

### `NebulaModSet`

```c
    NEBULA_SET_LIFE = 0,
    NEBULA_SET_MAX_LIFE,
    NEBULA_SET_POWER_UP,
    NEBULA_SET_ADD_COINS,
    NEBULA_SET_ADD_STAR_BITS,
    NEBULA_SET_ADD_LIVES
```

### `NebulaPowerUp`

```c
    NEBULA_POWER_UP_NONE = 0,
    NEBULA_POWER_UP_INVINCIBLE = 1,
    NEBULA_POWER_UP_FIRE = 2,
    NEBULA_POWER_UP_ICE = 3,
    NEBULA_POWER_UP_BEE = 4,
    NEBULA_POWER_UP_SPRING = 5,
    NEBULA_POWER_UP_BOO = 6,
    NEBULA_POWER_UP_FLYING = 7,
    NEBULA_POWER_UP_TORNADO = 9
```

### `NebulaModPlay`

```c
    NEBULA_PLAY_ANIMATION = 0, /* its action: animations, effects and sounds together */
    NEBULA_PLAY_EFFECT,
    NEBULA_PLAY_SOUND,
    NEBULA_PLAY_PLAYER_ANIMATION, /* on the player, by the animation's name */
    /* One part of a model's animation by its file's name, leaving the others playing. */
    NEBULA_PLAY_SKELETON,        /* .bck */
    NEBULA_PLAY_TEXTURE_MOVE,    /* .btk */
    NEBULA_PLAY_COLOUR,          /* .brk */
    NEBULA_PLAY_SHOWN_PARTS,     /* .bva */
    NEBULA_PLAY_TEXTURE_FRAMES,  /* .btp */
    NEBULA_PLAY_MATERIAL_COLOUR  /* .bpk */
```

### `NebulaObjectClass`

```c
    uint32_t size;
    const char* name;
    const char* archive; /* its model and animations: ObjectData/<archive>.arc. Null: nothing to see */
    float radius;        /* what it touches with */
    void* user;
    void (*start)(NebulaActor* actor, void* user);
    void (*update)(NebulaActor* actor, void* user);
    void (*touch_player)(NebulaActor* actor, void* user);
    /* The player attacked it. Return 1 if it took the attack. */
    int32_t (*attacked)(NebulaActor* actor, void* user);
    /* The same, saying how: NEBULA_ATTACK_*. If set, it's called in place of `attacked`. */
    int32_t (*attacked_by)(NebulaActor* actor, int32_t how, void* user);
    /* What follows may be left out of an older plugin's class: `size` says. */
    int32_t kind;         /* NEBULA_OBJECT_*: how it touches things and where it's drawn */
    int32_t collision;    /* 1: its archive's collision model is solid, and moves with it */
    float shadow_radius;  /* a round shadow under it; 0: none */
    float clip_radius;    /* not run or drawn when off screen past this; 0: always */
    void (*appeared)(NebulaActor* actor, void* user);
    void (*killed)(NebulaActor* actor, void* user);
```

### `NebulaObjectKind`

```c
    NEBULA_OBJECT_ENEMY = 0,
    NEBULA_OBJECT_MAP,      /* a platform, a wall: the player stands on it */
    NEBULA_OBJECT_NPC,
    NEBULA_OBJECT_RIDE,
    NEBULA_OBJECT_DECORATION  /* nothing touches it */
```

### `NebulaAttack`

```c
    NEBULA_ATTACK_OTHER = 0,
    NEBULA_ATTACK_SPIN,
    NEBULA_ATTACK_STOMP,
    NEBULA_ATTACK_GROUND_POUND,
    NEBULA_ATTACK_STAR_BIT,
    NEBULA_ATTACK_FIRE,
    NEBULA_ATTACK_INVINCIBLE,
    NEBULA_ATTACK_SHELL
```

### `NebulaHit`

```c
    NEBULA_HIT_NORMAL = 0,
    NEBULA_HIT_STRONG,
    NEBULA_HIT_FIRE,
    NEBULA_HIT_ELECTRIC,
    NEBULA_HIT_FREEZE,
    NEBULA_HIT_FLIP
```

### `NebulaSwitch`

```c
    NEBULA_SWITCH_A = 0,
    NEBULA_SWITCH_B,
    NEBULA_SWITCH_APPEAR,
    NEBULA_SWITCH_DEAD
```

### `NebulaItem`

```c
    NEBULA_ITEM_COINS = 0,
    NEBULA_ITEM_STAR_BITS,
    NEBULA_ITEM_POWER_STAR /* count: the star's number in the galaxy; -1: the next */
```

### `NebulaShake`

```c
    NEBULA_SHAKE_WEAK = 0,
    NEBULA_SHAKE_NORMAL,
    NEBULA_SHAKE_NORMAL_STRONG,
    NEBULA_SHAKE_STRONG,
    NEBULA_SHAKE_VERY_STRONG
```

### `NebulaGalaxy`

```c
    uint32_t size;
    const char* name;
    const char* map_pane;           /* its icon's pane in the map's layout (MapGrandGalaxy). Null: not on the map */
    const char* open_condition[3];  /* the event flags that open it, or null */
    int32_t stars_to_open;          /* power stars it takes to open */
    int32_t dome;                   /* the dome it belongs to: 1 to 6, 0 for none */
    int32_t has_map_position;       /* 1: the game makes its icon's pane, at map_position */
    float map_position[2];          /* from the middle of its dome's page, or of the grand map with no dome: right and up, 608 wide by 456 high */
```

### `NebulaPowerUpClass`

```c
    uint32_t size;
    const char* name;
    int32_t frames;       /* how long it lasts, at 60 a second. 0: until something ends it */
    const char* music;    /* the music while it lasts: a BGM_ name, or null */
    int32_t color_frame;  /* the player's colours: a frame of Mario.arc's ColorChange (0 his own, 1 Flying's, 2 Fire's). 0: those of
                             looks_like. A mod's Mario.arc can add more. */
    void* user;
    void (*start)(void* user);
    void (*update)(void* user); /* every frame it lasts */
    void (*end)(void* user);
    /* What follows may be left out of an older plugin's class: `size` says. */
    const char* model;    /* a whole-body model, ObjectData/<model>.arc, on the player's skeleton. Null: the game's */
    int32_t looks_like;   /* with no model: the look of one of the game's (NEBULA_POWER_UP_*) */
```

### `NebulaNervePhase`

```c
    NEBULA_NERVE_ENTER = 0, /* frames is 0 */
    NEBULA_NERVE_FRAME,     /* before the game's frame of it: return 0 and the game's is not run (yours stands in) */
    NEBULA_NERVE_LEAVE      /* another nerve was set; the game's own end of this one has run */
```

### `NebulaHudPart`

```c
    NEBULA_HUD_COINS = 1u << 0,
    NEBULA_HUD_STAR_BITS = 1u << 1,
    NEBULA_HUD_LIVES = 1u << 2,
    NEBULA_HUD_STARS = 1u << 3,
    NEBULA_HUD_LIFE_METER = 1u << 4,
    NEBULA_HUD_POINTER = 1u << 5,
    NEBULA_HUD_TIMER = 1u << 6,
    NEBULA_HUD_ALL = 0x7Fu
```

### `NebulaObjectsIn`

```c
    NEBULA_OBJECTS_PLACED = 0,       /* whatever the game creates by name: a level's objects, areas */
    NEBULA_OBJECTS_DISPLAY_MODEL = 1, /* the still models things show before they're collected */
    NEBULA_OBJECTS_TREASURE_BOX = 2, /* what a treasure box holds */
    NEBULA_OBJECTS_IN_COUNT
```

### `NebulaButton`

```c
    NEBULA_BUTTON_JUMP = 1u << 0,
    NEBULA_BUTTON_SPIN = 1u << 1, /* the player's spin: held while it lasts, pressed on the frame it starts */
    NEBULA_BUTTON_CROUCH = 1u << 2,
    NEBULA_BUTTON_CAMERA = 1u << 3,
    NEBULA_BUTTON_FIRE = 1u << 4, /* the star-bit shot */
    NEBULA_BUTTON_DPAD_UP = 1u << 5,
    NEBULA_BUTTON_DPAD_DOWN = 1u << 6,
    NEBULA_BUTTON_DPAD_LEFT = 1u << 7,
    NEBULA_BUTTON_DPAD_RIGHT = 1u << 8,
    NEBULA_BUTTON_PAUSE = 1u << 9,
    NEBULA_BUTTON_MAP = 1u << 10
```
