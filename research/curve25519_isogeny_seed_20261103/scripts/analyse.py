#!/usr/bin/env python3
"""Decide R1-R3 of PROTOCOL.md (frozen 27482d63a0) from data/k*.jsonl."""
import json, glob, collections, numpy as np
rows = collections.defaultdict(list); errs = 0
for f in glob.glob('data/k*.jsonl'):
    for line in open(f):
        d = json.loads(line)
        if 'error' in d: errs += 1; continue
        rows[(d['k'], d['t'])].append(d)
cells = []; R1 = R2 = True; R1f = []; R2f = []
for (k, t) in sorted(rows):
    R = rows[(k, t)]
    sel = sorted(R, key=lambda x: (-x['J'], -x['columns'], x['seed']))[0]
    best = max(R, key=lambda x: x['covered_points'])
    s0 = next(x for x in R if x['seed'] == 0)
    mean = np.mean([x['covered_points'] for x in R])
    g_mean = sel['covered_points'] / mean
    if t >= 5 and g_mean < 1.5: R1 = False; R1f.append((k, t, g_mean))
    q = sel['covered_points'] / best['covered_points']
    if q < 0.9: R2 = False; R2f.append((k, t, q))
    search = sum(x['build_group_ops'] + x['selector_group_ops'] for x in R)
    cells.append(dict(k=k, t=t, n_seeds=len(R), seed0_cov=s0['covered_points'], seed0_cols=s0['columns'],
                      mean_cov=float(mean), sel_seed=sel['seed'], sel_cov=sel['covered_points'], sel_cols=sel['columns'],
                      oracle_cov=best['covered_points'], gain_vs_mean=g_mean, gain_vs_seed0=sel['covered_points'] / s0['covered_points'],
                      sel_over_oracle=q, seed0_pct=float(np.mean([x['covered_points'] < s0['covered_points'] for x in R])),
                      search_group_ops=int(search)))
c20 = next(c for c in cells if c['k'] == 20 and c['t'] == 6)
out = dict(R1=R1, R1_fail=R1f, R2=R2, R2_fail=R2f, R3=c20['gain_vs_seed0'] >= 1.5, R3_value=c20['gain_vs_seed0'], build_errors=errs, cells=cells)
json.dump(out, open('analysis/summary.json', 'w'), indent=1)
print({k: v for k, v in out.items() if k != 'cells'})
print(' k t | seed0 (pct) | mean   | sel seed  cov  cols | oracle | /mean /seed0 | search GAE')
for c in cells:
    print(f"{c['k']:2d} {c['t']} | {c['seed0_cov']:5d} ({c['seed0_pct']:.2f}) | {c['mean_cov']:6.1f} | {c['sel_seed']:5d} {c['sel_cov']:5d} {c['sel_cols']:3d} | {c['oracle_cov']:5d} | {c['gain_vs_mean']:4.2f} {c['gain_vs_seed0']:5.2f} | {c['search_group_ops']:.2e}")
