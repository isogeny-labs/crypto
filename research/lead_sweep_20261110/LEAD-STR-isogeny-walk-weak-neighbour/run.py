import json, random, sys, math
SMOKE = "--smoke" in sys.argv
def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
def order(a, b, p):
    sq = [0]*p
    for y in range(p): sq[y*y % p] += 1
    n = 1
    for x in range(p):
        n += sq[(x*x*x + a*x + b) % p]
    return n
def roots2(a, b, p):
    return [x for x in range(p) if (x*x*x + a*x + b) % p == 0]
def velu2(a, b, x0, p):
    t = (3*x0*x0 + a) % p; w = x0*t % p
    return (a - 5*t) % p, (b - 7*w) % p
def nonsing(a, b, p): return (4*a**3 + 27*b*b) % p != 0
def factor(n):
    f = []; d = 2
    while d*d <= n:
        while n % d == 0: f.append(d); n //= d
        d += 1
    if n > 1: f.append(n)
    return f
def flags(N, p):
    fs = factor(N); lp = max(fs)
    emb = next((k for k in range(1, 13) if pow(p, k, lp) == 1), 0) if lp > 1 else 0
    return (N == p, lp, emb > 0 and emb <= 6)
def add(P, Q, a, p):
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
def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R
def randpt(a, b, p, rng):
    while True:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        if r == 0: continue
        if pow(r, (p-1)//2, p) == 1:
            y = pow(r, (p+1)//4, p) if p % 4 == 3 else next(y for y in range(p) if y*y % p == r)
            return (x, y)
def rho_steps(a, b, p, q, G, rng, R=16):
    # Pollard rho with R-adding walk, collision of (a,b) coefficients -> count steps to Floyd meet
    k = rng.randrange(1, q); H = mul(k, G, a, p)
    tab = []
    for _ in range(R):
        u, v = rng.randrange(1, q), rng.randrange(1, q)
        tab.append((u, v, add(mul(u, G, a, p), mul(v, H, a, p), a, p)))
    def step(S):
        X, c, d = S
        if X is None: j = 0
        else: j = X[0] % R
        u, v, T = tab[j]
        return (add(X, T, a, p), (c+u) % q, (d+v) % q)
    s0 = (add(mul(rng.randrange(1, q), G, a, p), None, a, p), 0, 0)
    T_, Hh = s0[0], None
    x = (T_, 0, 0); y = x; n = 0
    while n < 20*q:
        x = step(x); y = step(step(y)); n += 1
        if x[0] == y[0]: return n
    return n
def setup(a, b, p, N, rng):
    fs = factor(N); q = max(fs)
    while True:
        P = randpt(a, b, p, rng); G = mul(N//q, P, a, p)
        if G is not None: return q, G
def main():
    rng = random.Random(20261110)
    ps_target = [1021, 4093] if SMOKE else [4093, 16381, 65521]
    nroots = 3 if SMOKE else 12
    depth = 4 if SMOKE else 8
    prime_list = []
    for t in ps_target:
        p = t
        while not (is_prime(p) and p % 4 == 3): p -= 1
        prime_list.append(p)
    tot = mism = flagmism = 0; ctrl_diff = ctrl_n = 0; null_eq = null_n = 0
    for p in prime_list:
        for _ in range(nroots):
            while True:
                a, b = rng.randrange(p), rng.randrange(p)
                if nonsing(a, b, p): break
            N0 = order(a, b, p); f0 = flags(N0, p)
            ca, cb = a, b
            for _s in range(depth):
                rs = roots2(ca, cb, p)
                if not rs: break
                x0 = rng.choice(rs); na, nb = velu2(ca, cb, x0, p)
                if not nonsing(na, nb, p): break
                N = order(na, nb, p); tot += 1
                if N != N0: mism += 1
                if flags(N, p) != f0: flagmism += 1
                ca, cb = na, nb
            # positive control: quadratic twist
            d = next(d for d in range(2, p) if pow(d, (p-1)//2, p) == p-1)
            Nt = order(a*d*d % p, b*d**3 % p, p); ctrl_n += 1
            if Nt != N0: ctrl_diff += 1
            # null: random curve, same order?
            while True:
                ra, rb = rng.randrange(p), rng.randrange(p)
                if nonsing(ra, rb, p): break
            null_n += 1; null_eq += (order(ra, rb, p) == N0)
    # rho cost comparison root vs 2-isogenous neighbour (prime order part), plus A/A
    p = prime_list[0]; trials = 10 if SMOKE else 150
    cand = []
    while len(cand) < (1 if SMOKE else 6):
        a, b = rng.randrange(p), rng.randrange(p)
        if not nonsing(a, b, p): continue
        N = order(a, b, p)
        if max(factor(N)) < (30 if SMOKE else 300): continue
        rs = roots2(a, b, p)
        if not rs: continue
        na, nb = velu2(a, b, rs[0], p)
        if nonsing(na, nb, p): cand.append((a, b, na, nb, N))
    ra_, rn_, rb_ = [], [], []
    for a, b, na, nb, N in cand:
        q, G = setup(a, b, p, N, rng); q2, G2 = setup(na, nb, p, N, rng)
        assert q == q2
        ra_ += [rho_steps(a, b, p, q, G, rng) for _ in range(trials)]
        rb_ += [rho_steps(a, b, p, q, G, rng) for _ in range(trials)]
        rn_ += [rho_steps(na, nb, p, q2, G2, rng) for _ in range(trials)]
    m = lambda v: sum(v)/max(1, len(v))
    ratio = m(rn_)/m(ra_); aa = m(rb_)/m(ra_)
    ok_ctrl = ctrl_n > 0 and ctrl_diff/ctrl_n >= 0.8
    if mism or flagmism: verdict = "falsified"
    elif ok_ctrl and 0.8 <= ratio <= 1.25 and 0.8 <= aa <= 1.25 and tot > 0: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-STR-isogeny-walk-weak-neighbour", "smoke": SMOKE, "seed": 20261110,
        "primes": prime_list, "roots_per_prime": nroots, "walk_depth": depth,
        "walk_curves": tot, "order_mismatch": mism, "weak_flag_mismatch": flagmism,
        "positive_control_twist_order_differs": [ctrl_diff, ctrl_n],
        "null_random_curve_same_order": [null_eq, null_n],
        "rho_mean_steps": {"root": m(ra_), "neighbour": m(rn_), "root_AA": m(rb_)},
        "rho_ratio_neighbour_over_root": ratio, "rho_ratio_AA": aa, "rho_trials_per_arm": len(ra_),
        "verdict": verdict}))
main()
