import json, random, sys, math, time
SMOKE = "--smoke" in sys.argv
def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
def count(p, a, b):
    sq = [0]*p
    for y in range(p): sq[y*y % p] += 1
    return 1 + sum(sq[(x**3+a*x+b) % p] for x in range(p))
def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1+y2) % p == 0: return None
        l = (3*x1*x1+a)*pow(2*y1, -1, p) % p
    else:
        l = (y2-y1)*pow(x2-x1, -1, p) % p
    x3 = (l*l-x1-x2) % p
    return (x3, (l*(x1-x3)-y1) % p)
def make_curve(p, rng):
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4*a**3+27*b*b) % p == 0: continue
        N = count(p, a, b)
        if isprime(N) and N > 100: return a, b, N
def logs_table(p, a, b, N, rng):
    pts = [(x, y) for x in range(p) for y in range(p) if False]  # unused
    while True:
        x = rng.randrange(p); r = (x**3+a*x+b) % p
        if pow(r, (p-1)//2, p) == 1:
            for y in range(p):
                if y*y % p == r: break
            G = (x, y); break
    tab = {}; P = None
    for k in range(1, N):
        P = add(P, G, p, a); tab[P] = k
    return tab
def sumset_cov(F, N, M):
    S = set(F); out = {1: len(S)/N}
    for m in range(2, M+1):
        S = {(s+f) % N for s in S for f in F}
        out[m] = len(S)/N
    return out
def sym(vals, N):
    s = set()
    for v in vals: s.add(v % N); s.add((-v) % N)
    s.discard(0); return sorted(s)
def run():
    rng = random.Random(20261110)
    primes = [1031] if SMOKE else [40009, 52837, 65521]
    M = 3 if SMOKE else 4
    nnull = 3 if SMOKE else 8
    rows = []
    for p in primes:
        while not isprime(p): p += 1
        a, b, N = make_curve(p, rng)
        tab = logs_table(p, a, b, N, rng)
        target_f = 12 if SMOKE else 40
        B = 1
        while True:
            fb = [tab[(x, y)] for (x, y) in tab if x < B]
            if len(fb) >= target_f: break
            B += 1
        F = sym(fb, N); f = len(F)
        act = sumset_cov(F, N, M)
        def null():
            half = rng.sample(range(1, N), f//2)
            return sumset_cov(sym(half, N), N, M)
        nulls = [null() for _ in range(nnull)]
        aa = null()
        k = f//2
        pos = sumset_cov(sym(range(1, k+1), N), N, M)  # arithmetic progression: structured
        r = {}
        for m in range(2, M+1):
            xs = [n[m] for n in nulls]; mu = sum(xs)/len(xs)
            sd = math.sqrt(sum((x-mu)**2 for x in xs)/(len(xs)-1)) or 1e-9
            r[m] = dict(actual=act[m], null_mean=mu, null_sd=sd, ratio=act[m]/mu,
                        z=(act[m]-mu)/sd, aa_ratio=aa[m]/mu, pos_ratio=pos[m]/mu)
        rows.append(dict(p=p, a=a, b=b, N=N, B=B, fb_size=f, arity=r))
    # decision: |ratio-1|<=0.05 for all cells and positive control ratio outside [0.8,1.25] at m=2
    cells = [c for row in rows for c in row["arity"].values()]
    near = all(abs(c["ratio"]-1) <= 0.05 for c in cells)
    pos_ok = all(abs(row["arity"][2]["pos_ratio"]-1) > 0.2 for row in rows)
    aa_ok = all(abs(c["aa_ratio"]-1) <= 0.05 for c in cells if c["null_mean"] < 0.9)
    if SMOKE: near = True if cells else False
    if not (pos_ok or SMOKE) or not aa_ok and not SMOKE: v = "inconclusive"
    elif near: v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps(dict(lead="LEAD-ICP-full-width-arity-grid", smoke=SMOKE, seed=20261110,
        params=dict(primes=primes, max_arity=M, null_draws=nnull, target_fb=target_f),
        statistic="coverage ratio actual/generic-null of m-fold sumset of small-x factor base logs",
        threshold="all |ratio-1|<=0.05; AA<=0.05; positive control |ratio-1|>0.2 at m=2",
        rows=rows, checks=dict(near_null=near, positive_control_detects=pos_ok, aa_ok=aa_ok), verdict=v)))
run()
