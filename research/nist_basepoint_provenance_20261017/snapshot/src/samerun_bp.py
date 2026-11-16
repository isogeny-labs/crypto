"""Same-run test: were the P-224/256/384/521 base points drawn from the very drand48 stream that produced
the curve's seed (before or after the published block)? Direct generation, so reduction mod p is no obstacle.
Chunks: top w bits of each new state, w in {8 (the seed's byte), 16, 24, 31 (lrand48), 32 (mrand48), 48}.
x = n consecutive chunks (n = ceil(bits/w) + e, e in 0..2), MSB- or LSB-first; forms: X mod p, top-bits
truncation, low-bits truncation. Compared with Gx and Gy at every start position."""
import sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
import gmpy2
from gmpy2 import mpz
from curves import CURVES

A, C, MASK = 0x5DEECE66D, 0xB, (1 << 48) - 1
RUNS = {"P-224": (891607927, 7026), "P-256": (871696354, 1842), "P-384": (872952608, 2688), "P-521": (871002724, 1136)}
EXTRA = 60000
WIDTHS = (8, 16, 24, 31, 32, 48)

def states(seedval, n):
    x = ((seedval & 0xFFFFFFFF) << 16) | 0x330E
    out = []
    for _ in range(n):
        x = (A * x + C) & MASK
        out.append(x)
    return out

def search(name, S, p, targets):
    bits = p.bit_length(); hits = []
    for w in WIDTHS:
        ch = [s >> (48 - w) for s in S]
        for e in range(3):
            n = -(-bits // w) + e; tot = n * w; full = (mpz(1) << tot) - 1; lowmask = (mpz(1) << bits) - 1
            for order in ("msb", "lsb"):
                X = mpz(0)
                for k in range(n):
                    X = (X << w) | ch[k] if order == "msb" else X | (mpz(ch[k]) << (w * k))
                for i in range(len(ch) - n):
                    cands = (("mod p", X % p), ("top bits", X >> (tot - bits)), ("low bits", X & lowmask))
                    for form, v in cands:
                        for tn, tv in targets:
                            if v == tv:
                                hits.append((tn, w, e, order, form, i))
                    nxt = ch[i + n]
                    X = ((X << w) | nxt) & full if order == "msb" else (X >> w) | (mpz(nxt) << (w * (n - 1)))
    return hits

for name, (seedval, block) in RUNS.items():
    p, seedhex, gx, gy = CURVES[name][0], CURVES[name][1], CURVES[name][4], CURVES[name][5]
    S = states(seedval, 20 * block + EXTRA)
    blockbytes = bytes(s >> 40 for s in S[20 * (block - 1): 20 * block]).hex()
    assert blockbytes == seedhex.lower(), (name, blockbytes, seedhex)
    targets = [("Gx", mpz(gx)), ("Gy", mpz(gy)), ("p-Gy", mpz(p - gy))]
    if name == "P-256":   # planted positive controls: bytes after the seed block, and lrand48 words
        bits = p.bit_length()
        X = mpz(0)
        for s in S[20 * block + 5: 20 * block + 5 + 33]: X = (X << 8) | (s >> 40)
        targets.append(("CONTROL bytes msb e=0 mod p", X % p))
        Y = mpz(0)
        for k, s in enumerate(S[20 * block + 101: 20 * block + 101 + 10]): Y |= mpz(s >> 17) << (31 * k)
        targets.append(("CONTROL lrand48 lsb e=1 top bits", Y >> (310 - bits)))
    hits = search(name, S, mpz(p), targets)
    print(f"{name}: stream {len(S)} outputs (seed block {block} verified); hits: {hits if hits else 'none'}", flush=True)
