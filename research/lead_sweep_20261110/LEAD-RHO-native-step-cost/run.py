#!/usr/bin/env python3
"""LEAD-RHO-native-step-cost: counted field-op price of a Koblitz (F_2^m) rho step with and
without the Frobenius+negation canonicaliser, and net work to first class collision. stdlib only."""
import json, sys, random, statistics, math

POLY = {9: (9, 4, 0), 13: (13, 4, 3, 1, 0), 15: (15, 1, 0), 17: (17, 3, 0), 19: (19, 5, 2, 1, 0), 21: (21, 2, 0)}

class F:
    def __init__(s, m):
        s.m = m; s.mod = sum(1 << e for e in POLY[m]); s.mask = (1 << m) - 1
        s.c = {"mul": 0, "sq": 0, "inv": 0}
    def red(s, r):
        m = s.m
        while r.bit_length() > m:
            r ^= s.mod << (r.bit_length() - 1 - m)
        return r
    def mul(s, a, b):
        s.c["mul"] += 1; r = 0
        while b:
            if b & 1: r ^= a
            a <<= 1; b >>= 1
        return s.red(r)
    def sq(s, a):
        s.c["sq"] += 1; r = 0; i = 0
        while a:
            if a & 1: r |= 1 << (2 * i)
            a >>= 1; i += 1
        return s.red(r)
    def inv(s, a):
        s.c["inv"] += 1
        u, v, g1, g2 = a, s.mod, 1, 0
        while u != 1:
            j = u.bit_length() - v.bit_length()
            if j < 0: u, v, g1, g2, j = v, u, g2, g1, -j
            u ^= v << j; g1 ^= g2 << j
        return s.red(g1)

def make_curve(m, a):
    f = F(m)
    mu = 1 if a == 1 else -1
    V = [2, mu]
    for k in range(2, m + 1): V.append(mu * V[-1] - 2 * V[-2])
    return f, a, (1 << m) + 1 - V[m]

def add(f, a, P, Q):
    """affine add on y^2+xy=x^3+a x^2+1; None = infinity"""
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if y1 != y2 or x1 == 0: return None
        l = x1 ^ f.mul(y1, f.inv(x1))
        x3 = f.sq(l) ^ l ^ a
        y3 = f.sq(x1) ^ f.mul(l ^ 1, x3)
        return (x3, y3)
    l = f.mul(y1 ^ y2, f.inv(x1 ^ x2))
    x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ a
    y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)

def on_curve(f, a, P):
    x, y = P
    x2 = f.sq(x)
    return f.sq(y) ^ f.mul(x, y) == f.mul(x2, x) ^ (x2 if a else 0) ^ 1

def rand_point(f, a, rng):
    m = f.m
    while True:
        x = rng.randrange(1, 1 << m)
        x2 = f.sq(x)
        c = x2 ^ (a * 1 and 1 and 0)  # placeholder replaced below
        # y^2+xy=x^3+a x^2+1 -> z=y/x: z^2+z = x + a + 1/x^2
        c = x ^ a ^ f.inv(x2)
        z = c; t = c
        for _ in range((m - 1) // 2):
            t = f.sq(f.sq(t)); z ^= t
        if f.sq(z) ^ z == c:
            return (x, f.mul(z, x))

def canon(f, P):
    """class rep under Frobenius and negation: min x over orbit; sign by min y."""
    x, y = P; best = (x, y); xx, yy = x, y
    for _ in range(f.m - 1):
        xx = f.sq(xx); yy = f.sq(yy)
        if xx < best[0]: best = (xx, yy)
    x, y = best
    return (x, min(y, y ^ x))

def walk_steps(m, a, seed, use_canon, cap):
    """steps to first repeated state; returns (steps, op counts) averaged-per-step data."""
    rng = random.Random(seed)
    f, a, N = make_curve(m, a)
    G = rand_point(f, a, rng)
    T = []
    while len(T) < 16:
        k = rng.randrange(2, 1 << m)
        R = None; B = G
        e = k
        while e:
            if e & 1: R = add(f, a, R, B)
            B = add(f, a, B, B); e >>= 1
        if R is not None and R[0]: T.append(R)
    f.c = {"mul": 0, "sq": 0, "inv": 0}
    P = rand_point(f, a, rng)
    if use_canon: P = canon(f, P)
    seen = {P}
    steps = 0
    while steps < cap:
        Pn = add(f, a, P, T[P[0] & 15])
        if Pn is None: return None
        if use_canon: Pn = canon(f, Pn)
        steps += 1
        if Pn in seen: return steps, dict(f.c)
        seen.add(Pn); P = Pn
    return None

def cost(c, steps, wS, wI, wC):
    """mult-equivalents per step. Base: mul=1, sq=0.1 (priced), inv=wI. Canon squarings are the
    unpriced native work: counted via ops beyond the plain ones, weight wC."""
    return None

def price(plain_ops_per_step, extra_sq_per_step, wI, wC):
    mu, sq, inv = plain_ops_per_step
    return mu + 0.1 * sq + wI * inv + wC * extra_sq_per_step

def boot_ci(xs, rng, B=400):
    ms = sorted(statistics.median([rng.choice(xs) for _ in xs]) for _ in range(B))
    return ms[int(.025 * B)], ms[int(.975 * B)]

def main():
    smoke = "--smoke" in sys.argv
    sizes = [9] if smoke else [13, 15, 17, 19, 21]
    nseed = 4 if smoke else 24
    a = 1
    out = {"lead": "LEAD-RHO-native-step-cost", "params": {"sizes_m": sizes, "a": a, "seeds_per_size": nseed,
           "base_seed": 20261110, "inv_weights": [3, 10], "canon_sq_weights": [0, 0.1, 1.0], "smoke": smoke}, "rows": []}
    brng = random.Random(7)
    verdict_cells = []
    pos_ok = True; aa_ok = True
    for m in sizes:
        arms = {"plain": [], "canon": [], "plain_aa": []}
        ops = {"plain": [0, 0, 0, 0], "canon": [0, 0, 0, 0]}
        for i in range(nseed):
            for arm, uc, off in (("plain", 0, 0), ("canon", 1, 1), ("plain_aa", 0, 2)):
                if arm == "plain_aa":
                    r = walk_steps(m, a, 1000 * m + i, 0, 1 << 22) if False else walk_steps(m, a, 5000000 + 1000 * m + i, 0, 1 << 22)
                else:
                    r = walk_steps(m, a, 1000 * m + i, uc, 1 << 22)
                if r is None: continue
                arms[arm].append(r[0])
                if arm != "plain_aa":
                    c = r[1]; o = ops[arm]; o[0] += c["mul"]; o[1] += c["sq"]; o[2] += c["inv"]; o[3] += r[0]
        if not all(arms[k] for k in arms): continue
        # same seed means same curve/pool/start; plain_aa uses fresh seeds (A/A)
        sp = statistics.median(arms["plain"]); sc = statistics.median(arms["canon"]); sa = statistics.median(arms["plain_aa"])
        step_ratio = sc / sp; aa = sa / sp
        po = [ops["plain"][k] / ops["plain"][3] for k in range(3)]
        co = [ops["canon"][k] / ops["canon"][3] for k in range(3)]
        extra_sq = co[1] - po[1]
        cells = []
        for wI in (3, 10):
            for wC in (0, 0.1, 1.0):
                cp = price(po, 0, wI, 0)
                cc = price([co[0], co[1] - extra_sq, co[2]], extra_sq, wI, wC)
                R = step_ratio * cc / cp
                cells.append({"wI": wI, "wC": wC, "R_net": round(R, 4)})
        lo, hi = boot_ci([x / sp for x in arms["canon"]], brng)
        out["rows"].append({"m": m, "median_steps_plain": sp, "median_steps_canon": sc, "step_ratio": round(step_ratio, 4),
            "step_ratio_ci": [round(lo, 4), round(hi, 4)], "aa_ratio": round(aa, 4),
            "ops_per_step_plain": [round(v, 3) for v in po], "ops_per_step_canon": [round(v, 3) for v in co],
            "extra_canon_sq_per_step": round(extra_sq, 3), "cells": cells,
            "n_valid": {k: len(v) for k, v in arms.items()}})
        if m >= 17:
            verdict_cells += [c["R_net"] for c in cells if c["wC"] <= 0.1]
            pos_ok &= step_ratio < 0.5
        aa_ok &= 0.7 < aa < 1.4
    out["controls"] = {"A_A_plain_vs_plain_within_0.7_1.4": aa_ok,
                       "positive_control_step_ratio_lt_0.5_at_m>=17": pos_ok if not smoke else None,
                       "negative_control": "plain arm (no canonicaliser) is the null with R_net=1 by construction"}
    if smoke or not verdict_cells: v = "inconclusive"
    elif not aa_ok or not pos_ok: v = "inconclusive"
    elif max(verdict_cells) < 1.0: v = "consistent_with_prediction"
    elif min(verdict_cells) >= 1.0: v = "falsified"
    else: v = "inconclusive"
    out["verdict"] = v
    out["caveat"] = "[pcc:unverified] counted ops, first-repeat step count, not an end-to-end DLP solve"
    print(json.dumps(out))

main()
