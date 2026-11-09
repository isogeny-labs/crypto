#!/usr/bin/env python3
"""Pooled single-target analysis (PROTOCOL.md sections 3 and 5) -> data/pooled_measured.json."""
import json, os, sys, statistics, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import pooled, load, unpriced

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
ARMS = ['rho-neg', 'rho-aa', 'f2-inc', 'f1-x05', 'f1-x07', 'f1-x09']
PAIRS = [('f2-inc', 'f1-x07'), ('f2-inc', 'f1-x05'), ('f2-inc', 'f1-x09'),
         ('rho-neg', 'f1-x07'), ('rho-neg', 'f1-x05'), ('rho-neg', 'f1-x09'),
         ('rho-neg', 'f2-inc'), ('rho-neg', 'rho-aa')]
out = {}
for k in (20, 24):
    dirs = sorted(glob.glob(os.path.join(HERE, f'sessions/k{k}-s*')))
    if not dirs:
        continue
    recs = load(dirs)
    sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}.pop()
    means = {}
    for w in (0, 0.1, 1):
        for a in ARMS:
            v = [(x['cost']['total_gae'] + w * unpriced(x)) / sq for x in recs if x['arm'] == a]
            means.setdefault(a, {})[str(w)] = dict(mean=statistics.fmean(v), n=len(v))
    ratios = {f'{b}/{a}': {str(w): pooled(a, b, w, dirs) for w in (0, 0.1, 1)} for a, b in PAIRS}
    # F=1 diagnostics: per-run walk steps / probes where reported
    out[f'k{k}'] = dict(sessions=len(dirs), records=len(recs), S=means, ratios=ratios)
json.dump(out, open(os.path.join(HERE, 'data/pooled_measured.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, 'sessions', d['sessions'], 'records', d['records'])
    for a in ARMS:
        print('  S', a, ' '.join(f"{d['S'][a][w]['mean']:.3f}" for w in ('0', '0.1', '1')))
    for p, r in d['ratios'].items():
        print('  R', p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
