#!/usr/bin/env python3
"""Write the frozen single-target ecbench specs (PROTOCOL.md section 3)."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '..', 'specs')
CURVES = {
    20: dict(a=63700, b=872826, group_order=1049624, gx=349837, gy=122,
             kind='prime_explicit', p=1048573, r=131203),
    24: dict(a=16743506, b=2381926, group_order=16778792, gx=109, gy=3123379,
             kind='prime_explicit', p=16777213, r=2097349),
}
SESSIONS, ROUNDS = 8, 64
BASE1 = 'prime-projected-abscissa:size=1,scan_cap=32'
JUMPS = [16, 2, 4, 8, 32]


def pipe(name, role, oracle):
    return dict(name=name, role=role, method=dict(id='ic.pipeline', params=dict(
        factor_base=BASE1, oracle=oracle, targets='stride-generator')))


def arms():
    out = [dict(name='rho-neg', role='reference', method=dict(id='rho.negation'))]
    for j in JUMPS:
        out.append(pipe(f'twcj-j{j}', 'baseline' if j == 16 else 'candidate',
                        f'wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps={j},seed=run'))
    return out


def main():
    os.makedirs(OUT, exist_ok=True)
    for k, curve in CURVES.items():
        for s in range(SESSIONS):
            spec = {
                'schema': 'ecbench.spec/v1',
                'label': f'tw jump count, single target, k={k}, session {s}',
                'description': ('research/curve25519_tw_jumps_20261108: F=1 target-in-walk arm '
                                '(tw=1,cj=1) with jumps=2,4,8,16,32 vs rho-neg, one public target '
                                'per session. Frozen before data (PROTOCOL.md). Public synthetic '
                                'Curve25519-family miniature; no security claim.'),
                'arms': arms(),
                'measurement': dict(isolation_required='L0', order='alternate', rounds=ROUNDS,
                                    seed=20261108200 + 100 * k + s, timeout_seconds=1800, warmup=1),
                'workloads': dict(curves=[curve], target_kind='public',
                                  target_seed=20261108700 + 100 * k + s, targets_per_curve=1),
            }
            with open(os.path.join(OUT, f'k{k}-s{s}.json'), 'w') as f:
                json.dump(spec, f, indent=1, sort_keys=True); f.write('\n')
    print('wrote', len(CURVES) * SESSIONS, 'specs to', os.path.normpath(OUT))


if __name__ == '__main__':
    main()
