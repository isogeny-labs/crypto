"""Seed-agnostic checks of the P-224/256/384/521 base points against 1997-era library generators.
PARI 2.0 (bibli2.c): 32-bit  s <- (1000276549 s + 12347) mod 2^31, random() returns s (31 bits);
                     64-bit  s <- (1000000000000654397 s + 12347) mod 2^63.
  genrand() slices: (s >> 12) & 0xffff on 32-bit builds, (s >> 12) & 0xffffffff on 64-bit builds.
Maple V classic rand: s <- 427419669081 s mod 999999999989, 12-digit decimal outputs.
A slice of bits [lo, lo+w) of an LCG mod 2^K is the top w bits of the same LCG mod 2^(lo+w), so each slice
pattern is tested as "top w bits of an LCG mod 2^k" by brute force (hidden bits <= 16) or exact chaining.
x is scanned at every bit offset, both chunk orders; a hit needs a chain of >= 3 consecutive outputs."""
import sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from curves import CURVES

PATTERNS = [
    # name, a, c, modulus bits of the effective LCG, chunk width w (top w bits of that LCG), slot width
    ("PARI32 random() 31-bit words, 31-bit packing", 1000276549, 12347, 31, 31, 31),
    ("PARI32 random() 31-bit words, 32-bit slots", 1000276549, 12347, 31, 31, 32),
    ("PARI32 genrand 16-bit slices (s>>12)&0xffff", 1000276549, 12347, 28, 16, 16),
    ("PARI64 random() 63-bit words, 64-bit slots", 1000000000000654397, 12347, 63, 63, 64),
    ("PARI64 random() 63-bit words, 63-bit packing", 1000000000000654397, 12347, 63, 63, 63),
]

def chain_ok(ys, a, c, K, w):
    M = 1 << K; hid = K - w
    if hid == 0:
        return all((a * ys[i] + c) % M == ys[i + 1] for i in range(len(ys) - 1))
    for z in range(1 << hid):
        x = (ys[0] << hid) | z; ok = True
        for y in ys[1:]:
            x = (a * x + c) % M
            if x >> hid != y: ok = False; break
        if ok: return True
    return False

def scan_pattern(V, L, pat, need):
    name, a, c, K, w, slot = pat
    hits = []
    for order in ("msb", "lsb"):
        for off in range(slot):
            n = (L - off) // slot
            if n < need: continue
            ch = [(V >> (off + slot * j)) & ((1 << slot) - 1) for j in range(n)]
            ch = ch[::-1] if order == "msb" else ch
            if slot > w:
                ch = [x if x >> w == 0 else None for x in ch]
            for st in range(n - need + 1):
                win = ch[st:st + need]
                if None in win: continue
                if chain_ok(win, a, c, K, w): hits.append((order, off, st))
    return hits

MA, MM = 427419669081, 999999999989
def scan_maple(V):
    s = str(V); hits = []
    for off in range(12):
        t = s[: len(s) - off] if off else s
        d = [int(t[max(0, i - 12): i]) for i in range(len(t), 0, -12)]   # least significant group first
        for order, seq in (("lsb", d), ("msb", d[::-1])):
            for st in range(len(seq) - 2):
                x0, x1, x2 = seq[st:st + 3]
                if x0 and MA * x0 % MM == x1 and MA * x1 % MM == x2: hits.append((order, off, st))
    return hits

def synth(pat, seed, L, skip):
    name, a, c, K, w, slot = pat
    M = 1 << K; s = seed % M; X = 0; n = L // slot + 1
    for k in range(skip + n):
        s = (a * s + c) % M
        if k >= skip: X = (X << slot) | (s >> (K - w))
    return X >> (n * slot - L)

def synth_maple(seed, L):
    s = seed; t = ""
    while len(t) * 3.33 < L + 40:
        s = MA * s % MM; t += f"{s:012d}"
    D = len(str(1 << L)) - 1          # decimal digits that stay below 2^L
    return int(t[5:5 + D])            # offset 5 tests misaligned groups

for name in ("P-224", "P-256", "P-384", "P-521"):
    p, gx, gy = CURVES[name][0], CURVES[name][4], CURVES[name][5]
    L = p.bit_length()
    for pat in PATTERNS:
        need = 3 if pat[4] >= 31 else 4
        ctrl = synth(pat, 123456789, L, 17)
        r = {k: scan_pattern(v, L, pat, need) for k, v in (("Gx", gx), ("Gy", gy), ("control", ctrl))}
        print(f"{name} {pat[0]:46s} Gx {r['Gx'] or 'none'}  Gy {r['Gy'] or 'none'}  control {'found' if r['control'] else 'MISSED'}", flush=True)
    cm = synth_maple(31415926, L)
    print(f"{name} {'Maple V rand (12-digit decimal groups)':46s} Gx {scan_maple(gx) or 'none'}  Gy {scan_maple(gy) or 'none'}  control {'found' if scan_maple(cm) else 'MISSED'}", flush=True)
