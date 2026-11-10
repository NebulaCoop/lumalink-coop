"""tunable_names.py <Nebula tree> [--check]: crates/lumalink-core/src/tunable_names.rs from Nebula's docs/modding/TUNABLES.md.

Every tunable of the game, with its type ("f32", "s32", "bool", a list "f32[4]"), sorted by key, so that `lumalink
check` knows a key the game doesn't have and a value its type can't hold. Run it again when Nebula's list changes.
--check: exit 1 if the file differs from what would be written.
"""
import pathlib
import re
import sys

tree = pathlib.Path(sys.argv[1])
out = pathlib.Path(__file__).resolve().parent.parent / "crates/lumalink-core/src/tunable_names.rs"
row = re.compile(r"^\| `([^`]+)` \| ([a-z0-9]+(?:\[\d+\])?) \| ")
found = {}
for line in (tree / "docs/modding/TUNABLES.md").read_text(encoding="utf-8").splitlines():
    m = row.match(line)
    if m:
        found[m.group(1)] = m.group(2)
if len(found) < 1000:
    sys.exit(f"only {len(found)} tunables read: not Nebula's TUNABLES.md")
lines = [
    "//! Every tunable of the game and its type, sorted by key: written by tools/tunable_names.py from Nebula's",
    "//! docs/modding/TUNABLES.md. A list's type has its length: \"f32[4]\".",
    "",
    f"pub const TUNABLES: &[(&str, &str)] = &[",
]
for key in sorted(found, key=lambda k: k.encode()):
    lines.append(f'    ("{key}", "{found[key]}"),')
lines += ["];", ""]
text = "\n".join(lines)
if "--check" in sys.argv[2:]:
    sys.exit(0 if out.read_text(encoding="utf-8") == text else 1)
out.write_bytes(text.encode("utf-8"))
print(f"{len(found)} tunables")
