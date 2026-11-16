#!/usr/bin/env python3
"""Pre-data model for the k = 28, 32 scaling study (PROTOCOL.md section 4) -> data/model.json.
Inputs: the measured phases of twcj-j16/j32 and f1-x07 at k = 20, 24
(research/curve25519_f1_targetwalk_20261108 and research/curve25519_tw_jumps_20261108), the
set-up rules of TargetWalk::cheap_jumps, and rho-neg S at k = 28, 32 from
research/curve25519_autolab_20261104/ideas/OBSTRUCTION.md (1.04, 1.01). No data of this study."""
import json, math, os
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
R = {28: 33557809, 32: 536884639}
RHO = {28: 1.04, 32: 1.01}
WALK = {16: 1.37, 32: 1.345, 64: 1.335}   # measured j16 1.386/1.355, j32 1.352/1.335 at k = 20/24; j64 extrapolated
VERIFY_GAE = {28: 130, 32: 150}           # 92 (k=20), 114 (k=24): about 4 recomputations of (log r)-bit combinations
FIXED_GAE = {28: 70, 32: 75}              # 45 (k=20), 58 (k=24): base, LA, [d]G check
F1 = dict(walk=1.0, probes=0.58, setup_gae={28: 120, 32: 140})


def setup_gae(k, jumps):
    r = R[k]; sq = math.sqrt(r); t = round(1.5 * sq); w = round(1.2 * math.sqrt(t))
    nbits = w.bit_length(); rbits = (r - 1).bit_length()
    return (nbits - 1) + 2 * (rbits - 1) + 2 * (rbits - 1) + (1 + (nbits / 2 - 1) + 1) * jumps


out = {}
for k in (28, 32):
    sq = math.sqrt(R[k]); d = dict(arms={})
    for j in (16, 32, 64):
        total = (FIXED_GAE[k] + setup_gae(k, j) + VERIFY_GAE[k]) / sq + WALK[j]
        d['arms'][f'twcj-j{j}'] = dict(total=total, setup_sqrt_r=setup_gae(k, j) / sq, walk=WALK[j])
    f1 = (FIXED_GAE[k] + F1['setup_gae'][k]) / sq + F1['walk'] + F1['probes']
    d['arms']['f1-x07'] = dict(total=f1)
    d['rho_neg'] = RHO[k]
    t16 = d['arms']['twcj-j16']['total']
    d['ratios'] = {'twcj-j16/rho-neg': t16 / RHO[k], 'twcj-j16/f1-x07': t16 / f1,
                   'twcj-j32/twcj-j16': d['arms']['twcj-j32']['total'] / t16,
                   'twcj-j64/twcj-j16': d['arms']['twcj-j64']['total'] / t16}
    out[f'k{k}'] = d
json.dump(out, open(os.path.join(HERE, 'data', 'model.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, {a: round(v['total'], 3) for a, v in d['arms'].items()}, {p: round(v, 3) for p, v in d['ratios'].items()})
