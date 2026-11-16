#!/usr/bin/env python3
"""Compare the fresh pod rho runs with the pooled reference of research/jv_cover_negrho_rescore_20261101.
Usage: rho_compare.py <pooled results dir> <fresh rho dir> <out.json>
Keyed by (p, seed, run). Every field except wall_ms must be equal (JSON value equality)."""
import json, sys, glob, os
pd, fd, out = sys.argv[1:4]
def load(files):
    m = {}
    for f in files:
        for r in json.load(open(f)): m[(r['p'], r['seed'], r['run'])] = (r, os.path.basename(f))
    return m
pooled = load(glob.glob(os.path.join(pd, 'inputs', 'old_rhostrong', 'rhostrong_p*.json')) + glob.glob(os.path.join(pd, 'new', 'rhostrong_p*.json')))
fresh = load(glob.glob(os.path.join(fd, 'rhostrong_p*.json')))
res = []
for k in sorted(fresh):
    a = pooled.get(k); b = fresh[k][0]
    if a is None: res.append(dict(key=k, in_pooled=False)); continue
    a = a[0]; keys = (set(a) | set(b)) - {'wall_ms'}
    diff = sorted(x for x in keys if a.get(x) != b.get(x))
    res.append(dict(key=k, in_pooled=True, identical=not diff, differing=diff, correct=b['correct'], capped=b['capped'], steps=b['steps']))
n = len(res); ni = sum(1 for r in res if r.get('identical'))
summary = dict(fresh_runs=n, pooled_runs=len(pooled), identical_except_wall_ms=ni, all_correct=all(r.get('correct') for r in res if 'correct' in r),
               none_capped=not any(r.get('capped') for r in res), missing_from_fresh=sorted(set(pooled) - set(fresh)))
json.dump(dict(summary=summary, runs=res), open(out, 'w'), indent=1, default=list)
print(json.dumps(summary, default=list))
for r in res:
    if not r.get('identical'): print('DIFF', r)
