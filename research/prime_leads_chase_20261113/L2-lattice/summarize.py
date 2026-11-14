#!/usr/bin/env python3
import json, collections
from fractions import Fraction as Fr
rows = [json.loads(l) for l in open('grid.jsonl')]
ref = {'NEG_S3x': lambda d: 4*d, 'NEG_S3e': lambda d: 4*d, 'A1': lambda d: 4*d, 'A2': lambda d: 4*d,
       'A3': lambda d: 4*d, 'B': lambda d: 4*d, 'A4': lambda d: 12*d, 'A5': lambda d: 32*d,
       'F': lambda d: 12*d, 'POS': lambda d: 2*d, 'RAND_A2': None, 'RAND_A4': None}
g = collections.defaultdict(list)
cens = []
for r in rows:
    if r.get('censored'): cens.append(r); continue
    g[(r['system'], r['delta'], r.get('gamma','1'))].append(r)
print("| system | δ | γ(u) | ν_s/s by (s,T) | min ν_s/s | reference min(1,h_S) | verdict cell |")
print("|---|---|---|---|---:|---:|---|")
flags = []
for (sysn, d, gam), rs in sorted(g.items(), key=lambda k: (k[0][0], float(Fr(k[0][1])), k[0][2])):
    df = float(Fr(d))
    cells = " ".join(f"({r['s']},{r['T']}):{r['nu_over_s']:.3f}" if r['nu_over_s'] is not None else f"({r['s']},{r['T']}):none" for r in sorted(rs, key=lambda r:(r['s'],r['T'])))
    vals = [r['nu_over_s'] for r in rs if r['nu_over_s'] is not None]
    mn = min(vals) if vals else None
    f = ref[sysn]
    refv = None if f is None else (f(df) if sysn == 'POS' else min(1.0, f(df)))
    if sysn == 'POS':
        cell = 'fires' if mn is not None and mn < 1 - 1e-9 and abs(mn - refv) < 1e-9 else 'FAILS'
    elif refv is None:
        cell = 'n/a (random support)'
    else:
        below = mn is not None and mn < refv - 0.02
        cell = 'BELOW reference (H1 cell)' if below else ('matches' if abs(mn-refv) < 0.01 else 'above (truncation-limited)')
        if below: flags.append((sysn, d, gam, mn, refv))
    print(f"| {sysn} | {d} | {gam} | {cells} | {mn if mn is None else round(mn,3)} | {refv if refv is None else round(refv,3)} | {cell} |")
print("\ncensored:", len(cens), [ (c['system'],c['delta'],c['s'],c['T']) for c in cens])
print("H1 cells:", flags)
