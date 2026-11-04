# Music and sound

Modern files: `.wav` (16-bit PCM) or `.ogg` (Vorbis). They become the game's own form when the player applies the mod.

## Replace a track

Put a file with the track's name in `AudioRes/Stream`:

```text
layers/base/files/AudioRes/Stream/SMG_stage_egg.ogg
```

`lumalink files AudioRes/Stream --game game.pak` lists them.

## A new track

A file and a row:

```json
"music": [{
  "name": "MyTheme",
  "file": "AudioRes/Stream/MyTheme.ogg",
  "stages": [
    { "stage": "EggStarGalaxy" },
    { "stage": "EggStarGalaxy", "scenario": 3 },
    { "stage": "EggStarGalaxy", "change_area": 1 }
  ]
}]
```

| Key | Meaning |
|---|---|
| `name` | Yours. Names starting `SE_`, `BGM_`, `MBGM_`, `STM_`, `ME_` are the game's |
| `file` | Its place in `files/` |
| `stage` | A stage it plays in |
| `scenario` | One mission. Left out: every mission with no row of its own |
| `change_area` | What a `ChangeBgmCube` with that `Obj_arg1` starts |

A row here comes before the game's own music for that stage, comets and fortresses included.

## Loops

From the file: a WAV's `smpl` chunk, an Ogg's `LOOPSTART` with `LOOPEND` or `LOOPLENGTH` comments. Without them the whole track loops.

## Music from a plugin

```c
game->play_music("MyTheme");
game->stop_music(60);  /* over 60 frames */
```

## Sound effects from a file

A sound by its name, played from a file. A new name, or one of the game's to replace it wherever it plays:

```json
"layers": [ {
  "path": "layers/base",
  "sound_effects": [
    { "name": "SE_SY_GET_COIN", "file": "AudioRes/Stream/mycoin.wav" },
    { "name": "MyBoom", "file": "AudioRes/Stream/boom.ogg" }
  ]
} ]
```

The file goes in `files/` at that place. In a plugin, `game->play(actor, NEBULA_PLAY_SOUND, "MyBoom")` plays it from an object, and `game->play_system_sound("MyBoom")` from nowhere in particular. Game sound names are in the sound table: a level's `SoundEmitterCube`, an object's own table, and `lumalink table` on any file that names them show what's used.

## The game's sound banks

The Wii's sound effects are wave banks, one set per stage. With any mod on, all 53 stay loaded: an object placed away from its home stage still sounds right. To replace a bank's waves, put a file at its place in `AudioRes/Waves`, as Wii mods do.

An object's sounds come from its archive's sound table, played by the animation's action. To change which sound an action plays, change that table.
