#!/usr/bin/env python3
"""Toy for LEAD-ICP-prime-ic-constant-engineering: counted-op effect of batched
affine inversion (Montgomery trick) on an IC-style pair-table walk. stdlib only."""
import json, random, sys, time

def mk_curve(rng, bits):
    while True:
        p = rng.getrandbits(bits) | (1 << (bits - 1)) | 3
        if p % 4 != 3 or any(p % q == 0 for q in range(2, int(p**.5) + 1)):
            continue
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p:
            return p, a, b

def rpt(rng, p, a, b):
    while True:
        x = rng.randrange(p); r = (x**3 + a * x + b) % p
        y = pow(r, (p + 1) // 4, p)
        if y * y % p == r:
            return (x, y)

def add(P, Q, p, a, inv):  # affine add/double, returns None for infinity
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * inv(2 * y1) % p
    else:
        l = (y2 - y1) * inv(x2 - x1) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def run_arm(p, a, b, table, walks, steps_pts, W, nsteps, planted=None):
    """W walks in lockstep; batch = group size sharing one inversion (1 = baseline)."""
    cnt = {"I": 0, "M": 0}
    pts = list(walks); hits = []
    for t in range(nsteps):
        for g in range(0, len(pts), W):
            idx = range(g, min(g + W, len(pts)))
            dens, ok = [], []
            for i in idx:
                S = steps_pts[(pts[i][0] + t) % len(steps_pts)]
                d = (S[0] - pts[i][0]) % p
                ok.append(d != 0); dens.append(d if d else 1)
            if len(dens) == 1:
                invs = [pow(dens[0], -1, p)]; cnt["I"] += 1; cnt["M"] += 3
            else:  # Montgomery batch inversion: 3(n-1) M + 1 I; plus 3 M per add
                pre = [1]
                for d in dens: pre.append(pre[-1] * d % p)
                iv = pow(pre[-1], -1, p); invs = [0] * len(dens)
                for k in range(len(dens) - 1, -1, -1):
                    invs[k] = iv * pre[k] % p; iv = iv * dens[k] % p
                cnt["I"] += 1; cnt["M"] += 3 * (len(dens) - 1) + 3 * len(dens)
            for j, i in enumerate(idx):
                S = steps_pts[(pts[i][0] + t) % len(steps_pts)]
                if ok[j]:
                    x1, y1 = pts[i]; l = (S[1] - y1) * invs[j] % p
                    x3 = (l * l - x1 - S[0]) % p
                    pts[i] = (x3, (l * (x1 - x3) - y1) % p)
                else:
                    pts[i] = add(pts[i], S, p, a, lambda v: pow(v, -1, p)) or pts[i]
                if pts[i][0] in table:
                    hits.append((t, i, pts[i][0]))
    return cnt, hits, pts

def cost(cnt, ratio): return cnt["M"] + ratio * cnt["I"]

def cell(seed, bits, B, nwalk, nsteps, W):
    rng = random.Random(seed)
    p, a, b = mk_curve(rng, bits)
    fb = [rpt(rng, p, a, b) for _ in range(B)]
    table = set()
    for i in range(B):
        for j in range(i, B):
            for s in (1, -1):
                Q = fb[j] if s == 1 else (fb[j][0], -fb[j][1] % p)
                R = add(fb[i], Q, p, a, lambda v: pow(v, -1, p))
                if R: table.add(R[0])
    steps_pts = [rpt(rng, p, a, b) for _ in range(32)]
    walks = [rpt(rng, p, a, b) for _ in range(nwalk)]
    # planted positive control: x of walk 0 after 1 step is forced into table
    S = steps_pts[(walks[0][0]) % 32]
    P1 = add(walks[0], S, p, a, lambda v: pow(v, -1, p))
    planted = P1[0] if P1 else None
    if planted is not None: table.add(planted)
    out = {}
    for name, w, rep in (("base", 1, 0), ("batched", W, 0), ("base_AA", 1, 1), ("batch1", 1, 2)):
        t0 = time.perf_counter()
        c, h, _ = run_arm(p, a, b, table, walks, steps_pts, w, nsteps)
        out[name] = {"cnt": c, "hits": h, "wall": time.perf_counter() - t0}
    return p, planted, out

def main():
    smoke = "--smoke" in sys.argv
    sizes = [11] if smoke else [12, 14, 16]
    B, nwalk, nsteps, W = (8, 16, 20, 16) if smoke else (24, 64, 400, 64)
    prices = [5, 10, 20]; seeds = [101, 202, 303]
    cells = []; fail = []
    for bits in sizes:
        for sd in (seeds[:1] if smoke else seeds):
            p, planted, o = cell(sd * 1000 + bits, bits, B, nwalk, nsteps, W)
            same = o["base"]["hits"] == o["batched"]["hits"] == o["base_AA"]["hits"]
            pos = planted is not None and any(h[0] == 0 and h[1] == 0 for h in o["base"]["hits"])
            neg = o["batch1"]["cnt"] == o["base"]["cnt"]
            r = {str(c): cost(o["batched"]["cnt"], c) / cost(o["base"]["cnt"], c) for c in prices}
            aa = o["base_AA"]["wall"] / max(o["base"]["wall"], 1e-9)
            cells.append({"bits": bits, "seed": sd, "p": p, "hits": len(o["base"]["hits"]),
                          "hits_identical": same, "positive_control_found": pos,
                          "batch1_equals_base_counts": neg, "cost_ratio": r,
                          "wall_ratio_batched_over_base": o["batched"]["wall"] / max(o["base"]["wall"], 1e-9),
                          "AA_wall_ratio": aa})
            if not (same and pos and neg): fail.append((bits, sd))
    rs = {c: [x["cost_ratio"][str(c)] for x in cells] for c in prices}
    spread = {str(c): max(v) - min(v) for c, v in rs.items()}
    aa_ok = all(0.8 <= x["AA_wall_ratio"] <= 1.25 for x in cells)
    if fail: verdict = "inconclusive"
    elif any(max(v) >= 1.0 for v in rs.values()) or max(spread.values()) > 0.05: verdict = "falsified"
    elif not aa_ok and not smoke: verdict = "inconclusive"
    else: verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-ICP-prime-ic-constant-engineering", "smoke": smoke,
        "params": {"sizes_bits": sizes, "B": B, "walks": nwalk, "steps": nsteps, "W": W,
                   "I_over_M": prices, "seeds": seeds}, "statistic": {"max_cost_ratio": {str(c): max(v) for c, v in rs.items()},
        "ratio_spread_across_cells": spread}, "controls_failed": fail, "AA_wall_within_band": aa_ok,
        "cells": cells, "verdict": verdict}))
main()
