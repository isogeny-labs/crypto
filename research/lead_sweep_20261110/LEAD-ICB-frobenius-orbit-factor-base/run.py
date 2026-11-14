#!/usr/bin/env python3
"""LEAD-ICB-frobenius-orbit-factor-base: does a Frobenius-orbit-closed factor base give
a 2-decomposition yield beyond a random same-size base (beyond the trivial 1/m column saving)?
Stdlib only, deterministic."""
import sys, json, random, math

POLY = {7: 0x83, 11: (1 << 11) | 5, 13: (1 << 13) | 0x1B, 17: (1 << 17) | 9}
A_COEF = 1  # y^2+xy=x^3+x^2+1 (Koblitz, a=1,b=1)

def build_field(m):
    n = (1 << m) - 1
    exp = [0] * (2 * n + 2); log = [0] * (n + 1)
    x = 1
    for i in range(n):
        exp[i] = x; log[x] = i
        x <<= 1
        if x >> m: x ^= POLY[m]
    assert x == 1 and len(set(exp[:n])) == n, "poly not primitive"
    for i in range(n, 2 * n + 2): exp[i] = exp[i - n]
    return n, exp, log

def make_curve(m):
    n, EXP, LOG = build_field(m)
    def mul(a, b): return 0 if a == 0 or b == 0 else EXP[LOG[a] + LOG[b]]
    def inv(a): return EXP[n - LOG[a]]
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            l = x1 ^ mul(y1, inv(x1))
            x3 = mul(l, l) ^ l ^ A_COEF
            return (x3, mul(x1, x1) ^ mul(l ^ 1, x3))
        l = mul(y1 ^ y2, inv(x1 ^ x2))
        x3 = mul(l, l) ^ l ^ x1 ^ x2 ^ A_COEF
        return (x3, mul(l, x1 ^ x3) ^ x3 ^ y1)
    pts = [(0, 1)]
    for x in range(1, n + 1):
        xi = inv(x); c = x ^ 1 ^ mul(xi, xi)
        z = 0; t = c
        for _ in range((m + 1) // 2):
            z ^= t; t = mul(mul(t, t), mul(t, t))
        if mul(z, z) ^ z == c:
            pts.append((x, mul(x, z))); pts.append((x, mul(x, z ^ 1)))
    return add, mul, pts

def frob(P, mul): return (mul(P[0], P[0]), mul(P[1], P[1]))

def coverage(add, B):
    s = set(); L = list(B)
    for i in range(len(L)):
        for j in range(i, len(L)):
            s.add(add(L[i], L[j]))
    return len(s)

def stats(v):
    mu = sum(v) / len(v)
    sd = math.sqrt(sum((x - mu) ** 2 for x in v) / max(1, len(v) - 1))
    return mu, sd / math.sqrt(len(v))

def run_m(m, k, draws):
    add, mul, pts = make_curve(m)
    N = len(pts) + 1  # + infinity
    cand = [P for P in pts if mul(P[0], P[0]) != P[0]]
    seen = set(); orbits = []
    for P in cand:
        if P in seen: continue
        o = [P]; Q = frob(P, mul)
        while Q != P: o.append(Q); Q = frob(Q, mul)
        seen.update(o); orbits.append(o)
    assert all(len(o) == m for o in orbits)
    B = k * m
    cov = {"orbit": [], "random": [], "random_AA": [], "progression": []}
    # positive control: arithmetic progression jG (sums collapse), G = a point of large order
    G = cand[0]; prog = []; Q = None
    while len(prog) < B:
        Q = add(Q, G)
        if Q is not None: prog.append(Q)
        if Q is None and len(prog) < B: break  # tiny-order G: fall back handled below
    for d in range(draws):
        r1 = random.Random(1000003 * m + d)
        o = r1.sample(orbits, k); cov["orbit"].append(coverage(add, [P for x in o for P in x]) / N)
        r2 = random.Random(2000003 * m + d); cov["random"].append(coverage(add, r2.sample(cand, B)) / N)
        r3 = random.Random(3000003 * m + d); cov["random_AA"].append(coverage(add, r3.sample(cand, B)) / N)
    cov["progression"] = [coverage(add, prog) / N]
    mo, so = stats(cov["orbit"]); mr, sr = stats(cov["random"]); ma, sa = stats(cov["random_AA"])
    ratio = mo / mr
    se = ratio * math.sqrt((so / mo) ** 2 + (sr / mr) ** 2)
    aa = ma / mr; aase = aa * math.sqrt((sa / ma) ** 2 + (sr / mr) ** 2)
    return {"m": m, "group_order": N, "orbits": len(orbits), "B": B, "orbits_per_base": k,
            "draws": draws, "cov_orbit": mo, "cov_random": mr, "cov_random_AA": ma,
            "ratio_orbit_over_random": ratio, "ratio_se": se, "AA_ratio": aa, "AA_se": aase,
            "positive_control_progression_cov": cov["progression"][0],
            "positive_control_ratio": cov["progression"][0] / mr,
            "trivial_column_saving_factor": m,
            "beyond_trivial_factor": ratio}

def main():
    smoke = "--smoke" in sys.argv
    cfg = [(7, 2, 5)] if smoke else [(11, 3, 200), (13, 3, 200), (17, 11, 200)]
    rows = [run_m(*c) for c in cfg]
    band = 0.05
    ok = True; fal = False
    for r in rows:
        lo, hi = r["ratio_orbit_over_random"] - 2 * r["ratio_se"], r["ratio_orbit_over_random"] + 2 * r["ratio_se"]
        r["ci95"] = [lo, hi]
        within = lo >= 1 - band and hi <= 1 + band
        outside = hi < 1 - band or lo > 1 + band
        aa_ok = abs(r["AA_ratio"] - 1) <= 2 * r["AA_se"] + 1e-9
        pos_ok = r["positive_control_ratio"] < 0.5
        r["checks"] = {"within_band": within, "outside_band": outside, "aa_ok": aa_ok, "positive_control_ok": pos_ok}
        if not (aa_ok and pos_ok): ok = False
        if outside: fal = True
        if not within: ok = ok and False if not outside else ok
    consistent = all(r["checks"]["within_band"] for r in rows)
    if not all(r["checks"]["aa_ok"] and r["checks"]["positive_control_ok"] for r in rows): verdict = "inconclusive"
    elif fal: verdict = "falsified"
    elif consistent: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICB-frobenius-orbit-factor-base", "smoke": smoke, "band": band,
                      "seeds": "random.Random(k*m+draw), k in 1000003,2000003,3000003",
                      "rows": rows, "verdict": verdict}))
main()
