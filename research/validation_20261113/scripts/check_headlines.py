#!/usr/bin/env python3
"""Recompute RESULTS.md headline numbers and the N1-N9 scorecard from remeasure.json
with the validator's own code."""
import json, sys
R = '/home/user/crypto/research/negation_remeasure_20261113'
D = json.load(open(f'{R}/remeasure.json'))['targets']
SC = json.load(open(f'{R}/scorecard.json'))
BAND = [0.0, 0.1, 1.0]
H = {}
def row(t, p): return [r for r in D[t]['rows'] if r['p'] == p][0]
def negband(r):
    m = r['rho']['negation']
    return [m['total_ops'] + w * m['native'] for w in BAND]
def tot(a): return [a['group_ops'] + w * sum(a['native'].values()) for w in BAND]
def rat(a, b): return [x / y for x, y in zip(a, b)]
# headline: old vs new distances at the largest prime
r = row('v3', 1619); A = r['arms']['recorded']
H['v3_p1619_recorded_old'] = A['recorded_ratio_to_old_plain_rho']
H['v3_p1619_recorded_new'] = rat(tot(A), negband(r))
r = row('cover_pushforward', 12841); A = r['arms']['ic_jc']
H['cover_p12841_icjc_old'] = A['recorded_median_ratio_to_old_plain_rho']
H['cover_p12841_icjc_new'] = rat(tot(A), negband(r))
r = row('tower_large', 64007); A = r['arms']['g3_recorded']
H['tower_p64007_g3_old'] = A['recorded_ratio_to_old_plain_rho_la_priced']
H['tower_p64007_g3_new'] = rat(tot(A), negband(r))
# N7
r = row('v3', 1619)
H['v3_p1619_C_ops'] = r['arms']['C']['group_ops']
H['v3_p1619_C_totals'] = tot(r['arms']['C'])
H['v3_p1619_recorded_totals'] = tot(r['arms']['recorded'])
H['v3_p1619_C_over_neg'] = rat(tot(r['arms']['C']), negband(r))
r = row('tower_large', 16001)
H['tower_p16001_g3C_totals'] = tot(r['arms']['g3_C']); H['tower_p16001_g3rec_totals'] = tot(r['arms']['g3_recorded'])
r = row('cover_pushforward', 12841)
H['cover_p12841_C_totals'] = tot(r['arms']['C']); H['cover_p12841_icjc_totals'] = tot(r['arms']['ic_jc'])
# class changes
changes = []; recorded_rows = 0; below = []
for t, T in D.items():
    for rr in T['rows']:
        for name, a in rr['arms'].items():
            nb = negband(rr)
            tb = tot(a)
            above = all(x > y for x, y in zip(tb, nb)); bel = all(x < y for x, y in zip(tb, nb))
            if bel: below.append((t, rr['p'], name))
            if 'recorded_class' in a:
                recorded_rows += 1
                if a['recorded_class'] != a['class_neg_band']:
                    changes.append((t, rr['p'], name, a['recorded_class'], a['class_neg_band'],
                                    a.get('recorded_ratio_to_old_plain_rho', a.get('recorded_median_ratio_to_old_plain_rho', a.get('recorded_ratio_to_old_plain_rho_la_priced')))))
H['recorded_rows'] = recorded_rows; H['class_changes'] = changes; H['below_rows_negband'] = below
fc = [c for c in changes if c[0] == 'cover_pushforward']
H['cover_changed_old_ratio_range'] = [min(min(c[5]) for c in fc), max(max(c[5]) for c in fc)]
# rho: old plain / negation total ratio ranges
def rng(t):
    v = []
    for rr in D[t]['rows']:
        old = rr.get('old_plain_rho_ops', rr.get('old_plain_rho_ops_mean'))
        v.append(old / rr['rho']['negation']['total_ops'])
    return [min(v), max(v)]
H['old_over_neg_v3'] = rng('v3'); H['old_over_neg_cover'] = rng('cover_pushforward'); H['old_over_neg_tower'] = rng('tower_large')
# w_cal ranges
wc = {'mulp_g2_word': [], 'mulp_g3': [], 'canon_E': [], 'hash_E': []}
for t, T in D.items():
    for rr in T['rows']:
        ct, ta = rr['class_times_ns'], rr['t_add_ns']
        e = ta.get('E1', ta.get('E'))
        wc['canon_E'].append(ct['canon'] / e); wc['hash_E'].append(ct['hash'] / e)
        if t == 'tower_large':
            wc['mulp_g2_word'].append(ct['mul_p'] / ta['J_g2']); wc['mulp_g3'].append(ct['mul_p'] / ta['J_g3'])
        else:
            wc['mulp_g2_word'].append(ct['mul_p'] / ta['J_word_impl'])
H['w_cal_ranges'] = {k: [min(v), max(v)] for k, v in wc.items()}
# N2: native/step, four classes (canon+hash+lookup+insert)/steps
n2 = {'neg4': [], 'plain4': [], 'neg_all': []}
n3 = {'w1_over_w0_all': [], 'w1_over_w0_4': []}
for t, T in D.items():
    for rr in T['rows']:
        for k, key in (('negation', 'neg4'), ('plain', 'plain4')):
            m = rr['rho'][k]
            four = m['canonicalisations'] + m['partition_hashes'] + m['dp_lookups'] + m['dp_inserts']
            n2[key].append(four / m['steps'])
        m = rr['rho']['negation']
        n2['neg_all'].append(m['native'] / m['steps'])
        if t in ('v3', 'tower_large'):
            four = m['canonicalisations'] + m['partition_hashes'] + m['dp_lookups'] + m['dp_inserts']
            n3['w1_over_w0_all'].append((m['total_ops'] + m['native']) / m['total_ops'])
            n3['w1_over_w0_4'].append((m['total_ops'] + four) / m['total_ops'])
H['N2'] = {k: [min(v), max(v), sum(1 for x in v if not ((1.8 <= x <= 3.5) if k.startswith('neg') else (0.9 <= x <= 1.6)))] for k, v in n2.items()}
H['N3'] = {k: [min(v), max(v)] for k, v in n3.items()}
# N9 Spearman of arm C w0 distance vs p, per genus
def spearman(xs, ys):
    def rk(v):
        s = sorted(range(len(v)), key=lambda i: v[i]); r = [0] * len(v)
        for i, j in enumerate(s): r[j] = i
        return r
    a, b = rk(xs), rk(ys); n = len(xs)
    d2 = sum((x - y) ** 2 for x, y in zip(a, b))
    return 1 - 6 * d2 / (n * (n * n - 1))
for g in (2, 3):
    ps = [rr['p'] for rr in D['tower_large']['rows']]
    ds = [rr['arms'][f'g{g}_C']['ratio_neg_band'][0] for rr in D['tower_large']['rows']]
    H[f'N9_g{g}_spearman'] = spearman(ps, ds)
# N5 band vs cal disagreements
dis = []
for t, T in D.items():
    for rr in T['rows']:
        for name, a in rr['arms'].items():
            if a['class_neg_band'] != a['class_neg_cal']:
                dis.append((t, rr['p'], name, a['class_neg_band'], a['class_neg_cal']))
H['N5_disagreements'] = dis
# curves checked
cc = sum(len(rr.get('curves_checked_all', [])) for rr in D['tower_large']['rows']) + len(D['v3']['rows']) + len(D['cover_pushforward']['rows'])
H['curves_checked'] = cc
H['tower_curves_extra_aut'] = [c for rr in D['tower_large']['rows'] for c in rr.get('curves_checked_all', []) if c['automorphism_beyond_negation']]
json.dump(H, open(sys.argv[1], 'w'), indent=1, default=str)
for k, v in H.items():
    print(k, json.dumps(v, default=str)[:400])
