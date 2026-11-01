#!/usr/bin/env python3
"""S2 specs. Same law as R2W (target_seed 202610111901, 8 public targets, 1 round, alternate, L0).
usage: make_specs.py <outdir>.  AA_<cell>: lab at best l + WDSat (a) at its whole-best l + WDSat (b) at the same l (not swept).
A_<abl>_<cell>: lab at best l with abl=<abl>."""
import json, sys, os
CELLS = {'k1n11': (1, 11), 'k0n13': (0, 13), 'k0n15': (0, 15)}
LABL = {'k1n11': 5, 'k0n13': 6, 'k0n15': 7}
WDL = {'k1n11': 8, 'k0n13': 8, 'k0n15': 10}
LAB = 'pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192'
WDA = 'pdp3-binary:m=3,engine=wdsat-s4,sym=1,xg=1,find=first'
WDB = 'pdp3-binary:m=3,engine=wdsat-s4,sym=0,xg=0,find=first'
ABLS = ['memo0', 'nolin', 'noinh', 'splitlow', 'chainlayout', 'occmult', 'alloff']
def arm(name, l, oracle):
    return {'name': name, 'role': 'candidate', 'method': {'id': 'ic.pipeline', 'params': {
        'factor_base': f'binary-standard-subspace:dimension={l}', 'oracle': oracle, 'max_trials': '200000'}}}
def spec(label, cell, arms, targets=8, timeout=3600):
    a, n = CELLS[cell]
    return {'schema': 'ecbench.spec/v1', 'label': label, 'description': 'S2: lab-arm ablation vs WDSat (research/ic_s2_lab_ablation_20261101/PROTOCOL.md)',
            'workloads': {'curves': [{'kind': 'koblitz', 'a': a, 'n': n}], 'targets_per_curve': targets, 'target_seed': 202610111901, 'target_kind': 'public'},
            'arms': arms, 'measurement': {'rounds': 1, 'warmup': 0, 'order': 'alternate', 'seed': 202610111902, 'isolation_required': 'L0', 'timeout_seconds': timeout}}
out = sys.argv[1]; os.makedirs(out, exist_ok=True)
def dump(name, s): json.dump(s, open(os.path.join(out, f'SPEC-{name}.json'), 'w'), indent=1)
for c in CELLS:
    dump(f'AA_{c}', spec(f'S2 {c} A/A', c, [arm(f'lab-l{LABL[c]}', LABL[c], LAB)]))
    dump(f'WA_{c}', spec(f'S2 {c} WDSat a', c, [arm(f'wda-l{WDL[c]}', WDL[c], WDA)]))
    dump(f'WB_{c}', spec(f'S2 {c} WDSat b (not swept)', c, [arm(f'wdb-l{WDL[c]}', WDL[c], WDB)]))
    for x in ABLS:
        dump(f'A_{x}_{c}', spec(f'S2 {c} abl {x}', c, [arm(f'lab-l{LABL[c]}-{x}', LABL[c], LAB + f',abl={x}')]))
