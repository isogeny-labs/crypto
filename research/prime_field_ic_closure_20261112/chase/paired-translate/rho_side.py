#!/usr/bin/env python3
"""Does the sibling (second x-only point from the shared inversion) help DP rho?
Arms: rho-neg with L lanes at DP density 2^-t, side off / on; store-all (t=0) off / on.
Reports parent steps / sqrt(n) and field ops. [pcc:unverified]"""
import sys, json, math, random, statistics as st, time
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from toy import *

bits = int(sys.argv[1]); ncurves = int(sys.argv[2]); ntargets = int(sys.argv[3]); L = int(sys.argv[4])
thetas = [int(x) for x in sys.argv[5].split(',')]
out = sys.argv[6]
recs = []; t0 = time.time()
for c in range(ncurves):
    p, a, b, n = find_curve(bits, 5000 + 31*c + bits)
    E = Curve(p, a, b, n)
    x = 1
    while True:
        y = sqrt_mod(x**3 + a*x + b, p)
        if y: G = (x, y); break
        x += 1
    print(f'curve {c}: p={p} n={n} ({time.time()-t0:.0f}s)', flush=True)
    for tg in range(ntargets):
        rng = random.Random(991*c + tg + 7*bits); d = rng.randrange(1, n); Q = E.mul_u(d, G)
        for th in thetas:
            for side in (False, True):
                seed = hash((c, tg, th, side, 'rs')) & 0xffffffff
                if th == 0:
                    tt, ok = rho_run(E, G, Q, d, 1, 0, side, seed, store_all=True)
                else:
                    tt, ok = rho_run(E, G, Q, d, L, th, side, seed)
                recs.append(dict(curve=c, n=n, target=tg, theta=th, side=side, ok=ok, **tt.c))
json.dump(dict(bits=bits, L=L, records=recs), open(out, 'w'))
print('n records', len(recs), 'all ok', all(r['ok'] for r in recs))
for th in thetas:
    for side in (False, True):
        rs = [r for r in recs if r['theta'] == th and r['side'] == side]
        s = [r['steps'] / math.sqrt(r['n']) for r in rs]
        se = st.stdev(s) / math.sqrt(len(s))
        print(f'theta=2^-{th} {"store-all" if th == 0 else f"L={L}"} side={side!s:5}: parent steps/sqrt(n) mean {st.mean(s):.3f} (SE {se:.3f}), '
              f'restarts/run {st.mean(r["restart"] for r in rs):.1f}, cyc2/run {st.mean(r["cyc2"] for r in rs):.1f}, '
              f'per step M {sum(r["M"]-r["pre_M"] for r in rs)/sum(r["steps"] for r in rs):.3f} '
              f'S {sum(r["S"]-r["pre_S"] for r in rs)/sum(r["steps"] for r in rs):.3f}')
    a = [r['steps'] for r in recs if r['theta'] == th and not r['side']]
    b_ = [r['steps'] for r in recs if r['theta'] == th and r['side']]
    print(f'   side/no-side ratio of mean parent steps at theta=2^-{th}: {st.mean(b_)/st.mean(a):.3f}')
