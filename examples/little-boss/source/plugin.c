/* A small boss: it comes after the player, each touch costs a life point, three hits end it. A level places LittleBoss. */
#include "nebula_mod_api.h"

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int health;
static int rest;
static int said;

/* No C library here. */
static float root(float x) {
    float r = x > 1.0f ? x : 1.0f;
    int i;
    for (i = 0; i < 12; i++) {
        r = 0.5f * (r + x / r);
    }
    return r;
}

static void start(NebulaActor* actor, void* user) {
    (void)actor;
    (void)user;
    health = 3;
    rest = 0;
    said = 0;
}

static void update(NebulaActor* actor, void* user) {
    NebulaActor* player = game->player();
    float at[3], to[3], gap[3], length;
    (void)user;
    if (player == 0) {
        return;
    }
    game->get_position(actor, at);
    game->get_position(player, to);
    gap[0] = to[0] - at[0];
    gap[1] = to[1] - at[1];
    gap[2] = to[2] - at[2];
    length = root(gap[0] * gap[0] + gap[1] * gap[1] + gap[2] * gap[2]);
    /* Close enough to notice: its music starts, and it comes. */
    if (length > 2000.0f) {
        return;
    }
    if (!said) {
        said = 1;
        game->log(me, game->play_music("LittleBossTheme") != 0 ? "the boss has noticed" : "the boss has no music");
    }
    if (length > 1.0f) {
        at[0] += gap[0] / length * 4.0f;
        at[1] += gap[1] / length * 4.0f;
        at[2] += gap[2] / length * 4.0f;
        game->set_position(actor, at);
    }
    if (rest > 0) {
        rest--;
    }
}

static void touch_player(NebulaActor* actor, void* user) {
    (void)actor;
    (void)user;
    if (rest == 0) {
        rest = 120;
        if (game->value(NEBULA_VALUE_LIFE) > 1) {
            game->set_value(NEBULA_SET_LIFE, game->value(NEBULA_VALUE_LIFE) - 1);
        } else {
            game->kill_player();
        }
        game->log(me, "the boss hit the player");
    }
}

static int32_t attacked(NebulaActor* actor, void* user) {
    (void)user;
    if (--health > 0) {
        game->shake_camera();
        game->log(me, "the boss was hit");
        return 1;
    }
    game->kill(actor);
    game->stop_music(60);
    game->set_value(NEBULA_SET_ADD_COINS, 50);
    game->log(me, "the boss is beaten");
    return 1;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    static const NebulaObjectClass boss = {sizeof(NebulaObjectClass), "LittleBoss", "LittleBoss", 150.0f, 0, start, update, touch_player, attacked};
    game = api;
    me = self;
    game->register_object(&boss);
}
