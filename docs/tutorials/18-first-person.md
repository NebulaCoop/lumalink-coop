# The game in first person: a camera of your own

This tutorial walks through `examples/first-person`, a plugin that plays the whole game through Mario's eyes: play, cutscenes, launch stars and conversations. It's about 400 lines of C with no change to the game. It shows how a plugin takes the camera, follows the player on a round planet, keeps the camera out of walls, hides part of a model, reads the game's own camera and moves the talk box.

You need [Your first plugin](10-first-plugin.md) first.

## What it does

- The camera stays at Mario's eyes for the whole stage. In play it looks where he faces; the pointer above or below the screen's middle tilts the view.
- In a conversation it turns to the speaker's face, and the large talk box moves over the speaker.
- In a cutscene it looks the way the game's shot looks. In a launch star's flight it looks where he is going.
- His body stays visible: look down and you see it. Only his head is hidden, so it never fills the view.
- The camera button switches to the game's camera and back.
- Options: the field of view (60°, 75° or 90°), and whether each stage starts in first person or the game's camera.

## Each frame, before anything moves

`NEBULA_MOD_EVENT_STAGE_FRAME` runs once a frame, before the stage's objects move. The plugin reads where the player is, how fast he moves and which way is down, then sets the camera for the frame the game is about to draw:

```c
game->get_position(player, at);
game->get_velocity(player, moving);   /* this frame's step is still to come */
game->get_gravity(player, down);      /* on a planet, down is no one axis */
...
eye = at + moving + up * eye_height + front * EYE_FORWARD;
game->set_camera(eye, target, fov);
```

Adding this frame's velocity puts the eye where the player will be once he has moved, so the camera doesn't trail a frame behind.

## Up is wherever gravity says

On a small planet, up turns as the player walks around it. The plugin takes up from `get_gravity` and keeps its forward direction flat against it:

```c
along = dot(front, up);
front -= up * along;   /* along the ground */
normalise(front);
```

The forward direction is eased toward the player's each frame, so a sharp turn is felt but not a jolt. The eye height comes from the model's `Head` joint (`get_joint_position`), so crouching, swimming and a dive bring the view down with him. Before his model is posed (his arrival in a stage) the joint is far from him; the plugin then uses a fixed height.

## His head, not his body

```c
game->show_model(player, 1);
game->show_joint(player, "Head", 0);
```

`show_joint` hides one joint of a model and every joint under it, on that actor only. For the player, his face and cap go with `Head`, and the hidden head stays hidden in every power-up form. His arms, hands, body and legs still draw, and his shadow keeps its head. When the plugin gives the camera back it shows the head again with `show_joint(player, "Head", 1)`.

## Near things stay drawn

With the camera at his eyes, things close to the camera matter. Two settings decide what is drawn:

```c
game->set_camera_near(8.0f);                                  /* the near plane, from the game's 100 */
game->set_tunable("ClippingJudge.cNearClip", 8.0);            /* objects hidden near the camera */
game->set_tunable("ClippingJudge.cNearClipSubjective", 8.0);  /* the same in the game's own first-person view */
game->set_tunable("ObjUtil.cNearClipScale", 0.0);             /* objects that hide themselves near the camera */
```

Without these, platforms and enemies right in front of him vanish, since the game hides objects close to its camera. The plugin saves the old values with `get_tunable` and puts them back when it gives the camera back.

## Out of walls

The eye is in front of the body, so it can end up inside a wall. Before setting the camera, the plugin casts a ray from his chest to the eye; a hit moves the eye back to the wall's near side. Then six short rays (ahead, behind, both sides, up and down) push it off any wall closer than 22 units:

```c
if (game->raycast(body, eye_plus_gap, hit)) {
    eye = hit - way * WALL_GAP;
}
```

## Cutscenes and conversations

The plugin keeps the camera through every cutscene. It decides where to look from what the game reports:

- `talking_actor()` gives the character in conversation with the player. Small balloons report their speaker through `NEBULA_MOD_EVENT_TALK_LINE` instead, and the plugin keeps that speaker for a few seconds.
- `game_camera(position, target)` gives the shot the game would show this frame. In a cutscene the plugin looks along it, or straight at what it frames when that is away from Mario.
- In flight (a launch star), it looks along his velocity.

In play the eye sits over his body; in a cutscene it moves onto the head joint, so his animations (looking around, a bow) move the view. The two blend over a few frames.

## The talk box over the speaker

```c
head_of(talker, up, point);                 /* the speaker's head joint */
if (game->screen_position(point, xy)) {
    game->place_talk_box(1, xy[0], xy[1] - 70.0f);
}
```

`place_talk_box` centres the large talk box on a point of the screen and keeps it inside the screen; its pointer turns to the speaker. `place_talk_box(0, 0, 0)` puts it back.

## Giving the camera back

When the camera button is pressed or the stage ends:

```c
game->set_camera(0, 0, 0.0f);          /* the game's camera again */
game->set_camera_near(0.0f);           /* the game's near plane */
game->show_joint(player, "Head", 1);
```

and the saved tunables go back.

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
- Hide the HUD for a cleaner view: `show_hud(NEBULA_HUD_ALL, 0)`.
- Move other characters' balloons too: `talking_actor` and `screen_position` give their place.
