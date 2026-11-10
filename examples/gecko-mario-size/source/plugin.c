/* Mario's size, as the popular Gecko code did it. The player's model at the size the player chose in the mod's
   options, every frame of a stage (the game sets it back as a stage begins). Only the model: the player stands, jumps
   and is hit as at their own size. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;
static float size = 1.5f;

static int same(const char* a, const char* b) {
    while (*a != 0 && *a == *b) {
        a++;
        b++;
    }
    return *a == *b;
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    NebulaActor* player = game->player();
    (void)user;
    if (player != 0) {
        const float xyz[3] = {size, size, size};
        game->set_scale(player, xyz);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    const char* choice;
    game = api;
    me = self;
    choice = game->option(me, "size");
    if (choice != 0 && same(choice, "tiny")) {
        size = 0.5f;
    } else if (choice != 0 && same(choice, "small")) {
        size = 0.75f;
    } else if (choice != 0 && same(choice, "giant")) {
        size = 2.5f;
    }
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
