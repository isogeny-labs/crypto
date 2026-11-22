#!/usr/bin/env python3
"""Aggregate results/*.jsonl into RESULTS.json and RESULTS.md. Pure python."""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__))
rd = lambda n: [json.loads(l) for l in open(os.path.join(here, 'results', n)) if l.strip()]

def wilson(k, n, z=1.959963984540054):
    ph = k / n; den = 1 + z * z / n
    c = (ph + z * z / (2 * n)) / den
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / den
    return [c - h, c + h]

one = rd('onestep.jsonl')
pooled = {}
for p in sorted({r['p'] for r in one}):
    rs = [r for r in one if r['p'] == p]
    k = sum(r['full_success'] for r in rs); n = sum(r['n'] for r in rs)
    pooled[p] = {'k': k, 'n': n, 'rate': k / n, 'wilson95': wilson(k, n),
                 'per_seed': [{'seed': r['seed'], 'rate': r['rate'], 'wilson95': r['wilson95']} for r in rs]}
k = sum(v['k'] for v in pooled.values()); n = sum(v['n'] for v in pooled.values())
allp = {'k': k, 'n': n, 'rate': k / n, 'wilson95': wilson(k, n)}

mt_pre = rd('modeltest_d2to5_preregistered.jsonl')
mt_ext = rd('modeltest_extended_exploratory.jsonl')
mt = {
  'preregistered_d2_to_d5': {'instances': len(mt_pre),
      'instances_with_nonzero_kernel_d_le_5': sum(1 for r in mt_pre if r['min_degree'] is not None and r['min_degree'] <= 5),
      'min_points': min(r['n_points'] for r in mt_pre), 'max_points': max(r['n_points'] for r in mt_pre),
      'image_errors_total': sum(r['image_errors'] for r in mt_pre),
      'rational_pair_fraction': sum(r['rational_pairs'] for r in mt_pre) / sum(r['pairs'] for r in mt_pre)},
  'extended_exploratory_p101': {'instances': len(mt_ext),
      'min_degree_counts': {str(d): sum(1 for r in mt_ext if r['min_degree'] == d) for d in sorted({r['min_degree'] for r in mt_ext})},
      'kernel_dim_d2_to_d12_example': mt_ext[0]['kernel_dim_d2_to_d12']},
}

import statistics
canon = rd('canon.jsonl'); pf = rd('pushfwd.jsonl'); dl = rd('dlp.jsonl')
cn = {'instances': len(canon),
      'kernel_dims_d2_to_d6_counts': {str(k): sum(1 for r in canon if r['kernel_dim_d2_to_d6'] == list(k)) for k in sorted({tuple(r['kernel_dim_d2_to_d6']) for r in canon})},
      'by_p': {str(p): {'instances': sum(1 for r in canon if r['p'] == p),
                        'unique_quartic': sum(1 for r in canon if r['p'] == p and r['kernel_dim_d2_to_d6'][:3] == [0, 0, 1]),
                        'on_conic': sum(1 for r in canon if r['p'] == p and r['kernel_dim_d2_to_d6'][0] > 0)} for p in sorted({r['p'] for r in canon})},
      'min_points': min(r['n_points'] for r in canon), 'image_errors': sum(r['image_errors'] for r in canon), 'chart_skipped_points': sum(r['skipped'] for r in canon)}
pfo = [r for r in pf if 'quartic_f' in r]
pushd = {'instances': len(pf), 'with_smooth_quartic': len(pfo), 'no_quartic_conic_image': len(pf) - len(pfo),
         'pairs_tested': sum(r['pairs_tested'] for r in pfo), 'phi_none': sum(r['phi_failures'] for r in pfo),
         'library_panics_caught': sum(r['cantor_panics'] for r in pfo),
         'instances_with_tests': sum(1 for r in pfo if r['pairs_tested'] > 0),
         'instances_single_value': sum(1 for r in pfo if r['distinct_phiP_plus_phiiotaP'] == 1),
         'instances_multi_value': sum(1 for r in pfo if r['distinct_phiP_plus_phiiotaP'] > 1),
         'negative_control_instances_multivalued': sum(1 for r in pfo if r['negctl_distinct_sums'] > 1),
         'negative_control_instances_tested': sum(1 for r in pfo if r['negctl_pairs'] > 1)}
okd = [r for r in dl if r['status'] == 'ok']
PI = 3.141592653589793
dlp = {'by_p': {}}
for p in sorted({r['p'] for r in okd}):
    q = [r for r in okd if r['p'] == p]
    tg = sum(r['targets'] for r in q); fc = sum(r['logs_found_on_C'] for r in q)
    ver = sum(r['pullback_verified_on_H'] for r in q); nv = sum(r['found_on_C_but_not_verified_on_H'] for r in q)
    t8 = sum(r['unverified_with_difference_in_8_torsion'] for r in q)
    pr = [(r['dlog_phase_fq_total']['mul'] + r['dlog_phase_fq_total']['inv']) / r['targets'] for r in q]
    prob = [r['dlog_phase_fq_total']['probes'] / max(r['logs_found_on_C'], 1) for r in q]
    po = [r['order_phase_fq']['mul'] + r['order_phase_fq']['inv'] for r in q]
    rho_steps = [(PI * r['ord_GH'] / 4) ** 0.5 for r in q]
    rho_pr = [st * r['cantor_field_ops_per_add_H'] for st, r in zip(rho_steps, q)]
    PHI_NATIVE = 800  # estimate: 2 image points x ~400 F_p ops in canon_map/F_{p^2} (not instrumented)
    row = {'instances': len(q), 'targets': tg, 'logs_found_on_C': fc, 'verified_on_H': ver, 'found_not_verified': nv,
           'not_verified_with_difference_in_8_torsion': t8, 'ordH_over_ordC_counts': {str(e): sum(1 for r in q if r['ord_GH'] // r['ord_GC'] == e) for e in sorted({r['ord_GH'] // r['ord_GC'] for r in q})},
           'median_ord_GH': statistics.median(r['ord_GH'] for r in q),
           'ic_priced_fp_ops_per_target_median': statistics.median(pr), 'ic_priced_range': [min(pr), max(pr)],
           'ic_native_probes_per_target_median': statistics.median(prob), 'phi_native_estimate': PHI_NATIVE,
           'ic_order_phase_priced_median': statistics.median(po),
           'rho_steps_median': statistics.median(rho_steps), 'rho_priced_median': statistics.median(rho_pr), 'rho_priced_range': [min(rho_pr), max(rho_pr)],
           'cantor_field_ops_per_add_H_median': statistics.median(r['cantor_field_ops_per_add_H'] for r in q), 'S_over_rho': {}}
    ratios = {}
    for w in (0, 0.1, 1):
        rs = []
        for r, a, b, c_, d_ in zip(q, pr, prob, rho_pr, rho_steps):
            rs.append((a + w * (b + PHI_NATIVE)) / (c_ + w * d_))
        ratios[str(w)] = {'median': statistics.median(rs), 'min': min(rs), 'max': max(rs)}
    row['S_over_rho'] = ratios
    dlp['by_p'][str(p)] = row
import math
ps = sorted(int(k) for k in dlp['by_p'])
def slope(key):
    xs = [math.log(p) for p in ps]; ys = [math.log(dlp['by_p'][str(p)][key]) for p in ps]
    mx = sum(xs) / len(xs); my = sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
dlp['exponent_fit_log_log_over_p'] = {'ic_priced_per_target': slope('ic_priced_fp_ops_per_target_median'), 'rho_priced': slope('rho_priced_median'),
    'note': 'slope of ln(cost) vs ln(p) over p = 101, 211, 521 (three points, no interval); BSGS on C expects 1.5, rho on a group of size p^3 expects 1.5'}

cov = rd('coverage.jsonl')
res = {'status': '[pcc:unverified] local development checks', 'git_head_at_freeze': '3c24520f7cdc1c47bac26cc7778d058e655b79a8',
       'onestep_pooled_by_p': pooled, 'onestep_all': allp, 'modeltest': mt, 'coverage': cov,
       'canon_S2b': cn, 'pushforward_S3a': pushd, 'dlp_S3b': dlp,
       'rho_reference': 'analytic expectation sqrt(pi N / 4) group ops (negation map) x measured Cantor field ops per add on H; NOT a measured rho run',
       'S_over_rho_summary': {p: v['S_over_rho'] for p, v in dlp['by_p'].items()}}
json.dump(res, open(os.path.join(here, 'RESULTS.json'), 'w'), indent=1)

L = ['# RESULTS (all `[pcc:unverified]`)', '', '## S1a one-step Smith full_success, random octics, n = 3000 per seed', '',
     '| p | seed 1 | seed 2 | seed 3 | pooled | Wilson 95% |', '|---|---|---|---|---|---|']
for p, v in pooled.items():
    s = ' | '.join('%.4f' % x['rate'] for x in v['per_seed'])
    L.append('| %d | %s | %.4f | [%.4f, %.4f] |' % (p, s, v['rate'], *v['wilson95']))
L.append('| all | | | | %.4f | [%.4f, %.4f] |' % (allp['rate'], *allp['wilson95']))
L += ['', 'Prediction P1 band [0.14, 0.24]; point prediction 0.186.', '',
      '## S1b exhaustive class-level ceilings (existence of a Frobenius-stable kernel, no isogeny constructed)', '',
      '| p | curves (with twists) | direct pair kernel | direct any Lagrangian | class contains pair-kernel curve | class any Lagrangian |', '|---|---|---|---|---|---|']
for c in cov:
    f = c['frac']
    L.append('| %d | %d | %.4f | %.4f | %.4f | %.4f |' % (c['p'], c['curves'], f['direct_pair'], f['direct_any'], f['class_pair'], f['class_any']))
L += ['', '## S2 projection of X by the Mumford coefficients (b0 : b1 : b2)', '',
      '- preregistered cells (d = 2..5): %d instances (p = 37, 101; 3 seeds x 10), %d to %d points each; nonzero kernel at d <= 5 in %d instances.' % (
          mt['preregistered_d2_to_d5']['instances'], mt['preregistered_d2_to_d5']['min_points'], mt['preregistered_d2_to_d5']['max_points'],
          mt['preregistered_d2_to_d5']['instances_with_nonzero_kernel_d_le_5']),
      '- exploratory extension (p = 101, d up to 12): minimal degree counts %s; example kernel dims d=2..12: %s.' % (
          mt['extended_exploratory_p101']['min_degree_counts'], mt['extended_exploratory_p101']['kernel_dim_d2_to_d12_example']),
      '- fraction of H(F_p) points with an F_p-rational R-pair: %.3f; image errors: %d.' % (
          mt['preregistered_d2_to_d5']['rational_pair_fraction'], mt['preregistered_d2_to_d5']['image_errors_total']),
      '', 'p = 11 exhaustive coverage: started, killed unfinished after 28 CPU-minutes (not a result). Cited, not re-measured (research/smith_iso_granularity_20261015): D / C / U = 0.1758 / 0.4304 / 0.4897 at p = 7 and 0.1805 / 0.475 / 0.4943 at p = 11.',
      '']
open(os.path.join(here, 'RESULTS.md'), 'w').write('\n'.join(L) + '\n')

L += ['', '## S2b canonical map by the trace of the H differentials (amendment A1)', '',
      '%d instances (p = 37, 101; 3 seeds x 10): kernel-dim vectors (d = 2..6) and counts: %s' % (cn['instances'], cn['kernel_dims_d2_to_d6_counts']),
      'per p: %s; min points %d, image errors %d, chart-skipped points %d. [0,0,1,3,6] = unique plane quartic; [1,3,6,10,15] = image on a conic (X hyperelliptic).' % (cn['by_p'], cn['min_points'], cn['image_errors'], cn['chart_skipped_points']),
      '', '## S3a push-forward into Pic^0 of the quartic (amendment A1, P7)', '',
      '%d instances: %d with a smooth canonical quartic (PlaneQuartic::new smoothness certificate), %d on a conic (no quartic).' % (pushd['instances'], pushd['with_smooth_quartic'], pushd['no_quartic_conic_image']),
      'Principal-divisor test Phi(P)+Phi(iota P) independent of x: %d pairs tested, instances with tests %d, single value in %d, several values in %d; Phi None %d; library panics caught %d (plane_quartic_pic `complete` limit, counted as not tested).' % (
          pushd['pairs_tested'], pushd['instances_with_tests'], pushd['instances_single_value'], pushd['instances_multi_value'], pushd['phi_none'], pushd['library_panics_caught']),
      'Negative control (Phi(P)+Phi(P\') with P, P\' not related by iota): several values in %d of %d instances.' % (pushd['negative_control_instances_multivalued'], pushd['negative_control_instances_tested']),
      '', '## S3b/S4 toy DLP: BSGS on Pic^0(C), log pulled back and checked on H (amendment A2, A2b)', '',
      '| p | inst | targets | log found on C | verified on H | found, not verified | of which diff in 8-torsion | ord(G_H)/ord(G_C) |', '|---|---|---|---|---|---|---|---|']
for p, v in dlp['by_p'].items():
    tail_row = '| %s | %d | %d | %d | %d | %d | %d | %s |' % (p, v['instances'], v['targets'], v['logs_found_on_C'], v['verified_on_H'], v['found_not_verified'], v['not_verified_with_difference_in_8_torsion'], v['ordH_over_ordC_counts'])
    L.append(tail_row)
L += ['', 'Cost per target (median over instances, F_p mul+inv counted by FqCounter) and matched-rho reference (analytic, see PLAN A2), S/rho median [min, max] at native price w = 0, 0.1, 1 per unpriced op:', '',
      '| p | median ord(G_H) | IC-route priced ops/target | rho priced ops (analytic steps x Cantor field ops) | S/rho w=0 | w=0.1 | w=1 |', '|---|---|---|---|---|---|---|']
for p, v in dlp['by_p'].items():
    r = v['S_over_rho']
    L.append('| %s | %d | %.3g | %.3g | %.1f [%.1f, %.1f] | %.1f [%.1f, %.1f] | %.1f [%.1f, %.1f] |' % (p, v['median_ord_GH'], v['ic_priced_fp_ops_per_target_median'], v['rho_priced_median'],
        r['0']['median'], r['0']['min'], r['0']['max'], r['0.1']['median'], r['0.1']['min'], r['0.1']['max'], r['1']['median'], r['1']['min'], r['1']['max']))
L += ['', 'Log-log exponent fit over p (3 points, no interval): %s' % dlp['exponent_fit_log_log_over_p']]
open(os.path.join(here, 'RESULTS.md'), 'w').write('\n'.join(L) + '\n')
print('\n'.join(L))
