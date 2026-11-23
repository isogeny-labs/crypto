#!/usr/bin/env python3
"""Validator: 'subflat' presents the same base as 'sub'. Own arithmetic in discrete-log space (w = smallest primitive
root of p): the chain y0^e0 = y1, ..., y_{L-1}^e_{L-1} = 1 (e_j the prime factors of D, ascending) has solution set
{w^k : k in K_0} with K_{L-1} = {k : e_{L-1} k = 0 mod p-1}, K_j = {k : e_j k mod (p-1) in K_{j+1}}; the flat
constraint y^D = 1 gives {k : D k = 0 mod p-1}. Abscissae x = A/3 + y (Montgomery frame, gidx 0). Also counts the
columns (abscissae on E) and signed base points with Euler's criterion, and runs the engine's subcensus for both
kinds to compare its counts.  Usage: l3_subflat_set.py <out.json> <p> <A> <K-or-explicit> D1 [D2 ...]"""
import json, subprocess, sys
BIN = "/home/user/ctarget-f1/release/curve25519_family"
out_path, p, A, curve = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]; Ds = [int(x) for x in sys.argv[5:]]
def factor(n):
    o, f = [], 2
    while f * f <= n:
        while n % f == 0: o.append(f); n //= f
        f += 1
    if n > 1: o.append(n)
    return o
N = p - 1; qs = sorted(set(factor(N))); w = 2
while any(pow(w, N // q, p) == 1 for q in qs): w += 1
shift = A * pow(3, -1, p) % p
def roots_of(e, Kset):  # {k : e*k mod N in Kset}
    from math import gcd
    g = gcd(e, N); out = set(); Ng = N // g; einv = pow(e // g, -1, Ng)
    for t in Kset:
        if t % g: continue
        k0 = (t // g) * einv % Ng
        out.update(k0 + i * Ng for i in range(g))
    return out
rows = []
for D in Ds:
    fs = factor(D); K = roots_of(fs[-1], {0})
    for e in reversed(fs[:-1]): K = roots_of(e, K)
    Kflat = roots_of(D, {0})
    chain_x = sorted((shift + pow(w, k, p)) % p for k in K); flat_x = sorted((shift + pow(w, k, p)) % p for k in Kflat)
    us = [pow(w, k, p) for k in Kflat]
    rhs = [(u ** 3 + A * u * u + u) % p for u in us]
    cols = sum(1 for v in rhs if v == 0 or pow(v, N // 2, p) == 1); zeros = sum(1 for v in rhs if v == 0)
    bp = 2 * (cols - zeros) + zeros
    mmax = 3 if D <= 36 else 2
    eng = {}
    for kind in ("sub", "subflat"):
        r = subprocess.run([BIN, "subcensus", curve, kind, str(D), "0", "mont", str(mmax)], capture_output=True, text=True, timeout=600)
        eng[kind] = json.loads(r.stdout.strip().splitlines()[-1])
    strip = lambda j: {k: v for k, v in j.items() if k != "base"}
    rows.append({"D": D, "factors": fs, "chain_set_size": len(K), "flat_set_size": len(Kflat), "same_abscissae": chain_x == flat_x,
                 "own_columns": cols, "own_base_points": bp,
                 "engine_sub": eng["sub"], "engine_subflat": eng["subflat"],
                 "engine_kinds_agree": strip(eng["sub"]) == strip(eng["subflat"]),
                 "engine_matches_own_counts": all(eng[k]["v_size"] == D and eng[k]["columns"] == cols and eng[k]["base_points"] == bp for k in eng)})
res = {"p": p, "A": A, "prim_root": w, "shift": shift, "rows": rows,
       "all_ok": all(r["same_abscissae"] and r["chain_set_size"] == r["flat_set_size"] == r["D"] and r["engine_kinds_agree"] and r["engine_matches_own_counts"] for r in rows)}
json.dump(res, open(out_path, "w"), indent=1)
for r in rows: print(r["D"], r["factors"], r["same_abscissae"], r["chain_set_size"], r["own_columns"], r["own_base_points"], r["engine_kinds_agree"], r["engine_matches_own_counts"], r["engine_sub"]["covered_points"])
print("all_ok", res["all_ok"])
