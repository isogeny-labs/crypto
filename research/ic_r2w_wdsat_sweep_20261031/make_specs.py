#!/usr/bin/env python3
"""R2W: ecbench specs.  Same law as E1/R2/R5 (target_seed 202610111901, 8 public targets, 1 round, alternate, L0).
usage: make_specs.py <outdir>.  One spec per (session name, cell, WDSat l list, lab l list, targets, max_trials)."""
import json, sys, os
CELLS = {'k0n9': (0, 9), 'k1n9': (1, 9), 'k1n11': (1, 11), 'k0n13': (0, 13), 'k0n15': (0, 15)}
LAB = 'pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192'
WD = 'pdp3-binary:m=3,engine=wdsat-s4,sym=1,xg=1,find=first'
def arm(name, l, oracle, mt):
    return {'name': name, 'role': 'candidate', 'method': {'id': 'ic.pipeline', 'params': {
        'factor_base': f'binary-standard-subspace:dimension={l}', 'oracle': oracle, 'max_trials': str(mt)}}}
def spec(label, cell, wl, ll, targets=8, mt=200000, timeout=3600):
    a, n = CELLS[cell]
    return {'schema': 'ecbench.spec/v1', 'label': label,
            'description': 'R2W: WDSat (wdsat-s4, first-solution, -x -b) published arm swept over l, with the lab arm at its R2-best l as A/A.',
            'workloads': {'curves': [{'kind': 'koblitz', 'a': a, 'n': n}], 'targets_per_curve': targets,
                          'target_seed': 202610111901, 'target_kind': 'public'},
            'arms': [arm(f'inherited-f4-l{l}', l, LAB, mt) for l in ll] + [arm(f'wdsat-l{l}', l, WD, mt) for l in wl],
            'measurement': {'rounds': 1, 'warmup': 0, 'order': 'alternate', 'seed': 202610111902,
                            'isolation_required': 'L0', 'timeout_seconds': timeout}}
def dump(name, s):
    json.dump(s, open(os.path.join(sys.argv[1], f'SPEC-{name}.json'), 'w'), indent=1)
os.makedirs(sys.argv[1], exist_ok=True)
BEST_LAB = {'k0n9': 5, 'k1n9': 3, 'k1n11': 5, 'k0n13': 6, 'k0n15': 7}
for c in CELLS:
    dump(f'W_{c}', spec(f'R2W {c}', c, [4, 5], [BEST_LAB[c]]))                 # main: l=4,5 whole solves + lab A/A
    dump(f'V3_{c}', spec(f'R2W {c} l3 viability', c, [3], [], targets=1, mt=3000, timeout=600))   # l=3 viability trial
    dump(f'W6_{c}', spec(f'R2W {c} l6', c, [6], []))
    dump(f'W7_{c}', spec(f'R2W {c} l7', c, [7], []))
    for L in (9, 10, 11, 12, 13): dump(f'W{L}_{c}', spec(f'R2W {c} l{L} (extension)', c, [L], []))
    dump(f'W8_{c}', spec(f'R2W {c} l8 (extension beyond the pre-fixed grid)', c, [8], []))
for c in CELLS:
    dump(f'W3_{c}', spec(f'R2W {c} wdsat l3 full', c, [3], []))            # l=3 full session (200000 trials)
    dump(f'L3_{c}', spec(f'R2W {c} lab l3', c, [], [3]))                    # lab l=3 (only where WDSat l=3 is viable)
