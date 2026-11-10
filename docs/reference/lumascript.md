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
on event gumbo_paid(value) { }   // another script's `emit gumbo_paid value`
```

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
| `registry("form/Gold Flower")` | the row's index in the game's registry (resolved at load) |

## The game's functions (toolkit)

Any row of the toolkit is called by name: `MR::startBck(self, "Run", "")`, `MR::getPlayerPos()`. LumaLink's SDK holds
the list (`crates/lumalink-core/sdk/toolkit.tsv`); Nebula's `docs/modding/TOOLKIT.md` explains every row. A name with
overloads picks by argument count, or by its letters: `MR::isBindedGround:hh(self, other)`.

Letters: `a` none, `b` bool, `c` int, `d` float, `e` vec3, `g` text, `h` object. An upper-case letter is a value the
function also gives back: `let ok, out = f(self, none)`.

## Fields

`self.mPosition`, `self.mVelocity`, `obj.mScale`: an object's own fields by the game's names, read and set. Nested
members one level down: `self.mFlag.mIsDead`. Arrays: `self.mArray[i]` (read). A vector's components are read only:
to change one, set the whole vector (`self.mPosition = vec(p.x, 0.0, p.z)`). Nebula's `docs/modding/FIELDS.md` lists
every class's fields.

## Limits

255 values live at once per function. A script that runs too many steps in one frame stops there for that frame (the
log names it once). A script the game would refuse never leaves `lumalink pack`: the compiler checks its own output
the way the game's loader does.
