"""make_gold_flower.py <example folder>: the Gold Flower's own files. Python 3 alone, no packages.

- GoldFlowerItem: the flower's model (ObjectData/GoldFlowerItem.arc/GoldFlowerItem.bdl.glb): a stem, two leaves, six
  gold petals round a face, a skeleton of two joints and a looping "Wait" sway.
- Its particles (merge/ParticleData/Effect.arc/): GoldFlowerAura (two particles, on Mario while the power-up lasts)
  and GoldFlowerSparkle (on the flower), made from the game's star-power particles (InvincibleMario00 and 01) in gold,
  with a texture of their own (gold_spark.png). ParticleNames.bcsv.json names them; AutoEffectList.bcsv.json puts
  them on Mario and on the flower.
Lengths in metres (1 m = 100 of the game's units); +Y up."""
import json
import math
import os
import struct
import sys
import zlib


# ---- pictures
def png(width, height, pixel):
    rows = b"".join(b"\0" + bytes(c for x in range(width) for c in pixel(x, y)) for y in range(height))
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(rows, 9)) + chunk(b"IEND", b""))


def clamp(v):
    return max(0, min(255, int(v)))


def spark_pixel(x, y):
    # A four-pointed sparkle: a soft round middle and two thin rays, white-gold, clear around.
    u, v = (x + 0.5) / 16.0 - 1.0, (y + 0.5) / 16.0 - 1.0
    r = math.sqrt(u * u + v * v)
    core = math.exp(-r * r * 18.0)
    rays = max(math.exp(-abs(u) * 28.0) * max(0.0, 1.0 - abs(v)), math.exp(-abs(v) * 28.0) * max(0.0, 1.0 - abs(u)))
    a = min(1.0, core + rays * 0.9)
    return (255, clamp(225 + 30 * core), clamp(120 + 120 * core), clamp(255 * a))


def petal_pixel(x, y):
    t = y / 15.0
    return (255, clamp(200 + 40 * (1.0 - t)), clamp(40 + 60 * (1.0 - t)), 255)


def face_pixel(x, y):
    # The flower's face (u round the sphere, v down it): orange, two eyes at the front (u = 0.5).
    u, v = (x + 0.5) / 64.0, (y + 0.5) / 32.0
    r, g, b = 255, 150, 30
    for side in (-1, 1):
        du, dv = (u - (0.5 + side * 0.045)) / 0.018, (v - 0.45) / 0.09
        if du * du + dv * dv < 1.0:
            r, g, b = 20, 15, 10
    return (r, g, b, 255)


def stem_pixel(x, y):
    t = x / 7.0
    return (clamp(40 + 30 * t), clamp(150 + 40 * t), clamp(50 + 20 * t), 255)


# ---- shapes: positions, normals, coordinates, joints, indices
def sphere(center, radius, joint=0, squash=(1.0, 1.0, 1.0), rings=10, segments=16):
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


def turned_about_z(shape, degrees, about):
    positions, normals, coords, joints, indices = shape
    c, s = math.cos(math.radians(degrees)), math.sin(math.radians(degrees))
    def turn(p, origin):
        x, y = p[0] - origin[0], p[1] - origin[1]
        return (origin[0] + x * c - y * s, origin[1] + x * s + y * c, p[2])
    return ([turn(p, about) for p in positions], [turn(n, (0.0, 0.0, 0.0)) for n in normals], coords, joints, indices)


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
    attributes = {
        "POSITION": b.accessor(positions, "VEC3", 5126, "f", 34962, True),
        "NORMAL": b.accessor(normals, "VEC3", 5126, "f", 34962),
        "TEXCOORD_0": b.accessor(coords, "VEC2", 5126, "f", 34962),
        "JOINTS_0": b.accessor([(j, 0, 0, 0) for j in joints], "VEC4", 5123, "H", 34962),
        "WEIGHTS_0": b.accessor([(1.0, 0.0, 0.0, 0.0)] * len(joints), "VEC4", 5126, "f", 34962),
    }
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


HEAD = 0.62               # the flower's head, metres up the stem


def quat(axis, degrees):
    half = math.radians(degrees) / 2.0
    s = math.sin(half)
    return (axis[0] * s, axis[1] * s, axis[2] * s, math.cos(half))


def flower_model():
    b = Bin()
    images, textures, materials = [], [], []
    for i, (name, data, rough) in enumerate([
            ("GoldFlower_Petal", png(16, 16, petal_pixel), 0.4),
            ("GoldFlower_Face", png(64, 32, face_pixel), 0.6),
            ("GoldFlower_Stem", png(8, 8, stem_pixel), 0.8)]):
        images.append({"name": name, "mimeType": "image/png", "bufferView": b.view(data)})
        textures.append({"source": i, "sampler": 0})
        materials.append({"name": name, "pbrMetallicRoughness": {"baseColorTexture": {"index": i}, "metallicFactor": 0.0, "roughnessFactor": rough}})
    # Joint 0 Root (the stem and leaves), joint 1 Head (the face and petals).
    petals = []
    for k in range(6):
        a = 2.0 * math.pi * k / 6.0
        at = (math.cos(a) * 0.17, HEAD + math.sin(a) * 0.17, -0.02)
        petals.append(sphere(at, 0.11, 1, (1.0, 1.0, 0.35)))
    face = [sphere((0.0, HEAD, 0.02), 0.12, 1, (1.0, 1.0, 0.75), rings=12, segments=24)]
    stem = [cylinder(0.025, HEAD - 0.08, 0, 8)]
    for side in (-1, 1):
        leaf = sphere((side * 0.1, 0.2, 0.0), 0.1, 0, (1.0, 0.3, 0.55))
        stem.append(turned_about_z(leaf, side * 25.0, (0.0, 0.2, 0.0)))
    mesh = {"name": "GoldFlowerItem", "primitives": [primitive(b, petals, 0), primitive(b, face, 1), primitive(b, stem, 2)]}
    inverse = [(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1), (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, -HEAD, 0, 1)]
    skin = {"name": "GoldFlowerSkin", "joints": [0, 1], "skeleton": 0, "inverseBindMatrices": b.accessor(inverse, "MAT4", 5126, "f")}
    nodes = [{"name": "Root", "children": [1]}, {"name": "Head", "translation": [0.0, HEAD, 0.0]},
             {"name": "GoldFlowerItem", "mesh": 0, "skin": 0}]
    # Wait: the head sways and nods, 2 seconds, looping.
    keys = 24
    times = [2.0 * k / keys for k in range(keys + 1)]
    turns = []
    for t in times:
        a = 2.0 * math.pi * t / 2.0
        z, y = quat((0, 0, 1), 8.0 * math.sin(a)), quat((0, 1, 0), 10.0 * math.cos(a))
        turns.append((y[3] * z[0] + y[0] * z[3] + y[1] * z[2] - y[2] * z[1],
                      y[3] * z[1] - y[0] * z[2] + y[1] * z[3] + y[2] * z[0],
                      y[3] * z[2] + y[0] * z[1] - y[1] * z[0] + y[2] * z[3],
                      y[3] * z[3] - y[0] * z[0] - y[1] * z[1] - y[2] * z[2]))
    t_in = b.accessor(times, "SCALAR", 5126, "f")
    b.accessors[t_in]["min"], b.accessors[t_in]["max"] = [0.0], [2.0]
    animation = {"name": "Wait", "samplers": [{"input": t_in, "output": b.accessor(turns, "VEC4", 5126, "f"), "interpolation": "LINEAR"}],
                 "channels": [{"sampler": 0, "target": {"node": 1, "path": "rotation"}}]}
    document = {
        "asset": {"version": "2.0", "generator": "Nebula examples: make_gold_flower.py"},
        "scene": 0, "scenes": [{"nodes": [0, 2]}], "nodes": nodes, "meshes": [mesh], "skins": [skin],
        "materials": materials, "textures": textures, "images": images,
        "samplers": [{"magFilter": 9729, "minFilter": 9987, "wrapS": 10497, "wrapT": 33071}], "animations": [animation],
    }
    return glb(document, b)


# ---- particles
GOLD, PALE = "ffb020ff", "fff2b0ff"


def particles():
    # Each starts from the game's particle `like` and changes only what is given. Their names, in the same order, are
    # ParticleNames.bcsv.json's rows; an effect named GoldFlowerAura plays GoldFlowerAura00, 01, ...
    textures = ["GoldSpark", "mr_noise00_i"]   # the game's star dust swapped for our sparkle
    return {
        "textures": {"GoldSpark": "gold_spark.png"},
        "particles": [
            {"name": "GoldFlowerAura00", "like": "InvincibleMario00", "textures": textures, "emitter": {"rate": 1.2},
             "shape": {"primary": PALE, "environment": GOLD, "size": [2.4, 2.4]}},
            {"name": "GoldFlowerAura01", "like": "InvincibleMario01", "textures": textures, "emitter": {"rate": 0.8},
             "shape": {"primary": PALE, "environment": GOLD, "size": [2.0, 2.0]}},
            {"name": "GoldFlowerSparkle00", "like": "InvincibleMario00", "textures": textures,
             "emitter": {"rate": 0.15, "volume_size": 45, "translation": [0.0, 62.0, 0.0]},
             "shape": {"primary": PALE, "environment": GOLD, "size": [0.8, 0.8]}},
        ],
    }


def effect_rows():
    def row(group, unique, effect, joint):
        return {"GroupName": group, "UniqueName": unique, "EffectName": effect, "JointName": joint, "Affect": "T/R",
                "Follow": "T/R", "DrawOrder": "3D", "EndFrame": -1, "RateValue": 0.5, "ScaleValue": 1.4}
    sparkle = row("GoldFlowerItem", "Sparkle", "GoldFlowerSparkle", "")
    sparkle.update({"RateValue": 1.0, "ScaleValue": 1.0})
    aura = row("Mario", "GoldFlowerAura", "GoldFlowerAura", "Center")
    aura.update({"RateValue": 1.0, "ScaleValue": 1.2})
    return {"rows": [aura, sparkle]}


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
    effect = "merge/ParticleData/Effect.arc/"
    model = write(folder, "files/ObjectData/GoldFlowerItem.arc/GoldFlowerItem.bdl.glb", flower_model())
    write(folder, effect + "gold_spark.png", png(32, 32, spark_pixel))
    p = particles()
    write(folder, effect + "Particles.jpc.json", p)
    write(folder, effect + "ParticleNames.bcsv.json", {"rows": [{"name": q["name"]} for q in p["particles"]]})
    write(folder, effect + "AutoEffectList.bcsv.json", effect_rows())
    print(f"written: the flower {model} bytes, {len(p['particles'])} particles and their texture, the effect rows")


if __name__ == "__main__":
    main()
