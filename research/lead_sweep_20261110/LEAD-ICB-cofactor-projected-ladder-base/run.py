import json, random, sys, math
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

class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3*P[0]*P[0] + s.a) * pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
        x = (l*l - P[0] - Q[0]) % p
        return (x, (l*(P[0]-x) - P[1]) % p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R

def make_curve(p, h, rng, sq):
    # find curve with #E = h * prime exactly
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        N = p + 1
        for x in range(p):
            r = (x*x*x + a*x + b) % p
            N += 0 if r == 0 else (1 if r in sq else -1)
        if N % h == 0 and isprime(N // h) and N // h > 100:
            return Curve(p, a, b), N, N // h

def rand_point(C, rng, roots):
    while True:
        x = rng.randrange(C.p)
        r = (x**3 + C.a*x + C.b) % C.p
        if r in roots:
            return (x, roots[r] if rng.random() < .5 else (-roots[r]) % C.p)

def mean_reps(C, base, targets):
    S = set(base); tot = 0
    for T in targets:
        for f in base:
            if C.add(T, C.neg(f)) in S: tot += 1
    return tot  # ordered pairs (f1,f2), f1+f2=T

def row(p, h, seed, ntarg, cbase):
    rng = random.Random(seed)
    sq = {x*x % p for x in range(1, p)}
    roots = {}
    for x in range(1, p): roots.setdefault(x*x % p, x)
    C, N, n = make_curve(p, h, rng, roots)
    P = rand_point(C, rng, roots)
    while C.mul(h, P) is None: P = rand_point(C, rng, roots)
    Q = C.mul(h, P)
    assert C.mul(n, Q) is None
    B = int(cbase * math.sqrt(n))
    targets = [C.mul(rng.randrange(1, n), Q) for _ in range(ntarg)]
    full = [rand_point(C, rng, roots) for _ in range(B)]
    proj = [C.mul(h, f) for f in full]                    # projected [h]F
    proj2 = [C.mul(h, rand_point(C, rng, roots)) for _ in range(B)]  # independent projected (A/A)
    cf, cp, cp2 = (mean_reps(C, b, targets) for b in (full, proj, proj2))
    return dict(p=p, h=h, N=N, n=n, B=B, ntargets=ntarg, reps_full=cf, reps_proj=cp, reps_proj_indep=cp2)

def ratio_ci(a, b):
    r = a / b if b else float('nan')
    rel = math.sqrt(1/max(a, 1) + 1/max(b, 1))
    return r, rel

def main():
    cfg = dict(primes=[nextprime(300)], hs=[1, 2], ntarg=20, cbase=2.0, reps=1) if SMOKE else \
          dict(primes=[nextprime(6000), nextprime(9000)], hs=[1, 2, 4], ntarg=300, cbase=1.5, reps=2)
    rows = []; seed = 20261110
    for p in cfg['primes']:
        for h in cfg['hs']:
            for r in range(cfg['reps']):
                seed += 1
                rows.append(row(p, h, seed, cfg['ntarg'], cfg['cbase']))
    agg = {}
    for h in cfg['hs']:
        rs = [x for x in rows if x['h'] == h]
        cf = sum(x['reps_full'] for x in rs); cp = sum(x['reps_proj'] for x in rs); c2 = sum(x['reps_proj_indep'] for x in rs)
        r, rel = ratio_ci(cp, cf); r2, rel2 = ratio_ci(cp, c2)
        agg[h] = dict(ratio_proj_over_full=r, rel_se=rel, predicted=h, aa_ratio=r2, aa_rel_se=rel2,
                      reps=[cf, cp, c2])
    ok = True; inconcl = False
    for h, a in agg.items():
        if min(a['reps']) < 30: inconcl = True; continue
        if abs(a['ratio_proj_over_full'] - h) > 3*a['ratio_proj_over_full']*a['rel_se']: ok = False
        if abs(a['aa_ratio'] - 1) > 3*a['aa_ratio']*a['aa_rel_se']: ok = False  # A/A noise floor
    verdict = 'falsified' if not ok else ('inconclusive' if inconcl else 'consistent_with_prediction')
    if SMOKE and ok: verdict = 'inconclusive'
    print(json.dumps(dict(lead='LEAD-ICB-cofactor-projected-ladder-base', smoke=SMOKE, seed0=20261110, params=cfg,
                          rows=rows, aggregate={str(k): v for k, v in agg.items()}, verdict=verdict)))
main()
