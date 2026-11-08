#!/usr/bin/env python3
"""Write the frozen single-target ecbench specs (PROTOCOL.md section 3).

One spec per (k, session): targets_per_curve = 1, so each session solves one
public target; arms run `rounds` measured rounds each with a fresh run seed.
"""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '..', 'specs')

CURVES = {
    20: dict(a=63700, b=872826, group_order=1049624, gx=349837, gy=122,
             kind='prime_explicit', p=1048573, r=131203),
    24: dict(a=16743506, b=2381926, group_order=16778792, gx=109, gy=3123379,
             kind='prime_explicit', p=16777213, r=2097349),
}
SESSIONS = 8
ROUNDS = 64
BASE1 = 'prime-projected-abscissa:size=1,scan_cap=32'


def pipe(name, role, oracle):
    return dict(name=name, role=role, method=dict(id='ic.pipeline', params=dict(
        factor_base=BASE1, oracle=oracle, targets='stride-generator')))


def arms():
    return [
        dict(name='rho-neg', role='reference', method=dict(id='rho.negation')),
        pipe('f1-x07', 'baseline', 'wwalk:x=0.7,g=1,grow=2,seed=run'),
        pipe('tw-x15', 'candidate', 'wwalk:x=1.5,g=1,grow=2,tw=1,seed=run'),
        pipe('twcj-x15', 'candidate', 'wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,seed=run'),
    ]


def main():
    os.makedirs(OUT, exist_ok=True)
    for k, curve in CURVES.items():
        for s in range(SESSIONS):
            spec = {
                'schema': 'ecbench.spec/v1',
                'label': f'F=1 target-in-walk, single target, k={k}, session {s}',
                'description': ('research/curve25519_f1_targetwalk_20261108: F=1 wwalk with the '
                                'target inside the walk (tw=1, and tw=1 with chained jumps cj=1) vs '
                                'the F=1 table-then-probe arm vs rho-neg, one public target per '
                                'session. Frozen before data (PROTOCOL.md). Public synthetic '
                                'Curve25519-family miniature; no security claim.'),
                'arms': arms(),
                'measurement': dict(isolation_required='L0', order='alternate', rounds=ROUNDS,
                                    seed=20261108000 + 100 * k + s, timeout_seconds=1800, warmup=1),
                'workloads': dict(curves=[curve], target_kind='public',
                                  target_seed=20261108500 + 100 * k + s, targets_per_curve=1),
            }
            with open(os.path.join(OUT, f'k{k}-s{s}.json'), 'w') as f:
                json.dump(spec, f, indent=1, sort_keys=True)
                f.write('\n')
    print('wrote', len(CURVES) * SESSIONS, 'specs to', os.path.normpath(OUT))


if __name__ == '__main__':
    main()
