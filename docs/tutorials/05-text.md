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
