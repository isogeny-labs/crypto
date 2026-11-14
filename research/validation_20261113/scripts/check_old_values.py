#!/usr/bin/env python3
"""Compare the remeasure's in-process 'recorded' arms and old ratios with the
original committed result files (results_v3.json, tower_large.json, pushforward.json)."""
import json, sys, statistics
ROOT = '/home/user/crypto/research'
NR = json.load(open(f'{ROOT}/negation_remeasure_20261113/remeasure.json'))['targets']
V3 = json.load(open(f'{ROOT}/chained_prime_transfer_20261111/results_v3.json'))
TW = json.load(open(f'{ROOT}/chained_prime_transfer_20261111/tower_large/tower_large.json'))
CP = json.load(open(f'{ROOT}/cover_pushforward_20261112/pushforward.json'))
BAND = [0.0, 0.1, 1.0]
res = {'checks': 0, 'mismatches': [], 'notes': []}

def chk(label, a, b, tol=1e-9):
    res['checks'] += 1
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        ok = abs(a - b) <= tol * max(1, abs(a), abs(b))
    else:
        ok = a == b
    if not ok:
        res['mismatches'].append([label, a, b])

# v3
for row in NR['v3']['rows']:
    p = row['p']
    o = [x for x in V3['rows'] if x['p'] == p][0]
    for k_new, k_old in [('candidate', 'candidate'), ('r', 'r'), ('x', 'x'), ('e1_order', 'e1_order'),
                         ('e2_order', 'e2_order'), ('jac_order', 'jac_order'), ('old_plain_rho_ops', 'rho_src_ops')]:
        chk(f'v3 p{p} {k_new}', row[k_new], o[k_old])
    A = row['arms']['recorded']
    chk(f'v3 p{p} recorded ops', A['group_ops'], o['ic_jac_group_ops'])
    chk(f'v3 p{p} recorded la', A['native']['mul_r'], o['ic_jac_la_mulmods'])
    chk(f'v3 p{p} recorded trials', A['trials'], o['ic_jac_trials'])
    chk(f'v3 p{p} recorded relations', A['relations'], o['ic_jac_relations'])
    chk(f'v3 p{p} recorded fb', A['fb'], o['ic_jac_fb'])
    rr = [(o['ic_jac_group_ops'] + w * o['ic_jac_la_mulmods']) / o['rho_src_ops'] for w in BAND]
    for i in range(3):
        chk(f'v3 p{p} old ratio {BAND[i]}', A['recorded_ratio_to_old_plain_rho'][i], rr[i])
    chk(f'v3 p{p} recorded log', o['ic_claimed_log'], row['x'])

# tower
for row in NR['tower_large']['rows']:
    p = row['p']
    o = [x for x in TW['rows'] if x['p'] == p][0]
    chk(f'tower p{p} r_mean', row['r_mean'], o['r_mean'])
    chk(f'tower p{p} old rho mean', row['old_plain_rho_ops_mean'], o['rho_ops'])
    for gi, g in enumerate([2, 3]):
        A = row['arms'][f'g{g}_recorded']
        same_seeds = (A['runs'] == o['seeds'] * o['instances'])
        oic = o['ic'][gi]
        if same_seeds:
            chk(f'tower p{p} g{g} ops', A['group_ops'], oic['ops'])
            chk(f'tower p{p} g{g} la', A['la_mulmods'], oic['la_mulmods'])
            chk(f'tower p{p} g{g} inv', A['native'].get('inv_r', 0), oic['la_inversions'])
            chk(f'tower p{p} g{g} mul_p', A['native'].get('mul_p', 0), oic['oracle_fp_muls'] + oic['factor_base_fp_muls'])
            chk(f'tower p{p} g{g} canon', A['native'].get('canon', 0), oic['canonicalisations'])
            chk(f'tower p{p} g{g} draw', A['native'].get('draw', 0), oic['step_draws'])
            for i in range(3):
                chk(f'tower p{p} g{g} old ratio {BAND[i]}', A['recorded_ratio_to_old_plain_rho_la_priced'][i], o['ratio_la'][gi][i])
        else:
            res['notes'].append(f'tower p{p} g{g}: {A["runs"]} recorded-arm runs vs original {o["seeds"]}x{o["instances"]}; '
                                f'ops new {A["group_ops"]:.3f} vs original {oic["ops"]:.3f}; old ratio w0 new {A["recorded_ratio_to_old_plain_rho_la_priced"][0]:.4f} vs original {o["ratio_la"][gi][0]:.4f}, '
                                f'w0.1 {A["recorded_ratio_to_old_plain_rho_la_priced"][1]:.2f} vs {o["ratio_la"][gi][1]:.2f}, w1 {A["recorded_ratio_to_old_plain_rho_la_priced"][2]:.1f} vs {o["ratio_la"][gi][2]:.1f}')
    # instance identities
    for inst in [x for x in TW['instances'] if x['p'] == p]:
        pass

# cover
for row in NR['cover_pushforward']['rows']:
    p = row['p']
    o = [x for x in CP['rows'] if x['p'] == p][0]
    for k in ['candidate', 'r', 'e1_order', 'e2_order']:
        chk(f'cover p{p} {k}', row[k], o[k])
    seeds = o['seeds']
    chk(f'cover p{p} old rho per seed', row['old_plain_rho_ops_per_seed'], [float(s['arms']['rho_e1']['group_ops']) for s in seeds])
    for name in ['ic_jc', 'ic_e1_fibre_full', 'ic_e1_two_term', 'ic_jc_first_collision', 'ic_e1_fibre_first_collision', 'birthday_e1_first_collision']:
        A = row['arms'][name]
        ops = [s['arms'][name]['group_ops'] for s in seeds if name in s['arms']]
        chk(f'cover p{p} {name} mean ops', A['group_ops'], sum(ops) / len(ops))
        chk(f'cover p{p} {name} seeds', A['seeds'], len(ops))
        for i, w in enumerate(['ratio_w0', 'ratio_w0.1', 'ratio_w1']):
            rs = [s['arms'][name]['totals'][w] for s in seeds if name in s['arms']]
            chk(f'cover p{p} {name} old median {w}', A['recorded_median_ratio_to_old_plain_rho'][i], statistics.median(rs))
        if name in ('ic_jc',):
            la = [s['arms'][name]['la_mulmods'] for s in seeds]
            chk(f'cover p{p} {name} la mean', A['native']['mul_r'], sum(la) / len(la))
json.dump(res, open(sys.argv[1], 'w'), indent=1)
print('checks', res['checks'], 'mismatches', len(res['mismatches']))
for m in res['mismatches'][:30]: print(m)
for n in res['notes']: print('NOTE', n)
