# Your own screen: counters and pop-ups

The game's screens are layouts: `.brlyt` and `.brlan` files in `LayoutData/<name>.arc`, the Wii's own format. A plugin can open any of them, the game's or the mod's, and drive it.

## A counter

```c
static NebulaLayout* meter;

static int32_t stage_start(const NebulaModEventData* d, void* user) {
    meter = 0;   /* each stage has its own */
    return d->value;
}

static int32_t frame(const NebulaModEventData* d, void* user) {
    if (meter == 0) {
        meter = game->layout_open("CoinCounter");
        game->layout_move_pane(meter, "CoinCounter", -300.0f, 200.0f);
    }
    game->layout_set_number(meter, "Counter", bosses_left);
    return d->value;
}
```

| Call | What it does |
|---|---|
| `layout_open(name)` | Its archive loads now if it hasn't. Null: no such layout. Lasts for this stage |
| `layout_show(layout, show)` | |
| `layout_set_text(layout, pane, utf8)` | A text box's words |
| `layout_set_number(layout, pane, number)` | A text box's number |
| `layout_show_pane(layout, pane, show)` | One part of it |
| `layout_move_pane(layout, pane, x, y)` | Moved from where the layout has it |
| `layout_play(layout, animation)` | An animation by its name |
| `layout_is_playing(layout)` | |
| `layout_set_alpha(layout, alpha)` | 0 to 1 |

## Your own layout

Make it with any tool that writes the Wii's layouts (BenzinU), and put it in the mod:

```text
layers/base/files/LayoutData/BossMeter.arc/blyt/BossMeter.brlyt
layers/base/files/LayoutData/BossMeter.arc/anim/BossMeter_Appear.brlan
layers/base/files/LayoutData/BossMeter.arc/timg/bar.png
```

PNG pictures in `timg` become the game's form by themselves. Pane names are 16 letters at most: that's the file's form.

`lumalink files LayoutData/CoinCounter.arc --game game.pak` shows what one of the game's holds, to start from.

The screen is 608 by 456 in the layout's own units, and Nebula stretches it to any window. It stays in the right place at any aspect ratio.

## A short message

No layout needed:

```c
game->show_message("MyMessage", 1);   /* an id from `text`, or the words; 1: centred */
game->hide_message();
```

## The game's screens

To change the game's own screen, put your version of the file at its place: `layers/base/files/LayoutData/CoinCounter.arc/...`. Only the files you bring change.
