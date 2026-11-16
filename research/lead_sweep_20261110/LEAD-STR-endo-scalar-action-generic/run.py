#!/usr/bin/env python3
"""Endo-augmented generic rho in a GGM-style cyclic group Z/N (stdlib only).
Arms: eigenvalue-orbit size k in {1(plain),2,3,6,60}; step count and charged cost
(canonicalisation costs k group-op-equivalents per step). A/A arm = k=1 with disjoint seeds."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def find_N(lo):
    n = lo - lo % 60 + 1
    while not is_prime(n): n += 60
    return n

def subgroup(N, k):
    if k == 1: return [1]
    for c in range(2, N):
        h = pow(c, (N - 1) // k, N)
        if h != 1 and all(pow(h, k // q, N) != 1 for q in range(2, k + 1) if k % q == 0 and is_prime(q)):
            H = [1]
            for _ in range(k - 1): H.append(H[-1] * h % N)
            return H
    raise SystemExit("no subgroup")

def one_solve(N, H, d, rng, nj=24):
    """returns steps until a cross-walk collision recovers d (verified)."""
    ms = [(rng.randrange(N), rng.randrange(N)) for _ in range(nj)]
    seen = {}
    steps = 0
    cap = int(8 * math.sqrt(N)) + 50
    while True:
        a, b = rng.randrange(N), rng.randrange(N)
        walk = set()
        for _ in range(cap):
            x = (a + b * d) % N
            # canonicalise: choose h minimising h*x (label-only decision)
            best = min(H, key=lambda h: h * x % N)
            a, b = a * best % N, b * best % N
            x = x * best % N
            if x in walk: break              # fruitless cycle -> restart
            walk.add(x)
            if x in seen:
                a2, b2 = seen[x]
                if b2 != b:
                    dd = (a - a2) * pow(b2 - b, -1, N) % N
                    if dd == d: return steps
                break
            seen[x] = (a, b)
            j = (x * 2654435761 >> 5) % nj
            a, b = (a + ms[j][0]) % N, (b + ms[j][1]) % N
            steps += 1
    return steps

def arm(N, k, trials, seed0):
    H = subgroup(N, k)
    st = []
    for t in range(trials):
        rng = random.Random(seed0 + t)
        d = rng.randrange(2, N - 1)
        st.append(one_solve(N, H, d, rng))
    m = sum(st) / len(st)
    sd = (sum((s - m) ** 2 for s in st) / (len(st) - 1)) ** .5
    return m, sd / math.sqrt(len(st))

def main():
    smoke = "--smoke" in sys.argv
    N = find_N(1 << 10) if smoke else find_N(1 << 17)
    trials = 8 if smoke else 300
    ks = [1, 2, 3, 6] if smoke else [1, 2, 3, 6, 60]
    res = {}
    base, bse = arm(N, 1, trials, 1000)
    aa, aase = arm(N, 1, trials, 900000)
    for k in ks[1:]:
        m, se = arm(N, k, trials, 1000)   # same seeds as baseline (paired d/starts)
        ratio = m / base
        res[k] = {"mean_steps": m, "se": se, "step_ratio": ratio,
                  "predicted": 1 / math.sqrt(k), "charged_cost_ratio": ratio * k}
    aa_ratio = aa / base
    se_rel = lambda m, s: s / m
    tol_aa = 3 * math.hypot(se_rel(base, bse), se_rel(aa, aase)) + 0.02
    aa_ok = abs(aa_ratio - 1) <= tol_aa
    pos_ok = res[2]["step_ratio"] < 0.85            # negation detectable
    unit_ok = all(abs(res[k]["step_ratio"] / res[k]["predicted"] - 1) <= 0.25 for k in (2, 3, 6))
    big = res.get(60)
    nogain = True if big is None else big["charged_cost_ratio"] > 1.0
    if smoke or not aa_ok or not pos_ok:
        verdict = "inconclusive"
    elif unit_ok and nogain:
        verdict = "consistent_with_prediction"
    else:
        verdict = "falsified"
    print(json.dumps({"lead": "LEAD-STR-endo-scalar-action-generic", "smoke": smoke, "N": N,
        "trials_per_arm": trials, "seeds": {"paired": 1000, "aa": 900000},
        "baseline_mean_steps": base, "aa_ratio": aa_ratio, "aa_tol": tol_aa,
        "arms": {str(k): v for k, v in res.items()}, "positive_control_k2_ok": pos_ok,
        "unit_orbit_scaling_ok": unit_ok, "large_orbit_no_charged_gain": nogain,
        "verdict": verdict}))
main()
