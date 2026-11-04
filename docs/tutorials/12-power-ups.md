# A new power-up

## The game's own, changed

How long each lasts is in [player numbers](../reference/player-numbers.md): `mFireModeTime`, `mIceModeTime`, `mMetalHoldTime`, `mTornadoHoldTime`, `mRacketHoldTime`. Its look is a file: `ObjectData/BeeMario.arc`, `IceMario`, `HopperMario`, `InvincibleMario`, `TeresaMario`, and the Luigi versions.

With any mod on, every power-up is ready on every stage: giving one anywhere just works, from a plugin or from a placed item.

## A new one

```c
static void start(void* user) { game->log(me, "wings on"); }

static void update(void* user) {
    /* every frame it lasts */
    if (game->buttons_pressed() & NEBULA_BUTTON_JUMP) {
        const float up[3] = {0.0f, 30.0f, 0.0f};
        game->jump_player(up);
    }
}

static void end(void* user) { game->log(me, "wings off"); }

static NebulaPowerUpClass wings = {
    sizeof(NebulaPowerUpClass),
    "Wings",
    1800,            /* frames, at 60 a second. 0: until something ends it */
    "BGM_MUTEKI_A",  /* the music while it lasts, or null */
    0,               /* a frame of Mario.arc's ColorChange, 0 for his own */
    0,               /* your pointer */
    start, update, end,
    "WingMario",     /* a whole-body model, ObjectData/WingMario.arc. Null: the game's */
    NEBULA_POWER_UP_NONE  /* with no model: the look of one of the game's */
};

static int wings_number;

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    wings_number = game->add_power_up(&wings);
}
```

Give it with `game->set_value(NEBULA_SET_POWER_UP, wings_number)`, on any stage. Its number comes back from `add_power_up`, starting at 32; the game's are 0 to 9.

## Its look

| You want | Do this |
|---|---|
| Mario in a colour | `color_frame`: a frame of `Mario.arc`'s `ColorChange`. Your `Mario.arc` can add more |
| The look of one of the game's | `looks_like`: `NEBULA_POWER_UP_FIRE`, `ICE`, `BEE`, `SPRING`, `BOO`, `FLYING`, `INVINCIBLE` |
| A body of its own | `model`: an archive with a model on the player's skeleton, joint for joint, in `ObjectData/`. Start from `Mario.arc` or `IceMario.arc`: the game's animations move joints by name |

Up to 58 can have a model of their own, with no limit on the others.

## Its item

An object of your own that gives it:

```c
static void touch_player(NebulaActor* actor, void* user) {
    game->set_value(NEBULA_SET_POWER_UP, wings_number);
    game->kill(actor);
}

static NebulaObjectClass item = { sizeof(NebulaObjectClass), "WingItem", "WingItem", 80.0f, 0, 0, 0, touch_player, 0, 0, NEBULA_OBJECT_MAP };
```

Levels place `WingItem`; treasure boxes and plugins can give it. The player's own moves, such as what spin does, are the game's: for those, [hook](../reference/hooks.md) the function, or read the buttons as this example does.
