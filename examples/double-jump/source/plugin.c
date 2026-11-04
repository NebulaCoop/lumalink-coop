/* A new move: a second jump in the air. The game's own jump button, once for each time the player leaves the ground. */
#include "nebula_mod_api.h"

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int used;

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    NebulaActor* player = game->player();
    float down[3];
    (void)user;
    if (player == 0) {
        return data->value;
    }
    if (game->value(NEBULA_VALUE_ON_GROUND) != 0) {
        used = 0;
    } else if (!used && (game->buttons_pressed() & NEBULA_BUTTON_JUMP) != 0) {
        float up[3];
        used = 1;
        /* Up is against gravity: on a planet, that's no one axis. */
        game->get_gravity(player, down);
        up[0] = down[0] * -30.0f;
        up[1] = down[1] * -30.0f;
        up[2] = down[2] * -30.0f;
        game->jump_player(up);
        game->play(player, NEBULA_PLAY_PLAYER_ANIMATION, "jump");
        game->log(me, "a second jump");
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
