/* The Gold Flower: a power-up the game doesn't have.
   - The flower: GoldFlowerItem, placed in Good Egg (merge/.../objinfo.json), its own model (ObjectData/GoldFlowerItem.arc)
     swaying and sparkling; touching it gives the power-up and it goes.
   - The power-up: 20 seconds with the game's fire music and Fire Mario's colours; a gold glow of its own on the player;
     its own sounds (the .wav files in AudioRes/Stream). The glow and the sparkle are particles of their own
     (merge/ParticleData/Effect.arc: Particles.jpc.json, gold_spark.png).
   source/make_gold_flower.py writes the model and the particle files.
   - Its spin: an animation of its own, GoldFlowerSpin (luma.json's player_animations: the game's fire spin, faster),
     that burns every enemy within 600 units, as fire does, with a burst of light and its sound.
   - Its physics: the first jump goes a third higher while it lasts (set_player_constant), the game's again after.
   Only the stable interface: it builds for every device Nebula runs on (build.cmd or build.sh: lumalink build). */
#include "nebula_mod_api.h"

NEBULA_PLUGIN_FLOATS

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int32_t gold = -1; /* add_power_up's number */
static int give_at_start;
static int given;
static int spun; /* the first spin with its animation, logged once */

enum { NEAR_MAX = 32 };

/* The player's first jump while the power-up lasts, and the number it had before. */
static float jump_before;

static void power_start(void* user) {
    NebulaActor* player = game->player();
    (void)user;
    if (player != 0) {
        game->play(player, NEBULA_PLAY_EFFECT, "GoldFlowerAura");
        game->play(player, NEBULA_PLAY_SOUND, "GoldFlowerOn");
    }
    jump_before = game->player_constant(0, "mJumpHeight", 0);
    game->set_player_constant(0, "mJumpHeight", 0, jump_before * 1.33f);
    game->log(me, "gold flower: on");
}

static void power_end(void* user) {
    NebulaActor* player = game->player();
    (void)user;
    if (player != 0) {
        game->stop_effect(player, "GoldFlowerAura");
    }
    game->set_player_constant(0, "mJumpHeight", 0, jump_before);
    game->log(me, "gold flower: off");
}

/* A spin with the power-up burns what is near, as a fireball would. */
static int32_t on_spin(const NebulaModEventData* data, void* user) {
    NebulaActor* player = game->player();
    NebulaActor* near[NEAR_MAX];
    float at[3];
    int32_t count, i, burned = 0;
    (void)user;
    if (player == 0 || game->value(NEBULA_VALUE_POWER_UP) != gold) {
        return data->value;
    }
    game->get_position(player, at);
    count = game->actors_near(at, 600.0f, near, NEAR_MAX);
    for (i = 0; i < count && i < NEAR_MAX; i++) {
        if (near[i] != player && game->attack_actor(near[i], player, NEBULA_ATTACK_FIRE) != 0) {
            burned++;
        }
    }
    if (game->play(player, NEBULA_PLAY_PLAYER_ANIMATION, "GoldFlowerSpin") == 0) {
        game->log(me, "gold flower: the player has no GoldFlowerSpin");
    } else if (!spun) {
        spun = 1;
        game->log(me, "gold flower: spun with GoldFlowerSpin");
    }
    game->play(player, NEBULA_PLAY_EFFECT, "SpinLightBurst");
    game->play(player, NEBULA_PLAY_SOUND, "GoldFlowerZap");
    if (burned > 0) {
        game->log(me, "gold flower: burned what was near");
    }
    return data->value;
}

/* The flower sways and sparkles. */
static void flower_start(NebulaActor* actor, void* user) {
    (void)user;
    game->play(actor, NEBULA_PLAY_SKELETON, "Wait");
    game->play(actor, NEBULA_PLAY_EFFECT, "Sparkle");
}

/* The flower: picked up on touch. */
static void flower_touched(NebulaActor* actor, void* user) {
    (void)user;
    if (gold >= 0 && !game->is_dead(actor)) {
        game->set_value(NEBULA_SET_POWER_UP, gold);
        game->kill(actor);
        game->log(me, "gold flower: picked up");
    }
}

static int32_t on_stage_start(const NebulaModEventData* data, void* user) {
    (void)user;
    given = 0;
    return data->value;
}

/* The "start with it" option: given on the stage's first frame with a player. */
static int32_t on_stage_frame(const NebulaModEventData* data, void* user) {
    (void)user;
    if (give_at_start && !given && gold >= 0 && game->player() != 0) {
        given = 1;
        game->set_value(NEBULA_SET_POWER_UP, gold);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    static const NebulaPowerUpClass power = {
        sizeof(NebulaPowerUpClass), "GoldFlower", 20 * 60, "BGM_FIRE_A", 2, 0, power_start, 0, power_end,
        0, NEBULA_POWER_UP_FIRE,
    };
    static const NebulaObjectClass flower = {
        sizeof(NebulaObjectClass), "GoldFlowerItem", "GoldFlowerItem", 80.0f, 0, flower_start, 0, flower_touched, 0, 0,
        NEBULA_OBJECT_MAP, 0, 60.0f, 2000.0f, 0, 0,
    };
    const char* start = 0;
    game = api;
    me = self;
    gold = game->add_power_up(&power);
    if (gold < 0) {
        game->log(me, "gold flower: the power-up can't be added");
        return;
    }
    if (game->register_object(&flower) != 0) {
        game->log(me, "gold flower: the flower can't be registered");
    }
    game->add_sound_effect("GoldFlowerOn", "AudioRes/Stream/GoldFlowerOn.wav");
    game->add_sound_effect("GoldFlowerZap", "AudioRes/Stream/GoldFlowerZap.wav");
    game->subscribe(NEBULA_MOD_EVENT_PLAYER_SPIN, on_spin, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, on_stage_start, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, on_stage_frame, 0);
    start = game->option(me, "start");
    give_at_start = start != 0 && start[0] == 'y';
    game->log(me, "gold flower: ready");
}
