#!/usr/bin/env python3
"""LEAD-RHO-lhw-modular-filter-price: exact residue-distribution price of mod-M filters on W(m,w).
Stdlib only, deterministic. P(M,m,w) = M * sum_r p_r^2 (collision price), exact via DP."""
import json, random, sys, statistics

def order2(M):
    x, k = 2 % M, 1
    while x != 1:
        x = x * 2 % M; k += 1
    return k

def dist(M, m, wmax):
    # dp[j][r] = number of weight-j m-bit integers with residue r mod M
    dp = [[0] * M for _ in range(wmax + 1)]
    dp[0][0] = 1
    for i in range(m):
        c = pow(2, i, M)
        for j in range(wmax, 0, -1):
            prev = dp[j - 1]; cur = dp[j]
            rot = prev[-c:] + prev[:-c] if c else prev
            dp[j] = [a + b for a, b in zip(cur, rot)]
    return dp

def price(counts, M):
    t = sum(counts)
    return M * sum(x * x for x in counts) / (t * t)

def cell(M, m, ws):
    dp = dist(M, m, max(ws))
    out = {w: price(dp[w], M) for w in ws}
    allw = [sum(dp[j][r] for j in range(len(dp))) for r in range(M)]
    return out

def run(smoke):
    if smoke:
        S, MS, WS, NG, seed = [6], [12], [2, 3], 2, 20261110
    else:
        S, MS, WS, NG, seed = [8, 10, 12], [24, 32, 40], [4, 6, 8], 12, 20261110
    rng = random.Random(seed)
    rows, passes, total = [], 0, 0
    aa_ratios = []
    for s in S:
        for m in MS:
            gen = []
            while len(gen) < NG:
                M = rng.randrange(2 ** s + 1, 2 ** (s + 1), 2)
                if M % 3 == 0 and False: continue
                if order2(M) >= m and M not in gen:
                    gen.append(M)
            gp = [cell(M, m, WS) for M in gen]
            for w in WS:
                g = [c[w] for c in gp]
                half = NG // 2
                aa_ratios.append(statistics.median(g[:half]) / statistics.median(g[half:]))
                for name, M in (("mersenne", 2 ** s - 1), ("fermat", 2 ** s + 1)):
                    p = cell(M, m, WS)[w]
                    ok = p > 1.2 * max(g)
                    passes += ok; total += 1
                    rows.append({"s": s, "m": m, "w": w, "type": name, "P": round(p, 4),
                                 "generic_max": round(max(g), 4), "generic_median": round(statistics.median(g), 4),
                                 "pass": ok})
    # positive control: w=1, Mersenne, s | m  => P = M/s exactly
    s, m = (6, 12) if smoke else (8, 24)
    M = 2 ** s - 1
    pc = cell(M, m, [1])[1]
    pos_ok = abs(pc - M / s) < 1e-9
    # negative control: unrestricted-weight integers, special M vs generic M, price ~ 1
    s2, m2 = (6, 12) if smoke else (10, 40)
    def unres(M):
        dp = dist(M, m2, m2 if smoke else 14)  # smoke: full; full: truncated weights are NOT uniform, so use sampling
        return None
    r2 = random.Random(seed + 1)
    def sampled(M, n=200000 if not smoke else 5000):
        cnt = [0] * M
        for _ in range(n):
            cnt[r2.getrandbits(m2) % M] += 1
        return price(cnt, M)
    neg_special = sampled(2 ** s2 - 1)
    neg_generic = sampled(2 ** s2 + 3)
    # sampling floor: price of uniform with n samples ~ 1 + M/n
    neg_ok = abs(neg_special - neg_generic) < 0.25 * (1 if not smoke else 8)
    aa = statistics.median(aa_ratios)
    aa_ok = 0.9 < aa < 1.1
    frac = passes / total
    if not (pos_ok and neg_ok and aa_ok):
        verdict = "inconclusive"
    elif frac >= 0.75:
        verdict = "consistent_with_prediction"
    elif frac < 0.25:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    return {"lead": "LEAD-RHO-lhw-modular-filter-price", "smoke": smoke, "seed": seed,
            "params": {"s": S, "m": MS, "w": WS, "generic_per_cell": NG},
            "statistic": "fraction of (s,m,w,type) cells with P_special > 1.2*max P_generic",
            "pass_fraction": frac, "cells": total,
            "positive_control": {"P": pc, "expected": M / s, "ok": pos_ok},
            "negative_control": {"unrestricted_special": neg_special, "unrestricted_generic": neg_generic, "ok": neg_ok},
            "aa": {"median_ratio_generic_halves": aa, "ok": aa_ok},
            "rows": rows if not smoke else rows[:4], "verdict": verdict}

if __name__ == "__main__":
    print(json.dumps(run("--smoke" in sys.argv)))
