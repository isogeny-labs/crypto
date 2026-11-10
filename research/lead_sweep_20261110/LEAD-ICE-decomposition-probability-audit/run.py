import sys, json, random, math
SMOKE = "--smoke" in sys.argv

def nextprime34(n):
    n |= 3
    while True:
        if all(n % d for d in range(2, int(n**.5) + 1)):
            return n
        n += 4

def build(p, rng):
    # returns (n, log map x->k list of (x,k) for k in 1..n-1) for cyclic group <G> on random curve
    for _ in range(200):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        while True:
            x = rng.randrange(p); r = (x**3 + a*x + b) % p
            y = pow(r, (p+1)//4, p)
            if y*y % p == r and y: break
        G = (x, y); P = G; k = 1; pts = [G]
        while True:
            (x1, y1), (x2, y2) = P, G
            if x1 == x2:
                if (y1 + y2) % p == 0: break
                l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
            else:
                l = (y2 - y1) * pow(x2 - x1, -1, p) % p
            x3 = (l*l - x1 - x2) % p
            P = (x3, (l*(x1 - x3) - y1) % p); k += 1
            pts.append(P)
        n = k + 1  # order of G (P at step k is -G ... last step hit inverse of previous)
        # pts[j] = (j+1)G for j<k ; (k+1)G = infinity
        if n > 0.6 * p:
            return n, pts
    raise RuntimeError("no curve")

def disp(F, n):
    r = [0]*n
    for a in F:
        for b in F:
            r[(a+b) % n] += 1
    m = sum(r)/n
    v = sum((c-m)**2 for c in r)/n
    return v/m

def run(sizes, ncurves, ndraw, seed0):
    out = []
    for si, lg in enumerate(sizes):
        p = nextprime34(1 << lg)
        for c in range(ncurves):
            seed = seed0 + 1000*si + c
            rng = random.Random(seed)
            n, pts = build(p, rng)
            m = int(4*math.sqrt(n)) // 2 * 2
            h = m // 2
            # x-small FB: smallest x among points k=1..n//2 (one of +-)
            half = sorted((pts[j][0], j+1) for j in range(n//2 - 1 + (n % 2)) if (j+1)*2 != n)
            xs = [k for _, k in half[:h]]
            fb_x = xs + [n-k for k in xs]
            def rnd(rr):
                s = rr.sample(range(1, (n-1)//2 + 1), h)
                return s + [n-k for k in s]
            def interval():
                s = list(range(1, h+1)); return s + [n-k for k in s]
            R = random.Random(seed*7+1)
            null = [disp(rnd(R), n) for _ in range(ndraw)]
            mu = sum(null)/ndraw; sd = (sum((u-mu)**2 for u in null)/(ndraw-1))**.5
            dx = disp(fb_x, n); dp = disp(interval(), n)
            aa = disp(rnd(R), n); aa2 = disp(rnd(R), n)
            out.append(dict(lg=lg, p=p, n=n, m=m, seed=seed, null_mean=mu, null_sd=sd,
                z_xsmall=(dx-mu)/sd, z_pos=(dp-mu)/sd, z_aa=(aa-aa2)/(sd*2**.5), D_x=dx))
    return out

def main():
    if SMOKE: sizes, nc, nd = [9], 1, 6
    else: sizes, nc, nd = [12, 14, 16], 4, 24
    rows = run(sizes, nc, nd, 20261110)
    K = len(rows)
    stou = lambda k: sum(r[k] for r in rows)/K**.5
    sx, sp, sa = stou("z_xsmall"), stou("z_pos"), stou("z_aa")
    pos_ok = all(r["z_pos"] > 5 for r in rows)
    if not pos_ok or abs(sa) > 3: v = "inconclusive"
    elif abs(sx) < 3 and all(abs(r["z_xsmall"]) < 4 for r in rows): v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps(dict(lead="LEAD-ICE-decomposition-probability-audit", smoke=SMOKE,
        statistic="index of dispersion D=Var_T(r)/Mean_T(r) of 2-point decomposition counts; Stouffer z of x-small FB vs random-FB null",
        stouffer_z_xsmall=sx, stouffer_z_positive_control=sp, stouffer_z_AA=sa,
        positive_control_all_z_gt_5=pos_ok, seeds_base=20261110,
        params=dict(sizes_log2=sizes, curves_per_size=nc, null_draws=nd, fb_size="4*sqrt(n)"),
        rows=rows, verdict=v)))
main()
