"""Hash-derived base points: x from SHA-1 expansion of the curve's seed (as b is), or of seed+counter,
in the X9.62 style (SHA-1 of (seed + i) mod 2^160, concatenated), with an optional first hash of the bare
seed; candidates reduced mod p or truncated; compared to Gx and Gy; counters 0..2^16.
Control: a synthetic x built the same way for P-256 with counter 12345."""
import sys, hashlib
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from curves import CURVES

def sha(z): return int.from_bytes(hashlib.sha1((z % 2**160).to_bytes(20, "big")).digest(), "big")

def cand_xs(seed, p, ctr, bits):
    s = (bits - 1) // 160
    out = {}
    for first in (0, 1):                      # first block hashes seed+ctr (b-style) or seed+ctr+1
        W = 0
        for i in range(s + 1):
            W = (W << 160) | sha(seed + ctr + first + i)
        tot = 160 * (s + 1)
        out[f"mod p, off {first}"] = W % p
        out[f"top bits, off {first}"] = W >> (tot - bits)
        out[f"low bits, off {first}"] = W & ((1 << bits) - 1)
    return out

hits = []
for name in ("P-224", "P-256", "P-384", "P-521"):
    p, seedhex, gx, gy = CURVES[name][0], CURVES[name][1], CURVES[name][4], CURVES[name][5]
    seed = int(seedhex, 16); bits = p.bit_length()
    tg = {"Gx": gx, "Gy": gy}
    if name == "P-256":
        tg["CONTROL"] = cand_xs(seed, p, 12345, bits)["mod p, off 1"]
    found = []
    for ctr in range(0, 1 << 16):
        for form, v in cand_xs(seed, p, ctr, bits).items():
            for tn, tv in tg.items():
                if v == tv: found.append((tn, ctr, form))
    print(name, found or "none", flush=True)
