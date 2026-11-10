/* First person: the camera is fixed to Mario's head for the whole stage, cutscenes and conversations included.
   - In play it looks where he faces; the pointer tilts the view up and down.
   - In a conversation it turns to the speaker, and the large talk box is placed over the speaker.
   - In a cutscene it looks where his head points, or at what the game's shot frames when that is away from him; in
     flight it looks where he is going.
   - The eye is kept off walls, and the game's near clipping is turned down so nothing close disappears.
   - His body stays visible (look down to see it); his head is hidden so it never blocks the view, and his whole body
     while he flies through a cutscene.
   The camera button switches to the game's camera and back. */
#include <nebula_mod_api.h>

static const NebulaModApi* game;
static const NebulaPlugin* me;

static int wanted = 1;        /* 1: first person (the camera button flips it) */
static int start_wanted = 1;  /* what each stage starts with (option "start") */
static int active;            /* this plugin's camera is in use */
static int event_camera;      /* the game has an event camera running (CAMERA_CHANGE) */
static float fov = 75.0f;

/* Smoothed state, so turns and steps are felt, not jolts. */
static float eye_height;
static float forward[3];  /* facing along the ground */
static float look[3];     /* view direction */
static float tilt;
static float attached;    /* 0 in play (eye over the body), 1 in a cutscene (eye on the head joint) */
static int fresh = 1;

/* The speaker and when they last spoke, for talks the game reports only line by line. */
static NebulaActor* speaker;
static int speaker_frames;
static int box_placed;

static int body_hidden;  /* his body is hidden for a flight */
static int air_frames;

/* The game's near clipping, saved while it is turned down. */
static double saved_near, saved_near_subjective, saved_near_scale;

#define EYE_HEIGHT 130.0f     /* to the eyes when the head joint is of no use */
#define HEAD_TO_EYES 25.0f    /* from the head joint up to eye level */
#define EYE_FORWARD 14.0f     /* from the head joint forward to the eyes */
#define MOST_TILT 1.2f        /* pointer tilt at the screen's edge, as a tangent */
#define WALL_GAP 22.0f        /* closest the eye may come to a wall */
#define NEAR_PLANE 8.0f
#define TALK_HOLD 240         /* frames a speaker is kept after their last line */

static float dot(const float* a, const float* b) {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

static void set3(float* out, float x, float y, float z) {
    out[0] = x;
    out[1] = y;
    out[2] = z;
}

static void copy3(float* out, const float* in) {
    set3(out, in[0], in[1], in[2]);
}

/* 1/sqrt(x) without the C library. */
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

/* v with its part along `up` removed, made unit length. */
static int flatten(float* v, const float* up) {
    const float along = dot(v, up);
    v[0] -= up[0] * along;
    v[1] -= up[1] * along;
    v[2] -= up[2] * along;
    return normalise(v);
}

static void cross(float* out, const float* a, const float* b) {
    set3(out, a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]);
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

static double tunable(const char* key) {
    double now = 0.0;
    double original = 0.0;
    game->get_tunable(key, &now, &original);
    return now;
}

/* Objects close to the camera stay drawn while the camera is at his eyes. */
static void near_clipping_off(void) {
    saved_near = tunable("ClippingJudge.cNearClip");
    saved_near_subjective = tunable("ClippingJudge.cNearClipSubjective");
    saved_near_scale = tunable("ObjUtil.cNearClipScale");
    game->set_tunable("ClippingJudge.cNearClip", NEAR_PLANE);
    game->set_tunable("ClippingJudge.cNearClipSubjective", NEAR_PLANE);
    game->set_tunable("ObjUtil.cNearClipScale", 0.0);
    game->set_camera_near(NEAR_PLANE);
}

/* His head (cap and face too) is hidden while the camera is in it. */
static void show_head(NebulaActor* player, int on) {
    if (player != 0 && NEBULA_MOD_API_HAS(game, show_joint)) {
        game->show_joint(player, "Head", on);
    }
}

static void near_clipping_back(void) {
    game->set_tunable("ClippingJudge.cNearClip", saved_near);
    game->set_tunable("ClippingJudge.cNearClipSubjective", saved_near_subjective);
    game->set_tunable("ObjUtil.cNearClipScale", saved_near_scale);
    game->set_camera_near(0.0f);
}

static void place_box(int on, float x, float y) {
    if (!NEBULA_MOD_API_HAS(game, place_talk_box)) {
        return;
    }
    if (on || box_placed) {
        game->place_talk_box(x, y, on);
    }
    box_placed = on;
}

/* The game's camera again. */
static void give_back(void) {
    if (!active) {
        return;
    }
    active = 0;
    fresh = 1;
    game->set_camera(0, 0, 0.0f);
    near_clipping_back();
    show_head(game->player(), 1);
    if (body_hidden) {
        game->show_model(game->player(), 1);
        body_hidden = 0;
    }
    place_box(0, 0.0f, 0.0f);
}

/* A point to look at on a character: their head, else above their feet. */
static void head_of(NebulaActor* actor, const float* up, float* out) {
    float at[3];
    game->get_position(actor, at);
    game->get_joint_position(actor, "Head", out);
    if (out[0] == at[0] && out[1] == at[1] && out[2] == at[2]) {
        set3(out, at[0] + up[0] * 110.0f, at[1] + up[1] * 110.0f, at[2] + up[2] * 110.0f);
    }
}

/* Keeps the eye out of the map: back along the line from the body if a wall is between, then off nearby walls. */
static void keep_off_walls(const float* body, float* eye) {
    float way[3], to[3], hit[3], side[3], up_probe[3];
    float probes[6][3];
    float length;
    int i;
    set3(way, eye[0] - body[0], eye[1] - body[1], eye[2] - body[2]);
    length = dot(way, way);
    if (normalise(way)) {
        set3(to, eye[0] + way[0] * WALL_GAP, eye[1] + way[1] * WALL_GAP, eye[2] + way[2] * WALL_GAP);
        if (game->raycast(body, to, hit)) {
            set3(eye, hit[0] - way[0] * WALL_GAP, hit[1] - way[1] * WALL_GAP, hit[2] - way[2] * WALL_GAP);
            /* Never behind the body's middle. */
            set3(to, eye[0] - body[0], eye[1] - body[1], eye[2] - body[2]);
            if (dot(to, way) < 0.0f || length <= 0.0f) {
                copy3(eye, body);
            }
        }
    }
    /* Six directions around the eye: forward, back, the sides, up and down. */
    set3(up_probe, 0.0f, 1.0f, 0.0f);
    copy3(probes[0], look);
    set3(probes[1], -look[0], -look[1], -look[2]);
    cross(side, look, up_probe);
    if (!normalise(side)) {
        set3(side, 1.0f, 0.0f, 0.0f);
    }
    copy3(probes[2], side);
    set3(probes[3], -side[0], -side[1], -side[2]);
    copy3(probes[4], up_probe);
    set3(probes[5], 0.0f, -1.0f, 0.0f);
    for (i = 0; i < 6; i++) {
        float gap;
        set3(to, eye[0] + probes[i][0] * WALL_GAP, eye[1] + probes[i][1] * WALL_GAP, eye[2] + probes[i][2] * WALL_GAP);
        if (!game->raycast(eye, to, hit)) {
            continue;
        }
        set3(side, hit[0] - eye[0], hit[1] - eye[1], hit[2] - eye[2]);
        gap = WALL_GAP - dot(side, probes[i]);
        eye[0] -= probes[i][0] * gap;
        eye[1] -= probes[i][1] * gap;
        eye[2] -= probes[i][2] * gap;
    }
}

static int32_t stage_frame(const NebulaModEventData* data, void* user) {
    NebulaActor* player = game->player();
    NebulaActor* talker = 0;
    float at[3], moving[3], down[3], up[3], front[3], head[3], eyes[3], eye[3], body[3], target[3], want[3], centre[2], pointer[2];
    float head_turn[12], played[3], game_look[3], gaze[3], speed, aim, height, rate;
    int scene, flying, has_gaze, shot_away;
    (void)user;
    if ((game->buttons_pressed() & NEBULA_BUTTON_CAMERA) != 0) {
        wanted = !wanted;
    }
    if (!wanted || player == 0) {
        give_back();
        return data->value;
    }
    if (!active) {
        near_clipping_off();
        game->show_model(player, 1);
        show_head(player, 0);
        body_hidden = 0;
        air_frames = 0;
        active = 1;
    }

    game->get_position(player, at);
    game->get_velocity(player, moving);
    game->get_gravity(player, down);
    set3(up, -down[0], -down[1], -down[2]);
    if (!normalise(up)) {
        set3(up, 0.0f, 1.0f, 0.0f);
    }

    /* Who is talking: the game says (talking_actor), or the last line's speaker for a while. */
    if (NEBULA_MOD_API_HAS(game, talking_actor)) {
        talker = game->talking_actor();
    }
    if (talker == 0 && speaker != 0 && speaker_frames < TALK_HOLD) {
        talker = speaker;
    }
    speaker_frames++;
    scene = game->is_cutscene_playing() || event_camera || talker != 0;

    /* Facing along the ground, smoothed. */
    game->player_front(front);
    if (!flatten(front, up)) {
        copy3(front, forward);
    }
    if (fresh) {
        copy3(forward, front);
    } else {
        forward[0] += (front[0] - forward[0]) * 0.35f;
        forward[1] += (front[1] - forward[1]) * 0.35f;
        forward[2] += (front[2] - forward[2]) * 0.35f;
        if (!flatten(forward, up)) {
            copy3(forward, front);
        }
    }

    /* His body is hidden while he flies through a cutscene (his arrival, a launch star): he curls up around the eye. */
    air_frames = game->is_on_ground(player) ? 0 : air_frames + 1;
    flying = scene && air_frames > 6;
    if (flying != body_hidden) {
        game->show_model(player, !flying);
        body_hidden = flying;
    }

    /* His head: the Head joint's place, and which way his face points (the joint's x axis). Before he is posed (his
       arrival) the joint is far from him and of no use. */
    game->get_joint_matrix(player, "Head", head_turn);
    set3(head, head_turn[3], head_turn[7], head_turn[11]);
    set3(gaze, head_turn[0], head_turn[4], head_turn[8]);
    set3(eye, head[0] - at[0], head[1] - at[1], head[2] - at[2]);
    has_gaze = dot(eye, eye) < 400.0f * 400.0f && normalise(gaze);
    if (!has_gaze) {
        set3(head, at[0] + up[0] * (EYE_HEIGHT - HEAD_TO_EYES), at[1] + up[1] * (EYE_HEIGHT - HEAD_TO_EYES),
             at[2] + up[2] * (EYE_HEIGHT - HEAD_TO_EYES));
        copy3(gaze, forward);
    }

    /* Whether the game's shot frames something away from him. */
    shot_away = 0;
    if (scene && NEBULA_MOD_API_HAS(game, game_camera) && game->game_camera(0, game_look)) {
        set3(played, game_look[0] - at[0], game_look[1] - at[1], game_look[2] - at[2]);
        shot_away = dot(played, played) > 450.0f * 450.0f;
    }

    /* Where to look. */
    speed = dot(moving, moving);
    if (talker != 0) {
        /* A conversation: at the speaker's face. */
        head_of(talker, up, target);
        set3(want, target[0] - at[0] - up[0] * eye_height, target[1] - at[1] - up[1] * eye_height,
             target[2] - at[2] - up[2] * eye_height);
    } else if (scene && speed > 25.0f * 25.0f && !game->is_on_ground(player)) {
        /* Flying (a launch star): where he is going. */
        copy3(want, moving);
    } else if (shot_away) {
        /* A cutscene showing something away from him: at it. */
        set3(want, played[0] - up[0] * eye_height, played[1] - up[1] * eye_height, played[2] - up[2] * eye_height);
    } else if (scene && has_gaze) {
        /* A cutscene showing him: where his face points. */
        copy3(want, gaze);
    } else {
        /* Play: where he faces, tilted by the pointer above or below the screen's middle. */
        aim = 0.0f;
        set3(target, at[0] + forward[0] * 1000.0f, at[1] + forward[1] * 1000.0f, at[2] + forward[2] * 1000.0f);
        if (game->pointer(pointer) && !fresh && game->screen_position(target, centre) && centre[1] > 1.0f) {
            aim = (centre[1] - pointer[1]) / centre[1];
            aim = aim > 1.0f ? 1.0f : aim < -1.0f ? -1.0f : aim;
            aim *= MOST_TILT;
        }
        tilt = fresh ? 0.0f : tilt + (aim - tilt) * 0.15f;
        set3(want, forward[0] + up[0] * tilt, forward[1] + up[1] * tilt, forward[2] + up[2] * tilt);
    }
    if (!normalise(want)) {
        copy3(want, forward);
    }
    rate = scene ? 0.12f : 0.5f;
    if (fresh) {
        copy3(look, want);
    } else {
        look[0] += (want[0] - look[0]) * rate;
        look[1] += (want[1] - look[1]) * rate;
        look[2] += (want[2] - look[2]) * rate;
        if (!normalise(look)) {
            copy3(look, want);
        }
    }

    /* His eyes: above the head joint and in front along his face. */
    set3(eyes, head[0] + up[0] * HEAD_TO_EYES + gaze[0] * EYE_FORWARD,
         head[1] + up[1] * HEAD_TO_EYES + gaze[1] * EYE_FORWARD,
         head[2] + up[2] * HEAD_TO_EYES + gaze[2] * EYE_FORWARD);
    set3(eye, eyes[0] - at[0], eyes[1] - at[1], eyes[2] - at[2]);
    height = dot(eye, up);
    height = height > 20.0f && height < 300.0f ? height : EYE_HEIGHT;
    eye_height = fresh ? height : eye_height + (height - eye_height) * 0.25f;

    /* The eye: over the body in play (steady), at his eyes in a scene (his animations move it), blended. */
    attached = fresh ? (scene ? 1.0f : 0.0f) : attached + ((scene ? 1.0f : 0.0f) - attached) * 0.1f;
    copy3(front, look);
    if (!flatten(front, up)) {
        copy3(front, forward);
    }
    {
        float over_body;
        int i;
        for (i = 0; i < 3; i++) {
            over_body = at[i] + moving[i] + up[i] * eye_height + front[i] * EYE_FORWARD;
            eye[i] = over_body + (eyes[i] - over_body) * attached;
        }
    }
    set3(body, at[0] + up[0] * 80.0f, at[1] + up[1] * 80.0f, at[2] + up[2] * 80.0f);
    keep_off_walls(body, eye);

    set3(target, eye[0] + look[0] * 1000.0f, eye[1] + look[1] * 1000.0f, eye[2] + look[2] * 1000.0f);
    game->set_camera(eye, target, fov);
    /* The large talk box over the speaker, while they are on the screen. */
    if (talker != 0) {
        head_of(talker, up, target);
        target[0] += up[0] * 60.0f;
        target[1] += up[1] * 60.0f;
        target[2] += up[2] * 60.0f;
        if (game->screen_position(target, centre)) {
            place_box(1, centre[0], centre[1] - 70.0f);
        } else {
            place_box(0, 0.0f, 0.0f);
        }
    } else {
        place_box(0, 0.0f, 0.0f);
    }
    fresh = 0;
    return data->value;
}

static int32_t camera_change(const NebulaModEventData* data, void* user) {
    (void)user;
    event_camera = data->value == 1;
    return data->value;
}

static int32_t talk_line(const NebulaModEventData* data, void* user) {
    (void)user;
    if (data->actor != 0) {
        speaker = data->actor;
        speaker_frames = 0;
    }
    return data->value;
}

static int32_t stage_start(const NebulaModEventData* data, void* user) {
    (void)user;
    fresh = 1;
    event_camera = 0;
    speaker = 0;
    speaker_frames = TALK_HOLD;
    box_placed = 0;
    wanted = start_wanted;
    return data->value;
}

static int32_t stage_end(const NebulaModEventData* data, void* user) {
    (void)user;
    speaker = 0;
    give_back();
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
    speaker_frames = TALK_HOLD;
    game->subscribe(NEBULA_MOD_EVENT_STAGE_FRAME, stage_frame, 0);
    game->subscribe(NEBULA_MOD_EVENT_CAMERA_CHANGE, camera_change, 0);
    game->subscribe(NEBULA_MOD_EVENT_TALK_LINE, talk_line, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_START, stage_start, 0);
    game->subscribe(NEBULA_MOD_EVENT_STAGE_END, stage_end, 0);
}
