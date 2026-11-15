# SPDX-License-Identifier: GPL-3.0-only
"""The license texts of the Rust crates statically linked into LumaLink's programs.

    python installer/rust_licenses.py <target triple> <output> <package>...

Walks the packages' normal (not build or dev) dependencies for that target from `cargo metadata`, leaving out
procedural macros (they run in the compiler) and the workspace's own crates, and writes each crate's name, version,
license expression, authors, repository and license files (LICENSE*, COPYING*, NOTICE*, UNLICENSE*, and the license
texts of fonts a crate carries). A crate whose package has no license file (its repository keeps it at the top) is
listed with its authors and expression, and the text of each license such crates name is written once at the end.
installer/build-setup.ps1 and the release workflow write it into each package's licenses folder:

    python installer/rust_licenses.py x86_64-pc-windows-msvc licenses/rust-crates.txt lumalink lumalink-gui lumalink-setup
"""
import json
import pathlib
import re
import subprocess
import sys

target, output, *wanted = sys.argv[1:] if len(sys.argv) > 3 else sys.exit(__doc__)
root = pathlib.Path(__file__).resolve().parent.parent
metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--format-version", "1", "--filter-platform", target, "--locked"], cwd=root))
packages = {p["id"]: p for p in metadata["packages"]}
nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
local = {p["id"] for p in metadata["packages"] if p["source"] is None}

linked = set()
queue = []
for name in wanted:
    found = [p["id"] for p in metadata["packages"] if p["name"] == name and p["id"] in local]
    if not found:
        sys.exit(f"rust_licenses.py: no package {name} in the workspace")
    queue.extend(found)
while queue:
    current = queue.pop()
    for dep in nodes[current]["deps"]:
        if not any(kind["kind"] is None for kind in dep["dep_kinds"]):
            continue  # build scripts and tests only
        if any("proc-macro" in t["kind"] for t in packages[dep["pkg"]]["targets"]):
            continue  # runs in the compiler, not linked
        if dep["pkg"] not in linked:
            linked.add(dep["pkg"])
            queue.append(dep["pkg"])

crates = sorted((packages[i] for i in linked if i not in local), key=lambda p: (p["name"], p["version"]))
prefixes = ("license", "copying", "notice", "unlicense")


def license_files(directory):
    """The crate's license files: at its top, and the texts beside fonts it embeds (fonts/*.txt)."""
    top = [f for f in directory.iterdir() if f.is_file() and f.name.lower().startswith(prefixes)]
    fonts = directory / "fonts"
    if fonts.is_dir():
        top += [f for f in fonts.iterdir() if f.is_file() and f.suffix.lower() == ".txt"]
    return sorted(top)


# Where a license's full text is found among the crates' own files, by a line it starts with.
MARKERS = {
    "Apache-2.0": "Apache License",
    "BSL-1.0": "Boost Software License - Version 1.0",
    "CC0-1.0": "CC0 1.0 Universal",
    "ISC": "ISC License",
    "Zlib": "zlib License",
}
MIT = """MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."""

lines = ["Licenses of the Rust crates statically linked into LumaLink's programs",
         "(written by installer/rust_licenses.py from Cargo.lock and the crate sources; C libraries vendored by",
         " -sys crates are included in those crates' files)", ""]
named = set()  # licenses of crates with no file of their own: their texts go at the end
texts = {}     # a license's text, found in some crate's files
for crate in crates:
    directory = pathlib.Path(crate["manifest_path"]).parent
    expression = crate.get("license") or ""
    lines.append("=" * 78)
    lines.append(f"{crate['name']} {crate['version']} ({expression or 'see its files'})")
    if crate.get("repository"):
        lines.append(crate["repository"])
    files = license_files(directory)
    for file in files:
        text = file.read_text(encoding="utf-8", errors="replace").rstrip()
        for spdx, marker in MARKERS.items():
            if spdx not in texts and text.lstrip().startswith(marker):
                texts[spdx] = text
        lines.append(f"------------------------------ {file.name}")
        lines.append(text)
    if not files:
        ids = sorted(set(re.findall(r"[A-Za-z0-9.+-]+", expression)) - {"OR", "AND", "WITH"})
        used = expression
        # A choice of licenses ("MIT OR Apache-2.0"): used under one, MIT or Apache-2.0 when offered.
        if " OR " in expression and " AND " not in expression and " WITH " not in expression:
            choice = next((i for i in ("MIT", "Apache-2.0") if i in ids), ids[0])
            ids, used = [choice], f"{choice} (of {expression})"
        named.update(ids)
        if crate.get("authors"):
            lines.append("Copyright: " + ", ".join(crate["authors"]))
        lines.append(f"Used under {used}. The package carries no license file; the license text is at the end of this file.")
    lines.append("")

lines += ["=" * 78, "License texts named above", ""]
for spdx in sorted(named):
    lines.append("------------------------------ " + spdx)
    if spdx == "MIT":
        lines.append(MIT)
    elif spdx == "CC0-1.0" and spdx not in texts:
        lines.append("CC0 1.0 Universal (a public domain dedication; it asks for no notice): "
                     "https://creativecommons.org/publicdomain/zero/1.0/legalcode")
    elif spdx in texts:
        lines.append(texts[spdx])
    else:
        print(f"rust_licenses.py: no text of {spdx} among the crates' files", file=sys.stderr)
        lines.append(f"https://spdx.org/licenses/{spdx}.html")
    lines.append("")

out = pathlib.Path(output)
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
print(f"{len(crates)} crates -> {output}")
