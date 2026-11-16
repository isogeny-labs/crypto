#!/usr/bin/env python3
"""POST-HOC sensitivity (not frozen, written after the data; README section 5).

The frozen w-band prices each arm's `cost.unpriced` counters. rho-neg lists its
per-step negation canonicalisation there; the walk-table arms log their per-step
hash-table inserts as `work_native.table_insert_attempts` with a `counted:` marker,
so the band never charges them. Here every walk-table arm is also charged
w * table_insert_attempts, and the same pooled estimator as pooled_compare.py is
applied -> data/sensitivity_inserts.json.
"""
import glob, json, math, os, statistics, sys
from collections import defaultdict
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import load, unpriced, t975

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')


def inserts(x):
    return sum(p.get('work_native', {}).get('table_insert_attempts', 0) for p in x['phases'])


def pooled_sym(A, B, w, recs, sq):
    def S(x):
        return (x['cost']['total_gae'] + w * (unpriced(x) + inserts(x))) / sq
    ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == A}
    ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == B}
    cl = defaultdict(lambda: [0.0, 0.0])
    for k in sorted(set(ia) & set(ib)):
        cl[k[0]][0] += S(ia[k]); cl[k[0]][1] += S(ib[k])
    Ak = [v[0] for v in cl.values()]; Bk = [v[1] for v in cl.values()]
    R = sum(Bk) / sum(Ak); K = len(Ak); mA = statistics.fmean(Ak)
    z = [(bk - R * ak) / mA for ak, bk in zip(Ak, Bk)]
    se = statistics.stdev(z) / math.sqrt(K); t = t975(K - 1)
    return dict(ratio=R, ci95=[R * math.exp(-t * se / R), R * math.exp(t * se / R)], workloads=K)


out = {}
for k in (20, 24):
    dirs = sorted(glob.glob(os.path.join(HERE, f'sessions/k{k}-s*')))
    recs = load(dirs)
    sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}.pop()
    out[f'k{k}'] = {f'{b}/{a}': {str(w): pooled_sym(a, b, w, recs, sq) for w in (0, 0.1, 1)}
                    for a, b in [('rho-neg', 'twcj-x15'), ('rho-neg', 'tw-x15'), ('rho-neg', 'f1-x07'),
                                 ('f1-x07', 'twcj-x15')]}
json.dump(out, open(os.path.join(HERE, 'data/sensitivity_inserts.json'), 'w'), indent=1)
for k, d in out.items():
    for p, r in d.items():
        print(k, p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
