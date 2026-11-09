"""make_voice.py: the sounds of the new-voice sample, written into its layer as 16-bit 32 kHz mono WAV files.

Each is a short voiced "hup": a pitched buzz (a sum of harmonics) with a falling pitch and a quick rise and fall, so it is
heard plainly in place of the player's own jump voices. Luigi's is pitched higher. Run with python -I.
"""
import math
import os
import struct

RATE = 32000
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "layers", "base", "files", "AudioRes", "Stream")


def hup(start_hz, end_hz, seconds):
    frames = int(RATE * seconds)
    samples = []
    phase = 0.0
    for i in range(frames):
        t = i / frames
        hz = start_hz + (end_hz - start_hz) * t
        phase += 2.0 * math.pi * hz / RATE
        # Harmonics falling off as a voice's do.
        value = sum(math.sin(phase * k) / k for k in range(1, 9))
        envelope = min(1.0, t / 0.08) * (1.0 - t) ** 1.5
        samples.append(int(max(-1.0, min(1.0, value * 0.35 * envelope)) * 32767))
    return samples


def write_wav(path, samples):
    data = b"".join(struct.pack("<h", s) for s in samples)
    with open(path, "wb") as f:
        f.write(b"RIFF" + struct.pack("<I", 36 + len(data)) + b"WAVE")
        f.write(b"fmt " + struct.pack("<IHHIIHH", 16, 1, 1, RATE, RATE * 2, 2, 16))
        f.write(b"data" + struct.pack("<I", len(data)) + data)


os.makedirs(OUT, exist_ok=True)
write_wav(os.path.join(OUT, "HupSmall.wav"), hup(420.0, 300.0, 0.22))
write_wav(os.path.join(OUT, "HupMiddle.wav"), hup(380.0, 260.0, 0.30))
write_wav(os.path.join(OUT, "HupBig.wav"), hup(340.0, 220.0, 0.40))
write_wav(os.path.join(OUT, "HupLuigi.wav"), hup(560.0, 400.0, 0.26))
print("written to", os.path.normpath(OUT))
