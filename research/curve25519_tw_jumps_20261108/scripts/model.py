#!/usr/bin/env python3
"""Pre-data cost model for the jump-count sweep (PROTOCOL.md section 4) -> data/model.json.

Inputs are the measured phase means of research/curve25519_f1_targetwalk_20261108
(data/pooled_measured.json there; twcj-x15 at k = 20 and 24) and the set-up rules read
from TargetWalk::cheap_jumps in src/cryptanalysis/ic_boundary.rs. No data of this study.
"""
import json, math, os

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
R = {20: 131203, 24: 2097349}
# measured twcj-x15 phases, units of sqrt(r) (earlier study, README section 3 phase table)
MEAS = {20: dict(fixed=0.126, setup=0.368, walk=1.390, verify=0.255, total=2.138, rho=1.531),
        24: dict(fixed=0.040, setup=0.110, walk=1.394, verify=0.079, total=1.623, rho=1.150)}


def popcount_mean(bits):
    # mean popcount of a uniform integer in [1, 2^bits) ~ bits/2
    return bits / 2.0


def setup_gae(k, jumps):
    r = R[k]; sq = math.sqrt(r)
    t = round(1.5 * sq); w = round(1.2 * math.sqrt(t))
    nbits = w.bit_length(); rbits = (r - 1).bit_length()
    doublings = (nbits - 1) + 2 * (rbits - 1)
    # K_0, K_1: eval(0, gamma, beta): popcount(gamma) + popcount(beta) - 1 additions each
    k01 = 2 * (2 * popcount_mean(rbits) - 1)
    # per jump: one chained addition + eval(alpha) (popcount(alpha) - 1) + one addition
    per_jump = 1 + (popcount_mean(nbits) - 1) + 1
    return doublings + k01 + per_jump * jumps, dict(doublings=doublings, k01=k01, per_jump=per_jump, w=w)


out = {}
for k in (20, 24):
    sq = math.sqrt(R[k]); m = MEAS[k]
    base16, parts = setup_gae(k, 16)
    out[f'k{k}'] = dict(model_setup_gae_j16=base16, measured_setup_gae_j16=m['setup'] * sq, parts=parts, arms={})
    for j in (2, 4, 8, 16, 32):
        s, _ = setup_gae(k, j)
        d_setup = (s - base16) / sq  # applied to the measured set-up
        walk = m['walk']
        total = m['total'] + d_setup
        note = 'walk assumed unchanged (jumps >= 4)'
        if j == 2:
            # two jumps: the walk's distinct difference vectors grow like n^(3/2), so the
            # collision count after n steps is about n^(3/2)/r instead of n^2/r; the
            # second row needs n of order r^(2/3), above the cap 3T = 4.5 sqrt(r); the
            # arm then probes from a table of 4.5 sqrt(r) entries at rate 2*entries/r.
            walk = 4.5
            probes = R[k] / (2 * 4.5 * sq) / sq
            total = m['fixed'] + m['setup'] + d_setup + walk + m['verify'] + probes
            note = 'walk hits the cap; probes added (heuristic, direction only)'
        out[f'k{k}']['arms'][f'j{j}'] = dict(d_setup_sqrt_r=d_setup, walk=walk, total=total,
                                               ratio_to_j16=total / m['total'], ratio_to_rho=total / m['rho'], note=note)
json.dump(out, open(os.path.join(HERE, 'data', 'model.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, 'model set-up j16 =', round(d['model_setup_gae_j16'], 1), 'GAE, measured', round(d['measured_setup_gae_j16'], 1))
    for a, v in d['arms'].items():
        print('  ', a, 'ratio to j16 %.3f' % v['ratio_to_j16'], 'to rho %.3f' % v['ratio_to_rho'], v['note'])
