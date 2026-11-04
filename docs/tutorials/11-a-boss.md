# A whole boss

[`examples/little-boss`](../../examples/little-boss) is complete: model, music, three phases, a real fight. Build it with `build.cmd`, pack it, and fight it near Good Egg's start.

## An object of your own

```c
static NebulaObjectClass boss = {
    sizeof(NebulaObjectClass),
    "LittleBoss",    /* the name levels place */
    "LittleBoss",    /* ObjectData/LittleBoss.arc; null: nothing to see */
    150.0f,          /* what it touches with */
    0,               /* your pointer, passed back */
    start, update, touch_player, attacked, attacked_by,
    NEBULA_OBJECT_ENEMY,  /* what kind */
    0,                    /* 1: its archive's collision model is solid */
    120.0f,               /* a round shadow */
    0.0f,                 /* not run or drawn when off screen past this; 0: always */
    0, 0                  /* appeared, killed */
};

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    game->register_object(&boss);
}
```

A name the game already uses takes that object's place. Each callback can be null.

| Callback | When |
|---|---|
| `start(actor, user)` | Once, as it's made |
| `update(actor, user)` | Every frame |
| `touch_player(actor, user)` | The player is touching it |
| `attacked(actor, user)` | The player attacked it. Return 1 if it took the attack |
| `attacked_by(actor, how, user)` | The same, saying how, in place of `attacked`: `NEBULA_ATTACK_OTHER`, `SPIN`, `STOMP`, `GROUND_POUND`, `STAR_BIT`, `FIRE`, `INVINCIBLE`, `SHELL` |
| `appeared`, `killed` | As it comes and goes |

| Kind | What it is |
|---|---|
| `NEBULA_OBJECT_ENEMY` | Hit by the player, hits the player |
| `NEBULA_OBJECT_MAP` | A platform or a wall. With `collision` 1 the player stands on it |
| `NEBULA_OBJECT_NPC` | A character |
| `NEBULA_OBJECT_RIDE` | Something to ride |
| `NEBULA_OBJECT_DECORATION` | Nothing touches it |

Place it in a level by its name through [`merge/`](03-levels.md), or make it with `spawn`.

## The shape every boss takes

**A number for what it's doing, and how long it's been doing it.** Each step checks one thing and moves on, so nothing happens until the step before has finished.

```c
enum { WAITING, NOTICING, WALKING, WINDING_UP, CHARGING, STUNNED, HURT, DYING, GONE };

static void update(NebulaActor* actor, void* user) {
    const int time = game->state_frames(actor);
    switch (game->state(actor)) {
    case WAITING:
        if (game->player() != 0 && game->distance_to_player(actor) < 2000.0f) {
            game->play_music("LittleBossTheme");
            game->set_state(actor, NOTICING);
        }
        break;
    case WINDING_UP:
        game->turn_to_player(actor, 2.0f);
        if (time >= 40) {
            game->set_state(actor, CHARGING);
        }
        break;
    /* ... */
    }
}
```

To wait for an animation rather than a time, start it when the state begins:

```c
case ROARING:
    if (time == 0) {
        game->play(actor, NEBULA_PLAY_ANIMATION, "Roar");
    }
    if (game->is_animation_done(actor)) {
        game->set_state(actor, WALKING);
    }
    break;
```

## Moving on a planet

There's no fixed up. `get_gravity` says which way is down, `set_front` says the way it faces while standing against gravity, and `turn_to_player` turns it. Its model turns with this, not with its rotation. Move it along its front and it stays on the planet's surface: a straight line on a tangent leaves it.

## Being hit

`attacked_by` decides. Refuse it outside the state where the boss is vulnerable, so the sequence decides what can hurt it.

```c
static int32_t attacked_by(NebulaActor* actor, int32_t how, void* user) {
    if (game->state(actor) != STUNNED || how != NEBULA_ATTACK_STOMP) {
        return 0;
    }
    health--;
    game->set_state(actor, health > 0 ? HURT : DYING);
    return 1;
}
```

And hurting the player: `game->hit_player(actor, NEBULA_HIT_STRONG)`, also `NORMAL`, `FIRE`, `ELECTRIC`, `FREEZE`, `FLIP`. The player's invincibility after a hit applies, so it answers 0 then.

## Dying, and its star

```c
/* in start: the level counts it */
game->declare_item(actor, NEBULA_ITEM_POWER_STAR, -1);

/* as it dies */
game->appear_item(actor, NEBULA_ITEM_COINS, at, 10);
game->appear_item(actor, NEBULA_ITEM_POWER_STAR, at, -1);
game->kill(actor);
```

## The rest of a fight

- **An intro or a phase change**: `start_cutscene` in one state, with [the cutscene's own sheets](14-cutscenes-movies-lumas.md) running the camera, and `end_cutscene` when it's over.
- **A phase's camera**: `start_camera(name, frames)`, an event camera from the level.
- **Parts of the arena**: switches. `set_switch` opens what the level wired to it, `is_switch_on` starts the boss when the player reaches something.
- **Moving between points**: a rail from the level: `move_on_rail`, `rail_at_end`.
- **Attacks that fly**: objects of your own, made with `spawn`.
- **A health bar**: [a screen of your own](13-screens.md).
- **A heavy landing**: `stop_scene(4)` and `shake_camera_by(NEBULA_SHAKE_STRONG)`.

Its files, music and placement go in the same mod. See the example's `luma.json`.
