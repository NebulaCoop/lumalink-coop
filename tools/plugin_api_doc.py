"""plugin_api_doc.py <nebula_mod_api.h> <plugin-api.md> [--check]: the plugin reference written from the header.

Every call of NebulaModApi in its order, with the comment over it: a comment speaks for the calls under it up to the next
blank line or comment. Then every enum and struct the calls take, as the header has them. A call with no comment is an
error: the header is where a call is explained. --check: exit 1 if the file differs from what would be written.
"""
import re
import sys

HEADER, OUT = sys.argv[1], sys.argv[2]
CHECK = "--check" in sys.argv[3:]

text = open(HEADER, encoding="utf-8").read().replace("\r\n", "\n")

body = re.search(r"typedef struct NebulaModApi \{\n(.*?)\n\} NebulaModApi;", text, re.S)
if body is None:
    sys.exit("no NebulaModApi in the header")

MEMBER = re.compile(r"^\s*(.+?)\(\*(\w+)\)\((.*)\);\s*$")
calls = []  # (name, signature, comment)
missing = []
comment = None
pending = ""
in_comment = False
lines = body.group(1).split("\n")
i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    if in_comment:
        pending += " " + stripped
        if "*/" in stripped:
            in_comment = False
            comment = pending
        i += 1
        continue
    if stripped.startswith("/*"):
        pending = stripped
        if "*/" not in stripped:
            in_comment = True
        else:
            comment = pending
        i += 1
        continue
    if stripped == "":
        comment = None
        i += 1
        continue
    # A member may run over lines up to its ';'.
    joined = stripped
    while not joined.endswith(";") and i + 1 < len(lines):
        i += 1
        joined += " " + lines[i].strip()
    m = MEMBER.match(joined)
    if m:
        ret, name, args = m.group(1).strip(), m.group(2), m.group(3).strip()
        if comment is None:
            missing.append(name)
            comment = "/* */"
        words = re.sub(r"^/\*\s*|\s*\*/$", "", comment)
        words = re.sub(r"\s+", " ", words).strip()
        calls.append((name, f"{ret} {name}({args})", words))
    i += 1
if missing:
    sys.exit("no comment over these in the header: " + " ".join(missing))


def tidy(signature):
    # "NebulaActor* actor", "const char* name": the star on the type, one space after it.
    signature = re.sub(r"\s*\*\s*", "* ", signature)
    signature = signature.replace("* )", "*)").replace("* ,", "*,").replace("(* ", "(*")
    return re.sub(r"\s+", " ", signature).replace("* (", "*(").strip()


out = ["# The plugin interface, call by call", "",
       "Written from `nebula_mod_api.h` (by `tools/plugin_api_doc.py`), in its order: fields are only ever added at the end, so a "
       "plugin keeps working on later versions of Nebula. Check `game->size` before using a call newer than the oldest Nebula you "
       "support.", "",
       "A null object, layout, point or name where a call needs one makes the call do nothing: it answers 0, null, or -1 where "
       "-1 means \"no such\". Where null means something (`find_actor`'s `name`), the call says so. Calls about the player or "
       "the stage do nothing, and answer 0 or null, between stages and on the file select screen. A number that is NaN or "
       "infinite, where a position, motion, angle, scale, time or tuning number is set, changes nothing either.", "",
       "| Call | Signature |", "|---|---|"]
for name, signature, _ in calls:
    out.append(f"| `{name}` | `{tidy(signature)}` |")
out += ["", "## What each does", ""]
for name, signature, words in calls:
    out += [f"### `{name}`", "", "```c", tidy(signature), "```", "", words, ""]
out += ["## Numbers and names", ""]
for kind, block, name in re.findall(r"typedef (enum|struct) (?:\w+ )?\{\n(.*?)\n\} (\w+);", text, re.S):
    if name == "NebulaModApi":
        continue
    out += [f"### `{name}`", "", "```c", block.rstrip(), "```", ""]
result = "\n".join(out).rstrip() + "\n"

if CHECK:
    now = open(OUT, encoding="utf-8").read().replace("\r\n", "\n")
    if now != result:
        sys.exit(f"{OUT} is not as the header writes it: run tools/plugin_api_doc.py")
    print("same")
else:
    open(OUT, "w", encoding="utf-8", newline="\n").write(result)
    print(f"{len(calls)} calls")
