#!/usr/bin/env python3
"""Restriction of the PKM chain: blocks 1..m-1 fixed to points of V (x_i = g*zeta^{k_i}); block 0 free
(t tower levels), aux u_0..u_{m-3} free. Inconsistency checked by enumeration over V x signs."""
import json, random, argparse, itertools, sys
sys.path.insert(0, sys.path[0]); import gen
ap = argparse.ArgumentParser(); ap.add_argument('--p', type=int, default=40961); ap.add_argument('--m', type=int, required=True)
ap.add_argument('--t', type=int, required=True); ap.add_argument('--seed', type=int, default=1); ap.add_argument('-o', required=True); ap.add_argument('--free', type=int, default=1)
A = ap.parse_args(); p, m, t = A.p, A.m, A.t
rng = random.Random(f"restr-{p}-{m}-{A.seed}")   # curve, g, fixed points independent of t where possible
while True:
    a, b = rng.randrange(1, p), rng.randrange(1, p)
    if (4*a**3+27*b*b) % p and gen.is_prime(gen.curve_order(a, b, p)): break
g = rng.randrange(2, p)
zeta = next(z for z in (pow(h, (p-1) >> t, p) for h in range(2, p)) if pow(z, 1 << (t-1), p) != 1)
V = [g*pow(zeta, k, p) % p for k in range(1 << t)]
F = gen.F2(p)
rng2 = random.Random(f"restr2-{p}-{m}-{t}-{A.seed}")
r = A.free
fixed = [rng2.choice(V) for _ in range(m-r)]
while True:
    xR = rng2.randrange(1, p)
    if gen.legendre(xR**3+a*xR+b, p) != 1: continue
    # consistent iff some x0 in V with x(P0 ± P1 ± ... ) = xR
    pts_f = [gen.lift(F, x, a, b) for x in fixed]
    hit = False
    for xs0 in itertools.combinations_with_replacement(V, r):
        P0 = gen.lift(F, xs0[0], a, b)
        allp = [gen.lift(F, x, a, b) for x in xs0[1:]] + pts_f
        for signs in itertools.product((1, -1), repeat=m-1):
            S = P0
            for P, sg in zip(allp, signs):
                if sg < 0: P = (P[0], ((-P[1][0]) % p, (-P[1][1]) % p))
                S = gen.ec_add(F, S, P, a)
            if S is not None and S[0] == (xR, 0): hit = True; break
        if hit: break
    if not hit: break
R = gen.Ring(p, r, t); R.k = m-2
def cst(c): return {((0,)*r, (0,)*(m-2)): c % p}
def uu(j):
    e = [0]*(m-2); e[j] = 1; return {((0,)*r, tuple(e)): 1}
def xv(i):
    bb = [0]*r; bb[i] = 1; return {(tuple(bb), (0,)*(m-2)): g % p}
xs = [xv(i) for i in range(r)] + [cst(c) for c in fixed]
R.const = lambda c: cst(c) if c % p else {}
if m == 2: polys = [gen.S3(R, xs[0], xs[1], cst(xR), a, b)]
else:
    us = [uu(j) for j in range(m-2)]
    polys = [gen.S3(R, xs[0], xs[1], us[0], a, b)]
    for k in range(1, m-2): polys.append(gen.S3(R, us[k-1], xs[k+1], us[k], a, b))
    polys.append(gen.S3(R, us[m-3], xs[m-1], cst(xR), a, b))
out = dict(p=p, m=r, k=m-2, t=t, orig_m=m, a=a, b=b, g=g, xR=xR, fixed=fixed,
           polys=[[[list(kk[0])+list(kk[1]), v] for kk, v in sorted(P.items()) if v] for P in polys])
json.dump(out, open(A.o, 'w')); print(json.dumps({k: out[k] for k in ['p','orig_m','t','xR']}))
