/* First person: the camera at the player's eyes, looking where the player faces; the pointer tilts the view up and down.
   The camera button switches between this and the game's camera. Cutscenes, talks and the stage's own event cameras keep
   the game's camera while they last. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

/* The player's choice: 1 first person, 0 the game's camera (the camera button); each stage starts as the option says. */
static int wanted = 1;
static int start_wanted = 1;
/* What this frame was given: 1 the plugin's camera. */
static int active;
/* An event camera (a cutscene's, a talk's) holds the view. */
static int event_camera;
static float fov = 75.0f;

/* Smoothed, so a turn or a step down is felt and not a jolt. */
static float eye_height;
static float forward[3];
static float tilt;
static int fresh = 1;

/* The player is about this tall to the eyes when the model has no head joint to ask. */
#define EYE_HEIGHT 130.0f
/* From the head joint to the eyes, along the player's up. */
#define HEAD_TO_EYES 10.0f
/* How far the pointer tilts the view: the tangent of the angle at the screen's edge. */
#define MOST_TILT 1.2f

static float dot(const float* a, const float* b) {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/* 1/sqrt(x) without the C library: a first guess from the float's bits, then Newton's steps. */
static float inverse_root(float x) {
    union {
        float f;
        uint32_t u;
    } bits;
    float y;
    if (!(x > 1e-12f)) {
        return 0.0f;
    }
    bits.f = x;
    bits.u = 0x5F3759DFu - (bits.u >> 1);
    y = bits.f;
    y = y * (1.5f - 0.5f * x * y * y);
    y = y * (1.5f - 0.5f * x * y * y);
    y = y * (1.5f - 0.5f * x * y * y);
    return y;
}

/* v made unit length; 0 when it has none. */
static int normalise(float* v) {
    const float inverse = inverse_root(dot(v, v));
    if (inverse == 0.0f) {
        return 0;
    }
    v[0] *= inverse;
    v[1] *= inverse;
    v[2] *= inverse;
    return 1;
}

static int same_text(const char* a, const char* b) {
    if (a == 0 || b == 0) {
        return 0;
    }
    while (*a != 0 && *a == *b) {
        a++;
        b++;
    }
    return *a == *b;
}

/* The game's camera again, and the player seen. */
static void give_back(NebulaActor* player) {
    if (!active) {
        return;
    }
    active = 0;
    fresh = 1;
    game->set_camera(0, 0, 0.0f);
    if (NEBULA_MOD_API_HAS(game, set_camera_near)) {
        game->set_camera_near(0.0f);
    }
    if (player != 0) {
        game->show_model(player, 1);
    }
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    NebulaActor* player = game->player();
    float at[3], moving[3], down[3], up[3], front[3], head[3], eye[3], look[3], centre[2], pointer[2];
    float along, height, aim;
    (void)user;
    if ((game->buttons_pressed() & NEBULA_BUTTON_CAMERA) != 0) {
        wanted = !wanted;
    }
    if (!wanted || player == 0 || game->is_dead(player) || game->is_cutscene_playing() || event_camera) {
        give_back(player);
        return data->value;
    }

    game->get_position(player, at);
    /* This frame's step is still to come (the event runs before objects move): the eye goes where the player will be. */
    game->get_velocity(player, moving);
    game->get_gravity(player, down);
    up[0] = -down[0];
    up[1] = -down[1];
    up[2] = -down[2];
    if (!normalise(up)) {
        up[0] = 0.0f;
        up[1] = 1.0f;
        up[2] = 0.0f;
    }

    /* The eyes: the head joint's height over the feet (crouching, swimming and a long jump's dive bring it down). */
    game->get_joint_position(player, "Head", head);
    head[0] -= at[0];
    head[1] -= at[1];
    head[2] -= at[2];
    height = dot(head, up);
    height = height > 20.0f && height < 300.0f ? height + HEAD_TO_EYES : EYE_HEIGHT;
    eye_height = fresh ? height : eye_height + (height - eye_height) * 0.25f;

    /* Where the player faces, along the ground. */
    game->player_front(front);
    along = dot(front, up);
    front[0] -= up[0] * along;
    front[1] -= up[1] * along;
    front[2] -= up[2] * along;
    if (!normalise(front)) {
        front[0] = forward[0];
        front[1] = forward[1];
        front[2] = forward[2];
    }
    if (fresh) {
        forward[0] = front[0];
        forward[1] = front[1];
        forward[2] = front[2];
    } else {
        forward[0] += (front[0] - forward[0]) * 0.35f;
        forward[1] += (front[1] - forward[1]) * 0.35f;
        forward[2] += (front[2] - forward[2]) * 0.35f;
        /* Kept on the ground as the up turns on a round planet. */
        along = dot(forward, up);
        forward[0] -= up[0] * along;
        forward[1] -= up[1] * along;
        forward[2] -= up[2] * along;
        if (!normalise(forward)) {
            forward[0] = front[0];
            forward[1] = front[1];
            forward[2] = front[2];
        }
    }

    /* The pointer above or below the screen's middle tilts the view; off the screen, the view comes level. */
    aim = 0.0f;
    look[0] = at[0] + forward[0] * 1000.0f;
    look[1] = at[1] + forward[1] * 1000.0f;
    look[2] = at[2] + forward[2] * 1000.0f;
    if (game->pointer(pointer) && active && game->screen_position(look, centre) && centre[1] > 1.0f) {
        aim = (centre[1] - pointer[1]) / centre[1];
        aim = aim > 1.0f ? 1.0f : aim < -1.0f ? -1.0f : aim;
        aim *= MOST_TILT;
    }
    tilt = fresh ? 0.0f : tilt + (aim - tilt) * 0.15f;

    eye[0] = at[0] + moving[0] + up[0] * eye_height;
    eye[1] = at[1] + moving[1] + up[1] * eye_height;
    eye[2] = at[2] + moving[2] + up[2] * eye_height;
    look[0] = eye[0] + (forward[0] + up[0] * tilt) * 1000.0f;
    look[1] = eye[1] + (forward[1] + up[1] * tilt) * 1000.0f;
    look[2] = eye[2] + (forward[2] + up[2] * tilt) * 1000.0f;
    game->set_camera(eye, look, fov);
    if (!active) {
        /* Close walls and the ground at the player's feet are drawn, as in the game's own look-around view. */
        if (NEBULA_MOD_API_HAS(game, set_camera_near)) {
            game->set_camera_near(10.0f);
        }
        game->show_model(player, 0);
        active = 1;
    }
    fresh = 0;
    return data->value;
}

static int32_t camera_change(const NebulaModEventData* data, void* user) {
    (void)user;
    event_camera = data->value == 1;
    return data->value;
}

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    (void)user;
    /* A new stage: its player is a new actor, seen until the first frame in first person. */
    active = 0;
    fresh = 1;
    event_camera = 0;
    wanted = start_wanted;
    game->set_camera(0, 0, 0.0f);
    return data->value;
}

NEBULA_PLUGIN_ENTRY(api, self) {
    const char* view;
    game = api;
    me = self;
    view = game->option(me, "fov");
    if (same_text(view, "narrow")) {
        fov = 60.0f;
    } else if (same_text(view, "wide")) {
        fov = 90.0f;
    }
    start_wanted = !same_text(game->option(me, "start"), "game");
    wanted = start_wanted;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
    game->subscribe(NEBULA_MOD_EVENT_CAMERA_CHANGE, camera_change, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
}
