# The game in first person: a camera of your own

This tutorial walks through `examples/first-person`, a plugin that plays the whole game through Mario's eyes. It's about 220 lines of C, and it shows how a plugin takes the camera, follows the player on a round planet, hides the player's model and gives the game its cameras back for cutscenes and talks.

You need [Your first plugin](10-first-plugin.md) first.

## What it does

- The camera sits at Mario's eyes and looks where he faces. Pushing the stick ahead walks ahead; to the side, he turns, and the view turns with him.
- The pointer above or below the screen's middle tilts the view up or down. Off the screen, the view comes level.
- The camera button switches between his eyes and the game's camera.
- Cutscenes, talks, a launch star's flight, a star's "You got a Star!" and every camera a stage starts for an event keep the game's own camera while they last; his eyes come back when they end.
- Two options: the field of view (60°, 75° or 90°), and whether each stage starts in his eyes or the game's camera.

## Each frame, before anything moves

`NEBULA_MOD_EVENT_STAGE_FRAME` runs once a frame, before the stage's objects move. The plugin reads where the player is, how fast he moves and which way is down, then sets the camera for the frame the game is about to draw:

```c
game->get_position(player, at);
game->get_velocity(player, moving);   /* this frame's step is still to come */
game->get_gravity(player, down);      /* on a planet, down is no one axis */
...
eye = at + moving + up * eye_height;
game->set_camera(eye, look, fov);
```

Adding this frame's velocity puts the eye where the player will be once he has moved, so the camera doesn't trail a frame behind.

## Up is wherever gravity says

On a small planet, up turns as the player walks around it. The plugin takes up from `get_gravity`, and keeps its forward direction flat against it:

```c
along = dot(front, up);
front -= up * along;   /* along the ground */
normalise(front);
```

The forward direction is eased toward the player's (a third of the way each frame), so a sharp turn is felt but not a jolt. The eye height comes from the model's `Head` joint (`get_joint_position`), so crouching, swimming and a dive bring the view down with him.

## Hiding the player

```c
game->show_model(player, 0);
game->set_camera_near(10.0f);
```

`show_model` hides the model, his hands, face and power-up suit with it; he still moves, animates, touches and is hit, and his shadow stays on the ground under you. `set_camera_near` brings the near plane in from the game's 100 units to 10, as the game's own look-around view does, so a wall right in front of him is drawn instead of cut open.

## Giving the camera back

The plugin hands back when a cutscene plays, an event camera holds the view, the player is gone, or the camera button says so:

```c
if (!wanted || player == 0 || game->is_dead(player) || game->is_cutscene_playing() || event_camera) {
    game->set_camera(0, 0, 0.0f);      /* the game's camera again */
    game->set_camera_near(0.0f);       /* the game's near plane */
    game->show_model(player, 1);
}
```

`is_cutscene_playing` covers cutscenes, the stage's start and a star's get sequence. Event cameras (a talk's, a cutscene's, a launch star's, `start_camera`'s) come as `NEBULA_MOD_EVENT_CAMERA_CHANGE` with value 1; when the last one ends and the area's camera takes the view back, the same event comes with value 0:

```c
static int32_t camera_change(const NebulaModEventData* data, void* user) {
    (void)user;
    event_camera = data->value == 1;
    return data->value;
}
```

A launch star shows it: its cameras come one after another as Mario is shot off (value 1 each), and value 0 comes as he lands on the next planet, where his eyes take the view again.

## No C library

A plugin built for every device has no C library, so the plugin writes its own square root (a first guess from the float's bits, then Newton's steps) and its own text compare. `game->alloc` and `game->format` are there when a plugin needs memory or `printf`.

## Build and try it

```console
lumalink build examples/first-person
lumalink run examples/first-person
```

`lumalink build` makes the library for Windows, Linux, Android and macOS, x64 and arm64 each. `lumalink pack examples/first-person` makes the `.luma`.

## Going further

- Turn with the pointer at the screen's edges, as pointer shooters do: rotate `forward` about `up` while the pointer is near the left or right edge.
- Show the player's hands: a model of your own (`register_object`) placed at the eye each frame.
- Hide the HUD for a cleaner view: `show_hud(NEBULA_HUD_ALL, 0)`.
