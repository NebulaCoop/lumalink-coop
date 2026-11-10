# The plugin interface, call by call

Written from `nebula_mod_api.h` (by `tools/plugin_api_doc.py`), in its order: fields are only ever added at the end, so a plugin keeps working on later versions of Nebula. Check `game->size` before using a call newer than the oldest Nebula you support.

A null object, layout, point or name where a call needs one makes the call do nothing: it answers 0, null, or -1 where -1 means "no such". Where null means something (`find_actor`'s `name`), the call says so. Calls about the player or the stage do nothing, and answer 0 or null, between stages and on the file select screen. A number that is NaN or infinite, where a position, motion, angle, scale, time or tuning number is set, changes nothing either.

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
| `game_camera` | `int32_t game_camera(float* position, float* target)` |
| `talking_actor` | `NebulaActor* talking_actor(void)` |
| `place_talk_box` | `void place_talk_box(float x, float y, int32_t on)` |
| `show_joint` | `void show_joint(NebulaActor* actor, const char* joint, int32_t on)` |
| `joint_name` | `const char* joint_name(NebulaActor* actor, int32_t index)` |
| `get_joint_matrix` | `void get_joint_matrix(NebulaActor* actor, const char* joint, float* matrix)` |
| `give_star` | `int32_t give_star(const char* galaxy, int32_t star, int32_t grand)` |
| `give_grand_stars` | `int32_t give_grand_stars(int32_t count)` |
| `set_player_character` | `int32_t set_player_character(int32_t luigi)` |
| `list_materials` | `int32_t list_materials(const char* model, NebulaMaterialCallback callback, void* user)` |
| `water_point` | `int32_t water_point(int32_t index, float* xyz)` |
| `circle_wipe` | `int32_t circle_wipe(int32_t open, int32_t frames)` |
| `grant_spin_and_pointer` | `int32_t grant_spin_and_pointer(void)` |
| `toolkit_find` | `int32_t toolkit_find(const char* name)` |
| `toolkit_call` | `int32_t toolkit_call(int32_t index, const NebulaValue* args, uint32_t count, NebulaValue* out, uint32_t out_count)` |
| `actor_value` | `NebulaValue actor_value(NebulaActor* actor)` |
| `value_actor` | `NebulaActor* value_actor(const NebulaValue* value)` |
| `field_find` | `int32_t field_find(const NebulaValue* object, const char* name)` |
| `field_get` | `int32_t field_get(const NebulaValue* object, int32_t field, uint32_t element, NebulaValue* out)` |
| `field_set` | `int32_t field_set(const NebulaValue* object, int32_t field, uint32_t element, const NebulaValue* value)` |
| `class_name` | `const char* class_name(uint32_t id, uint32_t* parent)` |
| `is_a` | `int32_t is_a(const NebulaValue* object, const char* class_name)` |
| `luma_start` | `void luma_start(const struct NebulaModApi* api)` |
| `add_script_object` | `int32_t add_script_object(const NebulaPlugin* mod, const NebulaObjectClass* shape, const char* script)` |
| `add_mod_script` | `int32_t add_mod_script(const NebulaPlugin* mod, const char* script)` |

## What each does

### `log`

```c
void log(const NebulaPlugin* self, const char* text)
```

Writes one line to the game's log.

### `option`

```c
const char* option(const NebulaPlugin* self, const char* id)
```

The choice id the player picked for one of the mod's options. Null: no such option.

### `subscribe`

```c
int32_t subscribe(NebulaModEvent event, NebulaModCallback callback, void* user)
```

Only in nebula_plugin_start. Calls `callback` at each `event` (NEBULA_MOD_EVENT_*). Its return value matters where the event says so (a cancelled hit, changed coins); otherwise return data->value. Returns 0.

### `register_object`

```c
int32_t register_object(const NebulaObjectClass* object)
```

Adds an object class that levels can place by name, run by the plugin's callbacks (NebulaObjectClass). 0: done; -1: no name, a name already taken, or a class too old to read.

### `register_creator`

```c
int32_t register_creator(const char* name, void*(*create)(const char* name), const char* archive)
```

`create` builds the game's own object type (a NameObj) for a placed name; `archive` is loaded for it, or null.

### `hook`

```c
int32_t hook(void* target, void* replacement, void* * original)
```

Runs `replacement` instead of the game's function at `target`; `original` receives a function that runs the previous code (possibly another plugin's hook). Functions the compiler inlined everywhere can't be reached this way; use events. Works on Windows x64, and Linux and Mac on x64 and arm64. Returns -1 elsewhere, or when the function's first instructions can't be moved.

### `find_function`

```c
void* find_function(const char* symbol)
```

A game function by its symbol (on Windows, as dumpbin /exports names it). Null: not found.

### `player`

```c
NebulaActor* player(void)
```

The player. Null when no stage is running, or outside its update, animation and drawing (a hook on GameScene::draw is inside; "In a stage" calls below work there too).

### `get_position`

```c
void get_position(NebulaActor* actor, float* xyz)
```

Position: x, y, z in level units.

### `set_position`

```c
void set_position(NebulaActor* actor, const float* xyz)
```

Moves it there at once. The player is placed as after a warp; an object just moves.

### `get_velocity`

```c
void get_velocity(NebulaActor* actor, float* xyz)
```

Movement per frame: x, y, z. For the player, the game's own.

### `set_velocity`

```c
void set_velocity(NebulaActor* actor, const float* xyz)
```

Sets the movement; the game's gravity and friction continue from it.

### `value`

```c
int32_t value(NebulaModValue what)
```

A game value (NEBULA_VALUE_*). Player values are 0 when no stage is running.

### `set_value`

```c
void set_value(NebulaModSet what, int32_t value)
```

Sets a value (NEBULA_SET_*); does nothing when no stage is running. Life 0 or less kills the player; above the maximum is clamped. A power-up is the game's (NEBULA_POWER_UP_*) or one from add_power_up.

### `kill_player`

```c
void kill_player(void)
```

Kills the player, as a fall out of the level does.

### `jump_player`

```c
void jump_player(const float* velocity)
```

Launches the player off the ground with this velocity, like a spring.

### `appear`

```c
void appear(NebulaActor* actor)
```

Brings it into the level (after a kill, or before its first appearance): drawn, moving and touchable.

### `kill`

```c
void kill(NebulaActor* actor)
```

Removes it from the level, like a defeated enemy: not drawn, moved or touched. appear brings it back.

### `is_dead`

```c
int32_t is_dead(NebulaActor* actor)
```

1 while it is out of the level (killed, or not yet appeared).

### `show_model`

```c
void show_model(NebulaActor* actor, int32_t show)
```

Shows (1) or hides (0) its model; it still moves and collides. Does nothing for an object without a model. For the player this covers hands, face and power-up suit; the shadow stays.

### `play`

```c
int32_t play(NebulaActor* actor, NebulaModPlay what, const char* name)
```

1: started. 0: the object has no such animation, effect or sound, or nothing to play it on.

### `actor_name`

```c
const char* actor_name(NebulaActor* actor)
```

The name it was created with: a placed object's level name (Kuribo). Some of the game's own objects have Japanese names.

### `stage_name`

```c
const char* stage_name(void)
```

The current stage (EggStarGalaxy); null when there is none.

### `spawn`

```c
NebulaActor* spawn(const char* name, const float* xyz)
```

Creates a registered object at a point. Its archive must already be loaded (placed in the level, or part of the stage). Null: it can't be created now.

### `buttons_held`

```c
uint32_t buttons_held(void)
```

Buttons held now: NEBULA_BUTTON_* flags. 0 when no stage is running.

### `buttons_pressed`

```c
uint32_t buttons_pressed(void)
```

Buttons pressed this frame only: NEBULA_BUTTON_* flags.

### `stick`

```c
void stick(float* x, float* y)
```

The stick, each axis from -1 to 1 (either pointer may be null).

### `shake_camera`

```c
void shake_camera(void)
```

In a stage: shakes the camera as when something heavy lands (shake_camera_by sets the strength).

### `add_galaxy`

```c
int32_t add_galaxy(const NebulaGalaxy* galaxy)
```

Only in nebula_plugin_start. Adds a galaxy. 0: done; -2: no scenario archive (StageData/<name>/<name>Scenario.arc in the mod, StageData/<name>Scenario in the store), so it is skipped; -1: any other problem (its name is already one of the game's).

### `add_hungry_luma`

```c
int32_t add_hungry_luma(const char* galaxy, int32_t slot, int32_t star_bits)
```

A hungry Luma: placed in a level as TicoGalaxy with Arg7 set to `slot`, it opens `galaxy` when fed `star_bits`. The game uses slots 0 to 6; using one replaces the game's Luma.

### `add_text`

```c
int32_t add_text(const char* id, const char* utf8)
```

Replaces or adds a message by id: GalaxyName_<galaxy>, ScenarioName_<galaxy><n>, a conversation's id. An existing line keeps its talk settings. The text may use the game's message tags in brackets (nebula_text_tags.h).

### `add_power_up`

```c
int32_t add_power_up(const NebulaPowerUpClass* power_up)
```

Adds a power-up and returns its number for set_value; below 0 when it can't be added.

### `get_gravity`

```c
void get_gravity(NebulaActor* actor, float* xyz)
```

Its down direction, unit length (on a planet this is not a fixed axis).

### `add_music`

```c
int32_t add_music(const char* name, const char* file)
```

Only in nebula_plugin_start. Adds a music track: `file` is its path in the game, AudioRes/Stream/MyTrack.wav (.ogg, .ast). Pick any `name` not starting with SE_, BGM_, MBGM_, STM_ or ME_, which are the game's. 0: done.

### `set_stage_music`

```c
int32_t set_stage_music(const char* stage, int32_t scenario, const char* name)
```

Only in nebula_plugin_start. The music a stage plays: `scenario` is a mission, 0 for every mission without its own entry.

### `set_area_music`

```c
int32_t set_area_music(const char* stage, int32_t area, const char* name)
```

Only in nebula_plugin_start. The music a ChangeBgmCube with Obj_arg1 equal to `area` starts in the stage.

### `play_music`

```c
int32_t play_music(const char* name)
```

In a stage: plays this track (a mod's or the game's, by name) instead of the current one. 1: started.

### `stop_music`

```c
void stop_music(int32_t frames)
```

In a stage: fades out the current music over this many frames.

### `play_movie`

```c
int32_t play_movie(const char* file)
```

In a stage: plays a movie over the game by its path, MovieData/MyMovie.thp. The game waits while it plays. 1: started.

### `stop_movie`

```c
void stop_movie(void)
```

Ends the movie now; the game continues.

### `is_movie_playing`

```c
int32_t is_movie_playing(void)
```

1 while a movie plays.

### `set_event_flag`

```c
int32_t set_event_flag(const char* name, int32_t kind, int32_t a, int32_t b, const char* first, const char* second)
```

Only in nebula_plugin_start. Changes or adds a progression row by name: what unlocks what, and what each hungry Luma costs. `kind`: 1 stars, 2 a galaxy's stars, 3 a special star, 4 two flags, 5 story, 6 a galaxy, 7 a comet, 8 a hungry Luma. `a` and `b` depend on the kind: a star number, or a Luma's slot and price. 0: done.

### `set_comet_schedule`

```c
int32_t set_comet_schedule(int32_t dome, const char* const* galaxies, const int32_t* coins, int32_t count)
```

Only in nebula_plugin_start. A dome's full comet schedule, in order (domes 1 to 6 are the game's; 7, 8 and up add domes as long as the numbers have no gap): `galaxies[i]`, with `coins[i]` 1 for the purple coin comet.

### `set_object_change`

```c
int32_t set_object_change(int32_t where, const char* object, const char* becomes)
```

Replaces one of the game's objects with another, or removes it (`becomes` null). Where: NEBULA_OBJECTS_*. Call before stages load.

### `set_comet_time_limit`

```c
int32_t set_comet_time_limit(const char* galaxy, int32_t mission, int32_t seconds)
```

A comet mission's time limit in seconds.

### `is_animation_done`

```c
int32_t is_animation_done(NebulaActor* actor)
```

The skeleton animation started by play(NEBULA_PLAY_ANIMATION): 1 when it has reached its end, 0 while playing.

### `animation_frame`

```c
float animation_frame(NebulaActor* actor)
```

Current frame, from 0; 0 without a model.

### `animation_length`

```c
float animation_length(NebulaActor* actor)
```

Last frame; 0 without a model.

### `set_animation_frame`

```c
void set_animation_frame(NebulaActor* actor, float frame)
```

Continues from this frame.

### `set_animation_rate`

```c
void set_animation_rate(NebulaActor* actor, float rate)
```

Playback speed: 1 normal, 2 double, 0 paused, below 0 backwards.

### `stop_effect`

```c
void stop_effect(NebulaActor* actor, const char* name)
```

Stops its effect with this name; null stops all of them.

### `stop_sound`

```c
void stop_sound(NebulaActor* actor, const char* name)
```

Stops its sound with this name.

### `set_state`

```c
void set_state(NebulaActor* actor, int32_t state)
```

For register_object objects: the current step of the object's behaviour, a number of the plugin's choosing (from 0). state_frames counts from the set_state call that started it.

### `state`

```c
int32_t state(NebulaActor* actor)
```

The last value given to set_state; 0 before that, or for objects not from register_object.

### `state_frames`

```c
int32_t state_frames(NebulaActor* actor)
```

Frames since that set_state, from 0.

### `nerve`

```c
const char* nerve(NebulaActor* actor)
```

The game's own objects (and screens, and the states objects delegate to) run one nerve at a time: Kuribo walks in "Kuribo.Walk". Names are "<class>.<nerve>"; docs/modding/NERVES.md lists them all. nerve: the object's current nerve, or null for objects without nerves (register_object objects use set_state).

### `nerve_frames`

```c
int32_t nerve_frames(NebulaActor* actor)
```

Frames spent in the current nerve, from 0.

### `set_nerve`

```c
int32_t set_nerve(NebulaActor* actor, const char* nerve)
```

Switches to another nerve of the same class from the next frame; the game's code for that nerve runs as if the game set it. 0: done; -1: no such nerve in its class.

### `hook_nerve`

```c
int32_t hook_nerve(const char* nerve, NebulaNerveCallback callback, void* user)
```

Only in nebula_plugin_start. Calls `callback` for every object in this nerve: on entry, every frame, and on exit. 0: done; -1: no such nerve.

### `nerve_at`

```c
const char* nerve_at(int32_t index)
```

The name of the index-th nerve, for listing them all. Null: past the last.

### `get_rotation`

```c
void get_rotation(NebulaActor* actor, float* xyz)
```

Rotation about x, y and z in degrees, as levels place it.

### `set_rotation`

```c
void set_rotation(NebulaActor* actor, const float* xyz)
```

Sets the rotation in degrees. A register_object object with a front (set_front) turns by that instead.

### `get_scale`

```c
void get_scale(NebulaActor* actor, float* xyz)
```

Scale on each axis; 1 is normal size.

### `set_scale`

```c
void set_scale(NebulaActor* actor, const float* xyz)
```

Sets the scale on each axis.

### `set_front`

```c
void set_front(NebulaActor* actor, const float* xyz)
```

For register_object objects: the facing direction, upright against gravity. Its model then turns by this, not by its rotation.

### `get_front`

```c
void get_front(NebulaActor* actor, float* xyz)
```

The facing direction (from set_front, or from turning to the player); 0, 0, 1 for other objects.

### `turn_to_player`

```c
void turn_to_player(NebulaActor* actor, float degrees)
```

Turns to face the player, by at most `degrees` this frame.

### `set_touchable`

```c
void set_touchable(NebulaActor* actor, int32_t touchable)
```

For register_object objects: whether anything can touch it, including touches that hurt the player.

### `hit_player`

```c
int32_t hit_player(NebulaActor* actor, NebulaHit how)
```

Hurts the player through its body, as the game's enemies do (NEBULA_HIT_*). 1 if the player was hit.

### `placement_number`

```c
int32_t placement_number(NebulaActor* actor, const char* column, double* value)
```

For register_object objects: a column of its row in the level, by the name level editors show: Obj_arg0 to Obj_arg7 (-1 when unset), CommonPath_ID, ShapeModelNo, CameraSetId, MessageId, GroupId, ClippingGroupId, DemoGroupId, ViewGroupId, CastId, ParentID, the SW_ columns, l_id... placement_number returns 1 and the value for a number column; 0 for a missing or text column, or for an object not placed by the level (spawn). placement_text reads text columns (name), or returns null.

### `placement_text`

```c
const char* placement_text(NebulaActor* actor, const char* column)
```

A text column of its row (its level name, a camera name), or null.

### `has_switch`

```c
int32_t has_switch(NebulaActor* actor, NebulaSwitch which)
```

Its stage switches (NEBULA_SWITCH_*): 1 when the level assigned that switch. The other switch calls do nothing for an unassigned one.

### `is_switch_on`

```c
int32_t is_switch_on(NebulaActor* actor, NebulaSwitch which)
```

1 while that switch is on.

### `set_switch`

```c
void set_switch(NebulaActor* actor, NebulaSwitch which, int32_t on)
```

Turns that switch on or off for every object in the level that listens to it.

### `has_rail`

```c
int32_t has_rail(NebulaActor* actor)
```

Its rail from the level (CommonPath_ID): 1 when it has one. Without one, the rail calls do nothing or return 0.

### `move_on_rail`

```c
void move_on_rail(NebulaActor* actor, float speed)
```

Moves it `speed` along its rail (below 0: backwards) and places it there.

### `rail_at_end`

```c
int32_t rail_at_end(NebulaActor* actor)
```

1 once it reached the end it is moving toward.

### `rail_turn_back`

```c
void rail_turn_back(NebulaActor* actor)
```

Reverses its direction on the rail.

### `rail_coord`

```c
float rail_coord(NebulaActor* actor)
```

Distance along the rail from its start, in level units.

### `set_rail_coord`

```c
void set_rail_coord(NebulaActor* actor, float coord)
```

Places it this far along the rail (the next move_on_rail continues from there).

### `rail_length`

```c
float rail_length(NebulaActor* actor)
```

The rail's total length.

### `raycast`

```c
int32_t raycast(const float* from, const float* to, float* hit)
```

The world. Returns 1 and the hit point (`hit` may be null) when the line from `from` to `to` crosses the map's collision; 0 when it is clear.

### `is_on_ground`

```c
int32_t is_on_ground(NebulaActor* actor)
```

1 while standing on the map: the player, or an object with its own collision.

### `is_touching_wall`

```c
int32_t is_touching_wall(NebulaActor* actor)
```

1 while touching a wall of the map (objects with their own collision).

### `is_in_water`

```c
int32_t is_in_water(const float* xyz)
```

1 if a point is in the stage's water.

### `distance_to_player`

```c
float distance_to_player(NebulaActor* actor)
```

Distance to the player; 0 when there is no player.

### `get_joint_position`

```c
void get_joint_position(NebulaActor* actor, const char* joint, float* xyz)
```

Current position of one of its model's joints; its own position when there is no such joint or model.

### `player_front`

```c
void player_front(float* xyz)
```

The player's facing direction, unit length; 0, 0, 1 when there is no player.

### `set_player_control`

```c
void set_player_control(int32_t on)
```

0 takes the player's controls away (they stand still, as in a cutscene); 1 gives them back.

### `declare_item`

```c
void declare_item(NebulaActor* actor, NebulaItem what, int32_t count)
```

Items an object gives: declare them in the class's `start` (the game prepares them with the stage), then call appear_item when one comes out.

### `appear_item`

```c
void appear_item(NebulaActor* actor, NebulaItem what, const float* xyz, int32_t count)
```

Releases items at a point: `count` coins or star bits at a time, or the power star numbered `count` (below 0: the stage's).

### `start_cutscene`

```c
int32_t start_cutscene(NebulaActor* actor, const char* name)
```

Starts the actor's part in a stage cutscene (by name, from its DemoGroupId). 1: started; 0: another cutscene is playing (try again next frame).

### `end_cutscene`

```c
void end_cutscene(NebulaActor* actor, const char* name)
```

Ends the actor's part in that cutscene; the cutscene ends when all parts have.

### `is_cutscene_playing`

```c
int32_t is_cutscene_playing(void)
```

1 while any cutscene plays, while the stage opens (the opening camera and the player's arrival), and during a star's "You got a Star!" sequence, none of which send NEBULA_MOD_EVENT_CUTSCENE. The player has control when it is 0, which is the best time to show a message.

### `shake_camera_by`

```c
void shake_camera_by(NebulaShake how)
```

In a stage: shakes the camera with this strength (NEBULA_SHAKE_*).

### `start_camera`

```c
void start_camera(const char* name, int32_t frames)
```

In a stage: switches the view to one of the stage's event cameras (by its name in the camera table) over `frames` frames.

### `end_camera`

```c
void end_camera(const char* name, int32_t frames)
```

Hands the view back from that camera over `frames` frames.

### `fade_out`

```c
void fade_out(int32_t frames)
```

In a stage: fades the screen to black over `frames` frames.

### `fade_in`

```c
void fade_in(int32_t frames)
```

In a stage: fades the screen back in over `frames` frames.

### `change_stage`

```c
void change_stage(const char* galaxy, int32_t mission)
```

Sends the player to a galaxy's mission, as a launch star does (the stage ends and the new one loads).

### `back_to_dome`

```c
void back_to_dome(void)
```

In a stage: sends the player back to the dome, as leaving a galaxy does.

### `is_event_flag_on`

```c
int32_t is_event_flag_on(const char* name)
```

1 if the save has this event flag on (a progression name, or one added with set_event_flag). "SpinAndStarPointer" is the spin and Star Pointer the Gateway's Luma gives.

### `set_event_flag_on`

```c
void set_event_flag_on(const char* name)
```

Turns the flag on in the save (written at the next save). "SpinAndStarPointer" also grants the spin and the Star Pointer at once, in the current stage too.

### `has_star`

```c
int32_t has_star(const char* galaxy, int32_t star)
```

1 if the save has that galaxy's star with that number.

### `show_message`

```c
void show_message(const char* utf8_or_id, int32_t centre)
```

In a stage: shows the game's information box (at the top, or in the middle when `centre` is 1) with a message id (the game's or add_text's) or with this text (message tags allowed; lines wrap between words). It stays until hide_message. Requested while the stage opens, it appears once the player has control.

### `hide_message`

```c
void hide_message(void)
```

Hides the information box.

### `store_get`

```c
int32_t store_get(const NebulaPlugin* self, const char* key, int32_t missing)
```

A number saved for this mod across sessions (in the mods folder's .plugin-data, written at once). Keys can't contain '=' or line breaks. `missing` is returned for a key never set.

### `store_set`

```c
void store_set(const NebulaPlugin* self, const char* key, int32_t value)
```

Saves `value` under `key`.

### `set_stage_rules`

```c
int32_t set_stage_rules(const char* stage, int32_t scenario, const char* acts_as, int32_t own_rules)
```

Only in nebula_plugin_start. Makes a stage follow another stage's rules: the game's stage-specific rules (a Bowser arena's star, a hub's counters, ocean swimming) treat it as `acts_as`. own_rules 0: its own rules no longer apply. `scenario`: one mission, 0 for all. 0: done.

### `set_player_constant`

```c
int32_t set_player_constant(int32_t character, const char* name, int32_t index, float value)
```

Sets a value in the player's constants table by name (mJumpHeight, mFireModeTime). `character`: 0 Mario, 1 Luigi. `index`: position in a list, else 0. Any time. 0: done; -1: no such value.

### `player_constant`

```c
float player_constant(int32_t character, const char* name, int32_t index)
```

The current value; 0 when there is no such value.

### `set_object_class`

```c
int32_t set_object_class(const char* name, const char* like)
```

Only in nebula_plugin_start. A new object name levels can place, built like the game's object `like`, with its files from ObjectData/<name>.arc. No code needed. 0: done.

### `add_sound_effect`

```c
int32_t add_sound_effect(const char* name, const char* file)
```

Only in nebula_plugin_start. A sound effect played from a file (AudioRes/Stream/Boom.wav, .ogg, .ast): a new name, or one of the game's (SE_OJ_COIN_GET) to replace it everywhere, whether played by name or by number (player voices, SE_PV_*). "<name>@luigi" replaces it only while playing as Luigi. 0: done.

### `find_actor`

```c
NebulaActor* find_actor(const char* name, NebulaActor* after)
```

Finds an actor by its level name. Pass `after` null to start, or a found actor to continue from it. `name` null matches every actor.

### `actors_near`

```c
int32_t actors_near(const float* xyz, float radius, NebulaActor* * out, int32_t max)
```

Living actors within `radius`. Returns how many; up to `max` are written to `out`.

### `attack_actor`

```c
int32_t attack_actor(NebulaActor* target, NebulaActor* by, NebulaAttack how)
```

Attacks any actor as the player's attack `how` would. 1 if it hit. `by`: the attacker, null for the player.

### `actor_data`

```c
void* actor_data(NebulaActor* actor)
```

The plugin's own pointer stored on a register_object object (set_actor_data), else null.

### `set_actor_data`

```c
void set_actor_data(NebulaActor* actor, void* data)
```

Stores a pointer of the plugin's on the object; the plugin frees it (in the class's `kill` or at STAGE_END).

### `actor_placed_name`

```c
const char* actor_placed_name(NebulaActor* actor)
```

Its level name.

### `stop_scene`

```c
void stop_scene(int32_t frames)
```

Freezes the whole scene for `frames` frames (0 resumes at once): a hit-stop.

### `gravity_at`

```c
void gravity_at(const float* xyz, float* out)
```

The down direction at a point, unit length, from the stage's gravity.

### `is_in_area`

```c
int32_t is_in_area(const char* area, const float* xyz)
```

1 if a point is inside one of the stage's areas of that kind (WaterArea, DeathArea, ChangeBgmCube...).

### `camera_position`

```c
void camera_position(float* xyz)
```

The camera's position.

### `camera_target`

```c
void camera_target(float* xyz)
```

A point 1000 units in front of the camera.

### `screen_position`

```c
int32_t screen_position(const float* xyz, float* xy)
```

Where a point appears on the screen; returns 0 if it is behind the camera.

### `play_system_sound`

```c
void play_system_sound(const char* name)
```

Plays a sound not tied to a place.

### `pointer`

```c
int32_t pointer(float* xy)
```

The pointer's position in screen coordinates. 0: off the screen.

### `event_value`

```c
int32_t event_value(const char* name)
```

Save data: an event value (a named number kept in the save).

### `set_event_value`

```c
void set_event_value(const char* name, int32_t value)
```

Sets it, clamped to 0..65535 (the save keeps 16 bits).

### `set_star`

```c
void set_star(const char* galaxy, int32_t star, int32_t have)
```

Gives or takes away a star.

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

Reads a tunable by its key, as set_tunable takes it ("GameRules.cPlayerHealth"): the current value and the game's default. 0: done; -1: no such tunable. Added in version 2 in place of setting and setting_by_number.

### `layout_open`

```c
NebulaLayout* layout_open(const char* name)
```

Opens a screen layout (LayoutData/<name>.arc, the game's or a mod's) for the current stage; it is removed with the stage. Null if there is no such archive.

### `layout_show`

```c
void layout_show(NebulaLayout* layout, int32_t show)
```

Shows (1) or hides (0) it.

### `layout_set_text`

```c
void layout_set_text(NebulaLayout* layout, const char* pane, const char* utf8)
```

Sets a pane's text (message tags allowed).

### `layout_set_number`

```c
void layout_set_number(NebulaLayout* layout, const char* pane, int32_t number)
```

Sets a pane's text to a number in the game's digits.

### `layout_show_pane`

```c
void layout_show_pane(NebulaLayout* layout, const char* pane, int32_t show)
```

Shows (1) or hides (0) a pane and the panes inside it.

### `layout_move_pane`

```c
void layout_move_pane(NebulaLayout* layout, const char* pane, float x, float y)
```

Moves a pane to x, y on the screen (screen_position's coordinates); it stays there.

### `layout_play`

```c
void layout_play(NebulaLayout* layout, const char* animation)
```

Plays one of its animations by name.

### `layout_is_playing`

```c
int32_t layout_is_playing(NebulaLayout* layout)
```

1 while its animation plays.

### `layout_set_alpha`

```c
void layout_set_alpha(NebulaLayout* layout, float alpha)
```

Opacity: 1 as designed, 0 invisible.

### `store_get_text`

```c
int32_t store_get_text(const NebulaPlugin* self, const char* key, char* out, int32_t size)
```

Text saved for this mod across sessions, next to store_get's numbers. Writes up to `size` - 1 bytes and a terminator to `out`; returns how many bytes were read (0 for a key never set).

### `store_set_text`

```c
void store_set_text(const NebulaPlugin* self, const char* key, const char* text)
```

Saves `text` under `key`.

### `open_scenario_select`

```c
void open_scenario_select(const char* galaxy)
```

Opens a galaxy's star select screen, as a launch star does.

### `set_story`

```c
int32_t set_story(const char* key, const char* value)
```

Only in nebula_plugin_start. Changes the game's story: "first_stage" (with "first_mission", a number as text) is where a new file starts instead of the prologue; "hub" is where the player returns after a galaxy instead of the Comet Observatory; "story_scenes" "off" skips the game's story scenes. 0: done.

### `save_game`

```c
void save_game(void)
```

Writes the save, as the game does after a star.

### `set_tunable`

```c
int32_t set_tunable(const char* key, double value)
```

Sets a tunable, keyed "<file stem>.<name>" ("GameRules.cPlayerHealth") or "<file stem>.<name>[<i>]" for a list entry, the same keys as luma.json's "tunables". Any time; most are read when an object starts, so luma.json's are applied before the first stage. Integer types are rounded and clamped. 0: done; -1: no such tunable.

### `tunable_at`

```c
int32_t tunable_at(int32_t index, const char* * file, const char* * name, int32_t* place, int32_t* kind, double* game_value, double* value)
```

The index-th tunable (each list entry counts): its file ("Kuribo.cpp"), name, list position (-1: not a list), type (1 f32, 2 f64, 3 s32, 4 u32, 5 s16, 6 u16, 7 s8, 8 u8, 9 bool), the game's default and the current value. 0: done; -1: past the last.

### `set_gravity_scale`

```c
void set_gravity_scale(float scale)
```

Game rules a plugin can change at any time. Each setting holds until changed and survives stage changes. Gravity strength for the player and every object: 1 normal, 0.5 half, 0 none; below 0 pushes away.

### `gravity_scale`

```c
float gravity_scale(void)
```

The current gravity scale.

### `timer`

```c
int32_t timer(float* seconds_left, float* seconds_limit)
```

The on-screen countdown (a comet's time limit). Returns 1 while one runs, with the seconds left and the total; 0 when none runs.

### `set_timer`

```c
int32_t set_timer(float seconds_left)
```

Sets the seconds left, within the total. 0: done; -1: no countdown is running.

### `pause_timer`

```c
void pause_timer(int32_t paused)
```

1 freezes every countdown, 0 lets them run.

### `set_camera`

```c
void set_camera(const float* position, const float* target, float fov)
```

Places the camera at `position` looking at `target`, with a vertical field of view of `fov` degrees. Null for a point, or 0 for fov, leaves that part to the game; all three left: the game's camera again. A set position is used that frame exactly, with no easing, and passes through walls.

### `camera_fov`

```c
float camera_fov(void)
```

The vertical field of view of the last drawn frame, in degrees.

### `show_hud`

```c
void show_hud(uint32_t parts, int32_t show)
```

Hides (0) or shows (1, as the game normally would) HUD parts (NEBULA_HUD_*).

### `set_game_speed`

```c
void set_game_speed(float speed)
```

Scene speed: 1 normal, 0.5 one step every other frame, 0 frozen (menus and the HUD still run). Between 0 and 1. Set in a STAGE_FRAME callback, it applies from that frame.

### `step_game`

```c
void step_game(int32_t steps)
```

While frozen: runs this many scene steps, one per frame.

### `draw_text`

```c
int32_t draw_text(int32_t line, float x, float y, float size, uint32_t rgba, const char* utf8)
```

Draws a line of text over the game in the game's message font. `line`: slot 0-15. x, y: top-left corner, 0 to 1 across and down the screen. `size`: height as a fraction of the screen height (0.04 matches the HUD numbers). `rgba`: colour, 0xRRGGBBAA. Null or "" removes the line; otherwise it stays, across stages. 0: done; -1: no such slot.

### `add_text_like`

```c
int32_t add_text_like(const char* id, const char* utf8, const char* like_id)
```

Like add_text; a new message also copies the camera, sound, talk and balloon settings of the game's message `like_id` (null or "" behaves like add_text). The text may use the game's message tags in brackets ([wait 30], [color 1], [player]...; see nebula_text_tags.h).

### `add_player_animation`

```c
int32_t add_player_animation(const char* name, const char* file, float rate, int32_t blend_frames)
```

A player animation: `name` plays the bck `file` from the player's animations (ObjectData/MarioAnime.arc: the game's, or one the mod adds there as a .bck or inside a .glb) at `rate` (1: the bck's own speed), blended in over `blend_frames` frames (0: the game's 6). Play it by name (NEBULA_PLAY_PLAYER_ANIMATION); a name the game already uses replaces that animation wherever the game plays it. Call from the plugin's entry, before the first stage. 0: done; -1: bad name or file, or called too late.

### `alloc`

```c
void* alloc(uint64_t bytes)
```

Memory from the game's C library, for plugins built without one (as portable plugins are, so they have no malloc). alloc returns null for 0 bytes or when memory runs out; the memory is aligned for any type. release(null) does nothing. Memory stays valid until released and is never moved.

### `release`

```c
void release(void* memory)
```

Memory from the game's C library, for plugins built without one (as portable plugins are, so they have no malloc). alloc returns null for 0 bytes or when memory runs out; the memory is aligned for any type. release(null) does nothing. Memory stays valid until released and is never moved.

### `format`

```c
int32_t format(char* buffer, uint64_t size, const char* format, ...)
```

printf-style formatting into `buffer` of `size` bytes, always zero-terminated when size is above 0. Returns what snprintf would: the length the full text needs, or below 0 for a bad format.

### `set_camera_near`

```c
void set_camera_near(float distance)
```

The camera's near plane in units: nothing closer is drawn. 0: the game's (100; 10 in its own first-person view). Kept until set again. The distances at which the game hides objects near its camera are tunables: ClippingJudge.cNearClip (500), ClippingJudge.cNearClipSubjective (100) and ObjUtil.cNearClipScale (1).

### `unlock_achievement`

```c
int32_t unlock_achievement(const NebulaPlugin* self, const char* id)
```

Unlocks one of the mod's achievements (luma.json "achievements", by id). Returns 1 when newly unlocked, 0 when already unlocked or the mod has no such id. Its pop-up shows and the board lists it under Mods.

### `game_camera`

```c
int32_t game_camera(float* position, float* target)
```

The game's own camera this frame, before set_camera changes it: its position and the point it looks at (either may be null). In a cutscene or a conversation this is the shot the game frames. 1 in a stage, 0 otherwise.

### `talking_actor`

```c
NebulaActor* talking_actor(void)
```

The character in conversation with the player now, or null when there is none.

### `place_talk_box`

```c
void place_talk_box(float x, float y, int32_t on)
```

While `on` is 1, the large talk box (the one with the A button, also used by signs) is centred on x, y in screen_position's coordinates, kept inside the screen, and its pointer turns to the speaker. 0 puts it back in its own place. Small balloons already follow their speaker. Reset when the stage ends.

### `show_joint`

```c
void show_joint(NebulaActor* actor, const char* joint, int32_t on)
```

`on` 0 hides one joint of an actor's model and every joint under it, on that actor only; 1 shows them again. The player keeps it in every form, and his face and hands go with the joint they hang from ("Head" takes his head, cap and face). Joint names as in get_joint_position. A new stage starts with all shown.

### `joint_name`

```c
const char* joint_name(NebulaActor* actor, int32_t index)
```

The name of joint `index` of an actor's model (0 is the root), or null past the last joint or without a model. Names are what get_joint_position and show_joint take.

### `get_joint_matrix`

```c
void get_joint_matrix(NebulaActor* actor, const char* joint, float* matrix)
```

A joint's place and turn this frame: 12 floats, the rows of a 3x4 matrix. Columns 0, 1 and 2 are the joint's own x, y and z axes in the world (scaled with the model), column 3 its position. Without the joint or a model: no turn, at the actor's position.

### `give_star`

```c
int32_t give_star(const char* galaxy, int32_t star, int32_t grand)
```

A star of the stage played won, as the player wins it: its result counted and its get demo (the Grand Star's with `grand` 1) once the stage is in play (after its opening camera and starter). `galaxy`: null or the stage played. 0: refused (no galaxy is played, or another one is named); 1: done.

### `give_grand_stars`

```c
int32_t give_grand_stars(int32_t count)
```

The save holds Grand Stars 1 to `count` (1-7), and the stage played starts again, so the Observatory's domes are built with them. 0: refused; 1: done.

### `set_player_character`

```c
int32_t set_player_character(int32_t luigi)
```

The file is played as Luigi (1) or Mario (0) from the next stage. 0: refused; 1: done.

### `list_materials`

```c
int32_t list_materials(const char* model, NebulaMaterialCallback callback, void* user)
```

The materials of the first living actor whose model is `model` (any case), each given to `callback`: how many.

### `water_point`

```c
int32_t water_point(int32_t index, float* xyz)
```

A point inside the stage's `index`-th body of water (its oceans first, then its water areas; the index wraps): how many bodies of water the stage has (0: none, `xyz` untouched).

### `circle_wipe`

```c
int32_t circle_wipe(int32_t open, int32_t frames)
```

The circle wipe closes (`open` 0) or opens (1) over `frames` frames. 0: refused (no stage); 1: done.

### `grant_spin_and_pointer`

```c
int32_t grant_spin_and_pointer(void)
```

The story's grant of the spin and the Star Pointer (the Gateway Luma's), which a new file has not reached. 0: refused (no stage); 1: done.

### `toolkit_find`

```c
int32_t toolkit_find(const char* name)
```

The toolkit: every game function scripts and plugins call (docs/modding/TOOLKIT.md). Its index by name ("MR::startBck"; an overloaded name with its value letters after ':'), or -1.

### `toolkit_call`

```c
int32_t toolkit_call(int32_t index, const NebulaValue* args, uint32_t count, NebulaValue* out, uint32_t out_count)
```

Calls it. `out` gets the result, then each value an in-out parameter gives back; returns how many values were written, or -1 when the call was refused (a missing or stale handle, a wrong class, too few arguments: logged once per reason).

### `actor_value`

```c
NebulaValue actor_value(NebulaActor* actor)
```

The value for an actor the API gave, and back (null for anything but a living object).

### `value_actor`

```c
NebulaActor* value_actor(const NebulaValue* value)
```

The value for an actor the API gave, and back (null for anything but a living object).

### `field_find`

```c
int32_t field_find(const NebulaValue* object, const char* name)
```

A field of an object's class (or a base's) by name ("mVelocity"): its index for field_get and field_set, or -1 (docs/modding/FIELDS.md). Handles, text and pointers are read-only. 1: done; 0: refused.

### `field_get`

```c
int32_t field_get(const NebulaValue* object, int32_t field, uint32_t element, NebulaValue* out)
```

A field of an object's class (or a base's) by name ("mVelocity"): its index for field_get and field_set, or -1 (docs/modding/FIELDS.md). Handles, text and pointers are read-only. 1: done; 0: refused.

### `field_set`

```c
int32_t field_set(const NebulaValue* object, int32_t field, uint32_t element, const NebulaValue* value)
```

A field of an object's class (or a base's) by name ("mVelocity"): its index for field_get and field_set, or -1 (docs/modding/FIELDS.md). Handles, text and pointers are read-only. 1: done; 0: refused.

### `class_name`

```c
const char* class_name(uint32_t id, uint32_t* parent)
```

A class's name and its parent's id, by id (0 is NameObj; null past the last), and whether an object is one.

### `is_a`

```c
int32_t is_a(const NebulaValue* object, const char* class_name)
```

A class's name and its parent's id, by id (0 is NameObj; null past the last), and whether an object is one.

### `luma_start`

```c
void luma_start(const struct NebulaModApi* api)
```

The host's: the table it completed (log, store, format), once, before any mod row. Plugins never call it.

### `add_script_object`

```c
int32_t add_script_object(const NebulaPlugin* mod, const NebulaObjectClass* shape, const char* script)
```

An object a script runs (luma.json "objects"): the mod's /LumaScripts/<mod id>/<script>.lumascript on `shape`, whose callbacks the script's handlers take. 0: added; -1: refused (logged with the reason).

### `add_mod_script`

```c
int32_t add_mod_script(const NebulaPlugin* mod, const char* script)
```

The host's: a mod-wide script (luma.json "scripts"), the mod's /LumaScripts/<mod id>/<script>.lumascript, one instance for the whole run. 0: started; -1: refused (logged with the reason).

## Numbers and names

### `NebulaPlugin`

```c
    uint32_t size;
    const char* id;       /* the mod's id */
    const char* version;  /* the mod's version */
```

### `NebulaModEvent`

```c
    /* Every frame, in every scene including menus. player() is null here. */
    NEBULA_MOD_EVENT_FRAME = 0,
    /* Every frame of a stage, before objects move. */
    NEBULA_MOD_EVENT_STAGE_FRAME,
    /* A stage started. name: the stage. value: the mission. */
    NEBULA_MOD_EVENT_STAGE_START,
    /* The player is about to lose one health. Return 0 to cancel the hit. */
    NEBULA_MOD_EVENT_PLAYER_DAMAGE,
    /* The player is about to die. value: the cause (0 is falling out of the level). Return -1 to cancel: the player
       stays where they are, so move them somewhere safe (set_position) and restore health (set_value) yourself. */
    NEBULA_MOD_EVENT_PLAYER_MISS,
    /* The player's power-up is about to change. value: the new one (NEBULA_POWER_UP_*). The return value is used. */
    NEBULA_MOD_EVENT_POWER_UP,
    /* Coins are about to be added. value: how many. The return value is added instead. */
    NEBULA_MOD_EVENT_COIN,
    /* Star bits are about to be added. value: how many. The return value is added instead. */
    NEBULA_MOD_EVENT_STAR_BIT,
    /* A star was collected. name: the galaxy. value: the star number. */
    NEBULA_MOD_EVENT_STAR_GET,
    /* A cutscene started or ended. name: the cutscene. */
    NEBULA_MOD_EVENT_CUTSCENE_START,
    NEBULA_MOD_EVENT_CUTSCENE_END,
    /* An object appeared or was killed. actor: the object. name: its name. Costs one call per object. */
    NEBULA_MOD_EVENT_ACTOR_APPEAR,
    NEBULA_MOD_EVENT_ACTOR_KILL,
    /* The player started a spin. */
    NEBULA_MOD_EVENT_PLAYER_SPIN,
    /* The player jumped off the ground, or landed. */
    NEBULA_MOD_EVENT_PLAYER_JUMP,
    NEBULA_MOD_EVENT_PLAYER_LAND,
    /* The pause menu opened (value 1) or closed (value 0). */
    NEBULA_MOD_EVENT_PAUSE,
    /* The stage is ending; all its actors and layouts are destroyed after this. name: the stage. */
    NEBULA_MOD_EVENT_STAGE_END,
    /* The game is about to write the save. */
    NEBULA_MOD_EVENT_GAME_SAVE,
    /* A power star appeared in the stage. value: its number. */
    NEBULA_MOD_EVENT_POWER_STAR_APPEAR,
    /* A line of dialogue opened in its balloon. actor: the speaker. name: the message id, as used by luma.json "text"
       and add_text (null for a line without an id). */
    NEBULA_MOD_EVENT_TALK_LINE,
    /* The player answered a yes/no question. value: 1 yes, 0 no. actor: who asked. name: the question's message id. */
    NEBULA_MOD_EVENT_TALK_CHOICE,
    /* A part of a cutscene started. value: the part's index, from 0. name: the part's name in the cutscene's
       time sheet. */
    NEBULA_MOD_EVENT_CUTSCENE_PART,
    /* The camera changed. value: 0 for an area camera (also when the last event camera ends), 1 for an event camera
       (from a cutscene, a conversation or start_camera). name: its id in the stage's camera table. */
    NEBULA_MOD_EVENT_CAMERA_CHANGE,
    /* A sound effect is about to play. value: its number. name: its name. The returned number plays instead (the same,
       another sound's, or -1 for silence). Costs one call per sound. */
    NEBULA_MOD_EVENT_SOUND_START,
    /* An object started an effect. actor: the object. name: the effect's name in the object's effect table. Costs one
       call per effect. */
    NEBULA_MOD_EVENT_EFFECT_START,
    /* A screen layout (a counter, a menu, a message box) was shown (value 1) or removed (value 0). name: its name in
       the game's code. */
    NEBULA_MOD_EVENT_SCREEN,
    /* An object received a message from another: an attack, a stomp, a push, a grab (numbers in
       docs/modding/EVENTS.md). actor: the receiver. name: its name. value: the message. Return -1 to cancel it, as if
       it was never sent. Costs one call per message. */
    NEBULA_MOD_EVENT_ACTOR_MESSAGE,
    /* The player started an action: swimming, hanging, sliding, falling (numbers in docs/modding/EVENTS.md).
       value: the action. */
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
    /* 0 or 1: what the player is doing. */
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

### `NebulaKind`

```c
    NEBULA_KIND_NONE = 0,
    NEBULA_KIND_BOOL,
    NEBULA_KIND_INT,
    NEBULA_KIND_FLOAT,
    NEBULA_KIND_VEC3,
    NEBULA_KIND_MATRIX, /* a handle to 12 floats (3 rows of 4) */
    NEBULA_KIND_TEXT,   /* UTF-8, owned by whoever made the value; valid for the call */
    NEBULA_KIND_HANDLE, /* one of the game's things: an object, a sensor, a triangle... (NebulaValue.type) */
    NEBULA_KIND_VEC4,
    NEBULA_KIND_VEC2
```

### `NebulaValue`

```c
    uint32_t kind; /* NebulaKind */
    uint32_t type; /* handles: NEBULA_HANDLE_* in the low 16 bits; for objects, the class id in the upper 16 */
    union {
        int64_t i;  /* bool and int */
        double f;   /* float */
        float v[4]; /* vec2, vec3, vec4 */
        const char* s;
        struct {
            void* p;
            uint32_t generation; /* the scene it was made in; a later scene reads it as none */
        } h;
    } u;
```

### `NebulaHandleType`

```c
    NEBULA_HANDLE_OBJECT = 1, /* an actor, area, layout or other scene object */
    NEBULA_HANDLE_SENSOR,
    NEBULA_HANDLE_TRIANGLE,
    NEBULA_HANDLE_COLLISION,
    NEBULA_HANDLE_JOINT,
    NEBULA_HANDLE_PLACEMENT,
    NEBULA_HANDLE_MATRIX,
    NEBULA_HANDLE_TALK,
    NEBULA_HANDLE_CAMERA_INFO
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
    NEBULA_PLAY_ANIMATION = 0, /* an action: its animations, effects and sounds together */
    NEBULA_PLAY_EFFECT,
    NEBULA_PLAY_SOUND,
    NEBULA_PLAY_PLAYER_ANIMATION, /* on the player, by animation name */
    /* One part of a model's animation by file name; the other parts keep playing. */
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
    const char* archive; /* model and animations: ObjectData/<archive>.arc. Null: invisible */
    float radius;        /* collision radius for touching */
    void* user;
    void (*start)(NebulaActor* actor, void* user);
    void (*update)(NebulaActor* actor, void* user);
    void (*touch_player)(NebulaActor* actor, void* user);
    /* The player attacked it. Return 1 if the attack hit. */
    int32_t (*attacked)(NebulaActor* actor, void* user);
    /* Like `attacked`, with the kind of attack (NEBULA_ATTACK_*). Called instead of `attacked` when set. */
    int32_t (*attacked_by)(NebulaActor* actor, int32_t how, void* user);
    /* The fields below may be missing from an older plugin's class; `size` tells. */
    int32_t kind;         /* NEBULA_OBJECT_*: what it collides with and how it is drawn */
    int32_t collision;    /* 1: the archive's collision model is solid and moves with the object */
    float shadow_radius;  /* round shadow underneath; 0: none */
    float clip_radius;    /* not updated or drawn when this far off screen; 0: always */
    void (*appeared)(NebulaActor* actor, void* user);
    void (*killed)(NebulaActor* actor, void* user);
    /* Any message another object sends it (an attack, a push, a grab: docs/modding/EVENTS.md lists the numbers), from
       `from` (null when it has no sender). Return 1 to take it; 0 leaves it to the game's own handling (attacked_by for
       the player's attacks). */
    int32_t (*message)(NebulaActor* actor, int32_t message, NebulaActor* from, void* user);
```

### `NebulaObjectKind`

```c
    NEBULA_OBJECT_ENEMY = 0,
    NEBULA_OBJECT_MAP,      /* a platform or wall the player can stand on */
    NEBULA_OBJECT_NPC,
    NEBULA_OBJECT_RIDE,
    NEBULA_OBJECT_DECORATION  /* nothing collides with it */
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
    NEBULA_ITEM_POWER_STAR /* count: the star's number in the galaxy; -1: the next one */
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
    const char* map_pane;           /* its icon's pane in the map layout (MapGrandGalaxy). Null: not on the map */
    const char* open_condition[3];  /* event flags that open it, or null */
    int32_t stars_to_open;          /* power stars needed to open it */
    int32_t dome;                   /* its dome: 1 to 6, 0 for none */
    int32_t has_map_position;       /* 1: the game creates the icon's pane at map_position */
    float map_position[2];          /* right and up from the middle of the dome's page (or the grand map with no dome), on a 608 x 456 page */
```

### `NebulaPowerUpClass`

```c
    uint32_t size;
    const char* name;
    int32_t frames;       /* how long it lasts, at 60 per second. 0: until something ends it */
    const char* music;    /* music while it lasts: a BGM_ name, or null */
    int32_t color_frame;  /* the player's colours: a frame of Mario.arc's ColorChange (0 normal, 1 Flying, 2 Fire). 0: those of
                             looks_like. A mod's Mario.arc can add more frames. */
    void* user;
    void (*start)(void* user);
    void (*update)(void* user); /* every frame while it lasts */
    void (*end)(void* user);
    /* The fields below may be missing from an older plugin's class; `size` tells. */
    const char* model;    /* a whole-body model, ObjectData/<model>.arc, on the player's skeleton. Null: the normal model */
    int32_t looks_like;   /* without a model: look like this power-up (NEBULA_POWER_UP_*) */
```

### `NebulaNervePhase`

```c
    NEBULA_NERVE_ENTER = 0, /* frames is 0 */
    NEBULA_NERVE_FRAME,     /* before the game's code for this frame; return 0 to skip the game's code */
    NEBULA_NERVE_LEAVE      /* another nerve was set; the game's exit code for this one has run */
```

### `NebulaMaterialCoord`

```c
    /* How the coordinate is made: the game's TexGen type, its source and its matrix slot. */
    uint32_t type, source, matrix;
    /* Its matrix's kind and projection; 255 for both when it has no matrix. */
    uint32_t matrix_kind, projection;
    /* 1 when a stage of the material reads it, 0 when none does. */
    uint32_t used;
```

### `NebulaMaterialInfo`

```c
    /* sizeof(NebulaMaterialInfo): fields are only added at the end. */
    uint32_t size;
    /* Its place in the model and its name. */
    int32_t index;
    const char* name;
    /* Its texture coordinates: coord_count of them at coords. */
    int32_t coord_count;
    const NebulaMaterialCoord* coords;
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
    NEBULA_OBJECTS_PLACED = 0,       /* anything the game creates by name: a level's objects, areas */
    NEBULA_OBJECTS_DISPLAY_MODEL = 1, /* the still models shown before an item is collected */
    NEBULA_OBJECTS_TREASURE_BOX = 2, /* what a treasure box holds */
    NEBULA_OBJECTS_IN_COUNT
```

### `NebulaButton`

```c
    NEBULA_BUTTON_JUMP = 1u << 0,
    NEBULA_BUTTON_SPIN = 1u << 1, /* held while the spin lasts, pressed on the frame it starts */
    NEBULA_BUTTON_CROUCH = 1u << 2,
    NEBULA_BUTTON_CAMERA = 1u << 3,
    NEBULA_BUTTON_FIRE = 1u << 4, /* the star bit shot */
    NEBULA_BUTTON_DPAD_UP = 1u << 5,
    NEBULA_BUTTON_DPAD_DOWN = 1u << 6,
    NEBULA_BUTTON_DPAD_LEFT = 1u << 7,
    NEBULA_BUTTON_DPAD_RIGHT = 1u << 8,
    NEBULA_BUTTON_PAUSE = 1u << 9,
    NEBULA_BUTTON_MAP = 1u << 10
```
