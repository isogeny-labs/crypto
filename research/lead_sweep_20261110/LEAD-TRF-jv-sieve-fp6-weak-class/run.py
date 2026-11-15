#!/usr/bin/env python3
"""Toy decomposition-rate check for the JV sieve lead (Fp6 over Fp2, n=3). stdlib only."""
import json, random, sys, time, math

T0 = time.time()
DEADLINE = 480.0

class Ctx:
    def __init__(s, p):
        s.p = p
        s.n = next(a for a in range(2, p) if pow(a, (p - 1) // 2, p) == p - 1)
        s.c = None
        for a in range(p):
            for b in range(1, p):
                if s.f2pow((a, b), (p * p - 1) // 3) != (1, 0):
                    s.c = (a, b); break
            if s.c: break
        s.q6 = p ** 6
        m = s.q6 - 1; s.s = 0
        while m % 2 == 0: m //= 2; s.s += 1
        s.m = m
        s.one = ((1, 0), (0, 0), (0, 0)); s.zero = ((0, 0),) * 3
        rng = random.Random(1)
        while True:
            z = s.rnd(rng)
            if z != s.zero and s.pw(z, (s.q6 - 1) // 2) != s.one: s.z = z; break
    def f2mul(s, u, v):
        p = s.p
        return ((u[0]*v[0] + s.n*u[1]*v[1]) % p, (u[0]*v[1] + u[1]*v[0]) % p)
    def f2pow(s, u, e):
        r = (1, 0)
        while e:
            if e & 1: r = s.f2mul(r, u)
            u = s.f2mul(u, u); e >>= 1
        return r
    def add(s, u, v):
        p = s.p
        return tuple(((a[0]+b[0]) % p, (a[1]+b[1]) % p) for a, b in zip(u, v))
    def sub(s, u, v):
        p = s.p
        return tuple(((a[0]-b[0]) % p, (a[1]-b[1]) % p) for a, b in zip(u, v))
    def neg(s, u):
        p = s.p
        return tuple(((-a[0]) % p, (-a[1]) % p) for a in u)
    def mul(s, u, v):
        m = s.f2mul; p = s.p
        def ad(a, b): return ((a[0]+b[0]) % p, (a[1]+b[1]) % p)
        r0 = ad(m(u[0], v[0]), m(s.c, ad(m(u[1], v[2]), m(u[2], v[1]))))
        r1 = ad(ad(m(u[0], v[1]), m(u[1], v[0])), m(s.c, m(u[2], v[2])))
        r2 = ad(ad(m(u[0], v[2]), m(u[1], v[1])), m(u[2], v[0]))
        return (r0, r1, r2)
    def pw(s, u, e):
        r = s.one
        while e:
            if e & 1: r = s.mul(r, u)
            u = s.mul(u, u); e >>= 1
        return r
    def inv(s, u): return s.pw(u, s.q6 - 2)
    def rnd(s, rng):
        return tuple((rng.randrange(s.p), rng.randrange(s.p)) for _ in range(3))
    def emb(s, a, b=0): return ((a, b), (0, 0), (0, 0))
    def is_sq(s, u): return s.pw(u, (s.q6 - 1) // 2) == s.one
    def sqrt(s, a):  # Tonelli-Shanks, a nonzero square
        M = s.s; c = s.pw(s.z, s.m); t = s.pw(a, s.m)
        R = s.pw(a, (s.m + 1) // 2)
        while t != s.one:
            i = 0; t2 = t
            while t2 != s.one: t2 = s.mul(t2, t2); i += 1
            b = c
            for _ in range(M - i - 1): b = s.mul(b, b)
            M = i; c = s.mul(b, b); t = s.mul(t, c); R = s.mul(R, b)
        return R

def make_curve(ctx, seed):
    rng = random.Random(seed); p = ctx.p
    while True:
        A, B = ctx.rnd(rng), ctx.rnd(rng)
        d = ctx.add(ctx.mul(ctx.emb(4), ctx.mul(A, ctx.mul(A, A))), ctx.mul(ctx.emb(27), ctx.mul(B, B)))
        if d != ctx.zero: return A, B

def fx(ctx, A, B, x):
    return ctx.add(ctx.add(ctx.mul(ctx.mul(x, x), x), ctx.mul(A, x)), B)

def fb_points(ctx, A, B, subfield):  # subfield 'p2' or 'p'
    pts = []
    for a in range(ctx.p):
        for b in (range(ctx.p) if subfield == 'p2' else [0]):
            x = ctx.emb(a, b); f = fx(ctx, A, B, x)
            if f == ctx.zero: continue
            if ctx.is_sq(f):
                y = ctx.sqrt(f); pts.append((x, y)); pts.append((x, ctx.neg(y)))
    return pts

def batch_inv(ctx, ds):
    pre = []; acc = ctx.one
    for d in ds: acc = ctx.mul(acc, d); pre.append(acc)
    ia = ctx.inv(acc); out = [None]*len(ds)
    for i in range(len(ds)-1, -1, -1):
        out[i] = ctx.mul(ia, pre[i-1]) if i else ia
        ia = ctx.mul(ia, ds[i])
    return out

def addpt(ctx, A, P, Q, invd):
    """P+Q affine; invd = inverse of denominator (x2-x1 or 2y1)."""
    (x1, y1), (x2, y2) = P, Q
    if P == Q:
        lam = ctx.mul(ctx.add(ctx.mul(ctx.emb(3), ctx.mul(x1, x1)), A), invd)
    else:
        lam = ctx.mul(ctx.sub(y2, y1), invd)
    x3 = ctx.sub(ctx.sub(ctx.mul(lam, lam), x1), x2)
    y3 = ctx.sub(ctx.mul(lam, ctx.sub(x1, x3)), y1)
    return (x3, y3)

def slow_add(ctx, A, P, Q):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if y1 != y2 or y1 == ctx.zero.__class__ and False: return None
        if ctx.add(y1, y2) == ctx.zero: return None
        d = ctx.add(y1, y1)
    else:
        d = ctx.sub(x2, x1)
    return addpt(ctx, A, P, Q, ctx.inv(d))

def pair_sums(ctx, A, FB):
    S = {}; items = []; dens = []
    for i in range(len(FB)):
        for j in range(i, len(FB)):
            P, Q = FB[i], FB[j]
            if P[0] == Q[0]:
                if P[1] == Q[1]: items.append((P, Q)); dens.append(ctx.add(P[1], P[1]))
                else: S['O'] = 1
            else:
                items.append((P, Q)); dens.append(ctx.sub(Q[0], P[0]))
    inv = batch_inv(ctx, dens) if dens else []
    for (P, Q), iv in zip(items, inv):
        S[addpt(ctx, A, P, Q, iv)] = 1
    return S

def decomposes(ctx, A, FB, S2, R):
    ds = []; idx = []
    for k, P in enumerate(FB):
        if P[0] == R[0]:
            if P[1] == R[1]:
                if 'O' in S2: return True
            else:
                Q = slow_add(ctx, A, R, (P[0], ctx.neg(P[1])))
                if Q is not None and Q in S2: return True
        else:
            ds.append(ctx.sub(P[0], R[0])); idx.append(k)
    inv = batch_inv(ctx, ds)
    for k, iv in zip(idx, inv):
        P = FB[k]; Q = addpt(ctx, A, R, (P[0], ctx.neg(P[1])), iv)
        if Q in S2: return True
    return False

def rand_point(ctx, A, B, rng):
    while True:
        x = ctx.rnd(rng); f = fx(ctx, A, B, x)
        if f != ctx.zero and ctx.is_sq(f):
            y = ctx.sqrt(f)
            return (x, y if rng.random() < .5 else ctx.neg(y))

def run_p(p, trials, null_trials, seed):
    ctx = Ctx(p); A, B = make_curve(ctx, seed)
    rng = random.Random(seed * 7919 + p)
    FB = fb_points(ctx, A, B, 'p2'); S2 = pair_sums(ctx, A, FB)
    FB0 = fb_points(ctx, A, B, 'p'); S20 = pair_sums(ctx, A, FB0)
    hits = []
    for _ in range(trials):
        if time.time() - T0 > DEADLINE: break
        hits.append(1 if decomposes(ctx, A, FB, S2, rand_point(ctx, A, B, rng)) else 0)
    nul = 0; nn = 0
    for _ in range(null_trials):
        if time.time() - T0 > DEADLINE: break
        nn += 1; nul += decomposes(ctx, A, FB0, S20, rand_point(ctx, A, B, rng))
    pl = 0; npl = 20
    for _ in range(npl):
        a, b, c = (FB[rng.randrange(len(FB))] for _ in range(3))
        R = slow_add(ctx, A, slow_add(ctx, A, a, b), c)
        if R is None: pl += 1; continue
        pl += decomposes(ctx, A, FB, S2, R)
    n = len(hits); h = sum(hits); r = h / n if n else float('nan')
    h1, n1 = sum(hits[0::2]), len(hits[0::2]); h2, n2 = sum(hits[1::2]), len(hits[1::2])
    pp = (h1 + h2) / max(1, n1 + n2)
    se = math.sqrt(max(pp*(1-pp), 1e-9) * (1/max(1, n1) + 1/max(1, n2)))
    return dict(p=p, q=p*p, group_log2=round(6*math.log2(p), 2), fb_size=len(FB), trials=n, rate=r,
                sigma0=math.sqrt((1/6)*(5/6)/max(1, n)), aa_z=((h1/max(1, n1)) - (h2/max(1, n2)))/se,
                null_fp_fb_size=len(FB0), null_rate=nul/max(1, nn), null_trials=nn, planted_found=pl, planted_total=npl)

def main():
    smoke = '--smoke' in sys.argv
    ps = [5] if smoke else [7, 11, 13]
    trials = 40 if smoke else 2000
    nt = 20 if smoke else 400
    rows = [run_p(p, trials, nt, seed=20261110) for p in ps]
    TARGET = 1/6; verdict = 'consistent_with_prediction'
    for r in rows:
        dev = abs(r['rate'] - TARGET) if r['rate'] == r['rate'] else 9
        if r['trials'] < trials or r['planted_found'] != r['planted_total'] or abs(r['aa_z']) > 3.5 or r['null_rate'] > 0.02:
            verdict = 'inconclusive'; break
        if dev > 3*r['sigma0'] + 0.06 and not smoke: verdict = 'falsified'; break
        if dev > 3*r['sigma0'] + 0.02 and verdict != 'falsified': verdict = 'inconclusive'
    if smoke and verdict == 'falsified': verdict = 'inconclusive'
    print(json.dumps(dict(lead='LEAD-TRF-jv-sieve-fp6-weak-class', smoke=smoke, statistic='rate of R=P1+P2+P3 with x(Pi) in Fp2, E/Fp6',
                          prediction=TARGET, seed=20261110, params=dict(ps=ps, trials=trials, null_trials=nt),
                          rows=rows, verdict=verdict, elapsed_s=round(time.time()-T0, 2))))
main()
