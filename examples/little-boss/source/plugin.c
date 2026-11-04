/* A boss of three phases, each a sequence: it notices the player, walks, winds up, charges, and is stunned at the end of a charge. A spin or a stomp
   only hurts it while it's stunned. It gets faster each phase and dies with an explosion, coins and a star bit shower. Every step waits for the last
   to finish: its state and how long it has been in it are the game's to keep. */
#include "nebula_mod_api.h"

static const NebulaModApi* game;
static const NebulaPlugin* me;

enum { WAITING, NOTICING, WALKING, WINDING_UP, CHARGING, STUNNED, HURT, DYING, GONE };

static int health;

static void state_is(NebulaActor* actor, int state, const char* line) {
    game->set_state(actor, state);
    game->log(me, line);
}

static void start(NebulaActor* actor, void* user) {
    (void)user;
    health = 3;
    game->declare_item(actor, NEBULA_ITEM_COINS, 10);
    game->declare_item(actor, NEBULA_ITEM_STAR_BITS, 20);
    game->set_state(actor, WAITING);
    game->log(me, "the boss is here");
}

static float root(float value) {
    float guess = value > 1.0f ? value : 1.0f;
    int i;
    for (i = 0; i < 12; i++) {
        guess = (guess + value / guess) * 0.5f;
    }
    return guess;
}

/* It looks straight at the player: on a small planet a level line would leave the ground. */
static void face_player(NebulaActor* actor) {
    float at[3], to[3], gap[3];
    float length;
    NebulaActor* player = game->player();
    if (player == 0) {
        return;
    }
    game->get_position(actor, at);
    game->get_position(player, to);
    gap[0] = to[0] - at[0];
    gap[1] = to[1] - at[1];
    gap[2] = to[2] - at[2];
    length = root(gap[0] * gap[0] + gap[1] * gap[1] + gap[2] * gap[2]);
    if (length > 1.0f) {
        game->set_front(actor, gap);
    }
}

static void move_ahead(NebulaActor* actor, float speed) {
    float at[3], front[3];
    int i;
    game->get_position(actor, at);
    game->get_front(actor, front);
    for (i = 0; i < 3; i++) {
        at[i] += front[i] * speed;
    }
    game->set_position(actor, at);
}

static void update(NebulaActor* actor, void* user) {
    /* Faster each time it's been hurt. */
    const float pace = 1.0f + (float)(3 - health) * 0.5f;
    const int time = game->state_frames(actor);
    (void)user;
    switch (game->state(actor)) {
    case WAITING:
        if (game->player() != 0 && game->distance_to_player(actor) < 2000.0f) {
            game->play_music("LittleBossTheme");
            state_is(actor, NOTICING, "the boss has noticed");
        }
        break;
    case NOTICING:
        face_player(actor);
        if (time == 0) {
            game->shake_camera_by(NEBULA_SHAKE_NORMAL);
        }
        if (time >= 45) {
            state_is(actor, WALKING, "walking");
        }
        break;
    case WALKING:
        face_player(actor);
        move_ahead(actor, 4.0f * pace);
        if (time >= 120 || game->distance_to_player(actor) < 300.0f) {
            state_is(actor, WINDING_UP, "winding up");
        }
        break;
    case WINDING_UP:
        face_player(actor);
        if (time >= 40) {
            state_is(actor, CHARGING, "charging");
        }
        break;
    case CHARGING:
        move_ahead(actor, 14.0f * pace);
        if (time >= 50) {
            game->shake_camera_by(NEBULA_SHAKE_WEAK);
            state_is(actor, STUNNED, "stunned: hit it now");
        }
        break;
    case STUNNED:
        if (time >= 180) {
            state_is(actor, WALKING, "it got up");
        }
        break;
    case HURT:
        if (time >= 40) {
            state_is(actor, health > 0 ? WALKING : DYING, health > 0 ? "the next phase" : "the boss is dying");
        }
        break;
    case DYING:
        if (time == 0) {
            float at[3];
            game->get_position(actor, at);
            game->stop_music(60);
            game->shake_camera_by(NEBULA_SHAKE_STRONG);
            game->appear_item(actor, NEBULA_ITEM_COINS, at, 10);
            game->appear_item(actor, NEBULA_ITEM_STAR_BITS, at, 20);
            game->store_set(me, "boss_beaten", game->store_get(me, "boss_beaten", 0) + 1);
        }
        if (time >= 60) {
            game->set_touchable(actor, 0);
            state_is(actor, GONE, "the boss is beaten");
            game->kill(actor);
        }
        break;
    default:
        break;
    }
}

static void touch_player(NebulaActor* actor, void* user) {
    const int state = game->state(actor);
    (void)user;
    /* Only moving it hurts. */
    if (state == WALKING || state == CHARGING) {
        if (game->hit_player(actor, state == CHARGING ? NEBULA_HIT_STRONG : NEBULA_HIT_NORMAL) != 0) {
            game->log(me, "the boss hit the player");
        }
    }
}

static int32_t attacked_by(NebulaActor* actor, int32_t how, void* user) {
    (void)user;
    if (game->state(actor) != STUNNED || (how != NEBULA_ATTACK_SPIN && how != NEBULA_ATTACK_STOMP && how != NEBULA_ATTACK_GROUND_POUND)) {
        return how == NEBULA_ATTACK_SPIN ? 1 : 0;
    }
    health--;
    game->shake_camera_by(NEBULA_SHAKE_NORMAL_STRONG);
    state_is(actor, HURT, how == NEBULA_ATTACK_SPIN ? "the boss was hit by a spin" : "the boss was hit from above");
    return 1;
}

static NebulaObjectClass boss = {
    sizeof(NebulaObjectClass),
    "LittleBoss",
    "LittleBoss",
    150.0f,
    0,
    start,
    update,
    touch_player,
    0,
    attacked_by,
};

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->register_object(&boss);
}
