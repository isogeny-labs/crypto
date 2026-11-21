import json, random, sys, math
SMOKE = '--smoke' in sys.argv
R = 32
def mix(c):
    return ((c * 0x9E3779B1 + 12345) >> 7) % R
def trial(n, neg, rng, jumps, cap):
    """Two interleaved walks in Z_n; collision = class seen by the other walk.
    Self-repeat (fruitless cycle) -> restart that walk at fresh random point. Returns (steps, traps)."""
    canon = (lambda x: x if x <= n - x else n - x) if neg else (lambda x: x)
    seen = {}; pos = [rng.randrange(n), rng.randrange(n)]
    steps = 0; traps = 0
    for w in (0, 1):
        seen[canon(pos[w])] = w
    while steps < cap:
        for w in (0, 1):
            x = canon(pos[w]); y = (x + jumps[mix(x)]) % n
            steps += 1
            c = canon(y)
            o = seen.get(c)
            if o is not None:
                if o != w: return steps, traps
                traps += 1; y = rng.randrange(n); c = canon(y)
                o = seen.get(c)
                if o is not None and o != w: return steps, traps
            pos[w] = y; seen[c] = w
    return steps, traps
def arm(n, neg, seed, T):
    rng = random.Random(seed)
    jumps = [rng.randrange(1, n) for _ in range(R)]
    tot = 0; traps = 0; xs = []
    for _ in range(T):
        s, t = trial(n, neg, rng, jumps, 200 * int(math.isqrt(n)) + 1000)
        xs.append(s); traps += t
    return xs, traps
def boot(a, b, rng, B=400):
    rs = []
    for _ in range(B):
        sa = sum(rng.choice(a) for _ in a); sb = sum(rng.choice(b) for _ in b)
        rs.append(sa / sb)
    rs.sort(); return rs[int(.025 * B)], rs[int(.975 * B) - 1]
def mean(x): return sum(x) / len(x)
sizes = [8191, 8209] if SMOKE else [65521, 262139, 1048573]  # primes (n = group order)
T = 20 if SMOKE else 300
out = {"seeds_base": 20261110, "T": T, "sizes": sizes, "jump_set": R, "rows": []}
rng = random.Random(7); ok = True; falsified = False
for n in sizes:
    plain, _ = arm(n, False, 101 + n, T)
    plain2, _ = arm(n, False, 202 + n, T)          # A/A
    neg, tr = arm(n, True, 303 + n, T)
    half, _ = arm(n // 2 | 1, False, 404 + n, T)    # ideal-quotient reference: plain on ~n/2
    r = mean(neg) / mean(plain); lo, hi = boot(neg, plain, rng)
    aa = mean(plain2) / mean(plain); alo, ahi = boot(plain2, plain, rng)
    rh = mean(half) / mean(plain)
    row = {"n": n, "ratio_neg_over_plain": r, "ci95": [lo, hi], "AA_ratio": aa, "AA_ci95": [alo, ahi],
           "ideal_halfgroup_ratio": rh, "traps_per_trial": tr / T,
           "plain_over_sqrt_n": mean(plain) / math.sqrt(n)}
    out["rows"].append(row)
    if not (alo <= 1 <= ahi): ok = False
    if lo > 0.80 or hi < 0.66: falsified = True
    if not (0.66 <= lo and hi <= 0.80): ok = False
out["thresholds"] = {"consistent_band": [0.66, 0.80], "AA_must_contain": 1.0}
out["verdict"] = "falsified" if falsified else ("consistent_with_prediction" if ok else "inconclusive")
out["smoke"] = SMOKE
print(json.dumps(out))
