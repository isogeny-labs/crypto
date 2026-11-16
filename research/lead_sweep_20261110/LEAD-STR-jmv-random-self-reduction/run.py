#!/usr/bin/env python3
"""Toy for LEAD-STR-jmv-random-self-reduction. stdlib only, deterministic."""
import json, math, random, sys

def walk_hits(h, gens, L, inS, n, rng):
    hits = 0
    ng = len(gens)
    for _ in range(n):
        x = 0
        for _ in range(L):
            g = gens[rng.randrange(ng)]
            x = (x + g) % h if rng.getrandbits(1) else (x - g) % h
        hits += inS[x]
    return hits

def make_set(h, delta, kind, rng):
    s = bytearray(h)
    m = int(delta * h)
    if kind == "random":
        for i in rng.sample(range(h), m):
            s[i] = 1
    else:  # interval centred on the start curve (adversarial, local)
        for i in range(-(m // 2), m - m // 2):
            s[i % h] = 1
    return s, m / h

def cell(h, k, kind, delta, n, seed):
    rng = random.Random(seed)
    gens = [1] if k == 1 else [rng.randrange(1, h) for _ in range(k)]
    L = 4 * math.ceil(math.log2(h))
    inS, d = make_set(h, delta, kind, rng)
    h1 = walk_hits(h, gens, L, inS, n // 2, random.Random(seed + 1))
    h2 = walk_hits(h, gens, L, inS, n - n // 2, random.Random(seed + 2))
    r1, r2 = h1 / (n // 2) / d, h2 / (n - n // 2) / d
    return {"h": h, "k": k, "kind": kind, "delta": d, "L": L, "walks": n,
            "ratio": (h1 + h2) / n / d, "ratio_half1": r1, "ratio_half2": r2,
            "aa_gap": abs(r1 - r2)}

def main():
    smoke = "--smoke" in sys.argv
    hs = [1021] if smoke else [16411, 65537, 262147]
    n = 400 if smoke else 20000
    delta = 0.05
    tol = 0.30 if smoke else 0.15
    exp, ctl = [], []
    for i, h in enumerate(hs):
        k = math.ceil(math.log2(h))
        for j, kind in enumerate(("random", "interval")):
            exp.append(cell(h, k, kind, delta, n, 1000 * i + 10 * j + 7))
        ctl.append(cell(h, 1, "interval", delta, n, 1000 * i + 99))   # positive control
        ctl.append(cell(h, k, "random", delta, n, 1000 * i + 55))     # null already in exp; extra seed
    pos = [c for c in ctl if c["k"] == 1]
    pos_ok = all(c["ratio"] > 3 for c in pos)
    aa_ok = all(c["aa_gap"] <= 2 * tol for c in exp)
    exp_ok = all(abs(c["ratio"] - 1) <= tol for c in exp)
    if not pos_ok or not aa_ok:
        v = "inconclusive"
    else:
        v = "consistent_with_prediction" if exp_ok else "falsified"
    print(json.dumps({"lead": "LEAD-STR-jmv-random-self-reduction", "smoke": smoke,
        "statistic": "P_hit(L)/delta over random walks on Cayley graph Z/h",
        "threshold": {"expander_abs_dev": tol, "positive_control_min_ratio": 3},
        "parameters": {"h_list": hs, "walks": n, "delta": delta, "L": "4*ceil(log2 h)",
                       "k": "ceil(log2 h) generators (expander), 1 (control)"},
        "seeds": "1000*i+{7,17,99,55}, +1/+2 for A/A halves",
        "expander_arms": exp, "positive_control": pos, "extra_null": [c for c in ctl if c["k"] > 1],
        "positive_control_ok": pos_ok, "aa_ok": aa_ok, "expander_ok": exp_ok,
        "verdict": v}))
main()
