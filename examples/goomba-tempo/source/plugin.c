/* Goomba tempo: every Goomba's animation plays at the speed the player picks in the mod's options: half, double or
   held still. The game starts a Goomba's animations at their own speed each time it changes one (walking, turning,
   being squashed), so the speed is set again every frame of a stage, for every Goomba (find_actor). Only the
   animation: how fast a Goomba walks stays the game's. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;
static float rate = 0.5f;
static int32_t frames;

static int is(const char* a, const char* b) {
    if (a == 0) return 0;
    while (*a != '\0' && *a == *b) {
        a++;
        b++;
    }
    return *a == *b;
}

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    const char* choice = game->option(me, "tempo");
    (void)user;
    frames = 0;
    rate = is(choice, "double") ? 2.0f : is(choice, "held") ? 0.0f : 0.5f;
    return data->value;
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    NebulaActor* goomba = 0;
    NebulaActor* first = 0;
    (void)user;
    while ((goomba = game->find_actor("Kuribo", goomba)) != 0) {
        /* A Goomba far from the player sleeps with no animation playing. */
        if (first == 0 && game->animation_length(goomba) > 0.0f) first = goomba;
        game->set_animation_rate(goomba, rate);
    }
    /* Once a second for the first ten with a Goomba awake, where its animation is: at half speed it moves on 30 frames
       a second. */
    if (first != 0 && ++frames <= 600 && frames % 60 == 0) {
        char line[96];
        game->format(line, sizeof(line), "a Goomba's animation at frame %d of %d",
                     (int)game->animation_frame(first), (int)game->animation_length(first));
        game->log(me, line);
    }
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
}
