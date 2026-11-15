import sys, json, random
SMOKE = '--smoke' in sys.argv
def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
def nextprime(n):
    while not isprime(n): n += 1
    return n
def mkcurve(p, rng):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p: break
    pts = []
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    for x in range(p):
        for y in sq.get((x**3 + a*x + b) % p, []): pts.append((x, y))
    return a, b, pts
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3*P[0]*P[0] + a) * pow(2*P[1], -1, p) % p
    else:
        l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
    x = (l*l - P[0] - Q[0]) % p
    return (x, (l*(P[0]-x) - P[1]) % p)
def run():
    ks = [8, 9] if SMOKE else [8, 9, 10, 11, 12, 13]
    per = 1 if SMOKE else 2
    npairs = 200 if SMOKE else 3000
    seed0 = 20261110
    # maps: name -> (f, op) ; op(u,v) combine in target
    maps = {
     'xadd': (lambda P, p: P[0], lambda u, v, p: (u+v) % p),
     'yadd': (lambda P, p: P[1], lambda u, v, p: (u+v) % p),
     'xmul': (lambda P, p: P[0], lambda u, v, p: u*v % p),
     'ymul': (lambda P, p: P[1], lambda u, v, p: u*v % p),
     'xy_poly': (lambda P, p: (P[0]*P[1]+P[0]*P[0]) % p, lambda u, v, p: (u+v) % p),
     'legy': (lambda P, p: 1 if pow(P[1], (p-1)//2, p) == 1 else -1, lambda u, v, p: u*v),
    }
    res = {n: [] for n in maps}; res['null_random'] = []
    ctrl = {'dbl': [], 'tpl': [], 'neg': []}
    aa = []; idx = 0; ncurves = 0
    for k in ks:
        p = nextprime(2**k)
        for c in range(per):
            rng = random.Random(seed0 + idx); idx += 1
            a, b, pts = mkcurve(p, rng)
            if len(pts) < 20: continue
            ncurves += 1
            nullv = {}
            def nullf(P):
                if P not in nullv: nullv[P] = rng.randrange(p)
                return nullv[P]
            for name, (f, op) in list(maps.items()) + [('null_random', (None, lambda u, v, p: (u+v) % p))]:
                rates = []
                for stream in 'AB':
                    r2 = random.Random(seed0 * 7 + idx * 13 + (1 if stream == 'A' else 2))
                    ok = tot = 0
                    while tot < npairs:
                        P, Q = r2.choice(pts), r2.choice(pts)
                        if P == Q or (P[0] == Q[0]): continue
                        S = add(P, Q, a, p)
                        if S is None: continue
                        fv = nullf if f is None else (lambda X: f(X, p))
                        tot += 1
                        if fv(S) == op(fv(P), fv(Q), p): ok += 1
                    rates.append(ok / tot)
                res[name].append(max(rates)) if name != 'legy' else res[name].append(rates[0])
                if name != "legy": aa.append(abs(rates[0] - rates[1]))
            for cn in ctrl:
                r2 = random.Random(seed0 + 999 + idx); ok = tot = 0
                while tot < npairs:
                    P, Q = r2.choice(pts), r2.choice(pts)
                    if P == Q or P[0] == Q[0]: continue
                    S = add(P, Q, a, p)
                    if S is None: continue
                    g = {'dbl': lambda X: add(X, X, a, p), 'tpl': lambda X: add(add(X, X, a, p), X, a, p),
                         'neg': lambda X: (X[0], (-X[1]) % p)}[cn]
                    tot += 1
                    if g(S) == add(g(P), g(Q), a, p): ok += 1
                ctrl[cn].append(ok / tot)
    neg_names = [n for n in maps if n != 'legy']
    maxneg = max(max(res[n]) for n in neg_names)
    nullmax = max(res['null_random'])
    ctrl_ok = all(min(v) == 1.0 for v in ctrl.values())
    aa_max = max(aa)
    legy_ok = all(r < 0.95 for r in res['legy'])
    if ctrl_ok and nullmax < 0.01 and aa_max <= 0.02 and maxneg < 0.01 and legy_ok:
        v = 'consistent_with_prediction'
    elif ctrl_ok and maxneg >= 0.05:
        v = 'falsified'
    else:
        v = 'inconclusive'
    if SMOKE and v == 'inconclusive' and not (ctrl_ok and maxneg < 0.05):
        pass
    print(json.dumps({'lead': 'LEAD-EXO-ideas-corpus-algebraic-transforms', 'smoke': SMOKE,
        'seed': seed0, 'primes_exponents': ks, 'curves': ncurves, 'pairs_per_cell': npairs,
        'max_negative_rate': maxneg, 'null_random_max': nullmax, 'aa_max_diff': aa_max,
        'legy_rates': res['legy'], 'controls_min': {k: min(v) for k, v in ctrl.items()},
        'verdict': v}))
run()
