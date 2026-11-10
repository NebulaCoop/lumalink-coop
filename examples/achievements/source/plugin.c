/* Unlocks the mod's "fifty-coins" achievement at the 50th coin since the game started. The other three in luma.json
   unlock by themselves, at the events their "when" names. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int32_t coins;

static int32_t coin(const NebulaModEventData* data, void* user) {
    (void)user;
    const int32_t before = coins;
    coins += data->value;
    if (before < 50 && coins >= 50 && NEBULA_MOD_API_HAS(game, unlock_achievement)) {
        game->unlock_achievement(me, "fifty-coins");
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->subscribe(NEBULA_MOD_EVENT_COIN, coin, 0);
}
