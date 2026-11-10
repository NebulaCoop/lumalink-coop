/* Hide HUD, as the popular Gecko code did it. No counters, no life meter, no pointer. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->show_hud(NEBULA_HUD_ALL, 0);
}
