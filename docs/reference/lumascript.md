# LumaScript

A mod's logic as text. `lumalink pack` compiles each script to a `.lumascript` file the game runs the same way on
Windows, Linux, Android, macOS and iOS: no native code, no compiler on the player's side.

```text
// scripts/gumbo.lscript
var home: vec3
var paid: int

on start {
    home = self.mPosition
    goto walk
}

state walk {
    on frame {
        let away = MR::getPlayerPos() - self.mPosition
        if length(away) < 1200.0 {
            self.mPosition = self.mPosition + normalize(away) * 4.0
        }
    }
}

on touch_player {
    if paid < 3 {
        paid = paid + 1
        MR::incCoin(3)
        goto rest
    }
}

state rest {
    on enter { wait 120; goto walk }
}
```

The whole sample is `examples/scripted-goomba`.

## Using a script

A layer's `objects` rows name new objects and their scripts:

```json
"objects": [
  {"name": "Gumbo", "script": "gumbo", "archive": "Kuribo", "kind": "npc",
   "radius": 80, "shadow_radius": 60, "clip_radius": 3000}
]
```

| Key | Meaning |
|---|---|
| `name` | The object's name: what a stage places, or what an `object_changes` row turns a placed object into. |
| `script` | `scripts/<script>.lscript` in the project. Folders are allowed (`enemies/gumbo`). |
| `archive` | The model archive it shows (`ObjectData/<archive>.arc`, the game's or the mod's). |
| `kind` | `enemy`, `map`, `npc`, `ride` or `decoration`: how the game groups it (clipping, sensors). |
| `radius` | Its body's sensor radius. |
| `shadow_radius` | Its drop shadow; 0 for none. |
| `clip_radius` | Past this distance from the camera it is clipped: not drawn, and its `frame` handler does not run. |

A layer's `scripts` list runs scripts for the whole mod, not tied to an object (see Mod-wide scripts):

```json
"scripts": ["rules"]
```

`lumalink check` compiles every script and reports problems with their lines. `lumalink inspect file.lscript` prints
the compiled steps.

## Values

| Kind | Written | Notes |
|---|---|---|
| `int` | `3`, `-12` | 64-bit; overflow wraps. |
| `float` | `2.5`, `1.0` | 64-bit. A number with a `.` is a float. |
| `bool` | `true`, `false` | |
| `vec3` | `vec(1.0, 0.0, 0.0)` | Three floats. `.x` `.y` `.z` read a component. |
| `text` | `"Run"` | |
| object | `self`, a toolkit result | One of the game's things: an actor, a sensor. Read as none after its scene ends. |
| none | `none` | Nothing. |

`int` and `float` mix: the result is a float. Two ints give an int (`7 / 2` is `3`). `vec3 * float` and
`vec3 / float` scale; `vec3 + vec3`, `vec3 - vec3` add and subtract. Dividing by zero, or arithmetic on values that do
not mix (text, none), gives none.

## Variables

```text
var speed: float = 2.5   // each object's own
global hits: int         // the mod's, shared by all its objects in the scene
saved best: int          // kept in the save file, per mod
let d = length(a - b)    // inside a handler or function, until the end of its block
let ok, ground = MR::calcGravity(self, none)   // a call that gives several values
```

Kinds: `int`, `float`, `bool`, `text`, `vec3`, `any`. A declared variable starts at its kind's zero (`0`, `0.0`,
`false`, `""`, `(0, 0, 0)`); `any` starts as none. A starting value (`= 2.5`) is set before `on start` runs.

## Handlers

```text
on start { }                 // once, when the object is made
on frame { }                 // every frame the object is not clipped
on touch_player { }          // the player touched it
on attacked(how) { }         // the player attacked it; return true to take the hit
on appear { }                // it appeared (after being hidden)
on kill { }                  // it was killed
on message(number, from) { }     // any message another object sends it; return true to take it
on message ACTMES_PUSH(from) { } // one message, by its name or number
on event gumbo_paid(value) { }   // another script's `emit gumbo_paid value`
on coin(value, name, actor) { }  // one of the game's events (below)
```

A name the game has no handler for is refused when the script compiles.

Messages are the game's way for objects to act on each other: a punch, a stomp, a push, a grab. `from` is the object
that sent it (none when nothing did). Returning `true` takes the message: the game's own handling does not run. Any
other result leaves it to the game (for the player's attacks, `on attacked`). A message's name is one of the game's
`ACTMES_` names; the SDK's `messages.tsv` lists all 183 with their numbers.

A handler outside a state runs in every state. Inside a state it runs only in that state:

```text
state chase {
    on enter { }   // on goto chase
    on frame { }
    on leave { }   // on goto elsewhere
}
```

`goto <state>` switches at the start of the next frame: `leave` of the old state, then `enter` of the new. A `goto`
to the state it is already in does nothing.

## Game events

Any script, an object's or a mod-wide one, answers the game's events by name. Each handler gets three values:
`value`, `name` (text or none) and `actor` (an object or none). Where the event uses a returned number, a handler that
returns a number (or a bool: 1 or 0) changes it, and the next script gets the changed value. Returning nothing keeps
it.

| Event | When | value, name, actor | A returned number |
|---|---|---|---|
| `stage_frame` | every frame of a stage, before objects move | | |
| `stage_start` | a stage started | the mission, the stage | |
| `stage_end` | the stage is ending (its objects go after this) | name: the stage | |
| `player_damage` | the player is about to lose one health | | 0 cancels the hit |
| `player_miss` | the player is about to die | the cause (0: fell out) | -1 cancels: move the player and restore health yourself |
| `power_up` | the player's form is about to change | the new form | the form used |
| `coin` | coins are about to be added | how many | added instead |
| `star_bit` | star bits are about to be added | how many | added instead |
| `star_get` | a star was collected | the star, the galaxy | |
| `power_star_appear` | a power star appeared | its number | |
| `cutscene_start`, `cutscene_end` | a cutscene started, ended | name: the cutscene | |
| `cutscene_part` | a part of a cutscene started | its index, its name | |
| `actor_appear`, `actor_kill` | an object appeared, was killed | its name, the object | |
| `actor_message` | an object received a message | the message, its name, the receiver | -1 cancels it |
| `player_spin`, `player_jump`, `player_land` | the player spun, jumped, landed | | |
| `player_action` | the player started an action (swimming, hanging, sliding) | the action | |
| `pause` | the pause menu opened (1) or closed (0) | 1 or 0 | |
| `game_save` | the game is about to write the save | | |
| `talk_line` | a line of dialogue opened | its message id, the speaker | |
| `talk_choice` | the player answered a yes/no question | 1 yes, 0 no; the question's id; who asked | |
| `camera_change` | the camera changed | 0 area, 1 event camera; its id | |
| `sound_start` | a sound effect is about to play | its number, its name | the sound played instead (-1: silence) |
| `effect_start` | an object started an effect | its name, the object | |
| `screen` | a screen layout was shown (1) or removed (0) | 1 or 0, its name | |

Nebula's `docs/modding/EVENTS.md` says exactly what each passes. The SDK's `events.tsv` lists every name.

## Mod-wide scripts

A script named in a layer's `scripts` runs once for the whole run, from the game's start: `self` is none, its `var`s
last the whole run, and `on frame` runs every frame. It answers the game's events and other scripts' `emit`s, and
calls the toolkit and the plugin API like any script. Rules that are not about one object belong here:

```text
// Every coin counts twice.
on coin(value, name, actor) { return value * 2 }
```

## Class scripts

A row of `object_classes` with a `script` runs that script on every object the stages place by the name: one of the
game's classes (`{ "name": "Kuribo", "script": "angry" }`, every Goomba) or a name the row makes `like` one. The game's
class still runs; the script rides along and can take its place. `self` is the object; each object has its own `var`s.
A name made `like` a class runs the class's scripts first, then its own.

```text
on start { }                   // the object was made and set up by its class
on control { return false }    // every frame before the class moves it; true: the class doesn't move it this frame
on touch(other, sensor) { }    // one of its sensors touches another object's; true: the class's reaction is skipped
on message(number, from) { }   // as for scripted objects; true or false answers in place of the class
on appear { }
on kill { }
```

`on frame`, states and `wait` work as in a scripted object (`frame` runs just before `control`). Several mods may name
one class: each script runs, in the order the mods load; the first `message` answer wins. A plugin does the same from
C with `set_class_hooks` (see the plugin API).

## Statements

| Statement | Meaning |
|---|---|
| `x = expr` | Set a variable. |
| `self.mField = expr` | Set an object's field (see Fields). |
| `if c { } else if c { } else { }` | |
| `while c { }`, `break` | |
| `return [expr]` | |
| `wait n` | In a handler: continue after `n` frames. The object keeps running its other handlers meanwhile. |
| `goto state` | |
| `emit name [expr]` | Runs `on event name` in every script of the scene with the value. |
| `f(a, b)` | A call whose result is not used. |

`;` separates statements on one line.

## Operators

From lowest to highest: `or` (`||`), `and` (`&&`), `==` `!=`, `<` `<=` `>` `>=`, `+` `-`, `*` `/` `%`, then unary `-`
`!` (`not`). `and` and `or` stop early. Numbers compare by value (`1 == 1.0` is true); other values are equal when
they are the same kind and the same value.

## Built-in functions

| Function | Gives |
|---|---|
| `vec(x, y, z)` | vec3 |
| `length(v)`, `normalize(v)`, `dot(a, b)`, `cross(a, b)` | |
| `lerp(a, b, t)` | numbers or vec3 |
| `min(a, b)`, `max(a, b)`, `abs(x)`, `floor(x)`, `sqrt(x)` | |
| `sin(x)`, `cos(x)`, `atan2(y, x)` | radians |
| `random(lo, hi)` | from `lo` up to, not including, `hi`: an int when both are ints, else a float |
| `int(x)`, `float(x)` | conversions |
| `format("{} of {}", a, b)` | text |
| `registry("form/Gold Flower")` | the row's number: a form's id, a message's number, a floor code, or its place in the game's table; -1 for no such row (resolved at load). The game's lists: Nebula's `docs/modding/REGISTRIES.md` |

## Registries

Every kind of thing the game has is a registry, by name: `object`, `form`, `area`, `camera`, `gravity`, `message`,
`floor`, `comet`, `hud`, `character`, and `player_state`, `attack`, `star`, `ride`, `item`, `projectile` as those parts
of the game become moddable. A kind holds the game's own rows and the ones mods add (`add_power_up` adds a `form`,
`register_object` an `object`); a later row of the same name replaces an earlier one, and the log says which mod won.

```
let fire = registry("form/fire")               // 2: compare with a power_up event's value
let kinds = nebula.registry_count("object")    // the game's objects and every mod's
let first = nebula.registry_name("object", 0)
let place = nebula.registry_find("area", "SwitchArea")
let code = nebula.registry_value("floor", "Slip")
let by = nebula.registry_mod("form", nebula.registry_find("form", "Gold Flower"))   // the mod that added it
```

## The game's functions (toolkit)

Any row of the toolkit is called by name: `MR::startBck(self, "Run", "")`, `MR::getPlayerPos()`. LumaLink's SDK holds
the list (`crates/lumalink-core/sdk/toolkit.tsv`); Nebula's `docs/modding/TOOLKIT.md` explains every row. A name with
overloads picks by argument count, or by its letters: `MR::isBindedGround:hh(self, other)`.

Letters: `a` none, `b` bool, `c` int, `d` float, `e` vec3, `g` text, `h` object. An upper-case letter is a value the
function also gives back: `let ok, out = f(self, none)`.

## The plugin API (nebula.*)

The calls plugins make are rows of the toolkit too, named `nebula.<call>`: `nebula.player()`,
`nebula.get_position(actor)`, `nebula.show_message("Hello", 1)`, `nebula.play_music("MyTrack")`. A call that takes a
mod (`log`, `option`, `store_get`, `unlock_achievement`) gets the script's own. A call that writes into a vector or
a number gives it back: `let position = nebula.get_position(self)`; `let found, value = nebula.get_tunable("key")`.
Calls that only work while plugins start (adding music, galaxies, story rows) are luma.json rows instead. The plugin
reference (plugin-api.md) explains every call.

## Fields

`self.mPosition`, `self.mVelocity`, `obj.mScale`: an object's own fields by the game's names, read and set. Nested
members one level down: `self.mFlag.mIsDead`. Arrays: `self.mArray[i]` (read). A vector's components are read only:
to change one, set the whole vector (`self.mPosition = vec(p.x, 0.0, p.z)`). Nebula's `docs/modding/FIELDS.md` lists
every class's fields.

## Limits

255 values live at once per function. A script that runs too many steps in one frame stops there for that frame (the
log names it once). A script the game would refuse never leaves `lumalink pack`: the compiler checks its own output
the way the game's loader does.
