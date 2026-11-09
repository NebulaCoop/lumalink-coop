"""make_lumie.py <out.glb>: Lumie, a small lantern spirit, as a glTF 2.0 binary any 3D tool could have written. Python
3 alone, no packages.

- A skeleton named as the game's Toad's (`lumalink list models --filter objectdata/kinopio.arc`): JointRoot, body, head,
  ShoulderL/ArmL, ShoulderR/ArmR, FootL, FootR. Lumie is made like a Toad (luma.json object_classes), so the Toad's code
  moves these joints by name.
- Two materials of its own, each with its own texture: Lumie_Body (a warm glow with sparks) and Lumie_Face (eyes and
  cheeks), drawn here.
- Animations named as the Toad's that a placed Toad plays: Wait, Talk, Walk, Run, Turn, Reaction, Spin, Pointing,
  Trampled, Surprised, Notice, Joy.
Lengths in metres (Nebula: 1 m = 100 units); +Y up, Lumie faces +Z."""
import json
import math
import struct
import sys
import zlib

# ---- the skeleton: name, parent, place (relative to its parent)
JOINTS = [
    ("JointRoot", -1, (0.0, 0.0, 0.0)),
    ("body", 0, (0.0, 0.45, 0.0)),
    ("head", 1, (0.0, 0.42, 0.0)),
    ("ShoulderL", 1, (0.30, 0.08, 0.0)),
    ("ArmL", 3, (0.12, -0.06, 0.0)),
    ("ShoulderR", 1, (-0.30, 0.08, 0.0)),
    ("ArmR", 5, (-0.12, -0.06, 0.0)),
    ("FootL", 0, (0.14, 0.07, 0.02)),
    ("FootR", 0, (-0.14, 0.07, 0.02)),
]
J = {name: i for i, (name, _, _) in enumerate(JOINTS)}


def world(i):
    name, parent, at = JOINTS[i]
    if parent < 0:
        return at
    p = world(parent)
    return (p[0] + at[0], p[1] + at[1], p[2] + at[2])


# ---- pictures
def png(width, height, pixel):
    rows = b"".join(b"\0" + bytes(c for x in range(width) for c in pixel(x, y)) for y in range(height))
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(rows, 9)) + chunk(b"IEND", b""))


def clamp(v):
    return max(0, min(255, int(v)))


def body_pixel(x, y):
    # A glow from gold at the top to amber below, with sparks.
    t = y / 63.0
    r, g, b = 255, 236 - 70 * t, 140 - 110 * t
    if (x * 7 + y * 13) % 29 == 0 or (x * 11 + y * 5) % 37 == 0:
        r, g, b = 255, 255, 235
    return (clamp(r), clamp(g), clamp(b), 255)


def face_pixel(x, y):
    # The head's map: u across (front at the middle), v down. Two eyes and two cheeks at the front.
    u, v = (x + 0.5) / 128.0, (y + 0.5) / 64.0
    r, g, b = 255, 244, 196
    for side in (-1, 1):
        du, dv = (u - (0.5 + side * 0.055)) / 0.022, (v - 0.47) / 0.07
        if du * du + dv * dv < 1.0:
            r, g, b = 40, 26, 60
            if (du + 0.35) ** 2 + (dv + 0.4) ** 2 < 0.12:
                r, g, b = 255, 255, 255
        cu, cv = (u - (0.5 + side * 0.1)) / 0.03, (v - 0.56) / 0.035
        if cu * cu + cv * cv < 1.0:
            r, g, b = 255, 170, 150
    return (r, g, b, 255)


# ---- the shapes: spheres, each moved by one joint
def sphere(center, radius, joint, squash=(1.0, 1.0, 1.0), rings=12, segments=20):
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
            coords.append((u, v))
            joints.append(joint)
    indices = []
    for ring in range(rings):
        for seg in range(segments):
            a = ring * (segments + 1) + seg
            b = a + segments + 1
            indices += [a, b, a + 1, a + 1, b, b + 1]
    return positions, normals, coords, joints, indices


def parts():
    w = [world(i) for i in range(len(JOINTS))]
    body = [
        sphere(w[J["body"]], 0.30, J["body"], (1.0, 0.92, 1.0)),
        sphere(w[J["ArmL"]], 0.085, J["ArmL"]),
        sphere(w[J["ArmR"]], 0.085, J["ArmR"]),
        sphere(w[J["FootL"]], 0.09, J["FootL"], (1.0, 0.7, 1.3)),
        sphere(w[J["FootR"]], 0.09, J["FootR"], (1.0, 0.7, 1.3)),
        sphere((w[J["head"]][0], w[J["head"]][1] + 0.36, w[J["head"]][2]), 0.06, J["head"]),
    ]
    face = [sphere((w[J["head"]][0], w[J["head"]][1] + 0.08, w[J["head"]][2]), 0.27, J["head"])]
    return body, face


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


# ---- the animations: per joint, rotation (axis, degrees) or translation keys over time
def quat(axis, degrees):
    half = math.radians(degrees) / 2.0
    s = math.sin(half)
    return (axis[0] * s, axis[1] * s, axis[2] * s, math.cos(half))


def wave(length, keys, f):
    return [(length * k / keys, f(2.0 * math.pi * k / keys)) for k in range(keys + 1)]


def animations():
    rest = {name: at for name, _, at in JOINTS}
    X, Y, Z = (1, 0, 0), (0, 1, 0), (0, 0, 1)
    def bob(height, length, keys=16):
        b = rest["body"]
        return ("body", "translation", wave(length, keys, lambda a: (b[0], b[1] + height * math.sin(a), b[2])))
    def swing(joint, axis, degrees, length, keys=16, phase=0.0, bias=0.0):
        return (joint, "rotation", wave(length, keys, lambda a: quat(axis, bias + degrees * math.sin(a + phase))))
    def step(joint, length, side, keys=16):
        r = rest[joint]
        return (joint, "translation", wave(length, keys, lambda a: (r[0], r[1] + max(0.0, 0.05 * math.sin(a * side)), r[2] + 0.08 * math.sin(a) * side)))
    return {
        "Wait": [bob(0.02, 2.0), swing("head", Z, 4.0, 2.0), swing("ShoulderL", Z, 6.0, 2.0), swing("ShoulderR", Z, -6.0, 2.0)],
        "Talk": [bob(0.015, 1.0), swing("head", X, 9.0, 0.5, 8), swing("ShoulderL", Z, 25.0, 1.0, 16, 0.0, 20.0)],
        "Walk": [bob(0.03, 0.8), step("FootL", 0.8, 1), step("FootR", 0.8, -1), swing("ShoulderL", X, 20.0, 0.8), swing("ShoulderR", X, -20.0, 0.8)],
        "Run": [bob(0.05, 0.5), step("FootL", 0.5, 1), step("FootR", 0.5, -1), swing("ShoulderL", X, 35.0, 0.5), swing("ShoulderR", X, -35.0, 0.5)],
        "Turn": [swing("JointRoot", Y, 15.0, 0.6, 12), bob(0.02, 0.6)],
        "Reaction": [("JointRoot", "translation", wave(0.6, 12, lambda a: (0.0, 0.25 * max(0.0, math.sin(a / 2.0)), 0.0))),
                     swing("ShoulderL", Z, 50.0, 0.6, 12), swing("ShoulderR", Z, -50.0, 0.6, 12)],
        "Spin": [("JointRoot", "rotation", [(0.5 * k / 16, quat(Y, 360.0 * k / 16)) for k in range(17)])],
        "Pointing": [("ShoulderR", "rotation", [(0.0, quat(Z, 0.0)), (0.3, quat(Z, -80.0)), (1.0, quat(Z, -80.0))]), bob(0.01, 1.0)],
        "Trampled": [("body", "translation", [(0.0, rest["body"]), (0.1, (0.0, 0.25, 0.0)), (0.5, (0.0, 0.25, 0.0)), (0.8, rest["body"])]),
                     ("head", "translation", [(0.0, rest["head"]), (0.1, (0.0, 0.30, 0.0)), (0.5, (0.0, 0.30, 0.0)), (0.8, rest["head"])])],
        "Surprised": [("JointRoot", "translation", wave(0.5, 10, lambda a: (0.0, 0.12 * max(0.0, math.sin(a / 2.0)), 0.0))),
                      swing("ShoulderL", Z, 70.0, 0.5, 10), swing("ShoulderR", Z, -70.0, 0.5, 10)],
        "Notice": [swing("head", Y, 25.0, 1.0, 16), bob(0.01, 1.0)],
        "Joy": [("JointRoot", "translation", wave(0.8, 16, lambda a: (0.0, 0.15 * abs(math.sin(a)), 0.0))),
                swing("ShoulderL", Z, 40.0, 0.4, 8, 0.0, 40.0), swing("ShoulderR", Z, -40.0, 0.4, 8, 0.0, -40.0)],
    }


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


def primitive(b, shapes, material):
    positions, normals, coords, joints, indices = join(shapes)
    return {
        "attributes": {
            "POSITION": b.accessor(positions, "VEC3", 5126, "f", 34962, True),
            "NORMAL": b.accessor(normals, "VEC3", 5126, "f", 34962),
            "TEXCOORD_0": b.accessor(coords, "VEC2", 5126, "f", 34962),
            "JOINTS_0": b.accessor([(j, 0, 0, 0) for j in joints], "VEC4", 5123, "H", 34962),
            "WEIGHTS_0": b.accessor([(1.0, 0.0, 0.0, 0.0)] * len(joints), "VEC4", 5126, "f", 34962),
        },
        "indices": b.accessor(indices, "SCALAR", 5123, "H", 34963),
        "material": material,
    }


def main():
    b = Bin()
    body, face = parts()
    nodes = []
    for i, (name, parent, at) in enumerate(JOINTS):
        node = {"name": name, "translation": list(at)}
        children = [k for k, (_, p, _) in enumerate(JOINTS) if p == i]
        if children:
            node["children"] = children
        nodes.append(node)
    inverse = []
    for i in range(len(JOINTS)):
        w = world(i)
        inverse.append((1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, -w[0], -w[1], -w[2], 1))
    skin = {"name": "LumieSkin", "joints": list(range(len(JOINTS))), "skeleton": 0,
            "inverseBindMatrices": b.accessor(inverse, "MAT4", 5126, "f")}
    images = [
        {"name": "Lumie_Body", "mimeType": "image/png", "bufferView": b.view(png(64, 64, body_pixel))},
        {"name": "Lumie_Face", "mimeType": "image/png", "bufferView": b.view(png(128, 64, face_pixel))},
    ]
    materials = [
        {"name": "Lumie_Body", "pbrMetallicRoughness": {"baseColorTexture": {"index": 0}, "metallicFactor": 0.0, "roughnessFactor": 0.8}},
        {"name": "Lumie_Face", "pbrMetallicRoughness": {"baseColorTexture": {"index": 1}, "metallicFactor": 0.0, "roughnessFactor": 0.9}},
    ]
    mesh = {"name": "Lumie", "primitives": [primitive(b, body, 0), primitive(b, face, 1)]}
    nodes.append({"name": "Lumie", "mesh": 0, "skin": 0})
    anims = []
    for name, channels in animations().items():
        samplers, targets = [], []
        for joint, path, keys in channels:
            times = [t for t, _ in keys]
            values = [v for _, v in keys]
            sampler = {"input": b.accessor(times, "SCALAR", 5126, "f", None, False),
                       "output": b.accessor(values, "VEC4" if path == "rotation" else "VEC3", 5126, "f"),
                       "interpolation": "LINEAR"}
            b.accessors[sampler["input"]]["min"] = [min(times)]
            b.accessors[sampler["input"]]["max"] = [max(times)]
            samplers.append(sampler)
            targets.append({"sampler": len(samplers) - 1, "target": {"node": J[joint], "path": path}})
        anims.append({"name": name, "samplers": samplers, "channels": targets})
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_lumie.py"},
        "scene": 0,
        "scenes": [{"nodes": [0, len(nodes) - 1]}],
        "nodes": nodes,
        "meshes": [mesh],
        "skins": [skin],
        "materials": materials,
        "textures": [{"source": 0, "sampler": 0}, {"source": 1, "sampler": 0}],
        "samplers": [{"magFilter": 9729, "minFilter": 9987, "wrapS": 10497, "wrapT": 33071}],
        "images": images,
        "animations": anims,
        "accessors": b.accessors,
        "bufferViews": b.views,
        "buffers": [{"byteLength": len(b.data)}],
    }
    text = json.dumps(document, separators=(",", ":")).encode()
    text += b" " * (-len(text) % 4)
    data = bytes(b.data) + b"\0" * (-len(b.data) % 4)
    out = b"glTF" + struct.pack("<II", 2, 12 + 8 + len(text) + 8 + len(data))
    out += struct.pack("<I", len(text)) + b"JSON" + text + struct.pack("<I", len(data)) + b"BIN\0" + data
    with open(sys.argv[1], "wb") as f:
        f.write(out)
    print(f"{sys.argv[1]}: {len(out)} bytes, {len(JOINTS)} joints, 2 materials, {len(anims)} animations")


if __name__ == "__main__":
    main()
