# Built-in mods

NebulaCoop's own mods, as sm64coopdx has its built-in mods. nebula-coop has this repository as its `lumalink`
submodule: its build embeds every `builtin/*.lumacoop` and writes them into the player's mods folder at every start,
off until they turn it on.

Each folder is the mod's source, and its package is committed beside it (`<id>.lumacoop`, a `.luma` by another
name). `cargo test` (`every_builtin_mod_is_packed`) fails while a package isn't its folder's. A change to a built-in
mod is:

1. Change its folder here, then pack it again with LLVM 22.1.6 on the PATH (lld beside it) and the game build's
   self-description (`--names`, with the SDK as `<names>/sdk`), so the code is built for all nine platforms:

   ```sh
   lumalink pack --names <names> --no-installed -o builtin/<id>.lumacoop builtin/<id>
   ```

   Commit the folder and the package. The built `code/` folders stay out of git.
2. In nebula-coop, move the `lumalink` submodule to that commit.

| Mod | What it does |
|---|---|
| [character-movesets](character-movesets) | sm64coopdx's Character Movesets: Luigi gets Super Mario Galaxy's own moves back (a higher, floatier jump, less grip and a long sliding brake, a stronger stomp bounce, his Boo form's handling, a swim spin that costs oxygen). Without it he moves as Mario does. Data only: the player constants under `pre_standard`. |
| [personal-star-counter](personal-star-counter) | sm64coopdx's Personal Star Counter: under the HUD's star counter, a red count of the Power Stars this player got in the room and a green one of all they ever got; the player list shows each player's room count. `/psc on` or `/psc off` shows or hides the HUD counts for this player. LumaScript on nebula-coop's `coop` service, which plain Nebula doesn't have. |
