#!/usr/bin/env python3
"""T11 frame selection from existing data only (no new measurement).

Reads the frozen k = 20 frame census of research/curve25519_frame_search_20261103
(confirm/k20.json, 2,000 translation frames per t, J = log-free coset-cancelling
signed pairs) and picks, at t = 5:
  - lo = 0 (the incumbent `bits` frame),
  - argmax J (tie-break: higher cols, then lower L; the parent's E6 rule),
  - argmax J / cols (tie-break: higher J, then lower L; the T9 rule on frames).
It then lists, on the ecbench Weierstrass model of the k = 20 miniature
(y^2 = x^3 + 63700 x + 872826 over F_1048573), the abscissae in [lo, lo + 32)
that carry a point, and checks the count equals the census `cols`.
Also derives the coverage-per-column model M = cols / covered (x r x mpt).
Writes selection/frames.json and prints a table.
"""
import json, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..'))
CENSUS = sys.argv[1] if len(sys.argv) > 1 else \
    '/home/user/crypto/research/curve25519_frame_search_20261103/confirm/k20.json'
ENGINE = '/home/user/crypto/research/curve25519_frame_search_20261103/engine'
p, a, b, r = 1048573, 63700, 872826, 131203
T = 5

def carriers(lo, t):
    out = []
    for k in range(1 << t):
        x = (lo + k) % p
        v = (x * x * x + a * x + b) % p
        if v == 0 or pow(v, (p - 1) // 2, p) == 1:
            out.append(x)
    return out

d = json.load(open(CENSUS))
assert d['p'] == p and d['r'] == r and d['A'] == 926
R = [x for x in d['rows'] if x['t'] == T]
lo0 = [x for x in R if x['family'] == 'bits_lo0'][0]
tr = [x for x in R if x['family'] == 'translate']
assert len(tr) == 2000
selJ = sorted(tr, key=lambda x: (-x['J'], -x['cols'], x['frame'][1]))[0]
selJc = sorted(tr, key=lambda x: (-x['J'] / x['cols'], -x['J'], x['frame'][1]))[0]
best_cov = max(x['covered'] for x in tr)
best_pc = max(x['covered'] / x['cols'] for x in tr)
mpt = {}
for f, lo in (('bits_t5_g3', 0), ('frame_t5_g3', 976503)):
    e = json.load(open(os.path.join(ENGINE, f + '.json')))
    mpt[lo] = e['muladds_per_target']
mpt_common = mpt[0]  # per-target F4 work is frame independent (ratio 1.0000014)
rows = {}
for name, x in (('lo0', lo0), ('J', selJ), ('Jcols', selJc)):
    lo = x['frame'][1]
    xs = carriers(lo, T)
    assert len(xs) == x['cols'], (name, lo, len(xs), x['cols'])
    rows[name] = dict(lo=lo, cols=x['cols'], J=x['J'], covered=x['covered'],
                      covered_per_col=x['covered'] / x['cols'],
                      trials_model=x['cols'] * r / x['covered'],
                      E_model_muladds=x['cols'] * r / x['covered'] * mpt_common,
                      abscissae=xs)
base = rows['lo0']['trials_model']
for k in rows:
    rows[k]['M_over_lo0'] = rows[k]['trials_model'] / base
out = dict(census=CENSUS, t=T, guess=3, mpt_t5_g3=mpt, census_best_covered=best_cov,
           census_best_covered_per_col=best_pc, frames=rows,
           M_Jcols_over_J=rows['Jcols']['trials_model'] / rows['J']['trials_model'])
os.makedirs(os.path.join(ROOT, 'selection'), exist_ok=True)
json.dump(out, open(os.path.join(ROOT, 'selection', 'frames.json'), 'w'), indent=1)
print(f"t={T}  best covered {best_cov}  best covered/col {best_pc:.3f}")
for k, v in rows.items():
    print(f"{k:6s} lo={v['lo']:<8d} cols={v['cols']:3d} J={v['J']:4d} covered={v['covered']:4d} "
          f"cov/col={v['covered_per_col']:.3f}  M/lo0={v['M_over_lo0']:.4f}  E={v['E_model_muladds']:.4e}")
print(f"M(Jcols)/M(J) = {out['M_Jcols_over_J']:.4f}")
