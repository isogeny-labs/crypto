#!/usr/bin/env python3
"""Toy: generic k-SUM relation-collection cost for a structureless S_k oracle, group Z_N (stdlib only)."""
import json, math, random, sys, time

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def prime_near(bits, rng):
    while True:
        n = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if is_prime(n): return n

def combos(B, r):
    from itertools import combinations
    return combinations(range(B), r)

def cost_run(N, B, k, rng, planted_M, cap):
    """Meet-in-the-middle k-SUM: table of a-subsets, scan b-subsets (a=k//2, b=k-a).
    Cost = table inserts + scanned b-subsets + B^2 (linear algebra charge), until B relations."""
    if planted_M:
        fb = [rng.randrange(-planted_M, planted_M+1) % N for _ in range(B)]
    else:
        fb = [rng.randrange(N) for _ in range(B)]
    a, b = k//2, k-k//2
    tsize = math.comb(B, a)
    if tsize + B*B > cap: return None
    tab = {}
    for c in combos(B, a):
        tab.setdefault(sum(fb[i] for i in c) % N, []).append(c)
    ops = tsize
    rel = 0
    for c in combos(B, b):
        ops += 1
        if ops > cap: return None
        v = (-sum(fb[i] for i in c)) % N
        h = tab.get(v)
        if h:
            for t in h:
                if not set(t) & set(c): rel += 1
            if rel >= B: return ops + B*B
    return None

def best_cost(N, k, rng, planted_M, cap, grid):
    best = None
    for e in grid:
        B = max(k+2, int(round(N**e)))
        c = cost_run(N, B, k, rng, planted_M, cap)
        if c is not None and (best is None or c < best[0]): best = (c, B)
    return best

def slope(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return sum((x-mx)*(y-my) for x,y in zip(xs,ys)) / sum((x-mx)**2 for x in xs)

def arm(bits_list, k, seeds, planted_frac, cap, grid, tag):
    xs, ys, detail = [], [], []
    for bits in bits_list:
        costs = []
        for s in seeds:
            rng = random.Random(f"{tag}-{k}-{bits}-{s}")
            N = prime_near(bits, rng)
            M = int(N**planted_frac) if planted_frac else 0
            r = best_cost(N, k, rng, M, cap, grid)
            if r: costs.append(r[0] / 1.0)
        if costs:
            m = sum(costs)/len(costs)
            xs.append(bits); ys.append(math.log2(m)); detail.append([bits, round(math.log2(m),3), len(costs)])
    sl = slope(xs, ys) if len(xs) >= 3 else None
    return sl, detail

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke:
        bits_list, seeds, cap = [10,12,14], [1], 20000
        grid = [0.2,0.3,0.4]
    else:
        bits_list, seeds, cap = [12,14,16,18,20,22], [11,12,13], 400000
        grid = [0.15 + 0.03*i for i in range(11)]
    out = {"seeds": seeds, "bits": bits_list, "grid_exponents": grid, "cap_ops": cap, "arms": {}}
    ks = [5] if smoke else [3,4,5,6]
    for k in ks:
        sl, det = arm(bits_list, k, seeds, 0, cap, grid, "main")
        out["arms"][f"k{k}"] = {"slope": sl, "points": det}
    # A/A: disjoint seed set, k=5
    aa_seeds = [s+100 for s in seeds]
    sl2, det2 = arm(bits_list, 5, aa_seeds, 0, cap, grid, "aa")
    out["arms"]["k5_AA"] = {"slope": sl2, "points": det2}
    # positive control: planted structure (FB values in [-N^0.25, N^0.25]) must give a cheaper slope
    slp, detp = arm(bits_list, 5, seeds, 0.25, cap, grid, "pos")
    out["arms"]["k5_planted"] = {"slope": slp, "points": detp}
    s5 = out["arms"]["k5"]["slope"]
    ctrl_ok = (sl2 is not None and s5 is not None and slp is not None
               and abs(sl2 - s5) < 0.08 and slp < s5 - 0.10)
    out["controls"] = {"AA_abs_diff": None if (sl2 is None or s5 is None) else abs(sl2-s5),
                       "planted_slope": slp, "controls_pass": bool(ctrl_ok)}
    if smoke:
        verdict = "inconclusive"
    elif not ctrl_ok or s5 is None:
        verdict = "inconclusive"
    elif s5 < 0.55:
        verdict = "falsified"
    elif 0.55 <= s5 <= 0.80:
        k6 = out["arms"].get("k6", {}).get("slope")
        verdict = "consistent_with_prediction" if (k6 is None or k6 >= 0.5) else "falsified"
    else:
        verdict = "inconclusive"
    out.update({"statistic": "slope of log2(mean min-over-B relation-phase cost) vs log2 N, k=5",
                "prediction": "k=5 slope ~ 2/3 (in [0.55,0.80]); k=3:1, k=4:2/3, k=6:3/5; all >= 0.5",
                "verdict": verdict, "wall_s": round(time.time()-t0,2), "smoke": smoke})
    print(json.dumps(out))
    return 0

if __name__ == "__main__":
    sys.exit(main())
