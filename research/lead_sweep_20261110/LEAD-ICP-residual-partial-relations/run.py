#!/usr/bin/env python3
"""Preregistered toy: does single-large-prime residual crediting change the exponent? stdlib only."""
import json, math, random, sys

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prime_near(n):
    k = 1 << n
    while not is_prime(k): k += 1
    return k

def arm_birthday(N, B, rng):
    """Single large prime: stored residuals R - F_i, B=0 means plain birthday on R. Returns ops to first useful collision (different v), solved x verified."""
    x = rng.randrange(1, N)
    F = [rng.randrange(N) for _ in range(B)]
    seen = {}
    ops = 0
    while True:
        u = rng.randrange(N); v = rng.randrange(1, N)
        ops += 1
        R = (u + v * x) % N
        pts = [R] if B == 0 else [(R - f) % N for f in F]
        ops += B
        for i, P in enumerate(pts):
            f = F[i] if B else 0
            o = seen.get(P)
            if o is not None and (o[1] - v) % N != 0:
                u2, v2, f2 = o
                # u+vx-f = u2+v2x-f2
                xs = ((u2 - f2 - u + f) * pow(v - v2, -1, N)) % N
                assert xs == x
                return ops
            seen[P] = (u, v, f)

def arm_table(N, B, l, rng):
    """Large primes restricted to a known table of size l (the 2LP-style table side)."""
    x = rng.randrange(1, N)
    F = [rng.randrange(N) for _ in range(B)]
    T = {}
    for _ in range(l):
        t = rng.randrange(N); T[t] = t
    ops = l
    while True:
        u = rng.randrange(N); v = rng.randrange(1, N)
        ops += 1 + B
        R = (u + v * x) % N
        for f in F:
            t = T.get((R - f) % N)
            if t is not None:
                xs = ((t + f - u) * pow(v, -1, N)) % N
                assert xs == x
                return ops

def mean_ci_slope(ns, data, rng, boots=400):
    """data: {n: [costs]}. Slope of log2(mean cost) vs n, bootstrap CI over reps."""
    def slope(means):
        xs = list(ns); ys = [math.log2(m) for m in means]
        mx = sum(xs) / len(xs); my = sum(ys) / len(ys)
        return sum((a - mx) * (b - my) for a, b in zip(xs, ys)) / sum((a - mx) ** 2 for a in xs)
    pt = slope([sum(data[n]) / len(data[n]) for n in ns])
    bs = []
    for _ in range(boots):
        ms = []
        for n in ns:
            d = data[n]; k = len(d)
            ms.append(sum(d[rng.randrange(k)] for _ in range(k)) / k)
        bs.append(slope(ms))
    bs.sort()
    return pt, bs[int(0.025 * boots)], bs[int(0.975 * boots) - 1]

def main():
    smoke = "--smoke" in sys.argv
    ns = [10, 12] if smoke else [12, 14, 16, 18, 20, 22]
    reps = 6 if smoke else 200
    Bs = [2, 8] if smoke else [2, 8, 128]
    boots = 50 if smoke else 400
    arms = {"P_B0_birthday_null": lambda N, r, n: arm_birthday(N, 0, r)}
    for B in Bs:
        arms["A_slp_B%d" % B] = (lambda B: lambda N, r, n: arm_birthday(N, B, r))(B)
    arms["T_table_B8_l_sqrtN"] = lambda N, r, n: arm_table(N, 8, int(math.isqrt(N)), r)
    primes = {n: prime_near(n) for n in ns}
    res = {}; allslopes = {}
    for ai, (name, fn) in enumerate(arms.items()):
        data = {}
        for n in ns:
            data[n] = []
            for k in range(reps):
                rng = random.Random(hash((1, ai, n, k)) if False else (ai * 1000003 + n * 7919 + k * 104729 + 20261110))
                data[n].append(fn(primes[n], rng, n))
        pt, lo, hi = mean_ci_slope(ns, data, random.Random(99 + ai), boots)
        res[name] = {"slope": pt, "ci95": [lo, hi], "mean_cost": {str(n): sum(data[n]) / reps for n in ns}}
        allslopes[name] = (pt, lo, hi)
        if name == "A_slp_B%d" % Bs[0]:
            # A/A: even vs odd reps of same arm
            ev = {n: data[n][0::2] for n in ns}; od = {n: data[n][1::2] for n in ns}
            a = mean_ci_slope(ns, ev, random.Random(5), boots); b = mean_ci_slope(ns, od, random.Random(6), boots)
            res["AA_halves_" + name] = {"slope_even": a[0], "slope_odd": b[0], "abs_diff": abs(a[0] - b[0])}
    # positive control: synthetic cost c*N^0.4 with same multiplicative dispersion (exp-distributed factor, like birthday)
    prng = random.Random(777)
    pos = {n: [3.0 * (2 ** (0.4 * n)) * prng.expovariate(1.0) for _ in range(reps)] for n in ns}
    pp = mean_ci_slope(ns, pos, random.Random(8), boots)
    res["positive_control_synthetic_exp0.4"] = {"slope": pp[0], "ci95": [pp[1], pp[2]]}
    pos_ok = pp[2] < 0.45
    lowest_hi = min(v[2] for v in allslopes.values())
    lowers = [k for k, v in allslopes.items() if v[2] < 0.45]
    null_ok = allslopes["P_B0_birthday_null"][1] <= 0.5 <= allslopes["P_B0_birthday_null"][2] or smoke
    aa = res["AA_halves_A_slp_B%d" % Bs[0]]["abs_diff"]
    if not pos_ok or not null_ok:
        verdict = "inconclusive"
    elif lowers:
        verdict = "falsified"
    elif all(0.45 <= v[0] <= 0.55 for v in allslopes.values()):
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    if smoke and verdict == "falsified": verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-residual-partial-relations", "smoke": smoke, "sizes_log2N": ns, "reps": reps,
        "factor_base_sizes": Bs, "seed_scheme": "ai*1000003+n*7919+k*104729+20261110", "arms": res,
        "statistic": "bootstrap slope of log2 mean ops vs log2 N", "threshold": "falsified if any arm CI upper < 0.45; consistent if all point slopes in [0.45,0.55]",
        "positive_control_detected": pos_ok, "null_covers_0.5": null_ok, "aa_abs_diff": aa, "arms_below_0.45": lowers,
        "verdict": verdict}))
main()
