#!/usr/bin/env python3
"""R5: ecbench specs.  Same law as the pilot / E1 specs (target_seed 202610111901, 8 public targets,
1 round, alternate order, L0).  Lab arms at l = 4, 5 repeat E1's (A/A against E1 and a paired ratio in
the same session).  Session R5_<cell>: WDSat built with __FIND_ALL_SOLUTIONS__ (every model, as msolve
returns every root).  Session R5f_<cell>: WDSat in its default first-solution mode (the pipeline uses
only the first root).  Both at l = 4, 5, `-x -b`.   usage: make_specs.py <outdir>"""
import json, sys, os
CELLS = {'k0n9': (0, 9), 'k1n9': (1, 9), 'k1n11': (1, 11), 'k0n13': (0, 13)}
def arm(name, l, oracle):
    return {'name': name, 'role': 'candidate', 'method': {'id': 'ic.pipeline', 'params': {
        'factor_base': f'binary-standard-subspace:dimension={l}', 'oracle': oracle, 'max_trials': '200000'}}}
LAB = 'pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192'
WD = 'pdp3-binary:m=3,engine=wdsat-s4,sym=1,xg=1'
os.makedirs(sys.argv[1], exist_ok=True)
for prefix, find, name in (('R5', 'all', 'wdsat'), ('R5f', 'first', 'wdsatf')):
    for cell, (a, n) in CELLS.items():
        spec = {'schema': 'ecbench.spec/v1', 'label': f'{prefix} {cell}',
                'description': f'R5 WDSat arm: wdsat-s4 (e-form S4, WDSat find={find}, -x -b) against the lab arm; E1 session law.',
                'workloads': {'curves': [{'kind': 'koblitz', 'a': a, 'n': n}], 'targets_per_curve': 8,
                              'target_seed': 202610111901, 'target_kind': 'public'},
                'arms': [arm('inherited-f4-l4', 4, LAB), arm('inherited-f4-l5', 5, LAB),
                         arm(f'{name}-l4', 4, WD + (',find=first' if find == 'first' else '')),
                         arm(f'{name}-l5', 5, WD + (',find=first' if find == 'first' else ''))],
                'measurement': {'rounds': 1, 'warmup': 0, 'order': 'alternate', 'seed': 202610111902,
                                'isolation_required': 'L0', 'timeout_seconds': 3600}}
        json.dump(spec, open(os.path.join(sys.argv[1], f'SPEC-{prefix}_{cell}.json'), 'w'), indent=1)
