## Summary

<!-- What does this change, and why? Link the issue it fixes, e.g. "Fixes #12". -->

## Testing

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets` is clean
- [ ] Tried in the game: <!-- which mod, alone or in a room, how many players -->

## Checklist

- [ ] Follows [CONTRIBUTING.md](https://github.com/NebulaCoop/lumalink-coop/blob/main/CONTRIBUTING.md): no copy of a `luma-validate` rule, nothing dropped
      silently, no path out of a mod's folder, no game files or mods committed.
- [ ] Every new line LumaLink says has a row in `LumaLinkText.tsv` in all 14 languages.
- [ ] A changed built-in mod is packed again (`builtin/<id>.lumacoop` matches its folder), and nebula-coop gets a PR
      moving its `lumalink` submodule here.
- [ ] A new or changed `coop.*` call is in the SDK and the modding docs.
- [ ] If this isn't co-op specific, I considered whether it belongs upstream in
      [keepvaibin/lumalink](https://github.com/keepvaibin/lumalink).

By contributing, I agree that my work is licensed under GPL-3.0-only.
