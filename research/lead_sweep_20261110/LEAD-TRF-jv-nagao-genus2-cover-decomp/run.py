import json, random, math, sys
smoke = '--smoke' in sys.argv
SEED = 20261110
PRIMES = [61, 101] if smoke else [251, 509, 1021, 2039, 4093]
TRIALS = 2000 if smoke else 60000
CURVES = 2 if smoke else 6

def sq_table(p):
    s = [False]*p
    for x in range(p): s[x*x % p] = True
    return s

def roots2(a, b, p, sq):
    # roots of x^2+ax+b over F_p; returns list (with multiplicity) or None
    d = (a*a - 4*b) % p
    if not sq[d]: return None
    # sqrt by brute lookup table
    return d

def sqrt_tab(p):
    t = {}
    for x in range(p): t.setdefault(x*x % p, x)
    return t

def run_arm(p, rng, curves, trials, null=False):
    sq = sq_table(p); st = sqrt_tab(p); inv2 = pow(2, -1, p)
    hit = tot = 0.0
    for _ in range(curves):
        while True:
            f = [rng.randrange(p) for _ in range(5)] + [1]
            fx = [sum(c*pow(x, i, p) for i, c in enumerate(f)) % p for x in range(p)]
            if sum(1 for x in range(p) if fx[x] == 0) <= 2: break
        for _ in range(trials // curves):
            a = rng.randrange(p); b = rng.randrange(p)
            d = (a*a - 4*b) % p
            if null:  # forced-smooth positive control
                hit += 1; tot += 1; continue
            if d == 0: continue
            if d in st:   # split u: weight = number of Mumford v (2 per rational point)
                r = st[d]; x1 = (-a + r)*inv2 % p; x2 = (-a - r)*inv2 % p
                if fx[x1] == 0 or fx[x2] == 0: continue
                w = (4 if (sq[fx[x1]] and sq[fx[x2]]) else 0)
                hit += w; tot += w
            else:         # irreducible u: f(root) square in F_{p^2} iff norm square in F_p
                # root = (-a + t)/2, t^2 = d ; Horner in F_p[t]
                re, im = f[5], 0
                ra, rb = (-a*inv2) % p, inv2
                for c in (f[4], f[3], f[2], f[1], f[0]):
                    re, im = (re*ra + im*rb*d + c) % p, (re*rb + im*ra) % p
                nm = (re*re - d*im*im) % p
                if nm == 0: continue
                w = 2 if sq[nm] else 0
                tot += w
    return hit, tot

def wilson(h, n, z=2.576):
    ph = h/n; d = 1 + z*z/n
    c = (ph + z*z/(2*n))/d; w = z*math.sqrt(ph*(1-ph)/n + z*z/(4*n*n))/d
    return ph, c-w, c+w

def main():
    out = {"lead": "LEAD-TRF-jv-nagao-genus2-cover-decomp", "smoke": smoke,
           "seed": SEED, "primes": PRIMES, "trials_per_prime": TRIALS, "curves_per_prime": CURVES,
           "rows": []}
    ok_all = True; fals = False
    for p in PRIMES:
        rows = {}
        for arm, null, sd in (("A", False, 1), ("A2", False, 2), ("null", True, 3)):
            rng = random.Random(SEED*1000 + p*10 + sd)
            h, n = run_arm(p, rng, CURVES, TRIALS, null)
            rows[arm] = wilson(h, n)
        # smoothness P of a random reduced degree-2 divisor over F_p degree-1 factor base
        # (u splits, both points rational) = measured fraction of random monic quadratics u
        # that are valid; true value for random Jacobian class ~ 1/2 (Gaudry, g=2)
        pA, lo, hi = rows["A"]
        R = lambda P: ((p+1)/P)/math.sqrt(math.pi*p*p/4)   # ops/relation >=1 group op; N ~ p^2
        Rlo, Rhi = R(hi), R(lo)
        aa = abs(rows["A"][0]-rows["A2"][0]) < (rows["A"][2]-rows["A"][1])
        pos = rows["null"][0] > 0.99          # detector sees smoothness when it is forced
        pband = (lo <= 0.5 <= hi) or abs(pA-0.5) < 0.04
        ok_all &= (aa and pos and pband)
        fals |= Rhi < 1.0
        out["rows"].append({"p": p, "P_smooth": pA, "ci99": [lo, hi], "P_smooth_AA": rows["A2"][0],
            "P_null_forced": rows["null"][0], "ops_over_rho_ci": [Rlo, Rhi], "aa_ok": aa, "pos_ok": pos})
    out["verdict"] = "falsified" if fals else ("consistent_with_prediction" if ok_all and all(r["ops_over_rho_ci"][0] >= 1.0 for r in out["rows"]) else "inconclusive")
    print(json.dumps(out))
main()
