#!/usr/bin/env python3
"""Toy Macaulay-closure refutation degree for Weil-descended Semaev S_3 over GF(2^n), n prime.
stdlib only; deterministic seeds."""
import sys, json, random, time, itertools
POLY = {7: (1 << 7) | 0b11, 11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011}

def fmul(a, b, n):
    p = POLY[n]; r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n: a ^= p
    return r

def padd(P, Q):
    R = dict(P)
    for m, c in Q.items():
        v = R.get(m, 0) ^ c
        if v: R[m] = v
        else: R.pop(m, None)
    return R

def pmul(P, Q, n):
    R = {}
    for m1, c1 in P.items():
        for m2, c2 in Q.items():
            m = m1 | m2; c = fmul(c1, c2, n)
            v = R.get(m, 0) ^ c
            if v: R[m] = v
            else: R.pop(m, None)
    return R

def lin(vecs, off, n):  # sum_i var_{off+i} * vecs[i]
    return {1 << (off + i): v for i, v in enumerate(vecs)}

def indep_basis(rng, n, l):
    while True:
        B = [rng.randrange(1, 1 << n) for _ in range(l)]
        span = {0}
        for b in B:
            if b in span: break
            span |= {s ^ b for s in span}
        else:
            return B

def semaev_system(n, l, X, B, b0=1):
    """S_3(x1,x2,X)=0, x1,x2 in span(B). Returns list of n polys over GF(2): sets of monomial masks."""
    x1 = lin(B, 0, n); x2 = lin(B, l, n)
    Xp = {0: X}
    e2 = padd(padd(pmul(x1, x2, n), pmul(x1, Xp, n)), pmul(x2, Xp, n))
    S = padd(padd(pmul(e2, e2, n), pmul(pmul(x1, x2, n), Xp, n)), {0: b0})
    return [{m for m, c in S.items() if (c >> j) & 1} for j in range(n)]

def solvable(polys, N):
    for a in range(1 << N):
        ok = True
        for P in polys:
            s = 0
            for m in P:
                if m & ~a == 0: s ^= 1
            if s: ok = False; break
        if ok: return True
    return False

def monos(N, d):
    out = []
    for k in range(d + 1):
        for c in itertools.combinations(range(N), k):
            out.append(sum(1 << i for i in c))
    return out

def refutes(polys, N, D):
    """True if 1 lies in the degree-D Macaulay span (boolean ring, Gaussian elimination)."""
    dmax = max((bin(m).count("1") for P in polys for m in P), default=0)
    mult = monos(N, max(0, D - dmax))
    cols = {m: i for i, m in enumerate(sorted(monos(N, D), key=lambda m: (-bin(m).count("1"), m)))}
    basis = {}
    for P in polys:
        for t in mult:
            acc = set()
            for m in P:
                acc ^= {m | t}
            if not acc: continue
            if any(m not in cols for m in acc): continue
            row = 0
            for m in acc: row |= 1 << cols[m]
            while row:
                h = row.bit_length() - 1
                if h in basis: row ^= basis[h]
                else: basis[h] = row; break
    one = 1 << cols[0]
    r = one
    while r:
        h = r.bit_length() - 1
        if h in basis: r ^= basis[h]
        else: return False
    return True

def random_polys(rng, N, n, dmax=4):
    ms = [m for m in monos(N, dmax)]
    return [{m for m in ms if rng.random() < 0.5 and m} for _ in range(n)]

def arm(n, l, seeds, Ds, kind="semaev"):
    N = 2 * l; rows = []
    for s in seeds:
        rng = random.Random(s)
        if kind == "semaev":
            B = indep_basis(rng, n, l); X = rng.randrange(1, 1 << n)
            polys = semaev_system(n, l, X, B)
        else:
            polys = random_polys(rng, N, n)
        sol = solvable(polys, N)
        ref = {D: refutes(polys, N, D) for D in Ds}
        rows.append((sol, ref))
    return rows

def stats(rows, Ds):
    uns = [r for r in rows if not r[0]]; sv = [r for r in rows if r[0]]
    return {"n_inst": len(rows), "n_unsolvable": len(uns), "n_solvable": len(sv),
            "refute_rate_unsolvable": {D: (sum(r[1][D] for r in uns) / len(uns) if uns else None) for D in Ds},
            "unsound_refutations_of_solvable": sum(r[1][D] for r in sv for D in Ds)}

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke:
        n, l, Ds, K = 7, 2, [4, 5], 6
        cfg = [(n, l)]
    else:
        Ds, K = [4, 5, 6], 40
        cfg = [(13, 5), (11, 5), (13, 6)]
    out = {"lead": "LEAD-ICB-prime-degree-algebraic-decomposition", "smoke": smoke, "Ds": Ds,
           "seeds": "primary 1000..1000+K, AA 5000..5000+K, null 9000..9000+K", "K": K, "cells": {}}
    thr = 0.5; D_star = 6
    verdicts = []
    for (n, l) in cfg:
        Kc = K if l < 6 else max(10, K // 4)
        A = arm(n, l, range(1000, 1000 + Kc), Ds)
        AA = arm(n, l, range(5000, 5000 + Kc), Ds)
        NU = arm(n, l, range(9000, 9000 + Kc), Ds, kind="random")
        sa, saa, sn = stats(A, Ds), stats(AA, Ds), stats(NU, Ds)
        out["cells"]["n%d_l%d" % (n, l)] = {"semaev": sa, "AA": saa, "null_random": sn}
        r = sa["refute_rate_unsolvable"][Ds[-1]]; r2 = saa["refute_rate_unsolvable"][Ds[-1]]
        rn = sn["refute_rate_unsolvable"][Ds[-1]]
        if r is None or r2 is None or sa["n_unsolvable"] < 5:
            verdicts.append("inconclusive")
        elif sa["unsound_refutations_of_solvable"] or saa["unsound_refutations_of_solvable"]:
            verdicts.append("inconclusive")
        elif abs(r - r2) > 0.25:
            verdicts.append("inconclusive")
        elif r >= thr and (rn is None or r > rn + 0.2):
            verdicts.append("consistent_with_prediction")
        elif r < thr:
            verdicts.append("falsified")
        else:
            verdicts.append("inconclusive")
    # positive control: heavily overdetermined random system must be refuted at D=Ds[-1]
    rng = random.Random(77); Np = 8
    pc = [refutes(random_polys(rng, Np, 24), Np, Ds[-1]) for _ in range(5 if smoke else 10)]
    out["positive_control_overdetermined_refuted_frac"] = sum(pc) / len(pc)
    if sum(pc) / len(pc) < 0.8 and not smoke:
        verdicts = ["inconclusive"]
    out["threshold"] = {"D_star": D_star if not smoke else Ds[-1], "min_rate": thr}
    out["per_cell_verdicts"] = verdicts
    if all(v == "consistent_with_prediction" for v in verdicts): v = "consistent_with_prediction"
    elif "falsified" in verdicts: v = "falsified"
    else: v = "inconclusive"
    out["verdict"] = v
    out["wall_s"] = round(time.time() - t0, 2)
    print(json.dumps(out))
main()
