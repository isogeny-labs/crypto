#!/usr/bin/env python3
"""P2 arm F (partial degree-2 base, fraction phi) paired with P1's c, D, E, R
on the same (g, p, curve, seed).  Control: E rerun by the P2 binary must equal
P1's E field for field.  Units as P1/analyze.py.  [pcc:unverified]"""
import glob, json, math, os, sys
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
FIELDS = ['trials', 'cantor_mul', 'cantor_inv', 'rel_mul', 'la_mul', 'relations', 'certified']
def runs(paths):
    out = {}
    for f in paths:
        for l in open(f):
            try: d = json.loads(l)
            except Exception: continue
            if d.get('kind') == 'run':
                out[(d['g'], d['p'], d['curve_seed'], d['seed'], d['arm'])] = d
    return out
def fpm(d, k=1):
    return d['cantor_mul'] + 10 * d['cantor_inv'] + d.get('rel_mul', 0) + k * d.get('la_mul', 0)
p1 = runs(glob.glob(os.path.join(HERE, '..', 'P1', 'main', '*.jsonl')) + glob.glob(os.path.join(HERE, '..', 'P1', 'supp', '*.jsonl')))
rows = defaultdict(list); bad_ctrl = 0; n_ctrl = 0
for d in sorted(glob.glob(os.path.join(HERE, 'pilot_phi*'))) + sorted(glob.glob(os.path.join(HERE, 'main_phi*'))):
    phi = d.split('phi')[-1]
    p2 = runs(glob.glob(os.path.join(d, '*.jsonl')))
    for (g, p, cs, sd, arm), r in p2.items():
        if arm.startswith('E'):
            ref = p1.get((g, p, cs, sd, arm))
            if ref:
                n_ctrl += 1
                if any(ref[k] != r[k] for k in FIELDS): bad_ctrl += 1; print('CONTROL MISMATCH', g, p, cs, sd, phi)
            continue
        c = p1.get((g, p, cs, sd, 'C_double_lp+pre')); D = p1.get((g, p, cs, sd, 'D_deg2_full'))
        E = p1.get((g, p, cs, sd, 'E_deg2_lp')); R = p1.get((g, p, cs, sd, 'R_rho_neg'))
        if not (c and D and E and R): continue
        rf = R['cantor_mul'] + 10 * R['cantor_inv']; re = rf / R['steps'] * (math.pi * R['r'] / 4) ** 0.5
        best = min(fpm(D), fpm(E))
        rows[(g, p, phi)].append(dict(F=fpm(r) / re, Fg=fpm(r, g * g) / re, cF=fpm(c) / fpm(r), bestF=best / fpm(r),
                                      cols=r['fb_cols'], cert=r['certified'] and not r.get('exhausted'), ex=c.get('exhausted')))
gm = lambda xs: math.exp(sum(math.log(x) for x in xs) / len(xs))
print(f'E control: {n_ctrl} reruns, {bad_ctrl} mismatches')
print('| g | p | phi | pairs | cols | F/rhoE | F/rhoE (g2) | c/F | min(D,E)/F | certified |')
print('|---|---|---|---|---|---|---|---|---|---|')
for k in sorted(rows, key=lambda k: (k[0], k[1], float(k[2]))):
    v = rows[k]; lb = '>=' if any(x['ex'] for x in v) else ''
    print(f"| {k[0]} | {k[1]} | {k[2]} | {len(v)} | {v[0]['cols']} | {gm([x['F'] for x in v]):.3g} | {gm([x['Fg'] for x in v]):.3g} | {lb}{gm([x['cF'] for x in v]):.3g} | {gm([x['bestF'] for x in v]):.3g} | {all(x['cert'] for x in v)} |")
