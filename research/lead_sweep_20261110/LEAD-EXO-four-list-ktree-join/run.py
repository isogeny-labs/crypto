#!/usr/bin/env python3
"""LEAD-EXO-four-list-ktree-join: bucket-variance W of a cheap filter on E(F_p) point sets. stdlib only."""
import json, math, random, sys

def curve_counts(p, a, b):
    """occ[x] = number of points with abscissa x (0,1,2)."""
    sq = bytearray(p)
    for y in range(p):
        sq[y * y % p] += 1
    sq[0] = 1
    return [sq[(x * x * x + a * x + b) % p] if True else 0 for x in range(p)]

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def W_of(counts_by_bucket):
    B = len(counts_by_bucket); N = sum(counts_by_bucket); m = N / B
    v = sum((c - m) ** 2 for c in counts_by_bucket) / B
    return v / (m * m), N

def bucketize(occ, B, mode, rng):
    cnt = [0] * B
    if mode == "xmod":
        for x, o in enumerate(occ): cnt[x % B] += o
    elif mode == "null":  # same multiset of points, bucket i.i.d. uniform
        for o in occ:
            for _ in range(o): cnt[rng.randrange(B)] += 1
    elif mode == "planted":  # 20% of x forced into bucket 0
        for x, o in enumerate(occ): cnt[0 if x % 5 == 0 else x % B] += o
    return cnt

def slope(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def run(bits_list, ncurves, seed):
    rng = random.Random(seed)
    rows = []
    for bits in bits_list:
        p = next_prime(2 ** bits + rng.randrange(2 ** (bits - 2)))
        acc = {"xmod": [], "null": [], "planted": [], "aa": []}
        Ns = []
        for c in range(ncurves):
            while True:
                a, b = rng.randrange(p), rng.randrange(p)
                if (4 * a ** 3 + 27 * b * b) % p: break
            occ = curve_counts(p, a, b)
            N = sum(occ) + 1
            B = max(2, int(math.isqrt(N)))
            Ns.append(N)
            for mode in ("xmod", "null", "planted"):
                acc[mode].append(W_of(bucketize(occ, B, mode, rng))[0])
            # A/A: filter on the odd/even x halves, same kernel, two independent half-samples
            h0 = [o if x % 2 == 0 else 0 for x, o in enumerate(occ)]
            h1 = [o if x % 2 == 1 else 0 for x, o in enumerate(occ)]
            w0 = W_of(bucketize(h0, B, "xmod", rng))[0] * (sum(h0) / B)
            w1 = W_of(bucketize(h1, B, "xmod", rng))[0] * (sum(h1) / B)
            acc["aa"].append(w0 / w1)
        mean = lambda l: sum(l) / len(l)
        Nm = mean(Ns); Bm = int(math.isqrt(int(Nm)))
        rows.append({"p": p, "N_mean": Nm, "B": Bm, "W_xmod": mean(acc["xmod"]),
                     "W_null": mean(acc["null"]), "W_planted": mean(acc["planted"]),
                     "W_pred_1_over_mean": Bm / Nm, "aa_ratio": mean(acc["aa"])})
    lx = [math.log(r["N_mean"]) for r in rows]
    s = {k: slope(lx, [math.log(r[k]) for r in rows]) for k in ("W_xmod", "W_null", "W_planted")}
    ratio = [r["W_xmod"] / r["W_null"] for r in rows]
    aa = [r["aa_ratio"] for r in rows]
    return rows, s, ratio, aa

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    bits = [9, 10, 11] if smoke else [12, 14, 16, 18, 20]
    nc = 2 if smoke else 6
    rows, s, ratio, aa = run(bits, nc, seed)
    # thresholds (preregistered)
    pos_ok = s["W_planted"] > -0.25 and all(r["W_planted"] > 3 * r["W_null"] for r in rows)
    null_ok = abs(s["W_null"] + 0.5) < 0.1
    aa_ok = all(0.7 < a < 1.43 for a in aa) if not smoke else True
    pred = abs(s["W_xmod"] + 0.5) < 0.15 and all(0.8 < r < 1.25 for r in ratio)
    if not (pos_ok and null_ok and aa_ok): verdict = "inconclusive"
    elif pred: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-EXO-four-list-ktree-join", "smoke": smoke, "seed": seed,
        "bits": bits, "curves_per_size": nc, "rows": rows, "loglog_slopes": s,
        "ratio_xmod_over_null": ratio, "aa_ratios": aa,
        "controls": {"positive_ok": pos_ok, "null_ok": null_ok, "aa_ok": aa_ok},
        "thresholds": {"slope_target": -0.5, "slope_tol": 0.15, "ratio_band": [0.8, 1.25]},
        "verdict": verdict}))
main()
