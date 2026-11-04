# Your first plugin

A plugin is a native library that the game loads and calls directly. It isn't sandboxed: it can do what the game can do. It's fast, and it's the way to behaviour no table can describe. Players are asked before it first runs.

## Start one

```console
lumalink new "My Mod" --plugin
cd "My Mod"
```

You get `source/plugin.c`, `build.cmd` and `plugins/`. You need [clang](https://releases.llvm.org/) and nothing else: a plugin using only this interface needs no C library and no game files.

## The smallest one

```c
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

/* Runs once, before the game starts. */
NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->log(me, "hello");
}
```

```console
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -I sdk -o plugins/my.dll source/plugin.c
cc -O2 -shared -fPIC -fvisibility=hidden -I sdk -o plugins/my.so source/plugin.c
```

```json
"plugins": [{ "api": 1, "binaries": { "windows-x64": "plugins/my.dll", "linux-x64": "plugins/my.so" } }]
```

Devices: `windows-x64`, `windows-arm64`, `linux-x64`, `linux-arm64`, `macos-x64`, `macos-arm64`, `android-arm64`, `android-x64`. A device with no library for a mod that has plugins says THIS MOD HAS NO VERSION FOR THIS DEVICE. A plugin's row takes a `when` as a layer does.

`game->log` writes to the game's log, with your mod's name on the line.

## Events

The game calls you when something happens. An event nobody subscribed to costs nothing.

```c
/* Any hit is the last one. */
static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    (void)user;
    if (game->value(NEBULA_VALUE_LIFE) > 1) {
        game->set_value(NEBULA_SET_LIFE, 1);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
```

Return `data->value` to change nothing. For the events that ask, what you return is used: return 0 from `PLAYER_DAMAGE` to refuse a hit.

| `NEBULA_MOD_EVENT_` | When | `value` | You return |
|---|---|---|---|
| `FRAME` | Every frame, menus too | | |
| `STAGE_FRAME` | Every frame of a stage, before objects move | | |
| `STAGE_START` | A stage starts. `name`: the stage | Its mission | |
| `STAGE_END` | It's ending. Every actor and layout of it goes after this. `name`: the stage | | |
| `PLAYER_DAMAGE` | About to lose health | 1 | 0 refuses the hit |
| `PLAYER_MISS` | About to die | How | |
| `PLAYER_SPIN` | A spin starts | | |
| `PLAYER_JUMP` | Leaves the ground in a jump | | |
| `PLAYER_LAND` | Lands | | |
| `POWER_UP` | The power-up is about to change | The new one | Used instead |
| `COIN` | Coins about to be added | How many | Added instead |
| `STAR_BIT` | Star bits about to be added | How many | Added instead |
| `STAR_GET` | A star is won. `name`: the galaxy | Its number | |
| `POWER_STAR_APPEAR` | A star comes into the stage | Its number | |
| `CUTSCENE_START`, `CUTSCENE_END` | `name`: the cutscene | | |
| `ACTOR_APPEAR`, `ACTOR_KILL` | Any object. `actor`, `name` | | |
| `PAUSE` | The pause menu | 1 open, 0 closed | |
| `GAME_SAVE` | About to write the save | | |

Subscribing happens only in `nebula_plugin_start`, as do `register_object`, `hook` and the row calls (`add_*`, `set_stage_rules`, `set_object_class`, `set_story` and so on).

## The player

```c
NebulaActor* player = game->player();   /* null between stages and on the file select screen */
float at[3];
game->get_position(player, at);
game->value(NEBULA_VALUE_COINS);
game->set_value(NEBULA_SET_POWER_UP, NEBULA_POWER_UP_FIRE);   /* any power-up, anywhere */
game->jump_player(velocity);
game->set_player_control(0);
if (game->buttons_pressed() & NEBULA_BUTTON_JUMP) { ... }
game->set_player_constant(0, "mJumpHeight", 0, 40.0f);        /* any number, at any time */
```

Buttons, whatever device they're on: `NEBULA_BUTTON_JUMP`, `SPIN`, `CROUCH`, `CAMERA`, `FIRE`, `DPAD_UP`, `DPAD_DOWN`, `DPAD_LEFT`, `DPAD_RIGHT`, `PAUSE`, `MAP`. `stick(&x, &y)` and `pointer(xy)` read the rest.

## Things in the world

```c
NebulaActor* coin = game->spawn("Coin", at);                 /* any object, on any stage */
for (NebulaActor* a = game->find_actor("Kuribo", 0); a; a = game->find_actor("Kuribo", a)) {
    game->attack_actor(a, 0, NEBULA_ATTACK_STOMP);           /* every Goomba */
}
NebulaActor* near[32];
int count = game->actors_near(at, 1000.0f, near, 32);
game->raycast(from, to, hit);
game->gravity_at(at, down);
game->is_in_area("WaterCube", at);
```

`spawn` loads what it needs. An object that can't exist away from a level's placement answers null.

## Kept across plays

```c
game->store_set(me, "best", 12);
int best = game->store_get(me, "best", 0);
game->store_set_text(me, "name", "Rosalina");
```

Per mod, in the settings folder, beside the player's saves. Saved at once.

## Everything else

[The whole interface](../reference/plugin-api.md) has 140 calls: animations, switches, rails, cameras, cutscenes, music, movies, messages, saves. Past that, the [game's own headers and hooks](../reference/hooks.md) reach any function.

## Examples

| Example | What it shows |
|---|---|
| [`one-health`](../../examples/one-health) | Reading and setting the player's state |
| [`coin-kills`](../../examples/coin-kills) | Events |
| [`boss-music`](../../examples/boss-music) | Starting and stopping music |
| [`double-jump`](../../examples/double-jump) | A new move: the buttons, gravity, an animation |
| [`little-boss`](../../examples/little-boss) | A new enemy, whole |
