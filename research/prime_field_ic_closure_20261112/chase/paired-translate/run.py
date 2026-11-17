#!/usr/bin/env python3
"""Driver: curves x targets x arms; writes results.json. [pcc:unverified]"""
import sys, json, math, random, time
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from toy import *

bits = int(sys.argv[1]) if len(sys.argv) > 1 else 18
ncurves = int(sys.argv[2]) if len(sys.argv) > 2 else 2
ntargets = int(sys.argv[3]) if len(sys.argv) > 3 else 6
N = int(sys.argv[4]) if len(sys.argv) > 4 else 0
theta = int(sys.argv[5]) if len(sys.argv) > 5 else 3
out = sys.argv[6] if len(sys.argv) > 6 else f'results-{bits}.json'

ARMS = [
    ('ic-single-L1', 'ic', dict(paired=False, L=1)),
    ('ic-paired-L1', 'ic', dict(paired=True, L=1)),
    ('ic-single-L16', 'ic', dict(paired=False, L=16)),
    ('ic-paired-L16', 'ic', dict(paired=True, L=16)),
    ('rho-neg-L1', 'rho', dict(L=1, side=False)),
    ('rho-neg-L16', 'rho', dict(L=16, side=False)),
    ('rho-neg-side-L16', 'rho', dict(L=16, side=True)),
    ('rho-neg-side-L1', 'rho', dict(L=1, side=True)),
    ('store-all-L1', 'rhoall', dict(L=1, side=False)),
    ('store-all-side-L1', 'rhoall', dict(L=1, side=True)),
]
recs = []
t0 = time.time()
for c in range(ncurves):
    p, a, b, n = find_curve(bits, 1000 + 17*c + bits)
    E = Curve(p, a, b, n)
    # generator: any non-identity point (n prime)
    x = 1
    while True:
        y = sqrt_mod(x**3 + a*x + b, p)
        if y: G = (x, y); break
        x += 1
    NB = N or max(8, round(1.0 * n ** (1/3)))
    print(f'curve {c}: p={p} n={n} N={NB}', flush=True)
    for tg in range(ntargets):
        rng = random.Random(77*c + tg + 1000*bits)
        d = rng.randrange(1, n)
        Q = E.mul_u(d, G)
        for name, kind, kw in ARMS:
            seed = hash((c, tg, name)) & 0xffffffff
            if kind == 'ic':
                tt, ok = ic_run(E, G, Q, d, NB, kw['paired'], kw['L'], seed)
            elif kind == 'rho':
                tt, ok = rho_run(E, G, Q, d, kw['L'], theta, kw['side'], seed)
            else:
                tt, ok = rho_run(E, G, Q, d, kw['L'], 0, kw['side'], seed, store_all=True)
            recs.append(dict(curve=c, p=p, n=n, N=NB, target=tg, arm=name, ok=ok, **tt.c))
    print(f'  done {time.time()-t0:.1f}s', flush=True)
json.dump(dict(bits=bits, theta_bits=theta, records=recs), open(out, 'w'))
print('wrote', out)
