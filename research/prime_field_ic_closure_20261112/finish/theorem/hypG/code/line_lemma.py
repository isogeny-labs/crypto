"""Numerical check of the tower line lemma.  [pcc:unverified]
Lemma L (line lemma). W = g*mu_N (N=2^k) in F_p, p odd.  q: W -> F_p nonzero with Q*q = P on W,
deg P < deg Q = r, 1 <= r < N.  Write q = sum_{a<N} C_a x^a (reduced mod x^N - c).  Then
   wt-deg(q) := max{popcount(a) : C_a != 0} >= k - ceil(log2 r).
Equality is attained by Q = x^{2^e} - alpha (r = 2^e).
Checks: (1) random Q of degree r (no roots on W), P random of degree < r; (2) functions supported on
s points (P = 0, Q = prod (x - v)); (3) tightness family.  Reports min(wt-deg - bound) per (k, r)."""
import random, math, json, sys
import numpy as np
import fastdelta as F

def tower_roots(p, k, g):
    N = 2 ** k
    for h in range(2, p):
        z = pow(h, (p - 1) // N, p)
        if pow(z, N // 2, p) != 1: break
    return [g * pow(z, j, p) % p for j in range(N)]

def wtdeg(vals, Vinv, wt):
    C = (Vinv @ (np.array(vals, dtype=np.int64) % p)) % p
    nz = np.nonzero(C)[0]
    return int(wt[nz].max()) if len(nz) else -1

def peval(c, x):
    s = 0
    for a in reversed(c): s = (s * x + a) % p
    return s

p = int(sys.argv[1]) if len(sys.argv) > 1 else 786433      # 3*2^18+1
rng = random.Random(20261115)
out = []
for k in range(2, 11):
    N = 2 ** k
    roots = tower_roots(p, k, rng.randrange(2, p))
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
    wt = np.array([bin(a).count("1") for a in range(N)])
    for r in sorted({1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 31, 32, 64, 128} & set(range(1, N))):
        bound = k - math.ceil(math.log2(r)) if r > 1 else k
        mins = {"rational": None, "support": None, "tight": None}
        for trial in range(40):
            # (1) rational function P/Q
            while True:
                Q = [rng.randrange(p) for _ in range(r)] + [rng.randrange(1, p)]
                if all(peval(Q, x) for x in roots): break
            P = [rng.randrange(p) for _ in range(rng.randrange(1, r + 1))]
            if not any(P): P[0] = 1
            vals = [peval(P, x) * pow(peval(Q, x), p - 2, p) % p for x in roots]
            d = wtdeg(vals, Vinv, wt) - bound
            mins["rational"] = d if mins["rational"] is None else min(mins["rational"], d)
            # (2) support on r points
            if r <= N:
                S = rng.sample(range(N), r)
                vals = [0] * N
                for j in S: vals[j] = rng.randrange(1, p)
                d = wtdeg(vals, Vinv, wt) - bound
                mins["support"] = d if mins["support"] is None else min(mins["support"], d)
        e = int(math.log2(r))
        if 2 ** e == r:
            while True:
                al = rng.randrange(1, p)
                if all((pow(x, r, p) - al) % p for x in roots): break
            vals = [pow((pow(x, r, p) - al) % p, p - 2, p) for x in roots]
            mins["tight"] = wtdeg(vals, Vinv, wt) - bound
            # support version of tightness: indicator of a coset of mu_{N/r}... support size r: coset of mu_r
            S = [j for j in range(N) if j % (N // r) == 0]
            vals = [1 if j in S else 0 for j in range(N)]
            mins["tight_support"] = wtdeg(vals, Vinv, wt) - bound
        out.append({"k": k, "r": r, "bound": bound, **mins})
        print(json.dumps(out[-1]), flush=True)
viol = [o for o in out if any(v is not None and v < 0 for kk, v in o.items() if kk not in ("k", "r", "bound"))]
print(json.dumps({"p": p, "cells": len(out), "violations": len(viol),
                  "tight_cells_with_equality": sum(1 for o in out if o.get("tight") == 0)}))
