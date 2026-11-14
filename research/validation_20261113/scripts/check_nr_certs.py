#!/usr/bin/env python3
"""Validator checker for the negation_remeasure replay dump (val_nr_dump output).

For each instance it checks, with the validator's own arithmetic (vlib.py, jcheck.py):
  - the curve and point data (p, r prime; G rational, on E, order r; T = [x]G;
    #E by point count where p < 20000; r || #E for v3/cover),
  - for v3/cover: E1 and the quintic model recomputed from the bielliptic
    coefficients c, the pulled-back divisors phi1^*(G), phi1^*(T) recomputed,
    [r]D_G = 0 and [x]D_G = D_T on J(C), the word-Jacobian divisors equal to them,
  - for the tower: [r]D = 0, [x]D = T on each rung (validator's genus-g Cantor),
    r | #J, Hasse-Weil window,
  - the j-invariant (no j = 0, 1728),
  - EVERY returned log: k = x mod r and [k]G = T on E (rho arms, E1 arms) or
    [k]D_G = D_T on the Jacobian (IC arms), computed with vlib,
and then recomputes the per-arm means from the dump and compares them with the
committed remeasure_*.json parts (to tie the dump to the measured runs).

usage: check_nr_certs.py DUMP.jsonl OUT.json
"""
import json, sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, '/home/user/crypto/research/validation_20261112/scripts')
from vlib import *
from jcheck import jinfo

R = '/home/user/crypto/research'
random.seed(20261113)
dump = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
NR = json.load(open(f'{R}/negation_remeasure_20261113/remeasure.json'))['targets']
CERTS = [json.loads(l) for l in open(f'{R}/chained_prime_transfer_20261111/certs_v3.jsonl') if l.strip()]
CP = json.load(open(f'{R}/cover_pushforward_20261112/pushforward.json'))
TL = json.load(open(f'{R}/chained_prime_transfer_20261111/tower_large/tower_large.json'))
U64 = (1 << 64) - 1

errs, notes = [], []
stats = {'logs': 0, 'logs_certified': 0, 'by_arm': {}, 'instances': 0, 'means_compared': 0}


def P(v):
    if v is None:
        return None
    assert v[2] == 0 and v[3] == 0, 'non-rational point'
    return (v[0], v[1])


def D(d):
    return (ptrim(list(d['u'])), ptrim(list(d['v'])))


def close(a, b, tol=1e-9):
    return abs(a - b) <= tol * max(1.0, abs(a), abs(b))


def cmp(label, mine, theirs):
    stats['means_compared'] += 1
    if theirs is None or not close(mine, theirs):
        errs.append(['mean differs', label, mine, theirs])


def cert_run(label, run, x, r, group, E=None, G=None, T=None, J=None, DG=None, DT=None):
    stats['logs'] += 1
    a = stats['by_arm'].setdefault(label + ':' + run['arm'], [0, 0])
    a[0] += 1
    k = run.get('k')
    ok = k is not None and k % r == x % r
    if ok and group == 'E':
        ok = E.mul(k, G) == T
    elif ok and group == 'J':
        ok = J.norm(J.mul(k, DG)) == J.norm(DT)
    if ok:
        stats['logs_certified'] += 1
        a[1] += 1
    else:
        errs.append(['log not certified', label, run.get('arm'), run.get('seed'), k, x])


def mean(rs, f):
    return sum(f(z) for z in rs) / len(rs)


def check_point_data(label, p, r, x, E, G, T, n_e=None):
    chk = {'p_prime': is_prime(p), 'r_prime': is_prime(r), 'G_on_E': E.on(G), 'G_nonzero': G is not None,
           'rG_zero': E.mul(r, G) is None, 'xG_eq_T': E.mul(x, G) == T}
    if n_e is not None and p < 20000:
        n = E.count()
        chk['n_e_count'] = n == n_e
    ji = jinfo(p, E.a2, E.a4, E.a6)
    chk['no_extra_automorphism'] = not ji['extra_automorphism']
    bad = [k for k, v in chk.items() if v is not True]
    if bad:
        errs.append(['instance check failed', label, bad])
    return chk, ji


def bielliptic_checks(label, p, c, x0_dump, e1, jac_f, G, aux_ok=True):
    x0, g5, E1, E2 = bielliptic(p, c)
    chk = {'x0': x0 == x0_dump, 'E1_from_c': [E1.a2, E1.a4, E1.a6] == list(e1),
           'quintic_from_c': [v % p for v in g5] == [v % p for v in jac_f]}
    bad = [k for k, v in chk.items() if v is not True]
    if bad:
        errs.append(['bielliptic check failed', label, bad])
    return x0, E1


def pull(p, c, x0, Q, E1, J, aux):
    return J.norm(pullback_e1_any(p, c, x0, Q, E1, J, aux))


per = []
for d in dump:
    tgt, p = d['target'], d['p']
    stats['instances'] += 1
    if d.get('none'):
        errs.append(['no instance', tgt, p, d.get('instance')])
        continue
    if tgt == 'v3':
        r, x = d['r'], d['x']
        E = EC(p, *d['e1'])
        G, T = P(d['gen']), P(d['target_pt'])
        lab = f'v3 p{p}'
        chk, ji = check_point_data(lab, p, r, x, E, G, T, d['e1_order'])
        # tie to the original certificates (validated 2026-11-13)
        oc = [c for c in CERTS if c['p'] == p and c['arm'] == 'rho_e1'][0]
        chk['matches_certs_v3'] = ([oc['group']['a2'], oc['group']['a4'], oc['group']['a6']] == d['e1']
                                   and tuple(oc['g']) == G and tuple(oc['t']) == T and oc['claimed_k'] == x and oc['r'] == r)
        if not chk['matches_certs_v3']:
            errs.append(['v3 instance differs from certs_v3.jsonl', p])
        x0, E1 = bielliptic_checks(lab, p, d['c'], d['x0'], d['e1'], d['jac_f'], G)
        J = Jac2(p, d['jac_f'])
        DG, DT = J.norm(D(d['dg'])), J.norm(D(d['dt']))
        jchk = {'DG_valid': J.valid(DG), 'DT_valid': J.valid(DT), 'DG_is_pullback_G': DG == pull(p, d['c'], x0, G, E1, J, G),
                'DT_is_pullback_T': DT == pull(p, d['c'], x0, T, E1, J, G), 'rDG_zero': J.is_id(J.mul(r, DG)),
                'xDG_eq_DT': J.norm(J.mul(x, DG)) == DT,
                'word_curve_same_f': [v % p for v in d['h_f']] == [v % p for v in d['jac_f']],
                'word_divisors_equal': J.norm(D(d['hg'])) == DG and J.norm(D(d['ht'])) == DT}
        bad = [k for k, v in jchk.items() if v is not True]
        if bad:
            errs.append(['v3 Jacobian check failed', p, bad])
        chk.update(jchk)
        for run in d['runs']:
            g = 'E' if run['group'] == 'E1' else 'J'
            cert_run(lab, run, x, r, g, E, G, T, J, DG, DT)
        # tie to remeasure_v3.json
        row = [z for z in NR['v3']['rows'] if z['p'] == p][0]
        by = lambda a: [z for z in d['runs'] if z['arm'] == a]
        cmp(f'{lab} candidate', d['candidate'], row['candidate'])
        cmp(f'{lab} old rho ops', by('old_rho')[0]['ops'], row['old_plain_rho_ops'])
        cmp(f'{lab} recorded ops', by('recorded')[0]['ops'], row['arms']['recorded']['group_ops'])
        cmp(f'{lab} recorded la', by('recorded')[0]['la'], row['arms']['recorded']['native']['mul_r'])
        cmp(f'{lab} L la', by('L')[0]['la'], row['arms']['L']['native']['mul_r'])
        for a in 'ABC':
            rs = by(a)
            cmp(f'{lab} {a} ops', mean(rs, lambda z: z['ops']), row['arms'][a]['group_ops'])
            cmp(f'{lab} {a} mul_p', mean(rs, lambda z: z['rel_mul']), row['arms'][a]['native']['mul_p'])
            cmp(f'{lab} {a} mul_r', mean(rs, lambda z: z['la_mul']), row['arms'][a]['native']['mul_r'])
        for a, k in (('rho_neg', 'negation'), ('rho_plain', 'plain')):
            rs = by(a)
            cmp(f'{lab} {a} total', mean(rs, lambda z: z['ops']), row['rho'][k]['total_ops'])
            cmp(f'{lab} {a} native', mean(rs, lambda z: z['native']), row['rho'][k]['native'])
            if len(rs) != row['rho'][k]['runs']:
                errs.append(['rho run count', lab, a, len(rs), row['rho'][k]['runs']])
        if ji['j'] != row['rho']['automorphism_fold']['j_invariant']:
            errs.append(['j differs', lab, ji['j'], row['rho']['automorphism_fold']['j_invariant']])
        per.append({'target': tgt, 'p': p, 'r': r, 'j': ji['j'], 'checks': chk})
    elif tgt == 'cover_pushforward':
        r = d['r']
        E = EC(p, *d['e1'])
        G = P(d['gen'])
        lab = f'cover p{p}'
        o = [z for z in CP['rows'] if z['p'] == p][0]
        if list(o['coeffs']) != list(d['c']) or o['candidate'] != d['candidate'] or o['seed0'] != d['seed0'] or o['r'] != r:
            errs.append(['cover instance differs from pushforward.json', p])
        x0, E1 = bielliptic_checks(lab, p, d['c'], d['x0'], d['e1'], d['jac_f'], G)
        J = Jac2(p, d['jac_f'])
        DG = J.norm(pull(p, d['c'], x0, G, E1, J, G))
        n = E.count()
        chk = {'p_prime': is_prime(p), 'r_prime': is_prime(r), 'G_on_E': E.on(G), 'rG_zero': E.mul(r, G) is None,
               'n_e1_count': n == d['e1_order'], 'r_exactly_divides': n % r == 0 and n % (r * r) != 0,
               'rDG_zero': J.is_id(J.mul(r, DG)), 'word_curve_same_f': [v % p for v in d['h_f']] == [v % p for v in d['jac_f']]}
        ji = jinfo(p, E.a2, E.a4, E.a6)
        chk['no_extra_automorphism'] = not ji['extra_automorphism']
        row = [z for z in NR['cover_pushforward']['rows'] if z['p'] == p][0]
        arm_ops = {}
        for sd in d['seeds']:
            seed = sd['seed']
            x = ((seed * 2246822519) & U64) + 7
            x = (x & U64) % r
            if x != sd['x']:
                errs.append(['cover x formula', p, sd['k'], x, sd['x']])
            T = E.mul(x, G)
            if T != P(sd['target_pt']):
                errs.append(['cover T != [x]G', p, sd['k']])
            DT = J.norm(pull(p, d['c'], x0, T, E1, J, G))
            if J.norm(D(sd['dg'])) != DG or J.norm(D(sd['dt'])) != DT:
                errs.append(['cover pull-back differs', p, sd['k']])
            if J.norm(J.mul(x, DG)) != DT:
                errs.append(['cover [x]DG != DT', p, sd['k']])
            for run in sd['runs']:
                g = 'E' if run['group'] == 'E1' else 'J'
                cert_run(lab + f' s{sd["k"]}', run, x, r, g, E, G, T, J, DG, DT)
                arm_ops.setdefault(run['arm'], []).append(run)
            # tie per-seed values to pushforward.json
            os_ = o['seeds'][sd['k']]
            for mine, theirs in (('rho_e1_recorded', 'rho_e1'), ('ic_jc', 'ic_jc'), ('ic_e1_fibre_full', 'ic_e1_fibre_full'),
                                 ('ic_e1_two_term', 'ic_e1_two_term'), ('ic_jc_first_collision', 'ic_jc_first_collision'),
                                 ('ic_e1_fibre_first_collision', 'ic_e1_fibre_first_collision'),
                                 ('birthday_e1_first_collision', 'birthday_e1_first_collision')):
                m = [z for z in sd['runs'] if z['arm'] == mine]
                if theirs in os_['arms'] and m:
                    cmp(f'{lab} s{sd["k"]} {mine} ops vs pushforward.json', m[0]['ops'], os_['arms'][theirs]['group_ops'])
        cmp(f'{lab} old rho per seed', sum(z['ops'] for z in arm_ops['rho_e1_recorded']), sum(row['old_plain_rho_ops_per_seed']))
        for name in ['ic_jc', 'ic_jc_L', 'ic_e1_fibre_full', 'ic_e1_two_term', 'ic_jc_first_collision',
                     'ic_e1_fibre_first_collision', 'birthday_e1_first_collision']:
            rs = arm_ops.get(name, [])
            if name in row['arms']:
                cmp(f'{lab} {name} ops', mean(rs, lambda z: z['ops']), row['arms'][name]['group_ops'])
                if 'la' in (rs[0] if rs else {}):
                    cmp(f'{lab} {name} la', mean(rs, lambda z: z['la']), row['arms'][name]['native'].get('mul_r', 0.0))
                if 'lookups' in (rs[0] if rs else {}):
                    cmp(f'{lab} {name} lookups', mean(rs, lambda z: z['lookups']), row['arms'][name]['native'].get('lookup', 0.0))
        i0 = d['inst0']
        x = i0['x']
        T = E.mul(x, G)
        DT = J.norm(pull(p, d['c'], x0, T, E1, J, G))
        if T != P(i0['target_pt']) or J.norm(D(i0['dg'])) != DG or J.norm(D(i0['dt'])) != DT \
                or J.norm(D(i0['hg'])) != DG or J.norm(D(i0['ht'])) != DT:
            errs.append(['cover inst0 data', p])
        if x != d['seeds'][0]['x']:
            errs.append(['cover inst0 is not seed 0', p])
        for run in i0['runs']:
            g = 'E' if run['group'] == 'E1' else 'J'
            cert_run(lab + ' inst0', run, x, r, g, E, G, T, J, DG, DT)
        by = lambda a: [z for z in i0['runs'] if z['arm'] == a]
        for a in 'ABC':
            rs = by(a)
            cmp(f'{lab} {a} ops', mean(rs, lambda z: z['ops']), row['arms'][a]['group_ops'])
            cmp(f'{lab} {a} mul_p', mean(rs, lambda z: z['rel_mul']), row['arms'][a]['native']['mul_p'])
            cmp(f'{lab} {a} mul_r', mean(rs, lambda z: z['la_mul']), row['arms'][a]['native']['mul_r'])
        for a, k in (('rho_neg', 'negation'), ('rho_plain', 'plain')):
            rs = by(a)
            cmp(f'{lab} {a} total', mean(rs, lambda z: z['ops']), row['rho'][k]['total_ops'])
            cmp(f'{lab} {a} native', mean(rs, lambda z: z['native']), row['rho'][k]['native'])
        if ji['j'] != row['rho']['automorphism_fold']['j_invariant']:
            errs.append(['j differs', lab])
        bad = [k for k, v in chk.items() if v is not True]
        if bad:
            errs.append(['cover instance check failed', p, bad])
        per.append({'target': tgt, 'p': p, 'r': r, 'j': ji['j'], 'checks': chk})
    elif tgt == 'tower_large':
        r, x, inst = d['r'], d['x'], d['instance']
        E = EC(p, *d['e'])
        G, T = P(d['gen']), P(d['target_pt'])
        lab = f'tower p{p} i{inst}'
        chk, ji = check_point_data(lab, p, r, x, E, G, T, d['n_e'])
        chk['j_matches_dump'] = ji['j'] == d['j_invariant']
        rec = [z for z in TL['instances'] if z['p'] == p and z['instance'] == inst]
        if rec:
            rec = rec[0]
            chk['matches_tower_large_json'] = (rec['r'] == r and rec['n_e'] == d['n_e'] and rec['seed'] == d['seed']
                                               and rec['jac_order'] == [g['jac_order'] for g in d['rungs']])
        for g in d['rungs']:
            gg = g['genus']
            J = JacG(p, g['f'], gg)
            DG, DT = J.norm(D(g['d'])), J.norm(D(g['t']))
            N = g['jac_order']
            chk[f'g{gg}_valid'] = J.valid(DG) and J.valid(DT)
            chk[f'g{gg}_rD_zero'] = J.is_id(J.mul(r, DG)) and not J.is_id(DG)
            chk[f'g{gg}_xD_eq_T'] = J.norm(J.mul(x, DG)) == DT
            chk[f'g{gg}_r_divides_order'] = N % r == 0 and (N // r) % r != 0
            chk[f'g{gg}_order_in_HW_window'] = (p ** 0.5 - 1) ** (2 * gg) <= N <= (p ** 0.5 + 1) ** (2 * gg)
            for run in g['runs']:
                if 'error' in run:
                    errs.append(['tower solver error', lab, gg, run])
                    continue
                cert_run(lab + f' g{gg}', run, x, r, 'J', J=J, DG=DG, DT=DT)
        for run in d['rho']:
            cert_run(lab, run, x, r, 'E', E, G, T)
        bad = [k for k, v in chk.items() if v is not True]
        if bad:
            errs.append(['tower instance check failed', lab, bad])
        per.append({'target': tgt, 'p': p, 'instance': inst, 'r': r, 'j': ji['j'], 'checks': chk, '_d': d})

# tower means per prime vs remeasure
tw = {}
for z in per:
    if z['target'] == 'tower_large':
        tw.setdefault(z['p'], []).append(z.pop('_d'))
for p, ds in tw.items():
    row = [z for z in NR['tower_large']['rows'] if z['p'] == p][0]
    lab = f'tower p{p}'
    for a, k in (('rho_neg', 'negation'), ('rho_plain', 'plain')):
        rs = [z for d in ds for z in d['rho'] if z['arm'] == a]
        cmp(f'{lab} {a} total', mean(rs, lambda z: z['ops']), row['rho'][k]['total_ops'])
        cmp(f'{lab} {a} native', mean(rs, lambda z: z['native']), row['rho'][k]['native'])
        if len(rs) != row['rho'][k]['runs']:
            errs.append(['rho run count', lab, a, len(rs), row['rho'][k]['runs']])
    om = sum(mean([z for z in d['rho'] if z['arm'] == 'old_rho'], lambda z: z['ops']) for d in ds) / len(ds)
    cmp(f'{lab} old rho mean', om, row['old_plain_rho_ops_mean'])
    for gi, gg in enumerate([2, 3]):
        for a in ['recorded', 'A', 'B', 'C']:
            rs = [z for d in ds for z in d['rungs'][gi]['runs'] if z.get('arm') == a and 'error' not in z]
            key = f'g{gg}_{a}'
            if key in row['arms'] and rs:
                cmp(f'{lab} {key} ops', mean(rs, lambda z: z['ops']), row['arms'][key]['group_ops'])
                if a == 'recorded':
                    cmp(f'{lab} {key} la', mean(rs, lambda z: z['la_mulmods']), row['arms'][key]['la_mulmods'])
                else:
                    cmp(f'{lab} {key} mul_p', mean(rs, lambda z: z['rel_mul']), row['arms'][key]['native']['mul_p'])
                    cmp(f'{lab} {key} mul_r', mean(rs, lambda z: z['la_mul']), row['arms'][key]['native']['mul_r'])
    cc = {c['inst']: c['j_invariant'] for c in row.get('curves_checked_all', [])}
    for d in ds:
        if cc.get(d['instance']) != d['j_invariant']:
            errs.append(['tower j differs from curves_checked_all', p, d['instance']])

res = {'stats': stats, 'errors': errs, 'notes': notes, 'per_instance': per}
json.dump(res, open(sys.argv[2], 'w'), indent=1)
print(f"instances {stats['instances']}; logs {stats['logs']}, certified {stats['logs_certified']}; "
      f"means compared {stats['means_compared']}; errors {len(errs)}")
for e in errs[:30]:
    print('  ', e)
