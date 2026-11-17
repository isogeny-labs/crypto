"""Part A view X: non-affine rank of the residual under XOR cubes on e1 vs literal cubes on x1."""
import json, random, sys, time
import numpy as np
from pdplib import *

CELLS = [(3, 11), (4, 13), (5, 17), (6, 19), (7, 23)]
rows = []
t0 = time.time()
for l, n in CELLS:
    F = Field(n)
    ninst = 8 if l <= 6 else 4
    maxc = 16 if l <= 6 else 8
    for i in range(ninst):
        seed = 1000 * l + i
        I = make_instance(F, l, seed, planted=(i < 4))
        full, _ = eval_table(F, I['basis'], I['xR'])
        nsol = int((full == 0).sum())
        for k in range(0, l + 1):
            for kind in ('xor', 'lit'):
                if k == 0 and kind == 'lit':
                    continue
                rng = random.Random(seed * 100 + k)
                cubes = list(range(1 << k)) if (1 << k) <= maxc else rng.sample(range(1 << k), maxc)
                for c in cubes:
                    vals, _ = eval_table(F, I['basis'], I['xR'], k, kind, c)
                    st = anf_stats(moebius(vals))
                    st.update(l=l, n=n, seed=seed, planted=i < 4, truth_nsol=nsol, k=k,
                              kind=('none' if k == 0 else kind), cube=c, cube_nsol=int((vals == 0).sum()))
                    rows.append(st)
    print('cell', l, n, 'done', round(time.time() - t0, 1), flush=True)
json.dump(rows, open(sys.argv[1], 'w'))
# summary
import collections
agg = collections.defaultdict(list)
for r in rows:
    agg[(r['l'], r['n'], r['kind'], r['k'])].append(r)
print('l n kind k | nvars | rank_nonlin min/max | rank_deg3 min/max | nonlin monos mean | maxdeg')
for key in sorted(agg):
    g = agg[key]
    print(*key, '|', g[0]['nvars'], '|', min(x['rank_nonlin'] for x in g), max(x['rank_nonlin'] for x in g), '|',
          min(x['rank_deg3'] for x in g), max(x['rank_deg3'] for x in g), '|',
          round(sum(x['n_nonlin_monos'] for x in g) / len(g), 1), '|', max(x['max_deg'] for x in g))
