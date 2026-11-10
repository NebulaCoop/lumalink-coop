/* Half damage: every other hit costs no health. It replaces the game's MarioActor::decLife with hook(), runs the game's
   own code, and gives the health back on every second hit. (The player_damage event can do this without a hook; this
   example shows how a hook is made.) */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

typedef void (*DecLife)(void* mario, uint16_t amount);
static DecLife original;
static int hits;

static void dec_life(void* mario, uint16_t amount) {
    int32_t before = game->value(NEBULA_VALUE_LIFE);
    original(mario, amount);
    if (game->value(NEBULA_VALUE_LIFE) < before && ++hits % 2 == 0) {
        game->set_value(NEBULA_SET_LIFE, before);
        game->log(me, "a free hit");
    }
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
#if defined(_WIN32)
    void* target = game->find_function("?decLife@MarioActor@@QEAAXG@Z");
#else
    void* target = game->find_function("_ZN10MarioActor7decLifeEt");
#endif
    if (target == 0 || game->hook(target, (void*)dec_life, (void**)&original) != 0) {
        game->log(me, "can't hook MarioActor::decLife on this device");
    }
}
