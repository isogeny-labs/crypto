#!/usr/bin/env python3
"""Write the frozen single-target ecbench specs (PROTOCOL.md section 3)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '..', 'specs')
CURVES = {
    28: dict(a=169487139, b=71222779, group_order=268462472, gx=178958571, gy=19523146,
             kind='prime_explicit', p=268435157, r=33557809),
    32: dict(a=1173263412, b=337904346, group_order=4295077112, gx=1431665017, gy=592057774,
             kind='prime_explicit', p=4294967197, r=536884639),
}
SESSIONS, ROUNDS = 8, 64
BASE1 = 'prime-projected-abscissa:size=1,scan_cap=32'


def pipe(name, role, oracle):
    return dict(name=name, role=role, method=dict(id='ic.pipeline', params=dict(
        factor_base=BASE1, oracle=oracle, targets='stride-generator')))


def arms():
    return [dict(name='rho-neg', role='reference', method=dict(id='rho.negation')),
            pipe('f1-x07', 'baseline', 'wwalk:x=0.7,g=1,grow=2,seed=run'),
            pipe('twcj-j16', 'candidate', 'wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=16,seed=run'),
            pipe('twcj-j32', 'candidate', 'wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=32,seed=run'),
            pipe('twcj-j64', 'candidate', 'wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=64,seed=run')]


def main():
    os.makedirs(OUT, exist_ok=True)
    for k, curve in CURVES.items():
        for s in range(SESSIONS):
            spec = {'schema': 'ecbench.spec/v1',
                    'label': f'tw scaling, single target, k={k}, session {s}',
                    'description': ('research/curve25519_tw_scaling_20261108: F=1 target-in-walk arm '
                                    '(tw=1,cj=1; jumps 16, 32, 64) vs the F=1 table-then-probe arm vs '
                                    'rho-neg at k=28, 32, one public target per session. Frozen before '
                                    'data (PROTOCOL.md). Public synthetic Curve25519-family miniature; '
                                    'no security claim.'),
                    'arms': arms(),
                    'measurement': dict(isolation_required='L0', order='alternate', rounds=ROUNDS,
                                        seed=20261108300 + 100 * k + s, timeout_seconds=1800, warmup=1),
                    'workloads': dict(curves=[curve], target_kind='public',
                                      target_seed=20261108800 + 100 * k + s, targets_per_curve=1)}
            with open(os.path.join(OUT, f'k{k}-s{s}.json'), 'w') as f:
                json.dump(spec, f, indent=1, sort_keys=True); f.write('\n')
    print('wrote', len(CURVES) * SESSIONS, 'specs to', os.path.normpath(OUT))


if __name__ == '__main__':
    main()
