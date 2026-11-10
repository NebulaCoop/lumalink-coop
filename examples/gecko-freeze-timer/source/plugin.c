/* Freeze timers, as the popular Gecko code did it. Comet countdowns never run down. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->pause_timer(1);
}
