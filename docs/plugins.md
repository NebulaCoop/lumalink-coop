# Plugins

A plugin is a mod's own compiled code. It is a native library: a `.dll` on Windows, a `.so` on Linux and Android, a `.dylib` on a Mac. The game loads it before its first line runs and calls it directly, with nothing in between.

Most mods don't need one. Levels, models, textures, music and text are files. Write a plugin when the mod has to change what the game does.

**A plugin is not sandboxed.** It can do anything the game can do on the player's device. Nebula tells players before a mod's code first runs. Only ship code you wrote or can vouch for.

## What you can do

| Level | What it gives you | Survives Nebula updates |
|---|---|---|
| Stable interface | `nebula_mod_api.h`: events, the player, objects, new object kinds | Yes |
| The game's headers | Subclass `LiveActor`, call any `MR::` function, read any field | Until those headers change |
| Function hooks | Run your code in place of any game function, and call the original | As long as the function exists |

Start with the stable interface. Go further only for what it can't do.

## The smallest plugin

```c
/* One-health: any hit is the last one. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    (void)user;
    if (game->value(NEBULA_VALUE_LIFE) > 1) {
        game->set_value(NEBULA_SET_LIFE, 1);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    (void)self;
    game = api;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
```

Build it with any C compiler that targets the device. On Windows, with clang:

```console
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -I <nebula>/native/include -o plugins/one_health.dll one_health.c
```

List it in `luma.json`:

```json
"plugins": [
  { "api": 2, "binaries": { "windows-x64": "plugins/one_health.dll" } }
]
```

Then `lumalink pack` checks that each library is really for the device it's listed under.

## luma.json

| Key | Needed | Meaning |
|---|---|---|
| `api` | yes | `NEBULA_MOD_API_VERSION` when you built it. |
| `binaries` | yes | A library for each device: `windows-x64`, `windows-arm64`, `linux-x64`, `linux-arm64`, `macos-x64`, `macos-arm64`, `android-arm64`, `android-x64`. |
| `when` | no | The same conditions as a layer. A plugin that only runs on one option, for example. |

A player on a device with no build sees THIS MOD HAS NO VERSION FOR THIS DEVICE.

A plugin reads the player's option with `game->option(self, "option-id")`. The answer is the choice's id.

## Events

`subscribe(event, callback, user)`, only in `nebula_plugin_start`. A callback gets the event's data and returns what to use: return `data->value` to change nothing.

| Event | When | `value` | What you return does |
|---|---|---|---|
| `FRAME` | Every frame, menus too | | |
| `STAGE_FRAME` | Every frame of a stage, before objects move | | |
| `STAGE_START` | A stage starts; `name` is the stage | Its mission | |
| `PLAYER_DAMAGE` | About to lose one health | 1 | 0 refuses the hit |
| `PLAYER_MISS` | About to die | How | |
| `POWER_UP` | Power-up about to change | The new one | Used instead |
| `COIN` | Coins about to be added | How many | Added instead |
| `STAR_BIT` | Star bits about to be added | How many | Added instead |
| `STAR_GET` | A star is won; `name` is the galaxy | Its number | |
| `CUTSCENE_START`, `CUTSCENE_END` | `name` is the cutscene | | |
| `ACTOR_APPEAR`, `ACTOR_KILL` | Any object; `actor` and `name` say which | | |

An event nobody subscribed to costs the game one load and one branch.

Objects made while a stage loads are reported on the loading thread. Everything else is on the game's thread.

`player()` is null outside a stage's frame. Actors are valid until the stage ends: keep none across `STAGE_START`.

## New objects

```c
static void update(NebulaActor* actor, void* user) { /* every frame */ }

static NebulaObjectClass spinner = {
    sizeof(NebulaObjectClass), "MySpinner", "MySpinner", 100.0f, 0, 0, update, 0, 0
};
game->register_object(&spinner);
```

A level then places `MySpinner` like any object, and `ObjectData/MySpinner.arc` loads with the stage. A name the game already uses takes that object's place.

`spawn(name, xyz)` makes any object during play, the mod's or the game's, on any stage: its files load if the stage hasn't them. Every call is in [REFERENCE.md](REFERENCE.md).

## Galaxies, hungry Lumas and messages

Most mods say these in `luma.json`, with no code: see [format.md](format.md). A plugin can too, in `nebula_plugin_start`, for what it works out when it runs:

```c
NebulaGalaxy galaxy = { sizeof(NebulaGalaxy), "MyGalaxy", "MyGalaxyPane", { 0, 0, 0 }, 0, 1 };
game->add_galaxy(&galaxy);
game->add_hungry_luma("MyGalaxy", 7, 400);
game->add_text("GalaxyName_MyGalaxy", "My Galaxy");
game->add_music("MyBossTheme", "AudioRes/Stream/MyBossTheme.wav");
game->set_stage_music("MyGalaxy", 0, "MyBossTheme");
```

## With the game's headers

Include `nebula_game_sdk.h` first and use `NEBULA_GAME_PLUGIN_ENTRY`. You build with the flags the game is built with, and link against `smg_game.lib`.

```cpp
class Pulse : public LiveActor { /* written as the game's own objects are */ };

void* createPulse(const char* name) { return new Pulse(name); }

NEBULA_GAME_PLUGIN_ENTRY(api, self) {
    api->register_creator("Pulse", createPulse, nullptr);
}
```

The plugin names the exact headers it was built from. On a Nebula built from other headers it isn't started, and the log says it was built for another version of the game. Rebuild it for each Nebula update that changes them.

## Function hooks

```cpp
void (*original)(MarioActor*, u16);

void myDecLife(MarioActor* self, u16 amount) {
    original(self, amount);
}

void* target = api->find_function("?decLife@MarioActor@@QEAAXG@Z");
api->hook(target, (void*)myDecLife, (void**)&original);
```

- Only in `nebula_plugin_start`.
- On Windows x64, and on Linux and Mac on x64 and arm64. A function whose first instructions can't be moved answers -1, and so does a device that won't allow it.
- A symbol's name is the compiler's: `?decLife@MarioActor@@QEAAXG@Z` on Windows, `_ZN10MarioActor7decLifeEt` on Linux and Mac.
- Several plugins can hook one function. The last one started runs first, and each one's `original` calls the one before.
- A function the compiler put inside its callers can't be reached this way. If your hook doesn't fire, use an event, or ask for one.
- Identical functions can be merged by the linker, so a hook on one can catch another.

## When something goes wrong

A plugin that can't start is left out, and the game's log says why:

| In the log | Why |
|---|---|
| a library it needs isn't there | It depends on a `.dll` you didn't include. |
| it was built for another version of the game | A game function it calls is gone, or the headers changed. |
| it is not for this kind of computer | The wrong build for the device. |
| it is not a Nebula plugin | It doesn't export `nebula_plugin_start`. |
| it needs a newer Nebula | It was built against a later interface. |

A plugin runs inside the game, so a crash in one takes the game down. The crash report names the plugin.

## Music and movies

```c
game->add_music("MyBossTheme", "AudioRes/Stream/MyBossTheme.ogg");   /* in nebula_plugin_start */
game->set_stage_music("MyGalaxy", 0, "MyBossTheme");                  /* 0: every mission without its own */
game->set_area_music("MyGalaxy", 1, "MyBossTheme");                   /* what a ChangeBgmCube with Obj_arg1 1 starts */

game->play_music("MyBossTheme");                                      /* in a stage */
game->stop_music(60);

game->play_movie("MovieData/MyMovie.thp");                            /* in a stage, over the game */
game->is_movie_playing();
game->stop_movie();
```

A wrong name does nothing, and the call says so where it returns a value.

## Cutscenes

`NEBULA_MOD_EVENT_CUTSCENE_START` and `NEBULA_MOD_EVENT_CUTSCENE_END` give each cutscene's name. The game's own names are in Shift-JIS. A mod's are what it placed. A cutscene itself is data: see [guide.md](guide.md).

## The game's progression

```c
game->set_event_flag("ExOpen1CocoonExGalaxy", 3, 1, 0, "SandClockGalaxy", NULL);

const char* const galaxies[] = { "BattleShipGalaxy", "OceanRingGalaxy" };
const int32_t coins[] = { 1, 0 };
game->set_comet_schedule(2, galaxies, coins, 2);
```

Both only in `nebula_plugin_start`, and both can be data instead: `event_flags` and `comet_schedule` in [format.md](format.md).

## Building on Linux and Mac

```console
# the stable interface: builds anywhere
cc -O2 -shared -fPIC -fvisibility=hidden -o plugins/my.so source/plugin.c

# with the game's headers on a Mac, whose linker wants to know where the game's functions will come from
c++ -std=gnu++17 -O2 -shared -fPIC -undefined dynamic_lookup -o plugins/my.dylib source/plugin.cpp
```

On Linux, a library's undefined functions are found when it's loaded, in the game. Name each build in `binaries`: `linux-x64`, `linux-arm64`, `macos-x64`, `macos-arm64`.
