/* Spin and pointer from the start, as the popular Gecko code did it. The grant the Gateway's Luma gives
   ("SpinAndStarPointer"), in the first frame with the player of every stage whose save lacks it. The spin and the Star
   Pointer work from that frame, and the save keeps them. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int checked;

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    (void)user;
    checked = 0;
    return data->value;
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    (void)user;
    /* In a stage with the player: a scene without one (the file select) has no save of its own yet. */
    if (!checked && game->player() != 0) {
        checked = 1;
        if (!game->is_event_flag_on("SpinAndStarPointer")) {
            game->set_event_flag_on("SpinAndStarPointer");
            game->log(me, "the spin and the pointer are on");
        }
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
