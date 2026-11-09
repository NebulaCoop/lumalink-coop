"""make_galaxy.py <example folder>: every file of Lantern Garden, a galaxy of its own, as a level editor, a 3D tool and a
music program could have written them. Python 3 alone, no packages.

- LanternGardenPlanet: a small round planet (15 m across... its radius is 15 m) of grass and flowers with lantern posts,
  its own model (two materials, two textures of its own) and its own solid ground (LanternGardenPlanet.kcl.glb). The
  game makes it a planet from its row in PlanetMapDataTable (merge/), as it does its own.
- Bramblet: a new enemy, a thorny little bush, made like a Goomba (luma.json object_classes): a skeleton named as the
  Goomba's (`lumalink list models --filter objectdata/kuribo.arc`), three materials with their own textures, and the 23
  animations the Goomba's code plays, by their names. Where it plays one once (squashed, hit), the game plays this
  one once too. The Goomba's tables and sounds come from the game.
- The galaxy: StageData/LanternGardenGalaxy.arc (where Mario starts, the planet, its gravity, Goombas, Bramblets,
  coins, a star for each of two missions, a cutscene when Mario arrives) and its missions,
  StageData/LanternGardenGalaxy/LanternGardenGalaxyScenario.arc. Tables as JSON, as `lumalink extract` writes them.
- The cutscene: ObjectData/DemoSheet.arc/demolanterngardenarrival (Mario looks up, then waves, to the galaxy's theme).
  Its camera is ArrivalCamera.canm.glb, a glTF camera animation, kept in the archive of ArrivalLantern, the lantern
  it follows (made like FloaterLandPartsFrame, placed in the cutscene's cast).
- The theme: AudioRes/Stream/LanternGardenTheme.wav, 24 seconds that loop.
Lengths in metres (1 m = 100 of the game's units); +Y up."""
import json
import math
import os
import struct
import sys
import zlib

R_PLANET = 15.0           # the planet's radius, metres
UNITS = 100.0             # the game's units in a metre


# ---- pictures
def png(width, height, pixel):
    rows = b"".join(b"\0" + bytes(c for x in range(width) for c in pixel(x, y)) for y in range(height))
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(rows, 9)) + chunk(b"IEND", b""))


def clamp(v):
    return max(0, min(255, int(v)))


def noise(x, y, seed):
    n = (x * 374761393 + y * 668265263 + seed * 2147483647) & 0xFFFFFFFF
    n = ((n ^ (n >> 13)) * 1274126177) & 0xFFFFFFFF
    return (n ^ (n >> 16)) / 4294967295.0


def grass_pixel(x, y):
    # Grass over the top, flowers in it, soil and stone towards the bottom (v down the sphere).
    v = y / 127.0
    g = noise(x, y, 1)
    if v < 0.62:
        r, gr, b = 70 + 30 * g, 150 + 50 * g, 60 + 20 * g
        f = noise(x // 3, y // 3, 2)
        if f > 0.965:
            r, gr, b = (255, 220, 90) if f > 0.985 else (250, 140, 190)
    elif v < 0.7:
        t = (v - 0.62) / 0.08
        r, gr, b = 70 + 70 * t, 150 - 50 * t, 60 - 10 * t
    else:
        r, gr, b = 120 + 40 * g, 100 + 30 * g, 85 + 25 * g
    return (clamp(r), clamp(gr), clamp(b), 255)


def lantern_pixel(x, y):
    # A paper lantern lit from inside: bright in the middle, ribs across.
    v = y / 31.0
    glow = 1.0 - abs(v - 0.5) * 1.2
    r, g, b = 255, 190 + 60 * glow, 90 + 90 * glow
    if y % 8 == 0:
        r, g, b = 200, 120, 60
    return (clamp(r), clamp(g), clamp(b), 255)


def post_pixel(x, y):
    g = noise(x, y // 4, 3)
    return (clamp(80 + 30 * g), clamp(55 + 20 * g), clamp(40 + 15 * g), 255)


def leaves_pixel(x, y):
    # The Bramblet's body: leaves, darker where they overlap, and its face at the front (u = 0.5).
    u, v = (x + 0.5) / 128.0, (y + 0.5) / 64.0
    g = noise(x // 2, y // 2, 4)
    r, gr, b = 40 + 30 * g, 110 + 60 * g, 50 + 25 * g
    if noise(x // 6, y // 6, 5) > 0.8:
        r, gr, b = r * 0.7, gr * 0.7, b * 0.7
    for side in (-1, 1):
        du, dv = (u - (0.5 + side * 0.06)) / 0.028, (v - 0.42) / 0.075
        if du * du + dv * dv < 1.0:
            r, gr, b = 255, 245, 200
            if (du - side * 0.2) ** 2 + (dv - 0.2) ** 2 < 0.28:
                r, gr, b = 200, 30, 20
        # Brows, slanting in: it is cross.
        bu, bv = (u - (0.5 + side * 0.06)) / 0.04, (v - 0.31 + side * (u - 0.5) * 0.9) / 0.012
        if abs(bu) < 1.0 and abs(bv) < 1.0:
            r, gr, b = 30, 40, 20
    mu, mv = (u - 0.5) / 0.05, (v - 0.6) / 0.012
    if abs(mu) < 1.0 and abs(mv) < 1.0:
        r, gr, b = 30, 40, 20
    return (clamp(r), clamp(gr), clamp(b), 255)


def thorn_pixel(x, y):
    t = y / 15.0
    return (clamp(150 - 60 * t), clamp(90 - 40 * t), clamp(120 - 30 * t), 255)


def bud_pixel(x, y):
    g = noise(x, y, 6)
    return (255, clamp(120 + 60 * g), clamp(170 + 40 * g), 255)


# ---- shapes: positions, normals, coordinates, joints, indices
def sphere(center, radius, joint=0, squash=(1.0, 1.0, 1.0), rings=12, segments=20, v_range=(0.0, 1.0)):
    positions, normals, coords, joints = [], [], [], []
    for ring in range(rings + 1):
        v = ring / rings
        phi = v * math.pi
        for seg in range(segments + 1):
            u = seg / segments
            theta = (u - 0.5) * 2.0 * math.pi
            n = (math.sin(phi) * math.sin(theta), math.cos(phi), math.sin(phi) * math.cos(theta))
            positions.append(tuple(center[k] + n[k] * radius * squash[k] for k in range(3)))
            normals.append(n)
            coords.append((u, v_range[0] + v * (v_range[1] - v_range[0])))
            joints.append(joint)
    indices = []
    for ring in range(rings):
        for seg in range(segments):
            a = ring * (segments + 1) + seg
            b = a + segments + 1
            indices += [a, b, a + 1, a + 1, b, b + 1]
    return positions, normals, coords, joints, indices


def cylinder(radius, height, joint=0, segments=10):
    """Standing on the origin, up +Y, open at both ends."""
    positions, normals, coords, joints, indices = [], [], [], [], []
    for ring in range(2):
        for seg in range(segments + 1):
            a = 2.0 * math.pi * seg / segments
            n = (math.sin(a), 0.0, math.cos(a))
            positions.append((n[0] * radius, height * ring, n[2] * radius))
            normals.append(n)
            coords.append((seg / segments, 1.0 - ring))
            joints.append(joint)
    for seg in range(segments):
        a, b = seg, seg + segments + 1
        indices += [a, a + 1, b, a + 1, b + 1, b]
    return positions, normals, coords, joints, indices


def cone(base, axis, length, radius, joint, segments=8):
    """Its base at `base`, its point `length` along `axis` (a unit vector)."""
    up = (0.0, 1.0, 0.0) if abs(axis[1]) < 0.9 else (1.0, 0.0, 0.0)
    s = cross(axis, up)
    s = scale(s, 1.0 / length_of(s))
    t = cross(s, axis)
    tip = add(base, scale(axis, length))
    positions, normals, coords, joints, indices = [], [], [], [], []
    for seg in range(segments + 1):
        a = 2.0 * math.pi * seg / segments
        d = add(scale(s, math.cos(a)), scale(t, math.sin(a)))
        n = add(d, scale(axis, radius / length))
        n = scale(n, 1.0 / length_of(n))
        positions += [add(base, scale(d, radius)), tip]
        normals += [n, n]
        coords += [(seg / segments, 1.0), (seg / segments, 0.0)]
        joints += [joint, joint]
    for seg in range(segments):
        a = seg * 2
        indices += [a, a + 2, a + 1]
    return positions, normals, coords, joints, indices


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def scale(a, k):
    return (a[0] * k, a[1] * k, a[2] * k)


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def length_of(a):
    return math.sqrt(a[0] * a[0] + a[1] * a[1] + a[2] * a[2])


def stand_on(shape, normal, at):
    """A shape built standing on the origin, up +Y, turned so its up is `normal`, then moved to `at`."""
    positions, normals, coords, joints, indices = shape
    k = cross((0.0, 1.0, 0.0), normal)
    s = length_of(k)
    c = normal[1]
    def turn(p):
        if s < 1e-6:
            return p if c > 0 else (p[0], -p[1], -p[2])
        axis = scale(k, 1.0 / s)
        # Rodrigues: p cos + (axis x p) sin + axis (axis . p)(1 - cos)
        dot = axis[0] * p[0] + axis[1] * p[1] + axis[2] * p[2]
        return add(add(scale(p, c), scale(cross(axis, p), s)), scale(axis, dot * (1.0 - c)))
    return ([add(turn(p), at) for p in positions], [turn(n) for n in normals], coords, joints, indices)


def join(shapes):
    out = [[], [], [], [], []]
    for positions, normals, coords, joints, indices in shapes:
        base = len(out[0])
        out[0] += positions
        out[1] += normals
        out[2] += coords
        out[3] += joints
        out[4] += [base + i for i in indices]
    return out


def on_sphere(theta_deg, phi_deg):
    """The unit direction theta degrees down from the top, turned phi degrees about +Y from +Z."""
    t, p = math.radians(theta_deg), math.radians(phi_deg)
    return (math.sin(t) * math.sin(p), math.cos(t), math.sin(t) * math.cos(p))


# ---- the file
class Bin:
    def __init__(self):
        self.data = bytearray()
        self.views = []
        self.accessors = []

    def view(self, raw, target=None):
        while len(self.data) % 4:
            self.data.append(0)
        v = {"buffer": 0, "byteOffset": len(self.data), "byteLength": len(raw)}
        if target:
            v["target"] = target
        self.data += raw
        self.views.append(v)
        return len(self.views) - 1

    def accessor(self, values, kind, component, fmt, target=None, bounds=False):
        flat = [c for v in values for c in (v if isinstance(v, tuple) else (v,))]
        raw = struct.pack("<%d%s" % (len(flat), fmt), *flat)
        a = {"bufferView": self.view(raw, target), "componentType": component, "count": len(values), "type": kind}
        if bounds:
            width = len(values[0]) if isinstance(values[0], tuple) else 1
            a["min"] = [min(v[k] for v in values) for k in range(width)]
            a["max"] = [max(v[k] for v in values) for k in range(width)]
        self.accessors.append(a)
        return len(self.accessors) - 1


def primitive(b, shapes, material, skinned):
    positions, normals, coords, joints, indices = join(shapes)
    assert len(positions) < 65536
    attributes = {
        "POSITION": b.accessor(positions, "VEC3", 5126, "f", 34962, True),
        "NORMAL": b.accessor(normals, "VEC3", 5126, "f", 34962),
        "TEXCOORD_0": b.accessor(coords, "VEC2", 5126, "f", 34962),
    }
    if skinned:
        attributes["JOINTS_0"] = b.accessor([(j, 0, 0, 0) for j in joints], "VEC4", 5123, "H", 34962)
        attributes["WEIGHTS_0"] = b.accessor([(1.0, 0.0, 0.0, 0.0)] * len(joints), "VEC4", 5126, "f", 34962)
    return {"attributes": attributes, "indices": b.accessor(indices, "SCALAR", 5123, "H", 34963), "material": material}


def glb(document, b):
    document["accessors"] = b.accessors
    document["bufferViews"] = b.views
    document["buffers"] = [{"byteLength": len(b.data)}]
    text = json.dumps(document, separators=(",", ":")).encode()
    text += b" " * (-len(text) % 4)
    data = bytes(b.data) + b"\0" * (-len(b.data) % 4)
    out = b"glTF" + struct.pack("<II", 2, 12 + 8 + len(text) + 8 + len(data))
    return out + struct.pack("<I", len(text)) + b"JSON" + text + struct.pack("<I", len(data)) + b"BIN\0" + data


def textured(b, images):
    """images: [(name, png bytes, roughness)] -> images, textures, materials"""
    out_images, textures, materials = [], [], []
    for i, (name, data, rough) in enumerate(images):
        out_images.append({"name": name, "mimeType": "image/png", "bufferView": b.view(data)})
        textures.append({"source": i, "sampler": 0})
        materials.append({"name": name, "pbrMetallicRoughness": {"baseColorTexture": {"index": i}, "metallicFactor": 0.0, "roughnessFactor": rough}})
    return out_images, textures, materials


SAMPLERS = [{"magFilter": 9729, "minFilter": 9987, "wrapS": 10497, "wrapT": 33071}]

# Where the lanterns stand: degrees down from the top, degrees about +Y.
LANTERNS = [(35, 20), (40, 150), (50, 260), (75, 80), (85, 200), (95, 320), (120, 40), (140, 180)]


def planet_model():
    b = Bin()
    images, textures, materials = textured(b, [
        ("Garden_Grass", png(128, 128, grass_pixel), 0.95),
        ("Garden_Post", png(16, 32, post_pixel), 0.9),
        ("Garden_Lantern", png(16, 32, lantern_pixel), 0.6),
    ])
    ground = [sphere((0.0, 0.0, 0.0), R_PLANET, rings=32, segments=64)]
    posts, lights = [], []
    for theta, phi in LANTERNS:
        n = on_sphere(theta, phi)
        at = scale(n, R_PLANET - 0.05)
        posts.append(stand_on(cylinder(0.08, 1.7), n, at))
        lights.append(stand_on(sphere((0.0, 1.95, 0.0), 0.3, squash=(1.0, 1.25, 1.0), rings=8, segments=12), n, at))
    mesh = {"name": "LanternGardenPlanet", "primitives": [
        primitive(b, ground, 0, False), primitive(b, posts, 1, False), primitive(b, lights, 2, False)]}
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "LanternGardenPlanet", "mesh": 0}],
        "meshes": [mesh], "materials": materials, "textures": textures, "samplers": SAMPLERS, "images": images,
    }
    return glb(document, b)


def planet_ground():
    # Solid ground: the sphere, a little coarser. Material extras are the ground's columns (Floor_code: 0, plain).
    b = Bin()
    positions, normals, coords, joints, indices = sphere((0.0, 0.0, 0.0), R_PLANET, rings=20, segments=40)
    mesh = {"primitives": [{
        "attributes": {"POSITION": b.accessor(positions, "VEC3", 5126, "f", 34962, True)},
        "indices": b.accessor(indices, "SCALAR", 5123, "H", 34963), "material": 0}]}
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0]}], "nodes": [{"name": "Ground", "mesh": 0}], "meshes": [mesh],
        "materials": [{"name": "Grass", "extras": {"Floor_code": 0, "Sound_code": 0}}],
    }
    return glb(document, b)


# ---- Bramblet: the Goomba's joints
BJOINTS = [
    ("Root", -1, (0.0, 0.0, 0.0)),
    ("Center", 0, (0.0, 0.6, 0.0)),
    ("Body", 1, (0.0, 0.0, 0.0)),
    ("Head", 2, (0.0, 0.32, 0.0)),
    ("LFoot", 0, (0.24, 0.1, 0.02)),
    ("RFoot", 0, (-0.24, 0.1, 0.02)),
]
BJ = {name: i for i, (name, _, _) in enumerate(BJOINTS)}


def bworld(i):
    name, parent, at = BJOINTS[i]
    if parent < 0:
        return at
    return add(bworld(parent), at)


def bramblet_parts():
    c = bworld(BJ["Center"])
    leaves = [sphere(c, 0.5, BJ["Body"], (1.0, 0.92, 1.0), rings=14, segments=24)]
    thorns = []
    for k in range(14):
        theta = 25.0 + (k % 5) * 25.0
        phi = k * 360.0 / 14.0 + (k % 2) * 13.0
        if abs(((phi + 180.0) % 360.0) - 180.0) < 40.0 and 40.0 < theta < 110.0:
            continue  # its face stays clear
        n = on_sphere(theta, phi)
        n = (n[0], n[1] * 0.92, n[2])
        n = scale(n, 1.0 / length_of(n))
        thorns.append(cone(add(c, scale(n, 0.42)), n, 0.26, 0.07, BJ["Body"]))
    for i in (BJ["LFoot"], BJ["RFoot"]):
        thorns.append(sphere(bworld(i), 0.15, i, (1.0, 0.65, 1.35), rings=8, segments=12))
    h = bworld(BJ["Head"])
    bud = [sphere(add(h, (0.0, 0.16, 0.0)), 0.12, BJ["Head"], (1.0, 1.2, 1.0), rings=8, segments=12)]
    return leaves, thorns, bud


def quat(axis, degrees):
    half = math.radians(degrees) / 2.0
    s = math.sin(half)
    return (axis[0] * s, axis[1] * s, axis[2] * s, math.cos(half))


def wave(length, keys, f):
    return [(length * k / keys, f(2.0 * math.pi * k / keys)) for k in range(keys + 1)]


def ramp(length, keys, f):
    """f of 0..1 over the length."""
    return [(length * k / keys, f(k / keys)) for k in range(keys + 1)]


def bramblet_animations():
    rest = {name: at for name, _, at in BJOINTS}
    X, Y, Z = (1, 0, 0), (0, 1, 0), (0, 0, 1)
    cy = rest["Center"][1]
    def center_y(length, keys, f):
        return ("Center", "translation", ramp(length, keys, lambda t: (0.0, cy + f(t), 0.0)))
    def squash(length, keys, f):
        return ("Center", "scale", ramp(length, keys, lambda t: (1.0 + (1.0 - f(t)) * 0.5, f(t), 1.0 + (1.0 - f(t)) * 0.5)))
    def turn(joint, axis, length, keys, f):
        return (joint, "rotation", ramp(length, keys, lambda t: quat(axis, f(t))))
    def step(joint, length, side, keys=16, lift=0.07, reach=0.1):
        r = rest[joint]
        return (joint, "translation", wave(length, keys, lambda a: (r[0], r[1] + max(0.0, lift * math.sin(a * side)), r[2] + reach * math.sin(a) * side)))
    def bob(height, length, keys=16):
        return ("Center", "translation", wave(length, keys, lambda a: (0.0, cy + height * abs(math.sin(a)), 0.0)))
    flat = lambda t: 1.0 - 0.7 * math.sin(min(t, 0.5) * math.pi)          # down to 0.3 by the middle, stays
    flatter = lambda t: 1.0 - 0.82 * math.sin(min(t, 0.5) * math.pi)
    up_again = lambda t: 0.3 + 0.7 * t
    return {
        # Loops
        "Wait": [bob(0.03, 1.2), turn("Head", Z, 1.2, 16, lambda t: 8.0 * math.sin(2.0 * math.pi * t))],
        "Walk": [bob(0.06, 0.8), step("LFoot", 0.8, 1), step("RFoot", 0.8, -1), turn("Body", Z, 0.8, 16, lambda t: 6.0 * math.sin(2.0 * math.pi * t))],
        "Run": [bob(0.1, 0.45), step("LFoot", 0.45, 1, 12, 0.1, 0.16), step("RFoot", 0.45, -1, 12, 0.1, 0.16),
                turn("Body", X, 0.45, 12, lambda t: 12.0)],
        "Stagger": [turn("Body", Z, 0.6, 16, lambda t: 18.0 * math.sin(2.0 * math.pi * t))],
        "SwoonWait": [turn("Head", Y, 1.0, 16, lambda t: 360.0 * t), turn("Body", Z, 1.0, 16, lambda t: 10.0 * math.sin(2.0 * math.pi * t))],
        "ChangeLoop": [bob(0.02, 0.6)],
        "ChangeReverseLoop": [bob(0.02, 0.6)],
        # Once: turning into another thing and back, as a Goomba does in a box
        "ChangeStart": [squash(0.3, 8, lambda t: 1.0 - 0.2 * math.sin(math.pi * t))],
        "Change": [turn("Center", Y, 0.8, 16, lambda t: 360.0 * t), squash(0.8, 16, lambda t: 1.0 - 0.15 * math.sin(math.pi * t))],
        "ChangeReverseStart": [squash(0.3, 8, lambda t: 1.0 - 0.2 * math.sin(math.pi * t))],
        # Once
        "Turn": [turn("Root", Y, 0.5, 10, lambda t: 25.0 * math.sin(math.pi * t)), bob(0.05, 0.5)],
        "Hit": [squash(0.4, 10, lambda t: 1.0 - 0.35 * math.sin(math.pi * t)), turn("Body", X, 0.4, 10, lambda t: -20.0 * math.sin(math.pi * t))],
        "FlatDown": [squash(0.5, 12, flat)],
        "FlatDownReverse": [squash(0.5, 12, up_again)],
        "HipDropDown": [squash(0.6, 12, flatter)],
        "HipDropDownReverse": [squash(0.6, 12, lambda t: 0.18 + 0.82 * t)],
        "Blow": [turn("Center", X, 0.8, 16, lambda t: -720.0 * t), center_y(0.8, 16, lambda t: 0.6 * math.sin(math.pi * t))],
        "PunchDamage": [turn("Body", X, 0.5, 10, lambda t: -40.0 * math.sin(math.pi * t))],
        "JumpStart": [squash(0.2, 6, lambda t: 1.0 - 0.25 * math.sin(math.pi * t))],
        "Jump": [squash(0.5, 10, lambda t: 1.0 + 0.15 * math.sin(math.pi * t)), step("LFoot", 0.5, 1, 10), step("RFoot", 0.5, -1, 10)],
        "Land": [squash(0.25, 8, lambda t: 1.0 - 0.3 * math.sin(math.pi * t))],
        "StaggerStart": [turn("Body", Z, 0.3, 8, lambda t: 18.0 * math.sin(0.5 * math.pi * t))],
        "StaggerEnd": [turn("Body", Z, 0.5, 10, lambda t: 18.0 * (1.0 - t))],
        "Recover": [center_y(0.6, 12, lambda t: 0.2 * math.sin(math.pi * t)), turn("Head", Z, 0.6, 12, lambda t: 20.0 * math.sin(2.0 * math.pi * t))],
        "SwoonAirStart": [turn("Center", X, 0.4, 10, lambda t: -180.0 * t)],
        "SwoonGround": [squash(0.4, 10, lambda t: 1.0 - 0.2 * math.sin(math.pi * t))],
        "SwoonLand": [squash(0.3, 8, lambda t: 1.0 - 0.3 * math.sin(math.pi * t))],
        "AppearFromBox": [("Center", "scale", ramp(0.6, 12, lambda t: (min(1.0, 0.1 + 1.2 * t),) * 3)),
                          center_y(0.6, 12, lambda t: 0.4 * math.sin(math.pi * t))],
    }


def bramblet_model():
    b = Bin()
    leaves, thorns, bud = bramblet_parts()
    nodes = []
    for i, (name, parent, at) in enumerate(BJOINTS):
        node = {"name": name, "translation": list(at)}
        children = [k for k, (_, p, _) in enumerate(BJOINTS) if p == i]
        if children:
            node["children"] = children
        nodes.append(node)
    inverse = []
    for i in range(len(BJOINTS)):
        w = bworld(i)
        inverse.append((1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, -w[0], -w[1], -w[2], 1))
    skin = {"name": "BrambletSkin", "joints": list(range(len(BJOINTS))), "skeleton": 0,
            "inverseBindMatrices": b.accessor(inverse, "MAT4", 5126, "f")}
    images, textures, materials = textured(b, [
        ("Bramblet_Leaves", png(128, 64, leaves_pixel), 0.85),
        ("Bramblet_Thorn", png(8, 16, thorn_pixel), 0.7),
        ("Bramblet_Bud", png(16, 16, bud_pixel), 0.6),
    ])
    mesh = {"name": "Bramblet", "primitives": [primitive(b, leaves, 0, True), primitive(b, thorns, 1, True), primitive(b, bud, 2, True)]}
    nodes.append({"name": "Bramblet", "mesh": 0, "skin": 0})
    anims = []
    for name, channels in bramblet_animations().items():
        samplers, targets = [], []
        for joint, path, keys in channels:
            times = [t for t, _ in keys]
            values = [v for _, v in keys]
            width = "VEC4" if path == "rotation" else "VEC3"
            sampler = {"input": b.accessor(times, "SCALAR", 5126, "f"), "output": b.accessor(values, width, 5126, "f"), "interpolation": "LINEAR"}
            b.accessors[sampler["input"]]["min"] = [min(times)]
            b.accessors[sampler["input"]]["max"] = [max(times)]
            samplers.append(sampler)
            targets.append({"sampler": len(samplers) - 1, "target": {"node": BJ[joint], "path": path}})
        anims.append({"name": name, "samplers": samplers, "channels": targets})
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0, len(nodes) - 1]}],
        "nodes": nodes, "meshes": [mesh], "skins": [skin],
        "materials": materials, "textures": textures, "samplers": SAMPLERS, "images": images, "animations": anims,
    }
    return glb(document, b), len(anims)


# ---- ArrivalLantern: a tall lantern by where Mario lands, made like FloaterLandPartsFrame (a plain solid object). The
# arrival cutscene's camera is in its archive, as the game keeps a cutscene's camera with the object it follows.
LANTERN_AT = 3.0          # metres from Mario's start, the way he faces (+Z)


def arrival_lantern_model():
    b = Bin()
    images, textures, materials = textured(b, [
        ("Arrival_Post", png(16, 32, post_pixel), 0.9),
        ("Arrival_Lantern", png(16, 32, lantern_pixel), 0.6),
    ])
    post = [cylinder(0.12, 2.6, segments=12)]
    light = [sphere((0.0, 2.95, 0.0), 0.45, squash=(1.0, 1.25, 1.0), rings=10, segments=16)]
    mesh = {"name": "ArrivalLantern", "primitives": [primitive(b, post, 0, False), primitive(b, light, 1, False)]}
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0]}], "nodes": [{"name": "Model", "mesh": 0}],  # FloaterLandPartsFrame's joint
        "meshes": [mesh], "materials": materials, "textures": textures, "samplers": SAMPLERS, "images": images,
    }
    return glb(document, b)


def arrival_lantern_ground():
    b = Bin()
    positions, normals, coords, joints, indices = cylinder(0.15, 2.6, segments=12)
    mesh = {"primitives": [{
        "attributes": {"POSITION": b.accessor(positions, "VEC3", 5126, "f", 34962, True)},
        "indices": b.accessor(indices, "SCALAR", 5123, "H", 34963), "material": 0}]}
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0]}], "nodes": [{"name": "Post", "mesh": 0}], "meshes": [mesh],
        "materials": [{"name": "Wood", "extras": {"Floor_code": 0, "Sound_code": 0}}],
    }
    return glb(document, b)


def look_at(eye, target):
    """The turn (a quaternion) of a glTF camera at `eye` looking at `target`, its up as near +Y as it can be."""
    f = add(target, scale(eye, -1.0))
    f = scale(f, 1.0 / length_of(f))
    back = scale(f, -1.0)
    x = cross((0.0, 1.0, 0.0), back)
    x = scale(x, 1.0 / length_of(x))
    y = cross(back, x)
    m = [[x[0], y[0], back[0]], [x[1], y[1], back[1]], [x[2], y[2], back[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0.0:
        s = math.sqrt(trace + 1.0) * 2.0
        return ((m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s)
    i = max(range(3), key=lambda k: m[k][k])
    j, k = (i + 1) % 3, (i + 2) % 3
    s = math.sqrt(1.0 + m[i][i] - m[j][j] - m[k][k]) * 2.0
    q = [0.0, 0.0, 0.0, (m[k][j] - m[j][k]) / s]
    q[i] = 0.25 * s
    q[j] = (m[j][i] + m[i][j]) / s
    q[k] = (m[k][i] + m[i][k]) / s
    return tuple(q)


def arrival_camera():
    """The cutscene's camera, ArrivalCamera.canm.glb: a glTF camera and its animation, relative to the lantern (its
    origin, +Y up). Over the "Look" part (150 steps, 2.5 s) it starts in front of Mario's face as he looks up, then
    circles round him, out and up, to show him, the lantern and the sky."""
    mario = (0.0, 0.0, -LANTERN_AT)
    head, lamp = add(mario, (0.0, 0.9, 0.0)), (0.0, 2.9, 0.0)
    keys = 24
    length = 150 / 60.0
    times, eyes, turns = [], [], []
    for k in range(keys + 1):
        t = k / keys
        s = t * t * (3.0 - 2.0 * t)
        # Round Mario from his front (+Z) by his left to behind him, moving out and up; watching his face, then the lamp.
        angle = math.radians(200.0 * s)
        r, h = 2.0 + 2.5 * s, 1.0 + 2.2 * s
        eye = add(mario, (r * math.sin(angle), h, r * math.cos(angle)))
        at = tuple(head[i] + (lamp[i] - head[i]) * s for i in range(3))
        # Keep each turn on the same side as the last, so the camera turns the short way between keys.
        q = look_at(eye, at)
        if turns and sum(a * b for a, b in zip(q, turns[-1])) < 0.0:
            q = tuple(-v for v in q)
        times.append(length * t)
        eyes.append(eye)
        turns.append(q)
    b = Bin()
    t_in = b.accessor(times, "SCALAR", 5126, "f")
    b.accessors[t_in]["min"], b.accessors[t_in]["max"] = [0.0], [length]
    samplers = [{"input": t_in, "output": b.accessor(eyes, "VEC3", 5126, "f"), "interpolation": "LINEAR"},
                {"input": t_in, "output": b.accessor(turns, "VEC4", 5126, "f"), "interpolation": "LINEAR"}]
    watch = 2.0
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_galaxy.py"},
        "scene": 0, "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "ArrivalCamera", "camera": 0, "translation": list(eyes[0]), "rotation": list(turns[0]),
                   "extras": {"nebula": {"watch_distance": round(watch, 3)}}}],
        "cameras": [{"type": "perspective", "perspective": {"yfov": math.radians(45.0), "znear": 0.1, "zfar": 1000.0}}],
        "animations": [{"name": "Arrival", "samplers": samplers, "channels": [
            {"sampler": 0, "target": {"node": 0, "path": "translation"}},
            {"sampler": 1, "target": {"node": 0, "path": "rotation"}}]}],
    }
    return glb(document, b)


# ---- the galaxy's tables
FLOATS =["pos_x", "pos_y", "pos_z", "dir_x", "dir_y", "dir_z", "scale_x", "scale_y", "scale_z"]


def table(rows, floats=()):
    return {"columns": {c: "float" for c in list(FLOATS) + list(floats)}, "rows": rows}


def placed(name, l_id, at, direction=(0.0, 0.0, 0.0), **args):
    row = {
        "name": name, "l_id": l_id,
        "pos_x": round(at[0], 2), "pos_y": round(at[1], 2), "pos_z": round(at[2], 2),
        "dir_x": direction[0], "dir_y": direction[1], "dir_z": direction[2],
        "scale_x": 1.0, "scale_y": 1.0, "scale_z": 1.0,
        "SW_APPEAR": -1, "SW_DEAD": -1, "SW_A": -1, "SW_B": -1, "SW_SLEEP": -1,
        "CommonPath_ID": -1, "ClippingGroupId": -1, "GroupId": -1, "DemoGroupId": -1, "MapParts_ID": -1,
        "ViewGroupId": -1, "CameraSetId": -1, "CastId": -1, "MessageId": -1, "ShapeModelNo": -1,
    }
    for k in range(8):
        row[f"Obj_arg{k}"] = -1
    row.update(args)
    return row


def on_surface_z(angle, height):
    """On the planet in the XY plane, `angle` degrees from the top towards -X, standing: turned about Z."""
    a = math.radians(angle)
    r = R_PLANET * UNITS + height
    return (-math.sin(a) * r, math.cos(a) * r, 0.0), (0.0, 0.0, float(angle))


def on_surface_x(angle, height):
    """On the planet in the YZ plane, `angle` degrees from the top towards +Z: turned about X."""
    a = math.radians(angle)
    r = R_PLANET * UNITS + height
    return (0.0, math.cos(a) * r, math.sin(a) * r), (float(angle), 0.0, 0.0)


def galaxy_tables():
    out = {}
    stage = "files/StageData/LanternGardenGalaxy.arc/jmp/"
    top = R_PLANET * UNITS
    out[stage + "start/common/startinfo.json"] = table([
        {"name": "Mario", "MarioNo": 0, "Camera_id": -1, "Obj_arg0": -1, "pos_x": 0.0, "pos_y": top + 50.0, "pos_z": 0.0,
         "dir_x": 0.0, "dir_y": 0.0, "dir_z": 0.0, "scale_x": 1.0, "scale_y": 1.0, "scale_z": 1.0}])
    out[stage + "list/stageinfo.json"] = {"rows": [{"Material": "Normal", "0xe4ef3738": "Normal"}]}
    out[stage + "placement/common/planetobjinfo.json"] = table([
        dict(placed("GlobalPointGravity", 1, (0.0, 0.0, 0.0)), Range=5000.0, Distant=0.0, Priority=15, Inverse=-1,
             Power="Normal", Gravity_type="Normal", FollowId=-1, ChildObjId=-1, Obj_ID=-1)], floats=("Range", "Distant"))
    common = [placed("LanternGardenPlanet", 2, (0.0, 0.0, 0.0))]
    l_id = 10
    for angle in (-40.0, -105.0):
        at, d = on_surface_z(angle, 10.0)
        common.append(placed("Kuribo", l_id, at, d))
        l_id += 1
    for angle in (55.0, -60.0, 125.0):
        at, d = on_surface_x(angle, 10.0)
        common.append(placed("Bramblet", l_id, at, d))
        l_id += 1
    for angle in (30.0, 50.0, 70.0, 90.0, 110.0, 130.0):
        at, d = on_surface_z(angle, 120.0)
        common.append(placed("Coin", l_id, at, d))
        l_id += 1
    # The lantern is one of the cutscene's cast (DemoGroupId: the DemoGroup's l_id), so the cutscene can use its camera.
    common.append(placed("ArrivalLantern", 30, (0.0, top, LANTERN_AT * UNITS), DemoGroupId=300))
    out[stage + "placement/common/objinfo.json"] = table(common)
    star1, d1 = on_surface_z(180.0, 220.0)
    out[stage + "placement/layera/objinfo.json"] = table([placed("PowerStar", 100, star1, d1, Obj_arg0=1)])
    star2, d2 = on_surface_x(95.0, 220.0)
    out[stage + "placement/layerb/objinfo.json"] = table([
        placed("PowerStar", 101, star2, d2, Obj_arg0=2),
        placed("Bramblet", 102, *on_surface_x(80.0, 10.0)),
        placed("Bramblet", 103, *on_surface_x(110.0, 10.0)),
    ])
    # The cutscene: Mario starts inside a switch cube, which turns on switch 120; the cutscene appears on it.
    cube = placed("SwitchCube", 200, (0.0, top + 50.0, 0.0), SW_A=120)
    cube.update({"scale_x": 3.0, "scale_y": 3.0, "scale_z": 3.0, "Obj_ID": -1})
    for k in ("ViewGroupId", "CameraSetId", "CastId", "MessageId", "ShapeModelNo", "SW_DEAD"):
        del cube[k]
    out[stage + "placement/common/areaobjinfo.json"] = table([cube])
    out[stage + "placement/common/demoobjinfo.json"] = table([{
        "name": "DemoGroup", "DemoName": "LanternGardenArrival", "TimeSheetName": "LanternGardenArrival", "l_id": 300,
        "SW_APPEAR": 120, "SW_DEAD": -1, "SW_A": -1, "SW_B": -1,
        "pos_x": 0.0, "pos_y": top, "pos_z": 0.0, "dir_x": 0.0, "dir_y": 0.0, "dir_z": 0.0,
        "scale_x": 1.0, "scale_y": 1.0, "scale_z": 1.0}])

    scenario = "files/StageData/LanternGardenGalaxy/LanternGardenGalaxyScenario.arc/"
    # The game's columns, as its own galaxies have them: those the game names by number stay 0.
    unnamed = ["0x0a3b36a3", "0x101ef11e", "0x197bdc57", "0x31da54ec", "0x3b43303f", "0x40026d8a", "0x432413b8",
               "0x5529c9dc", "0x72261740", "0x78e9b3f2", "0xa89b5d9e", "0xbb66da6d", "0xcabd1f4c", "0xe4b7f595",
               "0xe820a289", "0xee1b82bd", "0xfdad4876"]
    missions = []
    for no, layers in ((1, 1), (2, 2)):
        row = {"ScenarioNo": no, "PowerStarId": no, "LanternGardenGalaxy": layers, "ErrorCheck": 1, "IsHidden": 0,
               "Comet": "", "AppearPowerStarObj": "", "LuigiModeTimer": 0, "0xcdb16e5b": ""}
        row.update({c: 0 for c in unnamed})
        missions.append(row)
    out[scenario + "ScenarioData.bcsv.json"] = {"rows": missions}
    out[scenario + "ZoneList.bcsv.json"] = {"rows": [{"ZoneName": "LanternGardenGalaxy"}]}

    # The planet: a row as the game's own planets have, no other models (low, bloom, water) of its own.
    row = {"PlanetName": "LanternGardenPlanet", "LowFlag": 0, "MiddleFlag": 0, "BloomFlag": 0, "WaterFlag": 0, "IndirectFlag": 0}
    row.update({f"ForceLowScenarioName{k}": "" for k in range(8)})
    out["merge/ObjectData/PlanetMapDataTable.arc/PlanetMapDataTable.bcsv.json"] = {"rows": [row]}

    sheet = "files/ObjectData/DemoSheet.arc/demolanterngardenarrival/demolanterngardenarrival"
    out[sheet + "player.bcsv.json"] = {"rows": [
        {"PartName": "Look", "BckName": "watchup", "PosName": ""},
        {"PartName": "Wave", "BckName": "demomeettico", "PosName": ""}]}
    out[sheet + "sound.bcsv.json"] = {"rows": [
        {"PartName": "Look", "Bgm": "LanternGardenTheme", "SystemSe": "", "ReturnBgm": 0, "BgmWipeoutFrame": -1},
        {"PartName": "Wave", "Bgm": "", "SystemSe": "", "ReturnBgm": 1, "BgmWipeoutFrame": -1}]}
    out[sheet + "time.bcsv.json"] = {"rows": [
        {"PartName": "Look", "TotalStep": 150, "SuspendFlag": 0},
        {"PartName": "Wave", "TotalStep": 150, "SuspendFlag": 0}]}
    # "Look" plays ArrivalCamera from the lantern's archive; "Wave" has the game's own camera.
    out[sheet + "camera.bcsv.json"] = {"rows": [
        {"PartName": "Look", "CameraTargetName": "ArrivalLantern", "CameraTargetCastID": -1,
         "AnimCameraName": "ArrivalCamera.canm", "AnimCameraStartFrame": 0, "AnimCameraEndFrame": -1, "IsContinuous": 0}]}
    return out


# ---- the theme: a lullaby in C, plucked over a soft pad, 96 beats a minute, 8 bars that loop
def theme(rate=32000):
    bpm = 96.0
    beat = 60.0 / bpm
    chords = [(60, 64, 67), (57, 60, 64), (53, 57, 60), (55, 59, 62), (60, 64, 67), (57, 60, 64), (53, 57, 60), (55, 59, 62, 65)]
    melody = [76, 74, 72, 74, 76, 76, 76, None, 74, 74, 74, None, 76, 79, 79, None,
              76, 74, 72, 74, 76, 76, 76, 76, 74, 74, 76, 74, 72, None, None, None]
    length = int(rate * beat * 4 * len(chords))
    out = [0.0] * length
    def hz(note):
        return 440.0 * 2.0 ** ((note - 69) / 12.0)
    def pluck(start, note, dur, amp):
        f = hz(note)
        n0 = int(start * rate)
        for i in range(int(dur * rate)):
            if n0 + i >= length:
                break
            t = i / rate
            env = math.exp(-t * 4.0) * min(1.0, t * 200.0)
            out[n0 + i] += amp * env * (math.sin(2 * math.pi * f * t) + 0.3 * math.sin(4 * math.pi * f * t))
    def pad(start, notes, dur, amp):
        n0 = int(start * rate)
        for i in range(int(dur * rate)):
            if n0 + i >= length:
                break
            t = i / rate
            env = min(1.0, t / 0.4) * min(1.0, (dur - t) / 0.4)
            out[n0 + i] += amp * env * sum(math.sin(2 * math.pi * hz(n - 12) * t) for n in notes) / len(notes)
    for bar, chord in enumerate(chords):
        start = bar * beat * 4
        pad(start, chord, beat * 4, 0.16)
        for k in range(8):
            pluck(start + k * beat / 2, chord[k % len(chord)] + (12 if k % 4 == 3 else 0), beat * 1.5, 0.12)
    for k, note in enumerate(melody):
        if note is not None:
            pluck(k * beat, note, beat * 2.0, 0.2)
    peak = max(abs(s) for s in out) or 1.0
    pcm = struct.pack("<%dh" % length, *(int(s / peak * 0.8 * 32767) for s in out))
    header = b"RIFF" + struct.pack("<I", 36 + len(pcm)) + b"WAVE" + b"fmt " + struct.pack("<IHHIIHH", 16, 1, 1, rate, rate * 2, 2, 16)
    return header + b"data" + struct.pack("<I", len(pcm)) + pcm


def write(folder, path, data):
    full = os.path.join(folder, "layers", "base", *path.split("/"))
    os.makedirs(os.path.dirname(full), exist_ok=True)
    if isinstance(data, (dict, list)):
        data = (json.dumps(data, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    with open(full, "wb") as f:
        f.write(data)
    return len(data)


def main():
    folder = sys.argv[1]
    sizes = {}
    sizes["planet"] = write(folder, "files/ObjectData/LanternGardenPlanet.arc/LanternGardenPlanet.bdl.glb", planet_model())
    sizes["ground"] = write(folder, "files/ObjectData/LanternGardenPlanet.arc/LanternGardenPlanet.kcl.glb", planet_ground())
    model, animations = bramblet_model()
    sizes["bramblet"] = write(folder, "files/ObjectData/Bramblet.arc/Bramblet.bdl.glb", model)
    write(folder, "files/ObjectData/ArrivalLantern.arc/ArrivalLantern.bdl.glb", arrival_lantern_model())
    write(folder, "files/ObjectData/ArrivalLantern.arc/ArrivalLantern.kcl.glb", arrival_lantern_ground())
    write(folder, "files/ObjectData/ArrivalLantern.arc/ArrivalCamera.canm.glb", arrival_camera())
    for path, data in galaxy_tables().items():
        write(folder, path, data)
    sizes["theme"] = write(folder, "files/AudioRes/Stream/LanternGardenTheme.wav", theme())
    print(f"written: planet {sizes['planet']} bytes, its ground {sizes['ground']}, Bramblet {sizes['bramblet']} "
          f"({len(BJOINTS)} joints, 3 materials, {animations} animations), the galaxy's tables, the theme {sizes['theme']}")


if __name__ == "__main__":
    main()
