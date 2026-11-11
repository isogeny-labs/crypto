#!/usr/bin/env python3
"""Write the frozen single-target ecbench specs (PROTOCOL.md section 3).

One spec per (k, session): targets_per_curve = 1, so each session solves one
public target; arms run `rounds` measured rounds each with a fresh run seed.
"""
import json, os, sys

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


def pipe(name, role, size, x):
    return dict(name=name, role=role, method=dict(id='ic.pipeline', params=dict(
        factor_base=f'prime-projected-abscissa:size={size},scan_cap=32',
        oracle=f'wwalk:x={x},g=1,grow=2,seed=run',
        targets='stride-generator')))


def arms():
    return [
        dict(name='rho-neg', role='reference', method=dict(id='rho.negation')),
        dict(name='rho-aa', role='control', method=dict(id='rho.negation')),
        pipe('f2-inc', 'baseline', 2, 0.9),
        pipe('f1-x05', 'candidate', 1, 0.5),
        pipe('f1-x07', 'candidate', 1, 0.7),
        pipe('f1-x09', 'candidate', 1, 0.9),
    ]


def main():
    os.makedirs(OUT, exist_ok=True)
    for k, curve in CURVES.items():
        for s in range(SESSIONS):
            spec = {
                'schema': 'ecbench.spec/v1',
                'label': f'F=1 walk table, single target, k={k}, session {s}',
                'description': ('research/curve25519_f1_walk_20261107: one-seed (F=1) wwalk vs the '
                                'F=2 incumbent vs rho-neg, one public target per session. Frozen '
                                'before data (PROTOCOL.md). Public synthetic Curve25519-family '
                                'miniature; no security claim.'),
                'arms': arms(),
                'measurement': dict(isolation_required='L0', order='alternate', rounds=ROUNDS,
                                    seed=20261107000 + 100 * k + s, timeout_seconds=1800, warmup=1),
                'workloads': dict(curves=[curve], target_kind='public',
                                  target_seed=20261107500 + 100 * k + s, targets_per_curve=1),
            }
            path = os.path.join(OUT, f'k{k}-s{s}.json')
            with open(path, 'w') as f:
                json.dump(spec, f, indent=1, sort_keys=True)
                f.write('\n')
    print('wrote', len(CURVES) * SESSIONS, 'specs to', os.path.normpath(OUT))


if __name__ == '__main__':
    main()
