#!/usr/bin/env python3
"""Toy Bernstein-Lange DP precomputation vs table-free rho in Z_N (N prime), stdlib only."""
import json, math, random, sys

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def prime_near(bits):
    n = (1 << bits) + 1
    while not is_prime(n): n += 2
    return n

def make_walk(N, D, rng):
    jumps = [rng.randrange(1, N) for _ in range(16)]
    def step(p, a):
        h = ((p * 2654435761) >> 7) & 15
        return (p + jumps[h]) % N, (a + jumps[h]) % N
    return step

def walk_to_dp(step, p, a, D, cap):
    n = 0
    while n < cap:
        if p % D == 0: return p, a, n
        p, a = step(p, a); n += 1
    return None, None, n

def bl_trial(N, x, step, table, D, rng, budget):
    """online: restart walks from x+a0 until a DP is found in the table."""
    steps = 0
    cap = 20 * D
    while steps < budget:
        a0 = rng.randrange(N)
        p, a, n = walk_to_dp(step, (x + a0) % N, a0, D, cap)
        steps += n + 1
        if p is not None and p in table:
            xs = (table[p] - a) % N
            if xs == x: return steps, True
    return steps, False

def null_trial(N, x, step, D, rng, budget):
    """table-free two-sided DP rho: type0 trails from aG, type1 from xG+aG."""
    seen = {}
    steps = 0; cap = 20 * D
    while steps < budget:
        for t in (0, 1):
            a0 = rng.randrange(N)
            p, a, n = walk_to_dp(step, (a0 + t * x) % N, a0, D, cap)
            steps += n + 1
            if p is None: continue
            if p in seen and seen[p][0] != t:
                a1 = seen[p][1]
                xs = ((a1 - a) if t == 1 else (a - a1)) % N
                # t==1: x + a = a1 -> x = a1-a ; t==0 stored type1: x+a1=a -> x=a-a1
                if xs == x: return steps, True
            seen[p] = (t, a)
    return steps, False

def fit_slope(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return sum((a-mx)*(b-my) for a,b in zip(xs,ys)) / sum((a-mx)**2 for a in xs)

def mean(v): return sum(v)/len(v)

def run(bits_list, trials, seed):
    res = {"bl": [], "null": [], "pos": []}
    rows = []
    for bits in bits_list:
        N = prime_near(bits)
        rng = random.Random(seed * 1000 + bits)
        D = max(2, round(N ** (1/3)))
        m = max(2, round(N ** (1/3)))
        cap = 20 * D
        step = make_walk(N, D, rng)
        table = {}; pre = 0; made = 0
        while made < m:
            a0 = rng.randrange(N)
            p, a, n = walk_to_dp(step, a0, a0, D, cap)
            pre += n
            if p is not None: table[p] = a; made += 1
        # positive control: table 8x larger on same walk (online must shrink)
        table8 = dict(table); made = 0
        while made < 7 * m:
            a0 = rng.randrange(N)
            p, a, n = walk_to_dp(step, a0, a0, D, cap)
            if p is not None: table8[p] = a; made += 1
        budget = int(60 * math.sqrt(N))
        sb, sn, sp, fails = [], [], [], 0
        for t in range(trials):
            x = rng.randrange(1, N)
            s, ok = bl_trial(N, x, step, table, D, rng, budget); sb.append(s); fails += (not ok)
            s, ok = null_trial(N, x, step, D, rng, budget); sn.append(s); fails += (not ok)
            s, ok = bl_trial(N, x, step, table8, D, rng, budget); sp.append(s); fails += (not ok)
        rows.append({"bits": bits, "N": N, "D": D, "table_entries": len(table), "precompute_steps": pre,
                     "online_mean_bl": mean(sb), "online_mean_null": mean(sn), "online_mean_table8x": mean(sp),
                     "failures": fails})
    lx = [math.log2(r["N"]) for r in rows]
    sl = {k: fit_slope(lx, [math.log2(r[f]) for r in rows])
          for k, f in (("bl","online_mean_bl"),("null","online_mean_null"),("pos","online_mean_table8x"))}
    pre_sl = fit_slope(lx, [math.log2(r["precompute_steps"]) for r in rows])
    return rows, sl, pre_sl

def main():
    smoke = "--smoke" in sys.argv
    bits = [10, 12] if smoke else [12, 14, 16, 18, 20, 22]
    trials = 12 if smoke else 150
    rows, sl, pre_sl = run(bits, trials, 20261110)
    # A/A: same protocol, different seed
    _, sl2, _ = run(bits, trials, 7777) if not smoke else (None, sl, None)
    aa = abs(sl["bl"] - sl2["bl"])
    ratios = [r["online_mean_bl"] * r["table_entries"] * r["D"] / r["N"] for r in rows]
    bl_ok = 0.25 <= sl["bl"] <= 0.42
    null_ok = 0.42 <= sl["null"] <= 0.58
    pos_ok = all(r["online_mean_table8x"] < r["online_mean_bl"] for r in rows)
    aa_ok = aa < 0.08
    fails = sum(r["failures"] for r in rows)
    if smoke: verdict = "inconclusive"
    elif not (null_ok and pos_ok and aa_ok): verdict = "inconclusive"
    elif bl_ok: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-RHO-dp-multitarget-precomputation", "smoke": smoke, "seeds": [20261110, 7777],
        "bits": bits, "trials_per_size": trials, "rows": rows,
        "slope_online_bl": sl["bl"], "slope_online_null": sl["null"], "slope_online_table8x": sl["pos"],
        "slope_precompute": pre_sl, "aa_slope_diff": aa, "tradeoff_ratio_online_x_M_over_N": ratios,
        "controls": {"null_ok": null_ok, "positive_ok": pos_ok, "aa_ok": aa_ok}, "failures": fails,
        "verdict": verdict}))
main()
