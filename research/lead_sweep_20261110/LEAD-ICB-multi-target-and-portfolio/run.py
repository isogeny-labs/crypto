#!/usr/bin/env python3
"""LEAD-ICB-multi-target-and-portfolio: stdlib toy. See PROTOCOL.md."""
import json, math, random, sys

SMOKE = "--smoke" in sys.argv

def is_prime(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    i = 3
    while i * i <= n:
        if n % i == 0: return False
        i += 2
    return True

def safe_prime(start):
    q = start | 1
    while not (is_prime(q) and is_prime(2 * q + 1)): q += 2
    return q

def setup(q):
    P = 2 * q + 1
    g = pow(2, 2, P) if pow(4, q, P) == 1 and 4 != 1 else None
    return P, g

def run_multi(q, K, reps, seed, shared):
    """Median ops per target for K targets. shared=True: one tame table of
    ~sqrt(K q) points; shared=False: K independent K=1 runs (null)."""
    P, g = setup(q)
    rng = random.Random(seed)
    mm = [rng.randrange(1, 2 * int(math.sqrt(q)) + 2) for _ in range(16)]
    ss = [pow(g, m, P) for m in mm]
    def solve_set(targets_x, Tsteps):
        table = {}; ops = 0
        for _ in range(4):
            t = rng.randrange(q); x = pow(g, t, P); ops += 1
            for _ in range(max(1, Tsteps // 4)):
                table[x] = t
                j = x & 15; x = x * ss[j] % P; t = (t + mm[j]) % q; ops += 1
        cap = 4 * (q // max(1, len(table))) + 60
        for xi in targets_x:
            h = pow(g, xi, P); found = False
            while not found:
                r = rng.randrange(q); x = h * pow(g, r, P) % P; d = r; ops += 1
                for _ in range(cap):
                    if x in table:
                        if (table[x] - d) % q == xi: found = True
                        break
                    j = x & 15; x = x * ss[j] % P; d = (d + mm[j]) % q; ops += 1
        return ops
    res = []
    for _ in range(reps):
        xs = [rng.randrange(1, q) for _ in range(K)]
        if shared:
            res.append(solve_set(xs, int(math.sqrt(K * q))) / K)
        else:
            res.append(sum(solve_set([x], int(math.sqrt(q))) for x in xs) / K)
    res.sort()
    return res[len(res) // 2], res

def slope(xs, ys):
    lx = [math.log(x) for x in xs]; ly = [math.log(y) for y in ys]
    mx = sum(lx) / len(lx); my = sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)

def portfolio(rho, r, n, seed):
    """Paired MC. rho: empirical single-target cost samples (mean mu).
    Returns dict of min over (w,tau) of mean ratio, with SE, vs pure rho."""
    mu = sum(rho) / len(rho)
    rng = random.Random(seed)
    R = [rng.choice(rho) for _ in range(n)]
    base = sum(R) / n
    # attempt cost draws, shared across settings (common random numbers)
    U = [[rng.random() for _ in range(40)] for _ in range(n)]
    best = None
    for w in (0.5, 0.6, 0.7, 0.8, 0.9, 0.95):
        for tau_m in (0.5, 1.0, 2.0, 4.0, 1e9):
            tau = tau_m * mu; diffs = []
            for i in range(n):
                t = 0.0
                for u in U[i]:
                    x = r * mu * (1 - u) ** (-1 / 1.5)  # Pareto a=1.5, floor r*mu
                    if x <= tau: t += x; break
                    t += tau
                else:
                    t = 1e18
                tot = min(R[i] / w, t / (1 - w))
                diffs.append(tot - R[i])
            m = sum(diffs) / n
            sd = math.sqrt(sum((d - m) ** 2 for d in diffs) / (n - 1))
            cand = (m / base, sd / math.sqrt(n) / base, w, tau_m)
            if best is None or cand[0] < best[0]: best = cand
    return {"ratio_minus1_best": best[0], "se": best[1], "w": best[2], "tau_over_mu": best[3]}

def main():
    q = safe_prime(1 << (11 if SMOKE else 20))
    Ks = [1, 4, 16] if SMOKE else [1, 4, 16, 64]
    reps = 3 if SMOKE else 60
    seeds = {"multi": 20261110, "null": 20261111, "aa": 20261112, "portfolio": 20261113}
    med_s, med_n, med_a = [], [], []
    rho_samples = None
    for K in Ks:
        a, ra = run_multi(q, K, reps, seeds["multi"] + K, True)
        n_, _ = run_multi(q, K, max(2, reps // 3), seeds["null"] + K, False)
        a2, _ = run_multi(q, K, reps, seeds["aa"] + K, True)
        med_s.append(a); med_n.append(n_); med_a.append(a2)
        if K == 1: rho_samples = [x for x in ra]
    s_sh, s_null, s_aa = slope(Ks, med_s), slope(Ks, med_n), slope(Ks, med_a)
    n = 100 if SMOKE else 3000
    pf = {}
    for name, r in (("r0.1_positive", 0.1), ("r1.0", 1.0), ("r1.5", 1.5)):
        pf[name] = portfolio(rho_samples, r, n, seeds["portfolio"])
    pos_ok = pf["r0.1_positive"]["ratio_minus1_best"] < -0.2
    # obstruction: no setting with r>=1 beats pure rho by more than 2 SE
    obs_ok = all(pf[k]["ratio_minus1_best"] > -2 * pf[k]["se"] - 0.01 for k in ("r1.0", "r1.5"))
    share_ok = -0.65 <= s_sh <= -0.35
    null_ok = abs(s_null) <= 0.15
    aa_ok = abs(s_sh - s_aa) <= 0.1
    if not (pos_ok and null_ok and aa_ok):
        verdict = "inconclusive"
    elif share_ok and obs_ok:
        verdict = "consistent_with_prediction"
    else:
        verdict = "falsified"
    if SMOKE and verdict != "falsified": pass
    print(json.dumps({
        "lead": "LEAD-ICB-multi-target-and-portfolio", "smoke": SMOKE, "q": q, "Ks": Ks,
        "reps": reps, "seeds": seeds,
        "ops_per_target_shared": med_s, "ops_per_target_independent_null": med_n,
        "ops_per_target_aa": med_a,
        "slope_shared": s_sh, "slope_null": s_null, "slope_aa": s_aa,
        "portfolio": pf,
        "checks": {"positive_control": pos_ok, "obstruction_holds": obs_ok,
                   "share_slope_in_band": share_ok, "null_flat": null_ok, "aa_ok": aa_ok},
        "verdict": verdict}))

main()
