#!/usr/bin/env python3
"""Preregistered toy: generic (group-op + equality only) algorithms, incl. a representation/subset-sum
style two-list sampler, obey T >~ sqrt(N). Stdlib only, deterministic."""
import json, math, random, sys

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prime_near(k):
    n = (1 << k) + 1
    while not is_prime(n): n += 1
    return n

# Generic group: Z_N additive, generator g=1; the algorithms below use only +, equality/hash lookup,
# and each addition is counted as one op. Target Q=d (d hidden).
def rho(N, d, rng):
    ops = 0
    steps = [(rng.randrange(1, N), rng.randrange(1, N)) for _ in range(16)]
    while True:
        a, b = rng.randrange(N), rng.randrange(N)
        x = (a + b * d) % N; ops += 2
        def f(x, a, b):
            s = steps[x & 15]
            return (x + s[0] + s[1] * d) % N, (a + s[0]) % N, (b + s[1]) % N
        X, A, B = x, a, b
        Y, C, D2 = x, a, b
        for _ in range(8 * int(math.sqrt(N)) + 100):
            X, A, B = f(X, A, B); ops += 1
            Y, C, D2 = f(Y, C, D2); ops += 1
            Y, C, D2 = f(Y, C, D2); ops += 1
            if X == Y:
                db = (B - D2) % N
                if db:
                    return ops, ((C - A) * pow(db, -1, N)) % N == d
                break

def bsgs(N, d, rng):
    m = int(math.isqrt(N)) + 1; ops = 0
    tab = {}
    for j in range(m):
        tab.setdefault(j, j); ops += 1          # baby steps jP
    cur = d
    for i in range(m + 1):
        if cur in tab: return ops, True
        cur = (cur - m) % N; ops += 1           # giant steps Q - i*mP
    return ops, False

def rep(N, d, rng):
    """d = x + y; x, y drawn from two independent random-walk lists (each sample costs 1 add).
    Representations: N pairs (x, d-x); hit when xP == Q - yP."""
    sx = [rng.randrange(1, N) for _ in range(16)]
    sy = [rng.randrange(1, N) for _ in range(16)]
    X = rng.randrange(N); Y = rng.randrange(N); ops = 2
    tx, ty = {X: 1}, {(d - Y) % N: 1}
    if X in ty or (d - Y) % N in tx: return ops, True
    for _ in range(40 * int(math.sqrt(N)) + 100):
        X = (X + sx[rng.randrange(16)]) % N; ops += 1
        Y = (Y + sy[rng.randrange(16)]) % N; ops += 1
        qy = (d - Y) % N
        if X in ty or qy in tx or X == qy: return ops, True
        tx[X] = 1; ty[qy] = 1
    return ops, False

def ph(N, d, rng):
    """Positive control: reads coordinate structure (N=2^k smooth, bitwise Pohlig-Hellman)
    in a cyclic group of order 2^k; cost counted in group ops ~ k^2/2 (not generic)."""
    k = N.bit_length() - 1; ops = 0; rec = 0
    for i in range(k):
        ops += (k - i) + 1
        rec |= ((d >> i) & 1) << i
    return ops, rec == d

def mean_ops(alg, N, trials, seed, smooth=False):
    rng = random.Random(seed); tot = 0; ok = 0
    for _ in range(trials):
        d = rng.randrange(1, N)
        o, good = alg(N, d, rng); tot += o; ok += good
    return tot / trials, ok / trials

def slope(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def boot_slope(per_size, rng, B=200):
    # per_size: list of (log2N, [per-trial ops]); bootstrap over trials
    out = []
    for _ in range(B):
        xs, ys = [], []
        for lg, v in per_size:
            s = [v[rng.randrange(len(v))] for _ in v]
            xs.append(lg); ys.append(math.log2(sum(s) / len(s)))
        out.append(slope(xs, ys))
    out.sort(); return out[int(0.025 * B)], out[int(0.975 * B)]

def collect(alg, ks, trials, seed, prime=True):
    rows = []
    for k in ks:
        N = prime_near(k) if prime else (1 << k)
        rng = random.Random(seed * 1000 + k); v = []; ok = 0
        for _ in range(trials):
            d = rng.randrange(1, N); o, good = alg(N, d, rng); v.append(o); ok += good
        rows.append((k, v, N, ok / trials))
    return rows

def main():
    smoke = "--smoke" in sys.argv
    ks = [10, 11, 12] if smoke else [12, 14, 16, 18, 20, 22]
    trials = 20 if smoke else 60
    seeds = {"A": 11, "B": 22}
    res = {}
    for name, alg in [("rho", rho), ("bsgs", bsgs), ("rep", rep)]:
        for tag, sd in seeds.items():
            if name != "rho" and tag == "B": continue
            rows = collect(alg, ks, trials, sd)
            ps = [(r[0], r[1]) for r in rows]
            sl = slope([r[0] for r in ps], [math.log2(sum(r[1]) / len(r[1])) for r in ps])
            lo, hi = boot_slope(ps, random.Random(5))
            res[name + "_" + tag] = {"slope": round(sl, 4), "ci95": [round(lo, 4), round(hi, 4)],
                "ops_over_sqrtN": [round(sum(r[1]) / len(r[1]) / math.sqrt(r[2]), 3) for r in rows],
                "success": [r[3] for r in rows]}
    rows = collect(ph, ks, trials, 33, prime=False)
    ps = [(r[0], r[1]) for r in rows]
    sl = slope([r[0] for r in ps], [math.log2(sum(r[1]) / len(r[1])) for r in ps])
    lo, hi = boot_slope(ps, random.Random(6))
    res["positive_control_pohlig_hellman"] = {"slope": round(sl, 4), "ci95": [round(lo, 4), round(hi, 4)]}
    generic = ["rho_A", "bsgs_A", "rep_A"]
    min_ratio = min(min(res[g]["ops_over_sqrtN"]) for g in generic)
    aa = abs(res["rho_A"]["slope"] - res["rho_B"]["slope"])
    pc_sees = res["positive_control_pohlig_hellman"]["ci95"][1] < 0.45
    all_ok = all(min(res[g]["success"]) >= 0.95 for g in generic)
    slopes_in = all(res[g]["slope"] >= 0.40 for g in generic)
    below = any(res[g]["ci95"][1] < 0.45 for g in generic) or min_ratio < 0.3
    if smoke:
        verdict = "inconclusive"
    elif not (pc_sees and aa < 0.1 and all_ok):
        verdict = "inconclusive"
    elif below:
        verdict = "falsified"
    elif slopes_in and min_ratio >= 0.5:
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-representation-subset-sum", "smoke": smoke, "log2N_sizes": ks,
        "trials": trials, "seeds": seeds | {"pc": 33}, "arms": res, "min_ops_over_sqrtN": round(min_ratio, 3),
        "aa_slope_diff": round(aa, 4), "positive_control_sees_subsqrt": pc_sees,
        "all_generic_success_ge_0.95": all_ok, "verdict": verdict}))
main()
