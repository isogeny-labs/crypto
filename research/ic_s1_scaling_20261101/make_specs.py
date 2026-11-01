#!/usr/bin/env python3
"""S1 (2026-11-01): ecbench specs.  Same law as E1/R2/R2W (target_seed 202610111901 except the n=13 second set, 1 round, alternate, L0).
usage: make_specs.py <outdir>   (specs are also generated on demand by spec(...) imports)."""
import json, sys, os
CELLS = {'k0n13': (0, 13), 'k1n11': (1, 11), 'k0n19': (0, 19), 'k1n19': (1, 19), 'k0n13s2': (0, 13),
         'k0n15': (0, 15), 'k1n15': (1, 15), 'k1n17': (1, 17), 'k0n9': (0, 9), 'k1n9': (1, 9)}
SEED = {'k0n13s2': 202611011301}
LAB = 'pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192'
CFG = {'A': (1, 1), 'B': (0, 0), 'C': (1, 0), 'D': (0, 1)}   # (xg, sym): A = -x -b, B = none, C = -x, D = -b
def wd(cfg):
    xg, sym = CFG[cfg]
    return f'pdp3-binary:m=3,engine=wdsat-s4,sym={sym},xg={xg},find=first'
def arm(name, l, oracle, mt=200000):
    return {'name': name, 'role': 'candidate', 'method': {'id': 'ic.pipeline', 'params': {
        'factor_base': f'binary-standard-subspace:dimension={l}', 'oracle': oracle, 'max_trials': str(mt)}}}
def spec(label, cell, wl=(), ll=(), targets=8, mt=200000, timeout=3600):
    """wl: list of (cfg, l); ll: list of lab l."""
    a, n = CELLS[cell]
    return {'schema': 'ecbench.spec/v1', 'label': label,
            'description': 'S1 2026-11-01: WDSat published arm (config A=-x -b, B=none, C=-x, D=-b) and lab arm; protocol S1_SCALING_20261101.md',
            'workloads': {'curves': [{'kind': 'koblitz', 'a': a, 'n': n}], 'targets_per_curve': targets,
                          'target_seed': SEED.get(cell, 202610111901), 'target_kind': 'public'},
            'arms': [arm(f'inherited-f4-l{l}', l, LAB, mt) for l in ll] + [arm(f'wdsat-{c}-l{l}', l, wd(c), mt) for c, l in wl],
            'measurement': {'rounds': 1, 'warmup': 0, 'order': 'alternate', 'seed': 202610111902,
                            'isolation_required': 'L0', 'timeout_seconds': timeout}}
def dump(outdir, name, s):
    os.makedirs(outdir, exist_ok=True)
    json.dump(s, open(os.path.join(outdir, f'SPEC-{name}.json'), 'w'), indent=1)
if __name__ == '__main__':
    out = sys.argv[1]
    for cell, lab_l, ls in (('k1n11', 5, (4, 8)), ('k0n13', 6, (4, 8))):
        wl = [(c, l) for l in ls for c in 'ABCD']
        dump(out, f'XT_{cell}', spec(f'S1 XG trial {cell}', cell, wl, [], targets=1, timeout=600))
        dump(out, f'XP_{cell}', spec(f'S1 XG probe {cell}', cell, wl, [lab_l], timeout=1800))
