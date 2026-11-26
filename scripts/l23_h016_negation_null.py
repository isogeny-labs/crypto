#!/usr/bin/env python3
"""L23: re-score H016 base yields against a negation-closed random null.

Uses the decrypted H016 result (inputs/h100_session/h016_base_yield.json) for
the stored qr/small_mult/random yields, and regenerates the curves from the
recorded (p,a,b,N,B) to draw two null families:

  uniform:    B distinct uniform logs in [1,N)
  symmetric:  B/2 distinct logs s in [1,(N-1)/2), base = {sG, -sG}

For each draw we count unordered triples i<=j<=k with P_i+P_j+P_k = R for 800
fresh uniform targets, then compare the stored structured means to the
symmetric-null distribution.
"""
import json
import random
import statistics
import sys
from math import isqrt

H016 = sys.argv[2] if len(sys.argv) > 2 else "h016.json"
NDRAW = int(sys.argv[1]) if len(sys.argv) > 1 else 200
NTARGET = 800
SEED = 20261124


def sqrt_mod(a, p):
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c = i, b * b % p
        t, r = t * c % p, r * b % p
    return r


class Curve:
    def __init__(self, p, a, b, n):
        self.p, self.a, self.b, self.n = p, a, b, n
        self.G = self._find_point()

    def _find_point(self):
        for x in range(1, self.p):
            y = sqrt_mod(x * x * x + self.a * x + self.b, self.p)
            if y:
                return (x, y)
        raise RuntimeError("no point")

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p = self.p
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P=None):
        P = self.G if P is None else P
        R = None
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def neg(self, P):
        return (P[0], (-P[1]) % self.p)

    def key(self, P):
        return None if P is None else P[0] * self.p + P[1]


def triple_counts(curve, base, targets):
    """Count unordered triples i<=j<=k with sum == each target."""
    B = len(base)
    tkeys = [curve.key(R) for R in targets]
    counts = {k: 0 for k in tkeys}
    pair = {}
    for k in range(B):
        Pk = base[k]
        for i in range(k + 1):
            s = curve.add(base[i], Pk)
            if s is not None:
                kk = curve.key(s)
                pair[kk] = pair.get(kk, 0) + 1
        for R, tkey in zip(targets, tkeys):
            if R is None:
                continue
            need = curve.add(R, curve.neg(Pk))
            if need is not None:
                counts[tkey] += pair.get(curve.key(need), 0)
    return counts


def draw_base(curve, B, kind, rng):
    if kind == "uniform":
        logs = rng.sample(range(1, curve.n), B)
        return [curve.mul(s) for s in logs]
    half = B // 2
    logs = rng.sample(range(1, (curve.n - 1) // 2 + 1), half)
    pts = []
    for s in logs:
        P = curve.mul(s)
        pts.append(P)
        pts.append(curve.neg(P))
    return pts


def main():
    d = json.load(open(H016))
    rng = random.Random(SEED)
    out = []
    for size in d["sizes"]:
        p, a, b, n, B = size["p"], size["a"], size["b"], size["N"], size["B"]
        curve = Curve(p, a, b, n)
        targets = [curve.mul(rng.randrange(1, n)) for _ in range(NTARGET)]
        row = dict(bits=size["bits"], p=p, N=n, B=B, theory=size["theory_mean_triples_per_target"],
                   stored={kk: vv["mean_per_target"] for kk, vv in size["bases"].items()})
        for kind in ("uniform", "symmetric"):
            means = []
            for _ in range(NDRAW):
                base = draw_base(curve, B, kind, rng)
                c = triple_counts(curve, base, targets)
                means.append(sum(c.values()) / NTARGET)
            row[kind] = dict(mean=statistics.mean(means), sd=statistics.stdev(means),
                             min=min(means), max=max(means))
        # z of each stored structured family against the symmetric null
        mu, sd = row["symmetric"]["mean"], row["symmetric"]["sd"]
        row["z_symmetric"] = {k: (v - mu) / sd if sd else None for k, v in row["stored"].items()}
        mu_u, sd_u = row["uniform"]["mean"], row["uniform"]["sd"]
        row["z_uniform"] = {k: (v - mu_u) / sd_u if sd_u else None for k, v in row["stored"].items()}
        print(json.dumps(row, indent=1), flush=True)
        out.append(row)
    json.dump(out, open("l23_rescore.json", "w"), indent=1)


if __name__ == "__main__":
    main()
