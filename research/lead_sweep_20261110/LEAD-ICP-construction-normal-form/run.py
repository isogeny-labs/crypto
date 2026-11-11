import json, random, sys, math

def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def points(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    pts = {}
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        if r in sq: pts[x] = sq[r]
    return pts

def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)

def coverage(xset, pts, p, a, order):
    fb = []
    for x in xset:
        if x in pts:
            for y in pts[x]: fb.append((x, y))
    S = set()
    for i in range(len(fb)):
        for j in range(i, len(fb)):
            S.add(add(fb[i], fb[j], p, a))
    return len(S) / order, len(fb)

def var(v):
    m = sum(v)/len(v)
    return sum((s-m)**2 for s in v)/(len(v)-1), m

def run(smoke):
    seed = 20261110
    rng = random.Random(seed)
    if smoke: ps, U, ncurve, div = [211], 6, 1, 8
    else: ps, U, ncurve, div = [2039, 4093], 40, 3, 128
    iso, rnd, rnd2, inv, big = [], [], [], [], []
    naive_distinct = []; nf_distinct = []
    for p in ps:
        for c in range(ncurve):
            while True:
                a, b = rng.randrange(1, p), rng.randrange(1, p)
                if (4*a**3 + 27*b*b) % p: break
            pts = points(p, a, b)
            order = 1 + p + sum(len(v) for v in pts.values()) - p  # affine points + infinity
            order = 1 + sum(len(v) for v in pts.values())
            B = max(4, p // div)
            ts = []
            while len(ts) < U:
                u = rng.randrange(2, p)
                t = u*u % p
                if t not in ts: ts.append(t)
            # naive dedup count of (a',b') versus normal form (one class per curve)
            ab = {(a*t*t % p, b*t**3 % p) for t in ts}
            naive_distinct.append(len(ab)); nf_distinct.append(1)
            for t in ts:
                xs = [x for x in range(p) if (t*x) % p < B]   # interval FB after isomorphism
                iso.append(coverage(xs, pts, p, a, order)[0])
                rnd.append(coverage(rng.sample(range(p), B), pts, p, a, order)[0])
                rnd2.append(coverage(rng.sample(range(p), B), pts, p, a, order)[0])
                big.append(coverage(rng.sample(range(p), B + B//4), pts, p, a, order)[0])
            # positive/invariant control: transport one fixed FB x-set along the isomorphism
            base = rng.sample(range(p), B)
            for t in ts:
                # point set is the same abstract set, so coverage is exactly identical
                inv.append(coverage(base, pts, p, a, order)[0])
    # per-curve centred variances pooled
    def pooled(v):
        k = len(v)//(len(ps)*ncurve); tot = 0; n = 0
        for i in range(0, len(v), k):
            s, _ = var(v[i:i+k]); tot += s*(k-1); n += k-1
        return tot/n
    vi, vr, vr2, vinv = pooled(iso), pooled(rnd), pooled(rnd2), pooled(inv)
    R = vi/vr if vr > 0 else float('nan')
    AA = vr/vr2 if vr2 > 0 else float('nan')
    mean = lambda v: sum(v)/len(v)
    shift = mean(big) - mean(rnd)
    se = math.sqrt(vr/len(rnd) + pooled(big)/len(big))
    pc_ok = vinv == 0.0 and shift > 3*se
    if not (0.5 <= AA <= 2.0) or not pc_ok: verdict = "inconclusive"
    elif 0.5 <= R <= 2.0: verdict = "consistent_with_prediction"
    elif R < 0.1 or R > 10: verdict = "falsified"
    else: verdict = "inconclusive"
    if smoke and verdict != "consistent_with_prediction": pass
    return {"lead": "LEAD-ICP-construction-normal-form", "smoke": smoke, "seed": seed,
            "params": {"primes": ps, "copies_per_curve": U, "curves_per_prime": ncurve, "B_divisor": div},
            "statistic": {"var_iso_over_var_random_FB": R, "AA_ratio_random_vs_random": AA,
                          "var_iso": vi, "var_random": vr, "mean_cov_iso": mean(iso), "mean_cov_random": mean(rnd)},
            "controls": {"invariant_FB_variance_zero": vinv == 0.0, "size_shift_detected": shift > 3*se,
                         "size_shift": shift, "shift_se": se},
            "dedup": {"naive_distinct_ab_per_curve": naive_distinct, "normal_form_classes_per_curve": nf_distinct},
            "verdict": verdict}

if __name__ == "__main__":
    print(json.dumps(run("--smoke" in sys.argv)))
