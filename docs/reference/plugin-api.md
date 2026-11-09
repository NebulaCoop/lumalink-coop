# The plugin interface, call by call

Written from `nebula_mod_api.h`, in its order: fields are only ever added at the end, so a plugin keeps working on later versions of Nebula. Check `game->size` before using a call newer than the oldest Nebula you support.

A call on an actor with a null pointer is your bug: nothing checks. Calls about the player or the stage do nothing, and answer 0 or null, between stages and on the file select screen.

| Call | Signature |
|---|---|
| `log` | `void log(const NebulaPlugin* self, const char* text)` |
| `option` | `const char* option(const NebulaPlugin* self, const char* id)` |
| `subscribe` | `int32_t subscribe(NebulaModEvent event, NebulaModCallback callback, void* user)` |
| `register_object` | `int32_t register_object(const NebulaObjectClass* object)` |
| `register_creator` | `int32_t register_creator(const char* name, void* (*create)(const char* name), const char* archive)` |
| `hook` | `int32_t hook(void* target, void* replacement, void** original)` |
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
| `actors_near` | `int32_t actors_near(const float* xyz, float radius, NebulaActor** out, int32_t max)` |
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
| `tunable_at` | `int32_t tunable_at(int32_t index, const char** file, const char** name, int32_t* place, int32_t* kind, double* game_value, double* value)` |

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

Only in nebula_plugin_start. 0: done.

### `register_object`

```c
int32_t register_object(const NebulaObjectClass* object)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `register_creator`

```c
int32_t register_creator(const char* name, void* (*create)(const char* name), const char* archive)
```

`create` makes the game's own object (a NameObj) from its placed name; `archive` is loaded for it, or null.

### `hook`

```c
int32_t hook(void* target, void* replacement, void** original)
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

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_position`

```c
void set_position(NebulaActor* actor, const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `get_velocity`

```c
void get_velocity(NebulaActor* actor, float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_velocity`

```c
void set_velocity(NebulaActor* actor, const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `value`

```c
int32_t value(NebulaModValue what)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_value`

```c
void set_value(NebulaModSet what, int32_t value)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `kill_player`

```c
void kill_player(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `jump_player`

```c
void jump_player(const float* velocity)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `appear`

```c
void appear(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `kill`

```c
void kill(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_dead`

```c
int32_t is_dead(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `show_model`

```c
void show_model(NebulaActor* actor, int32_t show)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `play`

```c
int32_t play(NebulaActor* actor, NebulaModPlay what, const char* name)
```

1: it started. 0: the object has no such thing, or nothing to play it with.

### `actor_name`

```c
const char* actor_name(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `stage_name`

```c
const char* stage_name(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `spawn`

```c
NebulaActor* spawn(const char* name, const float* xyz)
```

A registered object at a place. Its archive must already be loaded: a level places it, or the stage holds it. Null: it can't be made now.

### `buttons_held`

```c
uint32_t buttons_held(void)
```

The controls held, and those pressed on this frame: NEBULA_BUTTON_*.

### `buttons_pressed`

```c
uint32_t buttons_pressed(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `stick`

```c
void stick(float* x, float* y)
```

The stick, each from -1 to 1.

### `shake_camera`

```c
void shake_camera(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `add_galaxy`

```c
int32_t add_galaxy(const NebulaGalaxy* galaxy)
```

Only in nebula_plugin_start. 0: done.

### `add_hungry_luma`

```c
int32_t add_hungry_luma(const char* galaxy, int32_t slot, int32_t star_bits)
```

A hungry Luma: placed in a level as TicoGalaxy with its Arg7 at `slot`, it opens `galaxy` when fed `star_bits`. The game's own are slots 0 to 6: using one takes its place.

### `add_text`

```c
int32_t add_text(const char* id, const char* utf8)
```

A message of the game by its id, in place of its own or new: GalaxyName_<galaxy>, ScenarioName_<galaxy><n>, a talk's id.

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

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_movie_playing`

```c
int32_t is_movie_playing(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_event_flag`

```c
int32_t set_event_flag(const char* name, int32_t kind, int32_t a, int32_t b, const char* first, const char* second)
```

Only in nebula_plugin_start. A row of the game's progression, by its name: what opens what, and what each hungry Luma costs. A name the game has is changed, a new one added. `kind`: 1 stars, 2 a galaxy's stars, 3 a special star, 4 two flags, 5 story, 6 a galaxy, 7 a comet, 8 a hungry Luma. `a` and `b` follow the kind: the star's number, or the Luma's slot and its price. 0: done.

### `set_comet_schedule`

```c
int32_t set_comet_schedule(int32_t dome, const char* const* galaxies, const int32_t* coins, int32_t count)
```

Only in nebula_plugin_start. A dome's comets, 1 to 6, in the order they come, for the whole of its schedule: `galaxies[i]` with `coins[i]` at 1 for the hundred purple coins.

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

An object's animation: 1 while the last one started hasn't reached its end; its frame, its length, setting them.

### `animation_frame`

```c
float animation_frame(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `animation_length`

```c
float animation_length(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_animation_frame`

```c
void set_animation_frame(NebulaActor* actor, float frame)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_animation_rate`

```c
void set_animation_rate(NebulaActor* actor, float rate)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `stop_effect`

```c
void stop_effect(NebulaActor* actor, const char* name)
```

null: all of them

### `stop_sound`

```c
void stop_sound(NebulaActor* actor, const char* name)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_state`

```c
void set_state(NebulaActor* actor, int32_t state)
```

An object of register_object's own step in what it's doing. Starts at 0; state_frames counts from the set_state that began it.

### `state`

```c
int32_t state(NebulaActor* actor)
```

The number is the plugin's own: an enum of its steps.

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `state_frames`

```c
int32_t state_frames(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `get_rotation`

```c
void get_rotation(NebulaActor* actor, float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_rotation`

```c
void set_rotation(NebulaActor* actor, const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `get_scale`

```c
void get_scale(NebulaActor* actor, float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_scale`

```c
void set_scale(NebulaActor* actor, const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_front`

```c
void set_front(NebulaActor* actor, const float* xyz)
```

register_object's objects: the way it faces, standing against gravity. Its model then turns with this, not with its rotation.

### `get_front`

```c
void get_front(NebulaActor* actor, float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `turn_to_player`

```c
void turn_to_player(NebulaActor* actor, float degrees)
```

at most this far this frame

### `set_touchable`

```c
void set_touchable(NebulaActor* actor, int32_t touchable)
```

register_object's objects: whether anything can touch it, and hurting the player.

### `hit_player`

```c
int32_t hit_player(NebulaActor* actor, NebulaHit how)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `placement_number`

```c
int32_t placement_number(NebulaActor* actor, const char* column, double* value)
```

A column of a `register_object` object's row in the level, by the column's name as a level editor shows it: `Obj_arg0` to `Obj_arg7` (-1 where the level sets none), `CommonPath_ID`, `ShapeModelNo`, `CameraSetId`, `MessageId`, `GroupId`, `ClippingGroupId`, `DemoGroupId`, `ViewGroupId`, `CastId`, `ParentID`, the `SW_` columns, `l_id`. 1 and the number for a number column, whole or not; 0 for a column the row lacks, a column of words, or an object the level didn't place (`spawn`).

```c
double speed = 0;
if (game->placement_number(actor, "Obj_arg0", &speed) && speed >= 0) { /* the level set it */ }
```

### `placement_text`

```c
const char* placement_text(NebulaActor* actor, const char* column)
```

A column of words of the same row (`name`), or null.

### `has_switch`

```c
int32_t has_switch(NebulaActor* actor, NebulaSwitch which)
```

Its switches. is_valid: the level set one.

### `is_switch_on`

```c
int32_t is_switch_on(NebulaActor* actor, NebulaSwitch which)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_switch`

```c
void set_switch(NebulaActor* actor, NebulaSwitch which, int32_t on)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `has_rail`

```c
int32_t has_rail(NebulaActor* actor)
```

Its rail from the level. move_on_rail goes `speed` along and puts the object there.

### `move_on_rail`

```c
void move_on_rail(NebulaActor* actor, float speed)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `rail_at_end`

```c
int32_t rail_at_end(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `rail_turn_back`

```c
void rail_turn_back(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `rail_coord`

```c
float rail_coord(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_rail_coord`

```c
void set_rail_coord(NebulaActor* actor, float coord)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `rail_length`

```c
float rail_length(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `raycast`

```c
int32_t raycast(const float* from, const float* to, float* hit)
```

The world. raycast: 1 and where if the line from `from` to `to` meets the map.

### `is_on_ground`

```c
int32_t is_on_ground(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_touching_wall`

```c
int32_t is_touching_wall(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_in_water`

```c
int32_t is_in_water(const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `distance_to_player`

```c
float distance_to_player(NebulaActor* actor)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `get_joint_position`

```c
void get_joint_position(NebulaActor* actor, const char* joint, float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `player_front`

```c
void player_front(float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_player_control`

```c
void set_player_control(int32_t on)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `declare_item`

```c
void declare_item(NebulaActor* actor, NebulaItem what, int32_t count)
```

Items. declare_item in `start`; then appear_item at the moment it comes out.

### `appear_item`

```c
void appear_item(NebulaActor* actor, NebulaItem what, const float* xyz, int32_t count)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `start_cutscene`

```c
int32_t start_cutscene(NebulaActor* actor, const char* name)
```

Cutscenes and cameras. start_cutscene: 1 when it began (another may be playing: ask again next frame).

### `end_cutscene`

```c
void end_cutscene(NebulaActor* actor, const char* name)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_cutscene_playing`

```c
int32_t is_cutscene_playing(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `shake_camera_by`

```c
void shake_camera_by(NebulaShake how)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `start_camera`

```c
void start_camera(const char* name, int32_t frames)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `end_camera`

```c
void end_camera(const char* name, int32_t frames)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `fade_out`

```c
void fade_out(int32_t frames)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `fade_in`

```c
void fade_in(int32_t frames)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `change_stage`

```c
void change_stage(const char* galaxy, int32_t mission)
```

The game's progress and text.

### `back_to_dome`

```c
void back_to_dome(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `is_event_flag_on`

```c
int32_t is_event_flag_on(const char* name)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_event_flag_on`

```c
void set_event_flag_on(const char* name)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `has_star`

```c
int32_t has_star(const char* galaxy, int32_t star)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `show_message`

```c
void show_message(const char* utf8_or_id, int32_t centre)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `hide_message`

```c
void hide_message(void)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `store_get`

```c
int32_t store_get(const NebulaPlugin* self, const char* key, int32_t missing)
```

A number kept for this mod across plays.

### `store_set`

```c
void store_set(const NebulaPlugin* self, const char* key, int32_t value)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

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

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_object_class`

```c
int32_t set_object_class(const char* name, const char* like)
```

Only in nebula_plugin_start. A new name levels can place, made as the game's object `like`, with its files from ObjectData/<name>.arc. No code needed. 0: done.

### `add_sound_effect`

```c
int32_t add_sound_effect(const char* name, const char* file)
```

Only in nebula_plugin_start. A sound by its name, played from a file (AudioRes/Stream/Boom.wav, .ogg, .ast): a new name, or one of the game's (SE_OJ_COIN_GET) to replace it wherever it plays. 0: done.

### `find_actor`

```c
NebulaActor* find_actor(const char* name, NebulaActor* after)
```

Finding things. An actor by the name levels place it under; `after` null starts, a found one goes on from it. `name` null: every actor.

### `actors_near`

```c
int32_t actors_near(const float* xyz, float radius, NebulaActor** out, int32_t max)
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

A pointer of your own on any of register_object's objects.

### `set_actor_data`

```c
void set_actor_data(NebulaActor* actor, void* data)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

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

The world.

### `is_in_area`

```c
int32_t is_in_area(const char* area, const float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `camera_position`

```c
void camera_position(float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `camera_target`

```c
void camera_target(float* xyz)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

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

Save data. An event value (a named number the game keeps), by its name.

### `set_event_value`

```c
void set_event_value(const char* name, int32_t value)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `set_star`

```c
void set_star(const char* galaxy, int32_t star, int32_t have)
```

A star as if won, or taken away.

### `stars_in_galaxy`

```c
int32_t stars_in_galaxy(const char* galaxy)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `stars_won_in_galaxy`

```c
int32_t stars_won_in_galaxy(const char* galaxy)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `get_tunable`

```c
int32_t get_tunable(const char* key, double* value, double* game_value)
```

A tuning number by its key, as set_tunable takes it ("GameRules.cPlayerHealth"): the number now and the game's. 0: done; -1: no such number. Interface 2; it took the place of `setting` and `setting_by_number`.

### `layout_open`

```c
NebulaLayout* layout_open(const char* name)
```

A layout on the screen, for this stage. Null if it has no such archive.

### `layout_show`

```c
void layout_show(NebulaLayout* layout, int32_t show)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_set_text`

```c
void layout_set_text(NebulaLayout* layout, const char* pane, const char* utf8)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_set_number`

```c
void layout_set_number(NebulaLayout* layout, const char* pane, int32_t number)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_show_pane`

```c
void layout_show_pane(NebulaLayout* layout, const char* pane, int32_t show)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_move_pane`

```c
void layout_move_pane(NebulaLayout* layout, const char* pane, float x, float y)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_play`

```c
void layout_play(NebulaLayout* layout, const char* animation)
```

Its animation by name.

### `layout_is_playing`

```c
int32_t layout_is_playing(NebulaLayout* layout)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `layout_set_alpha`

```c
void layout_set_alpha(NebulaLayout* layout, float alpha)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

### `store_get_text`

```c
int32_t store_get_text(const NebulaPlugin* self, const char* key, char* out, int32_t size)
```

Text kept for this mod across plays, in place of store_get's numbers: how much was read.

### `store_set_text`

```c
void store_set_text(const NebulaPlugin* self, const char* key, const char* text)
```

See the overview in [the plugin guide](../tutorials/10-first-plugin.md).

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

A file's tuning number by "<file stem>.<name>" ("Kuribo.cWalkSpeed"), or a list's place by "<file stem>.<name>[<i>]", as luma.json's "tunables" set them. Any time; most are read when an object starts, so the host sets luma.json's before the first stage. Whole kinds round and clamp. 0: done; -1: no such number.

### `tunable_at`

```c
int32_t tunable_at(int32_t index, const char** file, const char** name, int32_t* place, int32_t* kind, double* game_value, double* value)
```

The index-th tuning number (a list counts each place): its file ("Kuribo.cpp"), name, place in its list (-1: one number), kind (1 f32, 2 f64, 3 s32, 4 u32, 5 s16, 6 u16, 7 s8, 8 u8, 9 bool), the game's number and the number now. 0: done; -1: past the last.

## Numbers and names

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
    /* The player is about to die. value: how. */
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
    NEBULA_MOD_EVENT_COUNT
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

### `NebulaPlugin`

```c
    uint32_t size;
    const char* id;       /* the mod's id */
    const char* version;  /* the mod's version */
```

### `NebulaModEventData`

```c
    int32_t value;
    NebulaActor* actor;
    const char* name;
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
    int32_t color_frame;  /* the player's colours: a frame of Mario.arc's ColorChange (0 his own, 1 Fire's). A mod's Mario.arc can add more. */
    void* user;
    void (*start)(void* user);
    void (*update)(void* user); /* every frame it lasts */
    void (*end)(void* user);
    /* What follows may be left out of an older plugin's class: `size` says. */
    const char* model;    /* a whole-body model, ObjectData/<model>.arc, on the player's skeleton. Null: the game's */
    int32_t looks_like;   /* with no model: the look of one of the game's (NEBULA_POWER_UP_*) */
```

