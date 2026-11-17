#!/usr/bin/env python3
"""P1 table: per (g, p, curve, seed) cell, every arm's counted cost and its
ratio to the matched negation-map rho on the same instance.

Units (PLAN.md section 4):
  FPM_k = cantor_mul + 10*cantor_inv + rel_mul + k*la_mul, k in {1, g^2}
          (la_mul is a product modulo r ~ p^g; k = g^2 prices it as a
          schoolbook multi-word product against one F_p product).
  rho   = cantor_mul + 10*cantor_inv of the executed walk; rho_E = the
          expected sqrt(pi r / 4) steps times that walk's measured cost per
          step (one walk's step count has a large spread).
  GAE_w = group_ops + w*(rel_mul + la_mul), w in {0, 0.1, 1}; rho: group_ops.
Every number is [pcc:unverified].
"""
import glob, json, sys, os
from collections import defaultdict

def load(paths):
    cells = defaultdict(dict)
    for f in paths:
        for line in open(f):
            try:
                d = json.loads(line)
            except Exception:
                continue
            if d.get('kind') not in ('run', 'skip'):
                continue
            key = (d['g'], d['p'], d['curve_seed'], d['seed'])
            cells[key][d['arm']] = d
    return cells

def fpm(d, k):
    return d['cantor_mul'] + 10 * d['cantor_inv'] + d.get('rel_mul', 0) + k * d.get('la_mul', 0)

def gae(d, w):
    return d['group_ops'] + w * (d.get('rel_mul', 0) + d.get('la_mul', 0))

def main(paths):
    cells = load(paths)
    rows = []
    hdr = ('g', 'p', 'curve', 'seed', 'arm', 'cert', 'trials', 'rels', 'cols',
           'FPM_1', 'FPM_g2', 'IC/rho_1', 'IC/rho_g2', 'IC/rhoE_1', 'IC/rhoE_g2', 'GAE0/rho', 'GAE1/rho')
    print('| ' + ' | '.join(hdr) + ' |')
    print('|' + '---|' * len(hdr))
    for key in sorted(cells):
        g, p, cs, sd = key
        arms = cells[key]
        rho = arms.get('R_rho_neg')
        rf = re = None
        if rho and rho['kind'] == 'run':
            rf = rho['cantor_mul'] + 10 * rho['cantor_inv']
            re = rf / rho['steps'] * (3.141592653589793 * rho['r'] / 4) ** 0.5
        for name in sorted(arms):
            d = arms[name]
            if name.startswith('R_'):
                if d['kind'] == 'run':
                    print(f"| {g} | {p} | {cs} | {sd} | {name} | {d['certified']} | {d['steps']} | - | - | {rf:.3e} | {rf:.3e} | 1 | 1 | {rf/re:.3g} | {rf/re:.3g} | 1 | 1 |")
                else:
                    print(f"| {g} | {p} | {cs} | {sd} | {name} | skip | ~{d.get('expected_steps',0):.2e} | - | - | - | - | - | - | - | - | - | - |")
                continue
            if d['kind'] != 'run':
                continue
            f1, fg = fpm(d, 1), fpm(d, g * g)
            r1 = f'{f1/rf:.4g}' if rf else '-'
            rg = f'{fg/rf:.4g}' if rf else '-'
            e1 = f'{f1/re:.4g}' if re else '-'
            eg = f'{fg/re:.4g}' if re else '-'
            g0 = f"{gae(d,0)/rho['group_ops']:.4g}" if rf else '-'
            g1 = f"{gae(d,1)/rho['group_ops']:.4g}" if rf else '-'
            cert = d['certified'] if not d.get('exhausted') else 'EXHAUSTED'
            print(f"| {g} | {p} | {cs} | {sd} | {name} | {cert} | {d['trials']} | {d['relations']} | {d['fb_cols']} | {f1:.3e} | {fg:.3e} | {r1} | {rg} | {e1} | {eg} | {g0} | {g1} |")

def summary(paths):
    """Per (g, p): geometric means over (curve, seed) pairs."""
    import math
    cells = load(paths)
    by = defaultdict(list)
    for key, arms in cells.items():
        by[key[:2]].append(arms)
    def gm(xs):
        return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else float('nan')
    print()
    print('| g | p | pairs | c/rhoE | D/rhoE | E/rhoE | D/rhoE (g2) | E/rhoE (g2) | c/D | c/E | c/D range | c/E range | c exhausted | all certified |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for (g, p) in sorted(by):
        rows = []
        for arms in by[(g, p)]:
            rho = arms.get('R_rho_neg')
            c, D, E = arms.get('C_double_lp+pre'), arms.get('D_deg2_full'), arms.get('E_deg2_lp')
            if not (rho and rho['kind'] == 'run' and c and D and E):
                continue
            rf = rho['cantor_mul'] + 10 * rho['cantor_inv']
            re = rf / rho['steps'] * (math.pi * rho['r'] / 4) ** 0.5
            rows.append(dict(c=fpm(c, 1) / re, D=fpm(D, 1) / re, E=fpm(E, 1) / re,
                             Dg=fpm(D, g * g) / re, Eg=fpm(E, g * g) / re,
                             cD=fpm(c, 1) / fpm(D, 1), cE=fpm(c, 1) / fpm(E, 1),
                             ex=bool(c.get('exhausted')),
                             cert=all(x['certified'] for x in (c, D, E) if not x.get('exhausted')) and rho['certified']))
        if not rows:
            continue
        f = lambda k: gm([r[k] for r in rows])
        rng = lambda k: f"{min(r[k] for r in rows):.3g}-{max(r[k] for r in rows):.3g}"
        nex = sum(r['ex'] for r in rows)
        lb = '>=' if nex else ''
        print(f"| {g} | {p} | {len(rows)} | {lb}{f('c'):.3g} | {f('D'):.3g} | {f('E'):.3g} | {f('Dg'):.3g} | {f('Eg'):.3g} | {lb}{f('cD'):.3g} | {lb}{f('cE'):.3g} | {rng('cD')} | {rng('cE')} | {nex} | {all(r['cert'] for r in rows)} |")

if __name__ == '__main__':
    paths = sys.argv[1:] or glob.glob(os.path.join(os.path.dirname(__file__), 'pilot', '*.jsonl'))
    main(paths)
    summary(paths)
