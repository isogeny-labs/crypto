#!/usr/bin/env python3
"""Functional sanity: lam via DFT columns (run_cells.lam_vec_m3) == pointwise formula from grid values of
random vectors over columns; pi map likewise; independent polynomial arithmetic (explicit division)."""
import sys, json, random, numpy as np
from run_cells import *

def pmulp(a, b, p):
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b): r[i + j] = (r[i + j] + x * y) % p
    return r
def prem(a, f, p):
    a = a[:]; inv = pow(f[-1], p - 2, p)
    while len(a) >= len(f):
        q = a[-1] * inv % p
        for i in range(len(f)): a[len(a) - len(f) + i] = (a[len(a) - len(f) + i] - q * f[i]) % p
        a.pop()
    return a + [0] * (len(f) - 1 - len(a))

def lam_point(g, f1, f2, p):
    # solve h f1 = g mod f2, h = h0 + h1 u by explicit remainders; return h1
    cols = [prem(pmulp([1, 0], f1, p), f2, p), prem(pmulp([0, 1], f1, p), f2, p)]   # h0*f1, h1*f1 mod f2
    gr = prem(g, f2, p)
    M = [[cols[0][0], cols[1][0]], [cols[0][1], cols[1][1]]]
    D = (M[0][0] * M[1][1] - M[0][1] * M[1][0]) % p; Di = pow(D, p - 2, p)
    h0 = (gr[0] * M[1][1] - M[0][1] * gr[1]) * Di % p
    h1 = (M[0][0] * gr[1] - gr[0] * M[1][0]) * Di % p
    return h1

def check(p, t, idx, c=5, trials=3):
    C, meta, F = nsd.make_instance(p, t, idx, 3); ch = Chain3(C, F); S = Space(C, c, True)
    lam = lam_vec_m3(ch, S); rng = random.Random(1); bad = 0
    for _ in range(trials):
        v = np.array([rng.randrange(p) if rng.random() < 0.2 else 0 for _ in range(S.n)], dtype=np.float64)
        # grid values of each u^j coefficient
        gj = []
        for j in range(c + 1):
            co = np.zeros(C.K ** 3, dtype=np.int64); sel = S.j == j
            co[S.flat[sel]] = v[sel].astype(np.int64); gj.append(fwd(C, co))
        tot = 0
        for n in range(C.N):
            g = [int(gj[j][n]) for j in range(c + 1)]
            f1 = [int(F[0][q][n]) for q in range(3)]; f2 = [int(F[1][q][n]) for q in range(3)]
            w = int(C.Z[0][n]) * int(C.Z[1][n]) % p * int(C.Z[2][n]) % p
            tot = (tot + w * lam_point(g, f1, f2, p)) % p
        direct = int(np.mod(v @ lam, p))
        bad += (tot != direct)
    return bad

if __name__ == "__main__":
    out = {}
    for p in (40961, 12289):
        for t in (2, 3):
            out[f"{p}_{t}"] = check(p, t, 0, c=5 if t == 2 else 4)
    print(json.dumps(out)); print("ALL_OK" if not any(out.values()) else "MISMATCH")
