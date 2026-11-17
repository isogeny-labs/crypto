#!/usr/bin/env python3
"""J2 pricing band (AGENTS.md 5a): cost_w = M-eq + w * (hash + cmp) * GAE, GAE = gae_mul + r_inv * gae_inv (M-eq),
w in {0, 0.1, 1}; paired ratio to c = 2.00 per (q, seed); reports whether c=1.60 < c=1.75 < c=2.00 holds at every w.
usage: j2_band.py <units dir> [<units dir> ...]"""
import json, glob, os, sys, statistics as st
PH = ('fb', 'relations', 'matrix', 'linear_algebra', 'log_recovery', 'descent', 'verify')
def recs(f): return [json.loads(l) for l in open(f) if l.startswith('{')]
calib, gae = {}, {}
units = {}
for U in sys.argv[1:]:
    for f in glob.glob(f'{U}/calib_*_*.jsonl'):
        q = int(os.path.basename(f).split('_')[1]); calib.setdefault(q, []).extend(recs(f))
    for f in glob.glob(f'{U}/ic_*_c*.jsonl'):
        q, sd, c = os.path.basename(f)[3:-6].split('_'); units[(int(q), int(sd), float(c[1:]))] = recs(f)
cal = {q: (st.median(r['r_inv'] for r in v), st.median(r['r_lmul'] for r in v)) for q, v in calib.items()}
def cost(rs, q, w):
    ri, rl = cal[q]
    rat = [r for r in rs if r.get('rec') == 'ratios']
    g = (rat[0]['gae_mul'] + ri * rat[0]['gae_inv']) if rat else 56 + ri
    tot = 0.0
    for r in rs:
        if r.get('arm') != 'DL1': continue
        tot += sum(r.get(f'{p}.mul', 0) + ri * r.get(f'{p}.inv', 0) + rl * r.get(f'{p}.lmul', 0) + w * g * (r.get(f'{p}.hash', 0) + r.get(f'{p}.cmp', 0)) for p in PH)
    return tot
out = {}
for q in sorted({k[0] for k in units}):
    for sd in sorted({k[1] for k in units if k[0] == q}):
        if (q, sd, 2.0) not in units: continue
        for w in (0, 0.1, 1):
            base = cost(units[(q, sd, 2.0)], q, w)
            row = {c: cost(units[(q, sd, c)], q, w) / base for c in (1.6, 1.75) if (q, sd, c) in units}
            out.setdefault((q, w), []).append(row)
allorder = True
for (q, w), rows in sorted(out.items()):
    r16 = [r[1.6] for r in rows if 1.6 in r]; r175 = [r[1.75] for r in rows if 1.75 in r]
    order = all(r.get(1.6, 0) < r.get(1.75, 9) < 1.0 for r in rows if 1.6 in r and 1.75 in r)
    allorder &= order
    print(f"q={q:8} w={w:<4} n={len(rows)}  c1.60/c2.00: {min(r16):.3f}-{max(r16):.3f}  c1.75/c2.00: {min(r175):.3f}-{max(r175):.3f}  order 1.60<1.75<2.00 per seed: {order}")
print('order holds at every (q, w):', allorder)
