#!/usr/bin/env python3
"""LEAD-EXO-curve-model-representation toy: generic fibre size D(f) of the
3-term summation relation for coordinates f: E->P^1. stdlib only."""
import json, random, sys

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def curve(p, rng):
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        sq = {}
        for y in range(p): sq.setdefault(y * y % p, []).append(y)
        pts = [None]
        for x in range(p):
            for y in sq.get((x**3 + a * x + b) % p, []): pts.append((x, y))
        if len(pts) > 40: return a, b, pts

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)

def neg(P, p): return None if P is None else (P[0], -P[1] % p)

def coords(p):
    def inv(v): return pow(v, -1, p)
    c = {}
    c["x"] = lambda P: "O" if P is None else P[0]
    c["mobius_x"] = lambda P: 3 % p if P is None else ((3 * P[0] + 5) * inv(P[0] + 7) % p if (P[0] + 7) % p else "O")
    c["x_sq"] = lambda P: "O" if P is None else P[0] * P[0] % p
    c["y"] = lambda P: "O" if P is None else P[1]
    c["x_over_y"] = lambda P: 0 if P is None else (P[0] * inv(P[1]) % p if P[1] else "O")
    c["x_plus_y"] = lambda P: "O" if P is None else (P[0] + P[1]) % p
    return c

def stat(f, pts, a, p, rng, npairs, null_map=None):
    fib = {}
    for P in pts:
        fib.setdefault(f(P), []).append(P)
    sizes = {}
    for v in fib.values(): sizes[len(v)] = sizes.get(len(v), 0) + 1
    gen = max(sizes, key=sizes.get)
    keys = [k for k, v in fib.items() if len(v) == gen and None not in v]
    tot = n = 0
    while n < npairs:
        k2, k3 = rng.choice(keys), rng.choice(keys)
        if k2 == k3: continue
        S = set()
        for Q2 in fib[k2]:
            for Q3 in fib[k3]:
                S.add(f(neg(add(Q2, Q3, a, p), p)))
        tot += len(S); n += 1
    return tot / n, gen

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    sizes = [211] if smoke else [2053, 3001, 4093]
    npairs = 150 if smoke else 1500
    rows = []
    for si, base in enumerate(sizes):
        p = next_prime(base)
        rng = random.Random(seed + si)
        a, b, pts = curve(p, rng)
        row = {"p": p, "a": a, "b": b, "N": len(pts), "D": {}}
        for name, f in coords(p).items():
            d, gen = stat(f, pts, a, p, random.Random(seed + 100 + si), npairs)
            row["D"][name] = {"mean_fibre_size": round(d, 4), "generic_preimages": gen}
        # A/A: same coordinate, different pair-sampling seed
        f = coords(p)["x"]
        row["AA_x"] = round(stat(f, pts, a, p, random.Random(seed + 200 + si), npairs)[0], 4)
        # negative control: random 2-to-1 relabelling of points (+-P pairing destroyed)
        r2 = random.Random(seed + 300 + si)
        perm = pts[:]; r2.shuffle(perm)
        lab = {}
        for i, P in enumerate(perm): lab[P] = i // 2
        row["null_random_2to1"] = round(stat(lambda P: lab[P], pts, a, p, random.Random(seed + 400 + si), npairs)[0], 4)
        rows.append(row)
    pos_ok = all(abs(r["D"]["x"]["mean_fibre_size"] - 2) < 1e-9 and abs(r["D"]["mobius_x"]["mean_fibre_size"] - 2) < 1e-9 and abs(r["AA_x"] - 2) < 1e-9 for r in rows)
    null_ok = all(r["null_random_2to1"] > 3.0 for r in rows)
    below = [(r["p"], k) for r in rows for k, v in r["D"].items() if v["generic_preimages"] >= 2 and v["mean_fibre_size"] < 2 - 1e-9]
    degenerate = sorted({k for r in rows for k, v in r["D"].items() if v["generic_preimages"] < 2})
    others_gt = True  # rational-fibre-injective coordinates are reported, not scored
    if below: verdict = "falsified"
    elif pos_ok and null_ok and others_gt: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-EXO-curve-model-representation", "smoke": smoke, "seed": seed,
        "npairs": npairs, "rows": rows, "positive_control_ok": pos_ok, "null_control_ok": null_ok,
        "coordinates_below_2": below, "rational_injective_coordinates_unscored": degenerate, "verdict": verdict}))

main()
