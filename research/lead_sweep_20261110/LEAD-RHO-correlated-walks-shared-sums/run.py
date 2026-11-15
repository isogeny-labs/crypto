#!/usr/bin/env python3
"""Toy test: shared index sequence (B) and shared pair-sum table (C) vs point-function r-adding rho in Z_n. Stdlib only."""
import json, math, random, sys
SMOKE = "--smoke" in sys.argv
SIZES = [2**12] if SMOKE else [2**14, 2**18, 2**20]
TRIALS = 12 if SMOKE else 300
W = 8
def H(x): return ((x * 0x9E3779B1) & 0xFFFFFFFF) >> 11   # 21 bits
def mkjumps(rng, n, r): return [rng.randrange(1, n) for _ in range(r)]
def make_step(arm, n, rng):
    if arm in ("A16", "A256", "A16b"):
        r = 256 if arm == "A256" else 16
        J = mkjumps(rng, n, r)
        return (lambda x, t: (x + J[H(x) % r]) % n), True
    if arm == "C":
        b = mkjumps(rng, n, 16)
        T = [[(b[i] + b[j]) % n for j in range(16)] for i in range(16)]
        def f(x, t):
            h = H(x) % 256
            return (x + T[h >> 4][h & 15]) % n
        return f, True
    if arm == "B":
        J = mkjumps(rng, n, 16); S = []
        def f(x, t):
            while len(S) <= t: S.append(J[rng.randrange(16)])
            return (x + S[t]) % n
        return f, False
def trial(arm, n, seed):
    rng = random.Random(seed)
    f, pointfn = make_step(arm, n, rng)
    cur = [rng.randrange(n) for _ in range(W)]
    seen = {}
    for i, x in enumerate(cur): seen[x] = (i, 0)
    limit = int(30 * math.sqrt(n))
    for t in range(1, limit):
        for i in range(W):
            x = f(cur[i], t - 1); cur[i] = x
            if x in seen:
                j, u = seen[x]
                if (j, u) == (i, t): continue
                # merge test: run both clones 16 more steps
                a = b = x; ta, tb = t, u
                for _ in range(16):
                    a = f(a, ta); b = f(b, tb); ta += 1; tb += 1
                return (t * W, a == b)
            seen[x] = (i, t)
    return (limit * W, False)
def run_arm(arm, n, base):
    res = [trial(arm, n, base + k) for k in range(TRIALS)]
    steps = [s for s, _ in res]; m = sum(1 for _, g in res if g) / len(res)
    mean = sum(steps) / len(steps)
    sd = (sum((s - mean) ** 2 for s in steps) / (len(steps) - 1)) ** .5
    return {"mean_steps": mean, "se": sd / len(steps) ** .5, "merge_frac": m, "norm": mean / math.sqrt(n)}
out = {"lead": "LEAD-RHO-correlated-walks-shared-sums", "smoke": SMOKE, "W": W, "trials": TRIALS, "sizes": SIZES, "per_size": {}}
ok_B = ok_C = ok_ctrl = True
for n in SIZES:
    base = 1000003 * n
    arms = {a: run_arm(a, n, base + 7919 * k) for k, a in enumerate(["A16", "A256", "B", "C"])}
    aa = run_arm("A16b", n, base + 555555)  # A/A: same algorithm, disjoint seeds
    ratio_aa = aa["mean_steps"] / arms["A16"]["mean_steps"]
    ratio_C = arms["C"]["mean_steps"] / arms["A256"]["mean_steps"]
    ratio_B = arms["B"]["mean_steps"] / arms["A16"]["mean_steps"]
    pos = arms["A16"]["merge_frac"] >= 0.95 and 0.8 <= arms["A16"]["norm"] / math.sqrt(math.pi / 2 * W / W) <= 1.6 * 1.0 if not SMOKE else True
    arms_out = dict(arms); arms_out["A16_seed_replicate"] = aa
    out["per_size"][str(n)] = {"arms": arms_out, "AA_ratio": ratio_aa, "C_over_A256": ratio_C, "B_over_A16": ratio_B,
                               "merge_B": arms["B"]["merge_frac"], "merge_A16": arms["A16"]["merge_frac"], "positive_control_ok": bool(pos)}
    ok_ctrl &= bool(pos) and 0.9 <= ratio_aa <= 1.1
    ok_B &= arms["B"]["merge_frac"] <= 0.15
    ok_C &= 0.95 <= ratio_C <= 1.10
out["criteria"] = {"controls_ok": ok_ctrl, "B_merge_le_0.15": ok_B, "C_over_A256_in_[0.95,1.10]": ok_C}
if not ok_ctrl: out["verdict"] = "inconclusive"
elif ok_B and ok_C: out["verdict"] = "consistent_with_prediction"
else: out["verdict"] = "falsified"
print(json.dumps(out))
