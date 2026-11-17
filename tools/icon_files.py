"""icon_files.py: LumaLink's icon as files, for the README and web pages: assets/icon/lumalink.svg and
assets/icon/lumalink-<size>.png (256, 512, 1024).

The icon is drawn in code by crates/lumalink-gui/icon.rs (the window's and the programs' icon). This script draws the same
shapes: a rounded square (inset 0.04, corner radius 0.22) shaded from sky blue at the top to deep blue at the bottom, and
two white four-point sparkles (|dx/r|^0.5 + |dy/r|^0.5 <= 1) centred at (0.46, 0.55) with r 0.33 and at (0.74, 0.26) with
r 0.13. The PNGs are sampled as icon.rs samples them (4 x 4 samples a pixel, premultiplied). Change both files together.

Run from the repository root: python tools/icon_files.py. Python 3, standard library only.
"""
import math
import pathlib
import struct
import zlib

INSET, RADIUS = 0.04, 0.22
TOP, BOTTOM = (52, 190, 240), (20, 126, 186)
SPARKLES = [(0.46, 0.55, 0.33), (0.74, 0.26, 0.13)]
OUT = pathlib.Path("assets/icon")


def sample(x, y):
    half = 0.5 - INSET
    qx = max(abs(x - 0.5) - (half - RADIUS), 0.0)
    qy = max(abs(y - 0.5) - (half - RADIUS), 0.0)
    if math.sqrt(qx * qx + qy * qy) > RADIUS:
        return (0.0, 0.0, 0.0, 0.0)
    for cx, cy, r in SPARKLES:
        if math.sqrt(abs(x - cx) / r) + math.sqrt(abs(y - cy) / r) <= 1.0:
            return (255.0, 255.0, 255.0, 255.0)
    t = min(max((y - INSET) / (1.0 - 2.0 * INSET), 0.0), 1.0)
    return tuple(TOP[i] + (BOTTOM[i] - TOP[i]) * t for i in range(3)) + (255.0,)


def png(size):
    n = 4
    rows = bytearray()
    for py in range(size):
        rows.append(0)
        for px in range(size):
            total = [0.0, 0.0, 0.0, 0.0]
            for sy in range(n):
                for sx in range(n):
                    c = sample((px + (sx + 0.5) / n) / size, (py + (sy + 0.5) / n) / size)
                    for i in range(3):
                        total[i] += c[i] * c[3] / 255.0
                    total[3] += c[3]
            alpha = total[3] / (n * n)
            for i in range(3):
                value = total[i] / (n * n) * 255.0 / alpha if alpha > 0 else 0.0
                rows.append(int(min(max(round(value), 0), 255)))
            rows.append(int(min(max(round(alpha), 0), 255)))

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))

    header = struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(bytes(rows), 9)) + chunk(b"IEND", b"")


def svg():
    s = 256

    def f(v):
        return ("%.2f" % (v * s)).rstrip("0").rstrip(".")

    paths = []
    for cx, cy, r in SPARKLES:
        points = []
        for k in range(720):
            t = 2 * math.pi * k / 720
            c, d = math.cos(t), math.sin(t)
            points.append("%s %s" % (f(cx + r * math.copysign(c ** 4, c)), f(cy + r * math.copysign(d ** 4, d))))
        paths.append('  <path fill="#fff" d="M' + " L".join(points) + 'Z"/>')
    return "\n".join([
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="256" height="256">',
        '  <title>LumaLink</title>',
        '  <defs>',
        '    <linearGradient id="sky" gradientUnits="userSpaceOnUse" x1="0" y1="%s" x2="0" y2="%s">' % (f(INSET), f(1 - INSET)),
        '      <stop offset="0" stop-color="#%02x%02x%02x"/>' % TOP,
        '      <stop offset="1" stop-color="#%02x%02x%02x"/>' % BOTTOM,
        '    </linearGradient>',
        '  </defs>',
        '  <rect x="%s" y="%s" width="%s" height="%s" rx="%s" fill="url(#sky)"/>' % (
            f(INSET), f(INSET), f(1 - 2 * INSET), f(1 - 2 * INSET), f(RADIUS)),
    ] + paths + ["</svg>", ""])


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "lumalink.svg").write_text(svg(), encoding="utf-8", newline="\n")
    for size in (256, 512, 1024):
        (OUT / ("lumalink-%d.png" % size)).write_bytes(png(size))
    print("wrote", ", ".join(sorted(p.name for p in OUT.iterdir())))
