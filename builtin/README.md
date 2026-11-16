# Built-in mods

NebulaCoop's own mods, as sm64coopdx has its built-in mods. Each folder is a mod project like those in
[examples](../examples), checked by `cargo test` (`every_builtin_mod_checks`) and packed with `lumalink pack` like any
other.

Each one is committed packed too, beside its folder (`<id>.lumacoop`). nebula-coop has this repository as its
`lumalink` submodule: its build embeds every `builtin/*.lumacoop` and writes them into the player's mods folder at
every start, off until they turn it on. So a change to a built-in mod is:

1. Change its folder here, then `LUMALINK_BLESS=1 cargo test every_builtin_mod_is_packed` to pack it again
   (`cargo test` fails while a package is stale) and commit both.
2. In nebula-coop, move the `lumalink` submodule to that commit.

| Mod | What it does |
|---|---|
| [character-movesets](character-movesets) | sm64coopdx's Character Movesets: Luigi gets Super Mario Galaxy's own moves back (a higher, floatier jump, less grip and a long sliding brake, a stronger stomp bounce, his Boo form's handling, a swim spin that costs oxygen). Without it he moves as Mario does. |
| [personal-star-counter](personal-star-counter) | sm64coopdx's Personal Star Counter: under the HUD's star counter, a red count of the Power Stars this player got in the room and a green one of all they ever got; the player list shows each player's room count. `/psc on` or `/psc off` shows or hides the HUD counts for this player. |

A built-in mod is a co-op mod (`"requires": ["coop"]`, packed as `.lumacoop`).
