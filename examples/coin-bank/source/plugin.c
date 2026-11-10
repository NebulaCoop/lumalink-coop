/* Coin bank: every coin the player has ever picked up, kept across plays in the mod's own store (store_get and
   store_set), with the last galaxy played (store_get_text and store_set_text). As a stage starts, the box at the bottom
   shows the total and where the player was last. The store is the mod's own: the game's save is never touched, and it
   stays when the mod is updated. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;
static int32_t banked;     /* every coin so far, this play's too */
static int32_t unsaved;    /* coins not yet in the store */
static int shown;          /* this stage's message has been shown */
static int32_t up;         /* frames left for it on screen */

static void keep(void) {
    if (unsaved != 0) {
        game->store_set(me, "coins", banked);
        unsaved = 0;
    }
}

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    char last[64];
    (void)user;
    shown = 0;
    up = 0;
    /* Where the player was before this stage: the text kept last time, then this stage's name in its place. */
    if (game->store_get_text(me, "last", last, sizeof(last)) == 0) last[0] = '\0';
    if (data->name != 0) game->store_set_text(me, "last", data->name);
    /* Kept as text to show at the first frame with the player. */
    game->store_set_text(me, "shown_last", last);
    return data->value;
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    char last[64];
    char line[160];
    (void)user;
    if (!shown && game->player() != 0) {
        shown = 1;
        if (game->store_get_text(me, "shown_last", last, sizeof(last)) > 0) {
            game->format(line, sizeof(line), "Coins banked: %d. Last played: %s.", banked, last);
        } else {
            game->format(line, sizeof(line), "Coins banked: %d.", banked);
        }
        game->show_message(line, 0);
        game->log(me, line);
        up = 240;  /* four seconds */
    } else if (up > 0 && --up == 0) {
        game->hide_message();
    }
    return data->value;
}

/* What the game adds stays as it is: the bank only counts it. */
static int32_t coin(const NebulaModEventData* data, void* user) {
    (void)user;
    if (data->value > 0) {
        banked += data->value;
        unsaved += data->value;
    }
    return data->value;
}

/* Written when the stage ends and when the game saves: not at every coin. */
static int32_t stage_end(const NebulaModEventData* data, void* user) {
    (void)user;
    keep();
    up = 0;
    return data->value;
}

static int32_t game_save(const NebulaModEventData* data, void* user) {
    (void)user;
    keep();
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    game = api;
    me = self;
    banked = game->store_get(me, "coins", 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
    game->subscribe(NEBULA_MOD_EVENT_COIN, coin, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_END, stage_end, 0);
    game->subscribe(NEBULA_MOD_EVENT_GAME_SAVE, game_save, 0);
}
