"""Un-reduce test for P-224 and P-256 base points. If x = X mod p with X built from drand48 chunks plus
e extra chunks, then X = Gx + q*p with q < 2^(w*e + 1). For every q, rebuild X and test whether its
chunks are consecutive drand48 outputs (top w bits of the 48-bit state), seed-agnostic, by lattice CVP.
Chunk widths: 8 (the seed's (int)(drand48()*256)) with e = 1, 2; 16 with e = 1. Both chunk orders."""
import sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from fpylll import IntegerMatrix, LLL, GSO
from curves import CURVES

A48, C48, K = 0x5DEECE66D, 0xB, 48
M = 1 << K

class Recover:
    """Precomputed lattice for n consecutive top-w-bit outputs of the 48-bit LCG."""
    def __init__(self, w, n):
        self.w, self.n, self.hid = w, n, K - w
        self.Ai, self.Ci = [1], [0]
        for _ in range(n - 1):
            self.Ai.append(self.Ai[-1] * A48 % M); self.Ci.append((self.Ci[-1] * A48 + C48) % M)
        B = IntegerMatrix(n, n); B[0, 0] = 1
        for i in range(1, n): B[0, i] = self.Ai[i]; B[i, i] = M
        LLL.reduction(B); self.B = B
        self.G = GSO.Mat(B); self.G.update_gso()
    def __call__(self, ys):
        hid = self.hid; half = 1 << (hid - 1)
        Di = [(self.Ai[i] * (ys[0] << hid) + self.Ci[i] - (ys[i] << hid)) % M for i in range(self.n)]
        target = [half] + [(half - Di[i]) % M for i in range(1, self.n)]
        coeffs = self.G.babai(target)
        z = sum(int(c) * self.B[j, 0] for j, c in enumerate(coeffs)) % M
        for dz in range(-3, 4):
            x = (ys[0] << hid) | ((z + dz) % (1 << hid)); ok = True
            for y in ys[1:]:
                x = (A48 * x + C48) % M
                if x >> hid != y: ok = False; break
            if ok: return True
        return False

def chunks(X, nbits, w, order):
    n = nbits // w
    c = [(X >> (w * j)) & ((1 << w) - 1) for j in range(n)]   # least significant first
    return c[::-1] if order == "msb" else c

def test(X, nbits, w, rec):
    for order in ("msb", "lsb"):
        c = chunks(X, nbits, w, order)
        # windows at both ends and the middle; the whole X is generator output under the hypothesis
        for st in {0, len(c) // 2 - rec.n // 2, len(c) - rec.n}:
            if rec(c[st:st + rec.n]): return order
    return None

CONFIGS = [(8, 1), (8, 2), (16, 1)]
REC = {8: Recover(8, 14), 16: Recover(16, 7)}

def synth(p, w, e, seedval, skip):
    x = ((seedval & 0xFFFFFFFF) << 16) | 0x330E
    n = -(-p.bit_length() // w) + e; X = 0
    for k in range(skip + n):
        x = (A48 * x + C48) % M
        if k >= skip: X = (X << w) | (x >> (K - w))
    return X % p

for name in ("P-224", "P-256"):
    p, gx, gy = CURVES[name][0], CURVES[name][4], CURVES[name][5]
    bits = p.bit_length()
    for w, e in CONFIGS:
        nbits = (-(-bits // w) + e) * w
        qmax = (1 << nbits) // p + 1
        ctrl = synth(p, w, e, 893000000, 1234)
        tg = [("Gx", gx), ("Gy", gy), ("CONTROL synthetic", ctrl)]
        res = {}
        for tn, tv in tg:
            res[tn] = [q for q in range(qmax + 1) if tv + q * p < (1 << nbits) and test(tv + q * p, nbits, w, REC[w])]
        print(f"{name} w={w:2d} extra={e} (q < {qmax}): Gx {res['Gx'] or 'none'}  Gy {res['Gy'] or 'none'}  control found at q={res['CONTROL synthetic']}", flush=True)
