"""book.py <Nebula tree> <out.md> [--check]: every page of docs/ and Nebula's generated lists as one book.

The pages in reading order (PARTS), each a chapter: its headings one level down, an anchor before each heading so a link
into a page (`tutorials/02-models.md#textures`) lands on that heading in the book. A link to a file outside docs/
(an example, a source file) points at the repository on GitHub. Nebula's lists (tunables, events, nerves, tables) come
from <Nebula tree>/docs/modding, which the game's tools write. A link that leads nowhere is an error. --check: exit 1 if
the book differs from what would be written.
"""
import pathlib
import re
import sys

REPO = "https://github.com/keepvaibin/lumalink/blob/main/"
NEBULA = "https://github.com/keepvaibin/nebula/blob/main/"
PARTS = [
    ("Start", ["README.md", "using-mods.md", "start-here.md", "concepts.md", "guide.md"]),
    ("Tutorials", [f"tutorials/{p}" for p in [
        "01-textures.md", "02-models.md", "03-levels.md", "04-music-and-sound.md", "05-text.md", "06-rules-and-physics.md",
        "07-options.md", "08-new-objects.md", "09-new-galaxy.md", "10-first-plugin.md", "11-a-boss.md", "12-power-ups.md",
        "13-screens.md", "14-cutscenes-movies-lumas.md", "15-converting.md", "16-total-conversion.md", "17-publishing.md",
        "18-first-person.md"]]),
    ("Reference", ["REFERENCE.md", "format.md", "plugins.md", "reference/plugin-api.md", "reference/events.md",
                   "reference/objects.md", "reference/settings.md", "reference/player-numbers.md",
                   "reference/stage-rules.md"]),
    ("The game's lists", ["nebula:EVENTS.md", "nebula:NERVES.md", "nebula:TABLES.md", "nebula:TUNABLES.md"]),
]
LINK = re.compile(r"(!?)\[([^\]]*)\]\(([^)\s]+)\)")
HEADING = re.compile(r"^(#{1,6})\s+(.*)$")


def slug(text):
    text = re.sub(r"`|\*|_(?=\w)|(?<=\w)_", "", text.strip().lower())
    text = re.sub(r"[^\w\- ]", "", text)
    return text.replace(" ", "-")


def page_id(page):
    return "p-" + re.sub(r"[^a-z0-9]+", "-", page.lower().replace("nebula:", "nebula-").removesuffix(".md")).strip("-")


def main():
    nebula, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    docs = pathlib.Path(__file__).resolve().parent.parent / "docs"
    root = docs.parent
    pages = [p for _, group in PARTS for p in group]

    def source(page):
        return nebula / "docs/modding" / page[7:] if page.startswith("nebula:") else docs / page

    texts = {p: source(p).read_text(encoding="utf-8").replace("\r\n", "\n") for p in pages}
    # Every heading's anchor, by page and its slug on that page.
    anchors = {}
    for page, text in texts.items():
        seen = {}
        fence = False
        for line in text.split("\n"):
            if line.startswith("```"):
                fence = not fence
            m = None if fence else HEADING.match(line)
            if m:
                s = slug(m.group(2))
                n = seen.get(s, 0)
                seen[s] = n + 1
                anchors[(page, s if n == 0 else f"{s}-{n}")] = f"{page_id(page)}--{s}" + (f"-{n}" if n else "")
    dead = []

    def resolve(page, target):
        if re.match(r"[a-z]+:", target):
            return target
        path, _, frag = target.partition("#")
        if page.startswith("nebula:"):
            base = "nebula:"
            key = base + path if path else page
            if key in texts:
                return f"#{anchors.get((key, frag), page_id(key))}" if frag else f"#{page_id(key)}"
            return NEBULA + "docs/modding/" + path
        if not path:
            key = page
        else:
            full = (docs / page).parent / path
            try:
                key = full.resolve().relative_to(docs.resolve()).as_posix()
            except ValueError:
                inside = full.resolve()
                if inside.exists():
                    return REPO + inside.relative_to(root.resolve()).as_posix()
                dead.append(f"{page}: {target}")
                return target
        if key in texts:
            if frag:
                if (key, frag) not in anchors:
                    dead.append(f"{page}: {target} (no such heading)")
                    return f"#{page_id(key)}"
                return f"#{anchors[(key, frag)]}"
            return f"#{page_id(key)}"
        if (docs / key).exists():
            return REPO + "docs/" + key
        dead.append(f"{page}: {target}")
        return target

    lines = ["# Modding Super Mario Galaxy with Nebula: the book", "",
             "Every page of LumaLink's docs and Nebula's generated lists, in reading order. Written by tools/book.py; edit the pages, not this file.", "",
             "## Contents", ""]
    for part, group in PARTS:
        lines.append(f"- **{part}**")
        for page in group:
            title = next((HEADING.match(l).group(2) for l in texts[page].split("\n") if HEADING.match(l)), page)
            lines.append(f"  - [{title}](#{page_id(page)})")
    lines.append("")
    for part, group in PARTS:
        lines += [f"# {part}", ""]
        for page in group:
            lines += [f'<a id="{page_id(page)}"></a>', ""]
            seen = {}
            fence = False
            for line in texts[page].split("\n"):
                if line.startswith("```"):
                    fence = not fence
                    lines.append(line)
                    continue
                if fence:
                    lines.append(line)
                    continue
                m = HEADING.match(line)
                if m:
                    s = slug(m.group(2))
                    n = seen.get(s, 0)
                    seen[s] = n + 1
                    anchor = anchors[(page, s if n == 0 else f"{s}-{n}")]
                    level = min(len(m.group(1)) + 1, 6)
                    lines += [f'<a id="{anchor}"></a>', ""]
                    line = "#" * level + " " + m.group(2)
                line = LINK.sub(lambda k: f"{k.group(1)}[{k.group(2)}]({resolve(page, k.group(3))})", line)
                lines.append(line)
            lines.append("")
    text = "\n".join(lines).rstrip("\n") + "\n"
    if dead:
        print("links that lead nowhere:\n  " + "\n  ".join(dead))
        sys.exit(1)
    if "--check" in sys.argv:
        same = out.exists() and out.read_text(encoding="utf-8").replace("\r\n", "\n") == text
        print("the book is current" if same else "the book is out of date: run tools/book.py")
        sys.exit(0 if same else 1)
    out.write_text(text, encoding="utf-8", newline="\n")
    print(f"{out}: {len(pages)} pages, {len(text.split())} words")


if __name__ == "__main__":
    main()
