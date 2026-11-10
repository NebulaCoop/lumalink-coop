# Text, and other languages

## A message

Any message in the game, by its id, in as many languages as you like:

```json
"text": {
  "GalaxyName_EggStarGalaxy": { "default": "The Very First Galaxy", "fr": "La toute première galaxie" },
  "GalaxyNameShort_EggStarGalaxy": "First Galaxy",
  "ScenarioName_EggStarGalaxy1": "A New Start"
}
```

Any number of mods can add messages, which replacing the message files doesn't allow. Where two set one, the higher on the list wins.

## Languages

`en`, `en-GB`, `fr`, `fr-CA`, `es`, `es-MX`, `de`, `it`, `nl`, `pt`, `ja`, `ko`, `zh-Hans`, `zh-Hant`. The game takes its own language, then that language without its region, then `default`.

## The ids

| Id | What it is |
|---|---|
| `GalaxyName_<galaxy>` | A galaxy's name |
| `GalaxyNameShort_<galaxy>` | Its short name |
| `ScenarioName_<galaxy><n>` | A mission's name |
| Anything else | Each talk and interface message. A level's `MessageId` names its message, and the message file names them all |

To list every message in the game, open its message file: `lumalink extract LocalizeData/UsEnglish/MessageData/Message.arc --game game.pak -o out`. In a language folder such as `UsEnglish`.

## Pauses, pages, colours: message tags

Words can hold the game's own message tags, in brackets. They work in `text`, in talk lines (below) and in a plugin's words.

| Tag | Does |
|---|---|
| `[wait 30]` | Stops the text for 30 frames |
| `[page]` | Ends the box; the rest comes on the next press |
| `[center]` | Centres the line |
| `[sound SE_NAME]` | Plays one of the game's system sounds as the text reaches it |
| `[icon 7]` | Picture 7 of the game's picture font: buttons, stars, coins |
| `[small]`, `[normal]`, `[large]` | The text's size from here |
| `[player]` | The player's name: Mario or Luigi |
| `[color 1]` | The game's text colour 1 from here; `[color 0]` goes back |
| `[tag G T u...]` | Any other tag of the game: its group, its number and its parameters |

`[[` is one `[`. Anything else in brackets stays as written. A message read out with `lumalink extract` shows its tags this way, so one copied and put back is the game's own.

A new message in `text` can take the talk settings (balloon, camera, sound) of one of the game's with `like`:

```json
"text": {
  "MyNpc_Hello": { "default": "Hi, [player]!", "like": "AstroGalaxy_Kinopio012" }
}
```

## Talk: new lines, and what follows them

A character's talk is lines joined by a flow: after this line, that one, or a question, or an event. A layer's `merge/MessageData/messages.json` adds lines and changes flows, in every language the game has (`merge/<Language>/MessageData/Message.arc/message.bmg.json` for one language). Any number of mods can add to it.

```json
{
  "lines": {
    "NewTalk_Ask": { "default": "Say, [player]...[wait 20] want to hear a secret?", "fr": "Dis, [player]...[wait 20] tu veux connaître un secret ?", "like": "AstroGalaxy_Kinopio012" },
    "NewTalk_Secret": { "default": "The Toad Brigade naps in shifts.[page]Don't tell the captain!", "like": "AstroGalaxy_Kinopio012" },
    "NewTalk_Later": { "default": "Suit yourself!", "like": "AstroGalaxy_Kinopio012" }
  },
  "flows": {
    "AstroGalaxy_Kinopio012": "NewTalk_Ask",
    "NewTalk_Ask": {
      "ask": "select",
      "choices": { "yes": { "default": "Yes!", "fr": "Oui !" }, "no": "Not now" },
      "yes": "NewTalk_Secret",
      "no": "NewTalk_Later"
    },
    "NewTalk_Secret": "AstroGalaxy_Kinopio013",
    "NewTalk_Later": "AstroGalaxy_Kinopio013"
  }
}
```

This is `examples/new-talk`: a Toad in the Comet Observatory asks a question of his own, then talks on as before. `examples/new-npc` gives a new character its own talk the same way.

### Lines

A line is its words (one string for every language), or an object of words by language and its talk settings. Languages are named as in `text` (`fr-CA`, then `fr`), or by the game's folder (`UsFrench`), then `default`. A line of the game given no words in a language keeps its own there. A new line needs words for every language: give it a `default`.

| Key | Sets |
|---|---|
| `like` | A game line whose talk settings a new line starts from. Without it: an ordinary talk |
| `talk` | `normal`, `short`, `event`, `compose`, `flow` or `null` |
| `balloon` | The balloon's kind, 0 to 255 |
| `camera` | `normal`, `event` or `none` |
| `camera_set` | The stage's camera for the line, 0 to 65535 |
| `sound` | The talker's voice, -128 to 127 |
| `area` | The line's area number, as the game's lines have it, -128 to 127 |
| `already_read` | The number the game marks the line read under (see `message_read`), -128 to 127 |

### Flows

`flows` says, for a line by its id, what comes after it. A step is:

| Step | Means |
|---|---|
| A line's id | That line next |
| `null` | The talk ends |
| `{ "ask": ..., "yes": step, "no": step }` | A question: one step if it holds, the other if not |
| `{ "event": ..., "value": n, "next": step }` | Something happens, then `next` |

What `ask` can be:

| `ask` | Holds when |
|---|---|
| `select` | The player answers yes. `choices` gives the two answers' words; or `param` picks one of the game's 19 answer pairs (0 to 18) |
| `func` | The talker's own test, with `param` |
| `near_player` | The player is near the talker |
| `switch_a`, `switch_b` | The talker's switch is on |
| `player_normal`, `player_bee`, `player_boo` | The player is in that form |
| `player_luigi` | The player is Luigi |
| `power_star_appeared` | The stage's power star has appeared |
| `already_done` | The talker has done its part already |
| `galaxy_result` | One of the Comet Observatory's results, `param` saying which |
| `time_keep_demo` | A timed scene is running |
| `message_read` | Message-read number `param` is set |

What `event` can be:

| `event` | Does |
|---|---|
| `func`, `func_stop` | The talker's own action, with `value`; the talk waits until it is done |
| `anime` | The talker's animation `value` |
| `switch_a`, `switch_b` | Turns the talker's switch on |
| `kill` | The talker's leaving, with `value` |
| `skip` | Ends the talk at once |
| `none` | Nothing: a place for a plugin to watch |

A plugin hears each line open (`NEBULA_MOD_EVENT_TALK_LINE`, with the line's id) and each answer (`NEBULA_MOD_EVENT_TALK_CHOICE`). `lumalink check` reads the lines and flows and names what's wrong: a flow naming no line, a new line without words, an unknown `ask` or `event`, a `select` with both `choices` and `param`.

## Whole message files

A mod can replace `Message.bmg` from a language's message archive, as Wii mods do. If a mod sets text in some language folders and not the one being played, the game shows the mod's text from the same language of another region, then English, then what it has.

## A language the game doesn't have

A fan translation goes in the language players pick with least to lose, with a layer limited to it:

```json
{ "path": "layers/russian", "when": { "language": ["UsEnglish"] } }
```

Or put it in `text` under `default`. Then players of every language get it. Replace the fonts if your alphabet needs other letters: `font.brfnt` in the same places.

## In a plugin

```c
game->add_text("MyMessage", "Hello!");   /* at start */
game->show_message("MyMessage", 1);      /* in play, centred */
game->hide_message();
```
