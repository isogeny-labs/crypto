import json, math, random, sys
SMOKE = "--smoke" in sys.argv
N = 4099 if SMOKE else 1048573          # group Z_N (additive), toy order
T = 20 if SMOKE else 1500                # trials per arm
RS = [2, 4] if SMOKE else [2, 4, 8, 16, 32, 64]
M64 = (1 << 64) - 1

def hmix(x, salt):
    x = (x * 0x9E3779B97F4A7C15 + salt) & M64
    x ^= x >> 29; x = (x * 0xBF58476D1CE4E5B9) & M64; x ^= x >> 32
    return x

def walk_len(r, mode, n, rng):
    steps = [rng.randrange(1, n) for _ in range(r)]
    salt = rng.getrandbits(64)
    x = rng.randrange(n); seen = {x}; k = 0
    while True:
        j = hmix(x, salt) % r if mode == "hash" else x % r
        x = (x + steps[j]) % n; k += 1
        if x in seen: return k
        seen.add(x)

def random_map_len(n, rng):
    f = {}; x = rng.randrange(n); seen = {x}; k = 0
    while True:
        if x not in f: f[x] = rng.randrange(n)
        x = f[x]; k += 1
        if x in seen: return k
        seen.add(x)

def arm(fn, seed):
    rng = random.Random(seed)
    v = [fn(rng) for _ in range(T)]
    m = sum(v) / T
    sd = math.sqrt(sum((a - m) ** 2 for a in v) / (T - 1)) if T > 1 else 0.0
    return m, sd / math.sqrt(T)

ref = math.sqrt(math.pi * N / 2)
def ratio(fn, seed):
    m, se = arm(fn, seed); return m / ref, se / ref

out = {"group": "Z_N additive", "N": N, "trials": T, "ref_sqrt_pi_N_over_2": ref, "smoke": SMOKE}
null = ratio(lambda g: random_map_len(N, g), 1001)
out["null_random_mapping"] = {"ratio": null[0], "se": null[1]}
res = {}
for r in RS:
    res[f"hash_r{r}"] = ratio(lambda g, r=r: walk_len(r, "hash", N, g), 2000 + r)
    res[f"mod_r{r}"] = ratio(lambda g, r=r: walk_len(r, "mod", N, g), 3000 + r)
out["arms"] = {k: {"ratio": v[0], "se": v[1]} for k, v in res.items()}
rA = RS[-1]
a1 = ratio(lambda g: walk_len(rA, "hash", N, g), 4001)
a2 = ratio(lambda g: walk_len(rA, "hash", N, g), 4002)
zaa = abs(a1[0] - a2[0]) / math.sqrt(a1[1] ** 2 + a2[1] ** 2 + 1e-18)
out["aa"] = {"r": rA, "ratio1": a1[0], "ratio2": a2[0], "z": zaa}
lo = res[f"hash_r{RS[0]}"][0]; hi = res[f"hash_r{rA}"][0]
pos_ok = lo > 1.25 if not SMOKE else True
null_ok = abs(null[0] - 1) < 3 * null[1] + 0.03 if not SMOKE else True
aa_ok = zaa < 3.5 if not SMOKE else True
out["controls"] = {"positive_r_small_ratio_gt_1.25": pos_ok, "null_within_3se": null_ok, "aa_z_lt_3.5": aa_ok}
mid = res.get("hash_r4", (None,))[0]
out["prediction"] = "hash r=64 ratio within [0.95,1.07] of random; r=4 ratio >=1.10; ratio never <0.95 for any walk"
out["seeds"] = {"null": 1001, "arms": "2000+r hash, 3000+r mod", "aa": [4001, 4002]}
if SMOKE or not (pos_ok and null_ok and aa_ok):
    v = "inconclusive"
else:
    below = min(x[0] for x in res.values()) < 0.95
    if below or hi > 1.10 or mid < 1.02: v = "falsified"
    elif 0.95 <= hi <= 1.07 and mid >= 1.10: v = "consistent_with_prediction"
    else: v = "inconclusive"
out["verdict"] = v
print(json.dumps(out))
