#!/usr/bin/env python3
"""LEAD-ICB-factor-base-subspace-choice toy: 3-sum relation yield vs structure of V. stdlib only."""
import sys, json, random, math

def build_field(m):
    # find irreducible poly via x^(2^m)=x and gcd test (brute: check generator order works)
    for low in range(1, 1 << m, 2):
        f = (1 << m) | low
        # trial: build exp table from x, check cycle length 2^m-1 (primitive implies irreducible)
        n = (1 << m) - 1
        e = [0] * n
        v = 1
        ok = True
        for i in range(n):
            e[i] = v
            v <<= 1
            if v >> m:
                v ^= f
            if v == 1 and i < n - 1:
                ok = False
                break
        if ok and v == 1:
            return f, e
    raise RuntimeError

def run(m, d, ncurves, nnull, ninst, seed, smoke=False):
    f, exp = build_field(m)
    n = (1 << m) - 1
    log = [0] * (1 << m)
    for i, v in enumerate(exp):
        log[v] = i
    def mul(a, b):
        return 0 if a == 0 or b == 0 else exp[(log[a] + log[b]) % n]
    def inv(a):
        return exp[(-log[a]) % n]
    sq = [0] + [exp[(2 * log[v]) % n] for v in range(1, 1 << m)]
    zmap = {}  # z^2+z -> z
    for z in range(1 << m):
        zmap.setdefault(sq[z] ^ z, z)
    rng = random.Random(seed)

    def span(basis):
        s = [0]
        for b in basis:
            s = s + [x ^ b for x in s]
        return s
    def rand_indep(k, within=None):
        basis, red = [], []
        while len(basis) < k:
            v = rng.randrange(1, 1 << m)
            if within is not None and not within(v):
                continue
            w = v
            for r in red:
                w = min(w, w ^ r)
            if w:
                red.append(w); basis.append(v)
        return basis
    def trace(v):
        t, x = 0, v
        for _ in range(m):
            t ^= x; x = sq[x]
        return t  # 0 or 1
    def dimVV(basis):
        red = []
        for i in range(len(basis)):
            for j in range(i, len(basis)):
                w = mul(basis[i], basis[j])
                for r in red:
                    w = min(w, w ^ r)
                if w:
                    red.append(w)
        return len(red)
    def fam_geom():
        g = rng.randrange(2, 1 << m)
        b = [1]
        for _ in range(d - 1):
            b.append(mul(b[-1], g))
        return b
    def fam_lowvv():
        best = None
        for _ in range(30):
            b = rand_indep(d); q = dimVV(b)
            if best is None or q < best[0]:
                best = (q, b)
        return best[1]
    def fam_trace0():
        return rand_indep(d, within=lambda v: trace(v) == 0)
    fams = {"random_null": lambda: rand_indep(d), "random_AA": lambda: rand_indep(d),
            "geometric_progression": fam_geom, "min_dimVV_of_30": fam_lowvv, "trace0_window": fam_trace0}

    def points(V, a, b):
        pts = []
        for x in V:
            if x == 0:
                continue
            c = x ^ a ^ mul(b, sq[inv(x)])
            z = zmap.get(c)
            if z is None:
                continue
            y = mul(x, z)
            pts.append((x, y)); pts.append((x, y ^ x))
        return pts
    def yield_of(V, a, b):
        Vs = set(V)
        pts = points(V, a, b)
        cnt = 0
        for i in range(len(pts)):
            x1, y1 = pts[i]
            for j in range(i + 1, len(pts)):
                x2, y2 = pts[j]
                if x1 == x2:
                    continue
                lam = mul(y1 ^ y2, inv(x1 ^ x2))
                if (sq[lam] ^ lam ^ a ^ x1 ^ x2) in Vs:
                    cnt += 1
        return cnt, len(pts)

    # per-curve log-ratios (family vs null), curve-blocked
    logr = {k: [] for k in fams if k != "random_null"}
    pos_ratios = []
    for c in range(ncurves):
        a = rng.choice([0, 1]); b = rng.randrange(1, 1 << m)
        def norm(basis):
            y, s = yield_of(span(basis), a, b)
            return y / max(s, 1) ** 3, y, s
        nullm = sum(norm(fams["random_null"]())[0] for _ in range(nnull)) / nnull
        for k in logr:
            vals = [norm(fams[k]())[0] for _ in range(ninst)]
            mv = sum(vals) / len(vals)
            logr[k].append(math.log((mv + 1e-9) / (nullm + 1e-9)))
        # positive control: subfield GF(2^(m/2)), curve over subfield
        h = m // 2
        sub = [0] + [exp[(k * ((1 << m) - 1) // ((1 << h) - 1)) % n] for k in range((1 << h) - 1)]
        # subfield elements are closed under + so span is subspace of dim h; take curve coeffs in subfield
        sb = sub[rng.randrange(1, len(sub))]
        # norm by |S|^3 with dim h subspace is not comparable in size; compare raw closure fraction instead
        y, s = yield_of(sub, 0, sb)
        tot = s * (s - 1) // 2
        pos_ratios.append((y / max(tot, 1)) / (((1 << h) / (1 << m))))  # fraction landing vs random-landing prob
    def tstat(xs):
        mu = sum(xs) / len(xs)
        var = sum((x - mu) ** 2 for x in xs) / max(len(xs) - 1, 1)
        se = math.sqrt(var / len(xs)) or 1e-12
        return mu, mu / se
    res = {}
    for k, xs in logr.items():
        mu, t = tstat(xs)
        res[k] = {"mean_log_ratio": round(mu, 4), "t": round(t, 3), "ratio": round(math.exp(mu), 4)}
    pos = sum(pos_ratios) / len(pos_ratios)
    T = 4.0
    aa_ok = abs(res["random_AA"]["t"]) <= T
    structured = {k: v for k, v in res.items() if k != "random_AA"}
    hit = [k for k, v in structured.items() if v["t"] > T]
    if not aa_ok or pos < 5:
        verdict = "inconclusive"
    elif hit:
        verdict = "falsified"
    else:
        verdict = "consistent_with_prediction"
    return {"lead": "LEAD-ICB-factor-base-subspace-choice", "m": m, "d": d, "ncurves": ncurves,
            "n_null": nnull, "n_inst": ninst, "seed": seed, "threshold_t": T,
            "families": res, "positive_control_subfield_closure_enrichment": round(pos, 2),
            "aa_ok": aa_ok, "families_above_threshold": hit, "verdict": verdict, "smoke": smoke}

if __name__ == "__main__":
    if "--smoke" in sys.argv:
        out = run(10, 4, 3, 4, 3, 1, True)
    else:
        out = run(16, 8, 12, 40, 8, 20261110)
    print(json.dumps(out))
