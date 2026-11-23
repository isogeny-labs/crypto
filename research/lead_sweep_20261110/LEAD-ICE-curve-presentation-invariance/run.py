#!/usr/bin/env python3
"""Toy test of LEAD-ICE-curve-presentation-invariance: does 2-decomposition
coverage of an x-window factor base differ across same-order prime-field curves
(different j / presentation) beyond the within-curve window noise? stdlib only."""
import json, math, random, sys, time

def next_prime(n):
    while True:
        n += 1
        if all(n % q for q in range(2, int(n ** .5) + 1)):
            return n

def curve_points(p, a, b, sq):
    pts = []
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        if r == 0:
            pts.append((x, 0))
        elif sq[r]:
            y = sqrt_mod(r, p)
            pts.append((x, y)); pts.append((x, p - y))
    return pts

def sqrt_mod(r, p):
    if p % 4 == 3:
        return pow(r, (p + 1) // 4, p)
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, rr = s, pow(z, q, p), pow(r, q, p), pow(r, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p; i += 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, t, rr = i, bb * bb % p, t * bb * bb % p, rr * bb % p
    return rr

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def coverage(base, a, p, N):
    s = set()
    n = len(base)
    for i in range(n):
        for j in range(i, n):
            s.add(add(base[i], base[j], a, p))
    return len(s) / N

def anova(rows):
    k, r = len(rows), len(rows[0])
    gm = sum(map(sum, rows)) / (k * r)
    ms_b = r * sum((sum(x) / r - gm) ** 2 for x in rows) / (k - 1)
    ms_w = sum((v - sum(x) / r) ** 2 for x in rows for v in x) / (k * (r - 1))
    icc = (ms_b - ms_w) / (ms_b + (r - 1) * ms_w) if ms_b + (r - 1) * ms_w > 0 else 0.0
    return icc, ms_b / ms_w if ms_w > 0 else float('inf')

def perm_p(rows, rng, B):
    icc0, _ = anova(rows)
    flat = [v for x in rows for v in x]; r = len(rows[0]); c = 0
    for _ in range(B):
        rng.shuffle(flat)
        if anova([flat[i * r:(i + 1) * r] for i in range(len(rows))])[0] >= icc0: c += 1
    return (c + 1) / (B + 1)

def run(p, k, R, ncand, rng, B, mfac):
    sq = bytearray(p)
    for x in range(1, p): sq[x * x % p] = 1
    classes = {}
    for _ in range(ncand):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        n = p + 1 + sum(1 if sq[r] else (-1 if r else 0) for r in ((x * x * x + a * x + b) % p for x in range(p)))
        j = (1728 * 4 * a ** 3 * pow(4 * a ** 3 + 27 * b * b, -1, p)) % p
        classes.setdefault(n, {})[j] = (a, b)
    n, js = max(classes.items(), key=lambda kv: len(kv[1]))
    curves = list(js.values())[:k]
    N = n; m = max(8, int(mfac * math.sqrt(N)))
    arms = {"xwindow": [], "null_random": [], "positive": []}
    for ci, (a, b) in enumerate(curves):
        pts = curve_points(p, a, b, sq)
        assert len(pts) + 1 == N, (len(pts), N)
        srt = sorted(pts)
        for arm in arms:
            row = []
            for rep in range(R):
                mm = m if not (arm == "positive" and ci == 0) else int(m * 1.15)
                if arm == "null_random":
                    base = rng.sample(pts, m)
                else:
                    w = rng.randrange(len(srt))  # window start (cyclic in x order)
                    base = [srt[(w + i) % len(srt)] for i in range(mm)]
                row.append(coverage(base, a, p, N))
            arms[arm].append(row)
    out = {"p": p, "N": N, "n_curves": len(curves), "fb_size": m, "reps": R}
    for arm, rows in arms.items():
        icc, F = anova(rows)
        out[arm] = {"icc": round(icc, 4), "F": round(F, 3), "perm_p": perm_p(rows, rng, B),
                    "mean_cov": round(sum(map(sum, rows)) / (len(rows) * R), 4)}
    return out

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    rng = random.Random(seed); t0 = time.time()
    if smoke: cfgs = [(next_prime(1000), 4, 4, 150, 100, 1.5)]
    else: cfgs = [(next_prime(2 ** 13), 8, 6, 900, 2000, 1.5), (next_prime(2 ** 15), 8, 6, 900, 2000, 1.5)]
    res = [run(p, k, R, nc, rng, B, mf) for p, k, R, nc, B, mf in cfgs]
    pos_ok = all(r["positive"]["perm_p"] < 0.01 for r in res)
    null_ok = all(r["null_random"]["perm_p"] > 0.01 for r in res)
    effect = [r["xwindow"]["perm_p"] < 0.01 and r["xwindow"]["icc"] > 0.05 for r in res]
    if not (pos_ok and null_ok): v = "inconclusive"
    elif any(effect): v = "falsified"
    else: v = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-ICE-curve-presentation-invariance", "seed": seed, "smoke": smoke,
        "statistic": "one-way ICC of 2-decomposition coverage across same-order curves, permutation p",
        "thresholds": {"effect": "perm_p<0.01 and icc>0.05", "pos_ctrl": "perm_p<0.01", "null": "perm_p>0.01"},
        "positive_control_ok": pos_ok, "null_ok": null_ok, "results": res,
        "seconds": round(time.time() - t0, 1), "verdict": v}))

main()
