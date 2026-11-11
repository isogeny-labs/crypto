import json, random, sys, math
SMOKE = "--smoke" in sys.argv
PRIMES = {12: 4093, 16: 65521, 20: 1048573}
SIZES = [12] if SMOKE else [12, 16, 20]
TRIALS = 20 if SMOKE else 300
SEED = 20261110

def walk_steps(N, r, rng, d):
    # generic cyclic group Z_N, P=1, Q=d; step adds a_j*P+b_j*Q (r-adding walk, partition by hashed x)
    M = [(rng.randrange(N) + d * rng.randrange(N)) % N for _ in range(r)]
    x = rng.randrange(N); seen = {x}; n = 0
    while True:
        x = (x + M[((x * 2654435761) >> 8) % r]) % N
        n += 1
        if x in seen: return n
        seen.add(x)

def random_map_steps(N, rng):
    seen = set(); n = 0
    while True:
        x = rng.randrange(N); n += 1
        if x in seen: return n
        seen.add(x)

def mean(f, T):
    return sum(f() for _ in range(T)) / T

out = {"lead": "LEAD-ICP-gfp5-rho-reference-audit", "seed": SEED, "smoke": SMOKE,
       "sizes_log2": SIZES, "trials": TRIALS, "rows": []}
ok = True; pos_ok = True; aa_ok = True
for b in SIZES:
    N = PRIMES[b]; ref = math.sqrt(math.pi * N / 2)
    rng = lambda s: random.Random(SEED * 1000 + b * 10 + s)
    ra, rb, rn, r2 = rng(1), rng(2), rng(3), rng(4)
    d = 12345 % N
    a1 = mean(lambda: walk_steps(N, 20, ra, d), TRIALS) / ref
    a2 = mean(lambda: walk_steps(N, 20, rb, d), TRIALS) / ref
    null = mean(lambda: random_map_steps(N, rn), TRIALS) / ref
    pos = mean(lambda: walk_steps(N, 2, r2, d), TRIALS) / ref
    row = {"log2N": b, "N": N, "ratio_r20_A": a1, "ratio_r20_B": a2,
           "ratio_random_map_null": null, "ratio_r2_positive_control": pos}
    out["rows"].append(row)
    tol = 0.35 if SMOKE else 0.15
    ok &= abs((a1 + a2) / 2 - 1) <= 0.10 + (0.25 if SMOKE else 0) and abs(null - 1) <= tol
    aa_ok &= abs(a1 - a2) <= tol
    pos_ok &= pos > 1.15 * ((a1 + a2) / 2)
out["controls"] = {"null_ok": bool(ok), "A_A_ok": bool(aa_ok), "positive_control_sees_inflation": bool(pos_ok)}
if not (aa_ok and pos_ok): out["verdict"] = "inconclusive"
elif ok: out["verdict"] = "consistent_with_prediction"
else: out["verdict"] = "falsified"
print(json.dumps(out))
