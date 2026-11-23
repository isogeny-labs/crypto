#!/usr/bin/env python3
"""Tables for RESULT.md from grid.jsonl, dual.jsonl, high_points.jsonl, coset.jsonl (run in this dir)."""
import json
from math import comb
def M(k, e): return sum(comb(k, d) for d in range(0, max(e, -1) + 1))
EMIN = {15: 3, 17: 3, 19: 4, 21: 4, 23: 4, 25: 4, 27: 5}
hp = {(d['n'], d['seed']): d['C_from_X1X2_eq_z'] for d in map(json.loads, open('high_points.jsonl'))}
dual = {}
for l in open('dual.jsonl'):
    d = json.loads(l); dual[(d['r']['n'], d['seed'], d['e'])] = d['r']
print('| n | k | seed | e | e_min | kernel dim | M(k-1,e-1) = rank T | T in K | excess | P | h | cols - E_eff | dim S_{k-e-1} (dual) | identity |')
print('|--:|--:|--:|--:|--:|--:|--:|:-:|--:|:-:|--:|--:|--:|:-:|')
for l in open('grid.jsonl'):
    d = json.loads(l); r = d['r']; n, k, e, s = r['n'], r['k'], r['e'], d['seed']
    em = EMIN[n]; x = r['excess']
    ok = 'P1 ok' if (e < em and x == 0) else 'P2 ok' if (e == em and x > 0) else ('P1 FAIL' if e < em else 'P2 FAIL')
    assert r['kernel_check_ok'] and r['trivial_in_kernel'] and r['trivial_rank'] == M(k - 1, e - 1) and r['F1F2_violations'] == 0
    eeff = (k + 1) * M(k, e + 1) - M(k, e + 2) + 1
    h = hp.get((n, s), '-')
    dd = dual.get((n, s, e)); ds = dd['dim_S'] if dd else '-'
    ident = '-'
    if dd:
        ident = 'yes' if r['kernel_dim'] == n * M(k, e) - (k + 1) * 2 ** k + dd['ann0'] + dd['dim_S'] else 'NO'
    print(f"| {n} | {k} | {s} | {e} | {em} | {r['kernel_dim']} | {r['trivial_rank']} | {'yes' if r['trivial_in_kernel'] else 'no'} | {x} | {ok} | {h} | {r['cols'] - eeff} | {ds} | {ident} |")
print()
print('| n | seed | e | degree filtration [d, dim K_<=d, dim T_<=d] (last two) | rank on H1 | dim vanishing on H1 | universal | Frobenius-closed | x-closed | fibre rank = dim A_C at every point |')
print('|--:|--:|--:|:--|--:|--:|--:|--:|--:|:-:|')
for l in open('grid.jsonl'):
    d = json.loads(l); r = d['r']
    full = all(hh['fibre_rank'] == hh['dimA'] for hh in r['fibre_hist'])
    h1zero = all(hh['fibre_rank'] == 0 for hh in r['fibre_hist'] if hh['ell0'] == 1)
    fr = 'yes' if full else ('no (0 on all of H1)' if h1zero else 'no')
    print(f"| {r['n']} | {d['seed']} | {r['e']} | {r['filtration_d_dimK_dimT'][-2:]} | {r['rank_on_H1']} | {r['dim_vanish_on_H1']} | {r['dim_universal']} | {r['dim_frobenius_closed']} | {r['dim_xmult_closed']} | {fr} |")
