#!/usr/bin/env python3
"""T9 design-time census analysis (exact; reads the parent census only).

For each (k, t) with t in {5, 6, 7}, over the 2,000 seeds of
research/curve25519_isogeny_seed_20261103/data/k*.jsonl:
  J-selected      = argmax J            tie-break (-columns, seed)  [parent rule, unchanged]
  J/cols-selected = argmax J / columns  tie-break (-J, seed)         [T9 rule]
Model cost proxy (end-to-end F4 work per solve, up to a per-relation factor)
  M = columns / covered_points   (relations needed ~ columns; relations per
                                   oracle call ~ covered_points / r)
Ratios of M are reported against seed 0 and between the two selected seeds.
J/columns is compared exactly as a rational (J1*c2 vs J2*c1); no floats decide.
Writes analysis/census_t9.json and prints a table.
"""
import json, sys, os
from fractions import Fraction
HERE = os.path.dirname(os.path.abspath(__file__))
CENSUS = sys.argv[1] if len(sys.argv) > 1 else \
    '/home/user/crypto/research/curve25519_isogeny_seed_20261103/data'
OUT = os.path.join(HERE, '..', 'analysis', 'census_t9.json')
cells = []
for k in (16, 18, 20, 22, 24):
    rows = {}
    for line in open(os.path.join(CENSUS, f'k{k}.jsonl')):
        d = json.loads(line)
        if 'error' in d:
            continue
        rows.setdefault(d['t'], []).append(d)
    for t in (5, 6, 7):
        R = rows[t]
        assert len(R) == 2000, (k, t, len(R))
        sJ = sorted(R, key=lambda x: (-x['J'], -x['columns'], x['seed']))[0]
        sP = sorted(R, key=lambda x: (-Fraction(x['J'], x['columns']), -x['J'], x['seed']))[0]
        s0 = next(x for x in R if x['seed'] == 0)
        oracle = sorted(R, key=lambda x: (Fraction(x['columns'], x['covered_points']), x['seed']))[0]
        M = lambda x: Fraction(x['columns'], x['covered_points'])
        search = sum(x['build_group_ops'] + x['selector_group_ops'] for x in R)
        rank = sum(M(x) < M(sP) for x in R)  # seeds strictly better on the exact proxy
        cells.append(dict(
            k=k, t=t,
            seed0=dict(seed=0, J=s0['J'], cols=s0['columns'], cov=s0['covered_points']),
            J_sel=dict(seed=sJ['seed'], J=sJ['J'], cols=sJ['columns'], cov=sJ['covered_points']),
            Jc_sel=dict(seed=sP['seed'], J=sP['J'], cols=sP['columns'], cov=sP['covered_points']),
            proxy_oracle=dict(seed=oracle['seed'], cols=oracle['columns'], cov=oracle['covered_points']),
            M_Jc_over_seed0=float(M(sP) / M(s0)),
            M_J_over_seed0=float(M(sJ) / M(s0)),
            M_Jc_over_J=float(M(sP) / M(sJ)),
            M_Jc_over_oracle=float(M(sP) / M(oracle)),
            seeds_strictly_better_than_Jc=rank,
            search_group_ops=int(search)))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
json.dump(dict(census_dir=CENSUS, cells=cells), open(OUT, 'w'), indent=1)
print(' k t | seed0 cols/cov | J-sel seed cols/cov | J/c-sel seed J cols/cov | Jc/s0  J/s0  Jc/J  Jc/orc | better | search')
for c in cells:
    a, b, z = c['Jc_sel'], c['J_sel'], c['seed0']
    print(f"{c['k']:2d} {c['t']} | {z['cols']:3d}/{z['cov']:5d} | {b['seed']:4d} {b['cols']:3d}/{b['cov']:5d} | "
          f"{a['seed']:4d} {a['J']:5d} {a['cols']:3d}/{a['cov']:5d} | {c['M_Jc_over_seed0']:.3f} {c['M_J_over_seed0']:.3f} "
          f"{c['M_Jc_over_J']:.3f} {c['M_Jc_over_oracle']:.3f} | {c['seeds_strictly_better_than_Jc']:4d} | {c['search_group_ops']:.3e}")
